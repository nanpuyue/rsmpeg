use std::{
    ffi::CStr,
    ops::Drop,
    ptr::{self, NonNull},
};

use crate::{error::*, ffi, shared::*};

wrap!(AVIOContext: ffi::AVIOContext);

impl AVIOContext {
    pub fn open(url: &CStr, flags: u32) -> Result<Self> {
        let mut io_context = ptr::null_mut();
        unsafe { ffi::avio_open(&mut io_context, url.as_ptr(), flags as i32) }
            .upgrade()
            .map_err(|_| RsmpegError::AVIOOpenError)?;
        Ok(unsafe { Self::from_raw(NonNull::new(io_context).unwrap()) })
    }
}

impl Drop for AVIOContext {
    fn drop(&mut self) {
        unsafe { ffi::avio_close(self.as_mut_ptr()) }
            .upgrade()
            .unwrap();
    }
}