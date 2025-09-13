use once_cell::sync::Lazy;

/// The container for cached offsets to code and data.
pub struct Offsets {
    pub stage_base_pre_setup: usize,
    pub is_flat_stage: usize,
    pub set_stage_random_settings: usize,
    pub set_stage_additional_settings: usize,
    pub create_stage_branch_table: usize,
}

impl Offsets {
    /// Constructs a new instance of `Offsets`.
    fn new() -> Self {
        let text = text();

        Self {
            stage_base_pre_setup: STAGE_BASE_PRE_SETUP_NEEDLE.find(text).unwrap(),
            is_flat_stage: IS_FLAT_STAGE_NEEDLE.find(text).unwrap(),
            set_stage_random_settings: SET_STAGE_RANDOM_SETTINGS_NEEDLE.find(text).unwrap(),
            set_stage_additional_settings: SET_STAGE_ADDITIONAL_SETTINGS_NEEDLE.find(text).unwrap(),
            create_stage_branch_table: CREATE_STAGE_BRANCH_TABLE_NEEDLE.find(text).unwrap(),
        }
    }

    /// Returns a reference to a `Lazy` containing the current instance of `Offsets`.
    pub const fn get() -> &'static Lazy<Self> {
        static INSTANCE: Lazy<Offsets> = Lazy::new(Offsets::new);

        &INSTANCE
    }
}

/// Returns a byte slice representing the code segment of the target application.
fn text() -> &'static [u8] {
    use std::slice;

    use skyline::hooks::{getRegionAddress, Region};

    unsafe {
        let ptr = getRegionAddress(Region::Text) as *const u8;
        let len = (getRegionAddress(Region::Data) as usize) - (ptr as usize);

        slice::from_raw_parts(ptr, len)
    }
}

/// A substring with an offset to displace the resulting index.
struct Needle {
    /// The byte slice to search for.
    bytes: &'static [u8],

    /// The offset of the resulting index, relative to the start of the byte slice.
    offset: isize,
}

impl Needle {
    /// Returns the index to the needle in the haystack.
    fn find(&self, haystack: &[u8]) -> Result<usize, FindNeedleError> {
        use memchr::memmem;

        let (Some(index), Some(rindex)) = (
            memmem::find(haystack, self.bytes),
            memmem::rfind(haystack, self.bytes),
        ) else {
            return Err(FindNeedleError::NotFound);
        };

        if index != rindex {
            return Err(FindNeedleError::MultipleResults { index, rindex });
        }

        let Some(index) = index.checked_add_signed(self.offset) else {
            return Err(FindNeedleError::Overflow);
        };

        Ok(index)
    }
}

/// An error returned from searching for a needle in a haystack.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum FindNeedleError {
    /// The needle was not found in the haystack.
    NotFound,

    /// The needle was found multiple times in the haystack.
    MultipleResults {
        /// The index of the first occurrence of a needle.
        index: usize,

        /// The index of the last occurrence of a needle.
        rindex: usize,
    },

    /// The resulting index overflowed when adding the offset to it.
    Overflow,
}

static STAGE_BASE_PRE_SETUP_NEEDLE: Needle = Needle {
    bytes: &[
        0xF8, 0x03, 0x00, 0xAA, // mov x24, x0
        0x20, 0x01, 0x3F, 0xD6, // blr x9
        0x16, 0xE3, 0x40, 0xF9, // ldr x22, [x24, #0x1c0]
        0x36, 0x04, 0x00, 0xB4, // cbz x22, #0x90
    ],
    offset: -0x38,
};

static IS_FLAT_STAGE_NEEDLE: Needle = Needle {
    bytes: &[
        0x08, 0x6C, 0x02, 0x51, // sub w8, w0, #0x9b
    ],
    offset: 0x0,
};

static SET_STAGE_RANDOM_SETTINGS_NEEDLE: Needle = Needle {
    bytes: &[
        0x08, 0x00, 0x40, 0xB9, // ldr w8, [x0]
        0x09, 0xF1, 0x02, 0x51, // sub w9, w8, #0xbc
        0xF4, 0x03, 0x01, 0x2A, // mov w20, w1
    ],
    offset: -0x20,
};

static SET_STAGE_ADDITIONAL_SETTINGS_NEEDLE: Needle = Needle {
    bytes: &[
        0x08, 0x2D, 0x0A, 0x9B, // madd x8, x8, x10, x11
        0x08, 0x05, 0x40, 0xB9, // ldr w8, [x8, #0x4]
        0x0A, 0x09, 0x00, 0x51, // sub w10, w8, #0x2
    ],
    offset: -0x20,
};

static CREATE_STAGE_BRANCH_TABLE_NEEDLE: Needle = Needle {
    bytes: &[
        0xB8, 0x7B, 0x13, 0xFE, // -32277576
    ],
    offset: 0x0,
};
