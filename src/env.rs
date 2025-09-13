use std::{ffi::CStr, slice};

use semver::Version;
use skyline::{
    hooks::{getRegionAddress, Region},
    nn,
};

/// Returns `true` if the version of the target application is supported.
pub fn is_app_version_compatible() -> bool {
    let app_version = app_version();
    let app_version_min = Version::new(3, 0, 0);

    app_version >= app_version_min
}

/// Returns the version of the target application.
pub fn app_version() -> Version {
    let mut display_version = nn::oe::DisplayVersion { name: [b'\0'; 16] };

    unsafe {
        nn::oe::GetDisplayVersion(&mut display_version);
    }

    let text = CStr::from_bytes_until_nul(&display_version.name)
        .expect("trailing nul terminator should exist")
        .to_str()
        .expect("version text should contain valid UTF-8");

    Version::parse(text).expect("target software version should use semantic versioning")
}

/// Returns a byte slice representing the code segment of the target application.
pub unsafe fn text() -> &'static [u8] {
    let ptr = getRegionAddress(Region::Text).cast::<u8>();
    let len = (getRegionAddress(Region::Rodata) as usize) - (ptr as usize);

    slice::from_raw_parts(ptr, len)
}
