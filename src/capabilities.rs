use crate::errors::Errcode;

use capctl::caps::Cap;
const CAPABILITIES_DROP: [Cap; 21] = [
    Cap::AUDIT_CONTROL,
    Cap::AUDIT_READ,
    Cap::AUDIT_WRITE,
    Cap::BLOCK_SUSPEND,
    Cap::DAC_READ_SEARCH,
    Cap::DAC_OVERRIDE,
    Cap::FSETID,
    Cap::IPC_LOCK,
    Cap::MAC_ADMIN,
    Cap::MAC_OVERRIDE,
    Cap::MKNOD,
    Cap::SETFCAP,
    Cap::SYSLOG,
    Cap::SYS_ADMIN,
    Cap::SYS_BOOT,
    Cap::SYS_MODULE,
    Cap::SYS_NICE,
    Cap::SYS_RAWIO,
    Cap::SYS_RESOURCE,
    Cap::SYS_TIME,
    Cap::WAKE_ALARM
];

use capctl::caps::FullCapState;

pub fn setcapabilities() -> Result<(), Errcode> {
    log::debug!("Clearing unwanted capabilities ...");
    if let Ok(mut caps) = FullCapState::get_current() {
        caps.bounding.drop_all(CAPABILITIES_DROP.iter().map(|&cap| cap));
        caps.inheritable.drop_all(CAPABILITIES_DROP.iter().map(|&cap| cap));
        Ok(())
    } else {
        Err(Errcode::CapabilitiesError(0))
    }
}
