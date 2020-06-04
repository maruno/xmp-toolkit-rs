// TO DO: Revise API documentation to fit the Rust wrapper.

use crate::ffi;

pub struct XmpDateTime {
    dt: *mut ffi::CXmpDateTime,
}

impl Drop for XmpDateTime {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpDateTimeDrop(self.dt);
        }
    }
}

impl XmpDateTime {
    /// Creates a new file struct that is associated with no file.
    pub fn new() -> XmpDateTime {
        XmpDateTime {
            dt: unsafe { ffi::CXmpDateTimeNew() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let mut _dt = XmpDateTime::new();
    }
}
