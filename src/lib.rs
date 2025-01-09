use std::io;

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
