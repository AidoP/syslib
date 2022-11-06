use crate::c_flags;

pub const MAP_FAILED: *mut core::ffi::c_void = usize::MAX as *mut _;

c_flags!{
    pub Flags(u32) {
        SHARED = 1,
        PRIVATE = 2
    } _ => Err(crate::Error::EINVAL)
}

c_flags!{
    pub Protection(u32) {
        EXEC = 0b001,
        READ = 0b010,
        WRITE = 0b100
    } _ => Err(crate::Error::EINVAL)
}

c_flags!{
    pub RemapFlags(u32) {
        MAY_MOVE = 0b001
    } _ => Err(crate::Error::EINVAL)
}