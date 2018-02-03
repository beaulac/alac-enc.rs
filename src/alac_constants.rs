/**
 * Lossless Definitions
 */

pub const CODEC_FORMAT: *const str = "alac";
pub const ALAC_VERSION: u32 = 0;
pub const COMPATIBLE_VERSION: u32 = ALAC_VERSION;
pub const DEFAULT_FRAME_SIZE: u32 = 4096;

pub const MAX_CHANNELS: usize = 8;
pub const MAX_ESCAPE_HEADER_BYTES: usize = 8;
pub const MAX_SEARCHES: usize = 16;
pub const MAX_COEFS: usize = 16;
pub const DEFAULT_FRAMES_PER_PACKET: u32 = 4096;

pub const ENCODER_MAGIC: *const str = "dpge";
pub const MAX_SAMPLE_SIZE: u32 = 32;
pub const DEFAULT_MIX_BITS: u32 = 2; /* max allowed bit width is 32 */
pub const DEFAULT_MIX_RES: i32 = 0;
pub const MAX_RES: i32 = 4;
pub const DEFAULT_NUM_UV: u32 = 8;
pub const MIN_UV: u32 = 4;
pub const MAX_UV: u32 = 8;

pub const ID_SCE: u32 = 0; /* Single Channel Element */
pub const ID_CPE: u32 = 1; /* Channel Pair Element */
pub const ID_CCE: u32 = 2; /* Coupling Channel Element */
pub const ID_LFE: u32 = 3; /* LFE Channel Element */
pub const ID_DSE: u32 = 4; /* Not yet supported */
pub const ID_PCE: u32 = 5; /* Not yet supported */
pub const ID_FIL: u32 = 6; /* Filler Element */
pub const ID_END: u32 = 7; /* Frame End */
