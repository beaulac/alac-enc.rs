use audio_format::AudioFormatDescription;
use dynamic_predictor::{DENSHIFT_DEFAULT, init_coefs};
use alac_constants::*;

/*
	Map Format: 3-bit field per channel which is the same as the "element tag" that should be placed
				at the beginning of the frame for that channel.  Indicates whether SCE, CPE, or LFE.
				Each particular field is accessed via the current channel index.  Note that the channel
				index increments by two for channel pairs.

	For example:

			C L R 3-channel input		= (ID_CPE << 3) | (ID_SCE)
				index 0 value = (map & (0x7ul << (0 * 3))) >> (0 * 3)
				index 1 value = (map & (0x7ul << (1 * 3))) >> (1 * 3)
			C L R Ls Rs LFE 5.1-channel input = (ID_LFE << 15) | (ID_CPE << 9) | (ID_CPE << 3) | (ID_SCE)
				index 0 value = (map & (0x7ul << (0 * 3))) >> (0 * 3)
				index 1 value = (map & (0x7ul << (1 * 3))) >> (1 * 3)
				index 3 value = (map & (0x7ul << (3 * 3))) >> (3 * 3)
				index 5 value = (map & (0x7ul << (5 * 3))) >> (5 * 3)
				index 7 value = (map & (0x7ul << (7 * 3))) >> (7 * 3)
*/
const sChannelMaps: [u32; MAX_CHANNELS] = [
    ID_SCE as u32,
    ID_CPE as u32,
    ((ID_CPE << 3) | (ID_SCE)) as u32,
    ((ID_SCE << 9) | (ID_CPE << 3) | (ID_SCE)) as u32,
    ((ID_CPE << 9) | (ID_CPE << 3) | (ID_SCE)) as u32,
    ((ID_SCE << 15) | (ID_CPE << 9) | (ID_CPE << 3) | (ID_SCE)) as u32,
    ((ID_SCE << 18) | (ID_SCE << 15) | (ID_CPE << 9) | (ID_CPE << 3) | (ID_SCE)) as u32,
    ((ID_SCE << 21) | (ID_CPE << 15) | (ID_CPE << 9) | (ID_CPE << 3) | (ID_SCE)) as u32
];

const SAMPLE_RATES: [u32; 9] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000];


/// An ALAC packet encoder.
pub struct Encoder {
    bit_depth: u32,
    fast_mode: bool,

    last_mix_res: [i32; MAX_CHANNELS],

    mix_u: Box<[i32]>,
    mix_v: Box<[i32]>,

    predictor_u: Box<[i32]>,
    predictor_v: Box<[i32]>,

    shift_uv: Box<[u16]>,

    work: Box<[u8]>,

    coefs_u: [[[i32; MAX_COEFS]; MAX_SEARCHES]; MAX_CHANNELS],
    coefs_v: [[[i32; MAX_COEFS]; MAX_SEARCHES]; MAX_CHANNELS],

    total_bytes_generated: u32,
    avg_bit_rate: u32,
    max_frame_bytes: u32,

    frame_size: u32,
    max_output_bytes: u32,
    num_channels: u32,
    output_sample_rate: u32,

}

fn calc_output_bytes(channels_per_frame: u32) -> u32 {
    return DEFAULT_FRAME_SIZE * channels_per_frame * ((10 + MAX_SAMPLE_SIZE) / 8) + 1;
}

impl Encoder {
    /// Creates an `Encoder` for a stream from an audio format description
    pub fn new(output_format: AudioFormatDescription) -> Encoder {
        Encoder {
            fast_mode: false,
            frame_size: DEFAULT_FRAME_SIZE,

            bit_depth: output_format.format_flags,
            output_sample_rate: output_format.sample_rate,
            num_channels: output_format.channels_per_frame,

            last_mix_res: [DEFAULT_MIX_RES; MAX_CHANNELS],

            max_output_bytes: calc_output_bytes(output_format.channels_per_frame),

            mix_u: vec![0; DEFAULT_FRAME_SIZE as usize].into_boxed_slice(),
            mix_v: vec![0; DEFAULT_FRAME_SIZE as usize].into_boxed_slice(),

            predictor_u: vec![0; DEFAULT_FRAME_SIZE as usize].into_boxed_slice(),
            predictor_v: vec![0; DEFAULT_FRAME_SIZE as usize].into_boxed_slice(),

            shift_uv: vec![0; DEFAULT_FRAME_SIZE as usize * 2].into_boxed_slice(),

            work: vec![0; calc_output_bytes(output_format.channels_per_frame) as usize].into_boxed_slice(),

            coefs_u: [[init_coefs(DENSHIFT_DEFAULT); MAX_SEARCHES]; MAX_CHANNELS],
            coefs_v: [[init_coefs(DENSHIFT_DEFAULT); MAX_SEARCHES]; MAX_CHANNELS],

            total_bytes_generated: 0,
            avg_bit_rate: 0,
            max_frame_bytes: 0,
        }
    }
}
