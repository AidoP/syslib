use crate::{enumeration, c_flags, Error, File, Fd};

pub const TIMEOUT_INFINITY: u32 = 0xFFFF_FFFF;

c_flags!{
    pub Flags(u32) {
        CLOSE_ON_EXEC = 0o2000000
    } _ => Err(crate::Error::EINVAL)
}

c_flags!{
    pub Events(u32) {
        INPUT = 0x01,
        OUTPUT = 0x04,
        ERROR = 0x08,
        HANG_UP = 0x10
    } _ => Err(crate::Error::EINVAL)
}

#[repr(C)]
pub union Data {
    pub ptr: *mut core::ffi::c_void,
    pub fd: Fd<'static>,
    pub u32: u32,
    pub u64: u64
}

#[repr(C)]
pub struct Event {
    pub events: Events,
    pub data: Data
}
pub enum Cntl {
    Add(Event),
    Delete,
    Modify(Event)
}
impl Cntl {
    pub fn cmd(&self) -> u32 {
        match self {
            Self::Add(_) => 1,
            Self::Delete => 2,
            Self::Modify(_) => 3,
        }
    }
    pub fn arg(&self) -> Option<&Event> {
        match self {
            Self::Add(e) => Some(e),
            Self::Delete => None,
            Self::Modify(e) => Some(e)
        }
    }
}