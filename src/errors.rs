use std::fmt;
use std::process::exit;

use crate::container::MINIMAL_KERNEL_VERSION;
// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Contains all possible errors in our tool
pub enum Errcode{
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    SocketError(u8),
    ContainerError(u8),
    ChildProcessError(u8),
    MountsError(u8),
    NamespacesError(u8),
    HostnameError(u8),
    CapabilitiesError(u8),
    SyscallsError(u8),
    RngError
}

impl Errcode{
    // Translate an Errcode::X into a number to return (the Unix way)
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}


#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum to be displayed by:
//      println!("{}", error);
//  in this case, it calls the function "fmt", which we define the behaviour below
impl fmt::Display for Errcode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self{

            Errcode::NotSupported(errtype) => {
                match errtype {
                    0 => write!(f, "Minimal kernel version required: {}", MINIMAL_KERNEL_VERSION),
                    1 => write!(f, "Only x86_64 architecture is supported"),
                    _ => write!(f, "{:?} (unknown id)", self),
                }
            },

            // Message to display when an argument is invalid
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),

            _ => write!(f, "{:?}", self) // For any variant not previously covered
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        },

        // If there's an error, print an error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
