use crate::Result;

use std::ffi::CString;
use std::os::raw::c_int;

use alpm_sys::*;

pub fn vercmp<S: Into<String>>(a: S, b: S) -> Result<Vercmp> {
    let a = CString::new(a.into())?;
    let b = CString::new(b.into())?;
    let ret = unsafe { alpm_pkg_vercmp(a.as_ptr(), b.as_ptr()) };
    Ok(ret.into())
}

pub enum Vercmp {
    Older,
    Equal,
    Newer,
}

impl From<c_int> for Vercmp {
    fn from(int: c_int) -> Vercmp {
        if int < 0 {
            Vercmp::Older
        } else if int > 0 {
            Vercmp::Newer
        } else {
            Vercmp::Equal
        }
    }
}
