use crate::errors::Errcode;
use crate::config::ContainerOpts;

use nix::unistd::Pid;
use nix::sched::clone;
use nix::sys::signal::Signal;
use nix::sched::CloneFlags;

const STACK_SIZE: usize = 1024 * 1024;

fn child(config: ContainerOpts) -> isize {
    log::info!("Starting container with command {} and args {:?}", config.path.to_str().unwrap(), config.argv);
    0
}

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);

    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32)
    )
    {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Errcode::ChildProcessError(0))
    }
}
