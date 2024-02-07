use std::{fmt, process::exit};

#[derive(Debug)]
pub enum Errcode {
    ContainerError(u8),
    ChildProcessError(u8),
    NotSupported(u8),
    SocketError(u8),
    ArgumentInvalid(&'static str),
    HostnameError(u8),
    RngError,
}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => {
                write!(f, "ArgumentInvalid:{}", element)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    // If it's a success , return ()

    match res {
        Ok(_) => {
            log::debug!("Exit without any error , returning 0");
            exit(0)
        }
        // If there's an error , print an error message and return the code
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{} Returning {}", e, retcode);
            exit(retcode)
        }
    }
}
