use std::{io, mem, ptr::NonNull};

#[allow(nonstandard_style)]
pub mod sys;

pub struct Udev(NonNull<sys::udev>);

impl Udev {
    pub fn new() -> io::Result<Self> {
        let inner = unsafe { sys::udev_new() };

        if inner.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self(unsafe { NonNull::new_unchecked(inner) }))
    }

    pub fn as_raw(&self) -> *mut sys::udev {
        self.0.as_ptr()
    }

    pub fn into_raw(self) -> *mut sys::udev {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }
}

impl Drop for Udev {
    fn drop(&mut self) {
        unsafe {
            sys::udev_unref(self.as_raw());
        }
    }
}
