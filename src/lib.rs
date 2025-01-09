use std::{
    ffi::{CStr, OsStr},
    io,
    os::unix::ffi::OsStrExt,
    path::Path,
};

#[allow(nonstandard_style)]
pub mod sys;

pub struct Udev {
    raw: *mut sys::udev,
}

impl Udev {
    pub fn new() -> io::Result<Self> {
        let raw = unsafe { sys::udev_new() };

        if raw.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::udev {
        self.raw
    }
}

impl Clone for Udev {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::udev_ref(self.as_raw()) },
        }
    }
}

impl Drop for Udev {
    fn drop(&mut self) {
        unsafe {
            sys::udev_unref(self.as_raw());
        }
    }
}

pub struct Device {
    raw: *mut sys::udev_device,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut sys::udev_device) -> Self {
        Self { raw }
    }

    pub fn devnode(&self) -> Option<&Path> {
        let devnode = unsafe { sys::udev_device_get_devnode(self.raw) };

        if devnode.is_null() {
            return None;
        }

        Some(Path::new(OsStr::from_bytes(
            unsafe { CStr::from_ptr(devnode) }.to_bytes(),
        )))
    }
}
