#![allow(dead_code, unused_variables, unused_mut)]

#[derive(Debug, Clone)]
pub struct Frame {
    rbp: u64,
    rsp: u64,
    rip: u64,
}

impl Frame {
    pub fn new(rbp: u64, rsp: u64, rip: u64) -> Frame {
        Frame {
            rbp: rbp,
            rsp: rsp,
            rip: rip,
        }
    }

    pub fn ip(&self) -> *mut u8 {
        todo!()
    }

    pub fn symbol_address(&self) -> *mut u8 {
        0 as *mut u8
    }
}

#[inline(always)]
pub fn trace_from(mut curframe: Frame, cb: &mut dyn FnMut(&crate::Frame) -> bool) {
    todo!()
}

#[inline(always)]
pub fn trace(cb: &mut dyn FnMut(&crate::Frame) -> bool) {
    todo!()
}
