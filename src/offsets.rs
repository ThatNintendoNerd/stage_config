use once_cell::sync::Lazy;

pub struct Offsets {
    pub stage_base_pre_setup: usize,
    pub is_flat_stage: usize,
    pub set_stage_random_settings: usize,
    pub set_stage_additional_settings: usize,
    pub create_stage_jump_table: usize,
}

impl Offsets {
    fn new() -> Self {
        let text = text();

        Self {
            stage_base_pre_setup: Self::find(text, STAGE_BASE_PRE_SETUP_SEARCH_CODE)
                .unwrap_or(0x25D8DE0),
            is_flat_stage: Self::find(text, IS_FLAT_STAGE_SEARCH_CODE).unwrap_or(0x261D770),
            set_stage_random_settings: Self::find(text, SET_STAGE_RANDOM_SETTINGS_SEARCH_CODE)
                .unwrap_or(0x177A980),
            set_stage_additional_settings: Self::find(
                text,
                SET_STAGE_ADDITIONAL_SETTINGS_SEARCH_CODE,
            )
            .unwrap_or(0x2498660),
            create_stage_jump_table: Self::find(text, CREATE_STAGE_JUMP_TABLE_SEARCH_CODE)
                .unwrap_or(0x4505DF0),
        }
    }

    pub fn get() -> &'static Lazy<Self> {
        static INSTANCE: Lazy<Offsets> = Lazy::new(Offsets::new);

        &INSTANCE
    }

    fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        use memchr::memmem;

        memmem::find(haystack, needle)
    }
}

fn text() -> &'static [u8] {
    use std::slice;

    use skyline::hooks::{getRegionAddress, Region};

    unsafe {
        let ptr = getRegionAddress(Region::Text) as *const u8;
        let len = (getRegionAddress(Region::Data) as usize) - (ptr as usize);

        slice::from_raw_parts(ptr, len)
    }
}

#[rustfmt::skip]
static STAGE_BASE_PRE_SETUP_SEARCH_CODE: &[u8] = &[
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
];

#[rustfmt::skip]
static IS_FLAT_STAGE_SEARCH_CODE: &[u8] = &[
    0x08, 0x6C, 0x02, 0x51,
];

#[rustfmt::skip]
static SET_STAGE_RANDOM_SETTINGS_SEARCH_CODE: &[u8] = &[
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
];

#[rustfmt::skip]
static SET_STAGE_ADDITIONAL_SETTINGS_SEARCH_CODE: &[u8] = &[
    0x09, 0xC0, 0x40, 0x39,
    0x29, 0x1B, 0x00, 0x34,
];

#[rustfmt::skip]
static CREATE_STAGE_JUMP_TABLE_SEARCH_CODE: &[u8] = &[
    0xB8, 0x7B, 0x13, 0xFE,
];
