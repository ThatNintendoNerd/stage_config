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

#[rustfmt::skip]
static STAGE_BASE_PRE_SETUP_NEEDLE: Needle = Needle {
    bytes: &[
        0xED, 0x33, 0xB7, 0x6D,
        0xEB, 0x2B, 0x01, 0x6D,
        0xE9, 0x23, 0x02, 0x6D,
        0xFC, 0x6F, 0x03, 0xA9,
        0xFA, 0x67, 0x04, 0xA9,
        0xF8, 0x5F, 0x05, 0xA9,
        0xF6, 0x57, 0x06, 0xA9,
        0xF4, 0x4F, 0x07, 0xA9,
        0xFD, 0x7B, 0x08, 0xA9,
        0xFD, 0x03, 0x02, 0x91,
        0xFF, 0x03, 0x08, 0xD1,
    ],
    offset: 0x0,
};

#[rustfmt::skip]
static IS_FLAT_STAGE_NEEDLE: Needle = Needle {
    bytes: &[
        0x08, 0x6C, 0x02, 0x51,
    ],
    offset: 0x0,
};

#[rustfmt::skip]
static SET_STAGE_RANDOM_SETTINGS_NEEDLE: Needle = Needle {
    bytes: &[
        0xE8, 0x0F, 0x19, 0xFC,
        0xFC, 0x6F, 0x01, 0xA9,
        0xFA, 0x67, 0x02, 0xA9,
        0xF8, 0x5F, 0x03, 0xA9,
        0xF6, 0x57, 0x04, 0xA9,
        0xF4, 0x4F, 0x05, 0xA9,
        0xFD, 0x7B, 0x06, 0xA9,
        0xFD, 0x83, 0x01, 0x91,
        0xFF, 0x43, 0x0E, 0xD1,
        0x08, 0x00, 0x40, 0xB9,
    ],
    offset: 0x0,
};

#[rustfmt::skip]
static SET_STAGE_ADDITIONAL_SETTINGS_NEEDLE: Needle = Needle {
    bytes: &[
        0x09, 0xC0, 0x40, 0x39,
        0x29, 0x1B, 0x00, 0x34,
    ],
    offset: 0x0,
};

#[rustfmt::skip]
static CREATE_STAGE_BRANCH_TABLE_NEEDLE: Needle = Needle {
    bytes: &[
        0xB8, 0x7B, 0x13, 0xFE,
    ],
    offset: 0x0,
};
