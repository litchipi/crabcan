use crate::errors::Errcode;

use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::hierarchies::V2;
use cgroups_rs::{MaxValue, CgroupPid};
use rlimit::{setrlimit, Resource};
use nix::unistd::Pid;

use std::fs::{canonicalize, remove_dir};
use std::convert::TryInto;

//                      K       M       G
const KMEM_LIMIT: i64 = 1024 * 1024 * 1024;
const MEM_LIMIT: i64 = KMEM_LIMIT;
const MAX_PID: MaxValue = MaxValue::Value(64);
const NOFILE_RLIMIT: u64 = 64;

pub fn restrict_resources(hostname: &String, pid: Pid) -> Result<(), Errcode>{
    log::debug!("Restricting resources for hostname {}", hostname);

    let cgs = CgroupBuilder::new(hostname)
        .cpu().shares(256).done()
        .memory().kernel_memory_limit(KMEM_LIMIT).memory_hard_limit(MEM_LIMIT).done()
        .pid().maximum_number_of_processes(MAX_PID).done()
        .blkio().weight(50).done()
        .build(Box::new(V2::new()));

    let pid : u64 = pid.as_raw().try_into().unwrap();
    if let Err(_) = cgs.add_task(CgroupPid::from(pid)) {
        return Err(Errcode::ResourcesError(0));
    };

    if let Err(_) = setrlimit(Resource::NOFILE, NOFILE_RLIMIT, NOFILE_RLIMIT){
        return Err(Errcode::ResourcesError(1));
    }

    Ok(())
}

pub fn clean_cgroups(hostname: &String) -> Result<(), Errcode>{
    log::debug!("Cleaning cgroups");
    match canonicalize(format!("/sys/fs/cgroup/{}/", hostname)){
        Ok(d) => {
            if let Err(_) = remove_dir(d) {
                return Err(Errcode::ResourcesError(2));
            }
        },
        Err(e) => {
            log::error!("Error while canonicalize path: {}", e);
            return Err(Errcode::ResourcesError(3));
        }
    }
    Ok(())
}
