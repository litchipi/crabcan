use crate::cli::Args;
use crate::errors::Errcode;
use crate::config::ContainerOpts;
use crate::child::generate_child_process;
use crate::namespaces::handle_child_uid_map;
use crate::resources::{restrict_resources, clean_cgroups};
use crate::mounts::clean_mounts;

use nix::unistd::Pid;
use nix::unistd::close;
use nix::sys::wait::waitpid;
use nix::sys::utsname::uname;

use std::os::unix::io::RawFd;
use std::path::PathBuf;

pub struct Container{
    sockets: (RawFd, RawFd),
    config: ContainerOpts,
    child_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let mut addpaths = vec![];
        for ap_pair in args.addpaths.iter(){
            let mut pair = ap_pair.to_str().unwrap().split(":");
            let frompath = PathBuf::from(pair.next().unwrap())
                .canonicalize().expect("Cannot canonicalize path")
                .to_path_buf();
            let mntpath = PathBuf::from(pair.next().unwrap())
                .strip_prefix("/").expect("Cannot strip prefix from path")
                .to_path_buf();
            addpaths.push((frompath, mntpath));
        }
        let (config, sockets) = ContainerOpts::new(
                args.command,
                args.uid,
                args.mount_dir,
                addpaths)?;

        Ok(Container {
            sockets,
            config,
            child_pid: None,
        })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        let pid = generate_child_process(self.config.clone())?;
        restrict_resources(&self.config.hostname, pid)?;
        handle_child_uid_map(pid, self.sockets.0)?;
        self.child_pid = Some(pid);
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode>{
        log::debug!("Cleaning container");
        clean_mounts(&self.config.mount_dir)?;

        if let Err(e) = close(self.sockets.0){
            log::error!("Unable to close write socket: {:?}", e);
            return Err(Errcode::SocketError(3));
        }

        if let Err(e) = close(self.sockets.1){
            log::error!("Unable to close read socket: {:?}", e);
            return Err(Errcode::SocketError(4));
        }

        if let Err(e) = clean_cgroups(&self.config.hostname){
            log::error!("Cgroups cleaning failed: {}", e);
            return Err(e);
        }

        Ok(())
    }
}


pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = uname();
    log::debug!("Linux release: {}", host.release());

    if let Ok(version) = scan_fmt!(host.release(), "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::NotSupported(0));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(Errcode::NotSupported(1));
    }

    Ok(())
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;
    log::debug!("Container sockets: ({}, {})", container.sockets.0, container.sockets.1);
    if let Err(e) = container.create(){
        container.clean_exit()?;
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    log::debug!("Container child PID: {:?}", container.child_pid);
    wait_child(container.child_pid)?;
    log::debug!("Finished, cleaning & exit");
    container.clean_exit()
}

pub fn wait_child(pid: Option<Pid>) -> Result<(), Errcode>{
    if let Some(child_pid) = pid {
        log::debug!("Waiting for child (pid {}) to finish", child_pid);
        if let Err(e) = waitpid(child_pid, None){
            log::error!("Error while waiting for pid to finish: {:?}", e);
            return Err(Errcode::ContainerError(1));
        }
    }
    Ok(())
}
