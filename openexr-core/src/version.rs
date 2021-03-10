use semver::Identifier;
use semver::Version;

pub fn get_library_version() -> semver::Version {
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        let mut ext = std::ptr::null();

        openexr_sys::exr_get_library_version(&mut major, &mut minor, &mut patch, &mut ext);

        let c_str = ::std::ffi::CStr::from_ptr(ext);
        let str_slice: &str = c_str.to_str().unwrap();
        let extra = str_slice.to_owned();

        let version = Version {
            major: major as u64,
            minor: minor as u64,
            patch: patch as u64,
            pre: vec![],
            build: vec![Identifier::AlphaNumeric(extra)],
        };

        return version;

        // return semver::Version::new(major as u64, minor as u64, patch as u64);
    }
}
