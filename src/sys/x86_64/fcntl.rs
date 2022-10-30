use crate::{Fd, open};

pub enum Fcntl<'a> {
    DupFd(Fd<'a>),
    GetFd,
    SetFd(open::Flags),
    GetFl,
    SetFl(open::Flags)
}
impl<'a> Fcntl<'a> {
    pub fn cmd(&self) -> u32 {
        match self {
            Self::DupFd(_) => 0,
            Self::GetFd => 1,
            Self::SetFd(_) => 2,
            Self::GetFl => 3,
            Self::SetFl(_) => 4
        }
    }
    pub fn arg(&self) -> Option<usize> {
        match self {
            Self::DupFd(fd) => Some(fd.0 as _),
            Self::GetFd => None,
            Self::SetFd(flags) => Some(flags.0 as _),
            Self::GetFl => None,
            Self::SetFl(flags) => Some(flags.0 as _)
        }
    }
}