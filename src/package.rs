use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Deserialize, Serialize, Eq, Clone)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) release: String,
    pub(crate) arch: String,
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub use libloading::Error;
use std::ffi::{c_char, CString};

pub fn rpmvercmp3(lib: &str, a: &str, b: &str) -> Result<std::cmp::Ordering, Error> {
    let a: CString = CString::new(a).unwrap();
    let b: CString = CString::new(b).unwrap();
    let ret = unsafe {
        let lib = libloading::Library::new(lib)?;
        let _rpmvercmp: libloading::Symbol<
            unsafe extern "C" fn(*const c_char, *const c_char) -> i32,
        > = lib.get(b"rpmvercmp")?;
        _rpmvercmp(a.as_ptr(), b.as_ptr())
    };

    Ok(match ret {
        -1 => std::cmp::Ordering::Less,
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
        _ => unimplemented!(),
    })
}

pub fn rpmvercmp(a: &str, b: &str) -> Result<std::cmp::Ordering, Error> {
    use std::path::Path;
    static LIST: &'static [(
        &'static str,
        fn(&str, &str, &str) -> Result<std::cmp::Ordering, Error>,
    )] = &[
        ("/usr/lib64/librpm.so.7", rpmvercmp3),
        ("/usr/lib64/librpm.so.7.0.1", rpmvercmp3),
        ("/usr/lib64/librpmio.so.7", rpmvercmp3),
        ("/usr/lib64/librpmio.so.7.0.1", rpmvercmp3),
    ];
    for (path, func) in LIST {
        if Path::new(path).exists() {
            return func(path, a, b);
        }
    }
    Err(Error::DlOpenUnknown)
}
