use crate::cli::Args;
use crate::errors::Errcode;
use crate::config::ContainerOpts;

use nix::sys::utsname::uname;
pub struct Container{
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let config = ContainerOpts::new(
                args.command,
                args.uid,
                args.mount_dir)?;
        Ok(Container {
            config,
        })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode>{
        log::debug!("Cleaning container");
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
    if let Err(e) = container.create(){
        container.clean_exit()?;
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    log::debug!("Finished, cleaning & exit");
    container.clean_exit()
}
