use alac_constants::MAX_COEFS;

pub const DENSHIFT_MAX: i32 = 15;
pub const DENSHIFT_DEFAULT: i32 = 9;
pub const AINIT: i32 = 38;
pub const BINIT: i32 = (-29);
pub const CINIT: i32 = (-2);
pub const NUMCOEPAIRS: i32 = 16;

pub fn init_coefs(denshift: i32) -> [i32; MAX_COEFS] {
    let den: i32 = 1 << denshift;

    // size: kALACMaxCoefs
    return [
        (AINIT * den) >> 4,
        (BINIT * den) >> 4,
        (CINIT * den) >> 4,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0
    ];
}
