use super::*;
use rand::prelude::*;

fn random_bytes<const BYTES: usize>() -> [u8; BYTES] {
    let mut bytes = [0u8; BYTES];
    rand::thread_rng().fill(&mut bytes[..]);
    bytes
}

// 256 bits
#[test]
fn test_random_piece_256_bits() {
    let expanded_iv = random_bytes();
    let piece = random_bytes();

    let layers = 4096 / 32;
    let mut encoding = piece.clone();
    encode(&mut encoding, expanded_iv, layers).unwrap();
    let mut decoding = encoding.clone();
    decode(&mut decoding, expanded_iv, layers);

    // println!("\nPiece is {:?}\n", piece.to_vec());
    // println!("\nDecoding is {:?}\n", decoding.to_vec());
    // println!("\nEncoding is {:?}\n", encoding.to_vec());

    assert_eq!(piece.to_vec(), decoding.to_vec());
}

static CORRECT_ENCODING: [u8; 4096] = [
    0x97, 0x70, 0x05, 0xd2, 0x7b, 0x0f, 0x2c, 0x8a, 0x8d, 0x22, 0xd9, 0x7e, 0x6c, 0x62, 0x88, 0xdf,
    0x5f, 0xaf, 0x1c, 0xc2, 0x90, 0x3e, 0x47, 0x67, 0x98, 0x7b, 0x63, 0x62, 0x7e, 0x8b, 0xd2, 0x0b,
    0x9e, 0x44, 0x12, 0x48, 0x0a, 0xaa, 0x18, 0xe4, 0x3f, 0xaf, 0xc2, 0xb8, 0x30, 0xad, 0x4b, 0x73,
    0x4a, 0x5b, 0x94, 0xbf, 0x43, 0x5c, 0xfc, 0xc7, 0x9f, 0xf0, 0x64, 0x7a, 0x89, 0x84, 0x8c, 0xb4,
    0x37, 0xdf, 0xf1, 0xf0, 0x15, 0x5c, 0x38, 0x13, 0x04, 0x3f, 0x84, 0x70, 0x01, 0x75, 0x84, 0x49,
    0x54, 0x1f, 0x5e, 0x75, 0xf1, 0x00, 0x0f, 0xe2, 0xe4, 0xd1, 0x08, 0xb7, 0xdf, 0x11, 0x7e, 0xba,
    0x8d, 0xe8, 0xf5, 0x6e, 0x76, 0x42, 0x88, 0xd8, 0xd8, 0x66, 0xde, 0x48, 0x86, 0xd3, 0xf2, 0xd4,
    0x78, 0x49, 0x7d, 0xa4, 0x9e, 0xaf, 0x80, 0x60, 0x58, 0x64, 0x01, 0x7c, 0x12, 0x17, 0xea, 0xbb,
    0x8d, 0x6f, 0x93, 0xed, 0x5d, 0xe0, 0xde, 0x4e, 0x6f, 0x11, 0x80, 0x1a, 0x82, 0x15, 0xd3, 0x1c,
    0x46, 0xf5, 0x5a, 0xa1, 0x28, 0xcc, 0xc0, 0x29, 0x90, 0x01, 0x72, 0xc9, 0xe4, 0xa7, 0x2e, 0x26,
    0x2e, 0x97, 0x02, 0xbf, 0x43, 0x46, 0x02, 0xc2, 0x55, 0xd0, 0xfc, 0x40, 0x23, 0xb0, 0xee, 0x9c,
    0x93, 0x42, 0x98, 0xcb, 0x88, 0xc4, 0x34, 0x7a, 0x17, 0x23, 0x09, 0xed, 0x9f, 0x30, 0x88, 0x28,
    0xa6, 0x93, 0xea, 0x7c, 0xa0, 0x86, 0xa3, 0xee, 0x5e, 0x79, 0x1d, 0x7e, 0xa1, 0xa3, 0xa6, 0x9e,
    0x24, 0xb3, 0x76, 0x6c, 0x1c, 0x80, 0xd8, 0x6f, 0x9a, 0xd8, 0xb8, 0x80, 0xf2, 0x31, 0xdd, 0x44,
    0x3b, 0xc3, 0x2a, 0x89, 0xb7, 0x7a, 0x3a, 0xac, 0xc8, 0xcb, 0x6b, 0xd2, 0x35, 0xd7, 0x13, 0x41,
    0x25, 0x9f, 0xf8, 0x14, 0x86, 0x67, 0xe8, 0x8e, 0x8b, 0x2d, 0xf2, 0xe0, 0xe1, 0xdf, 0x63, 0xef,
    0x86, 0xd9, 0x2d, 0x17, 0xfe, 0xaf, 0x52, 0x77, 0x83, 0x15, 0x78, 0x65, 0xf9, 0x60, 0x2c, 0xfc,
    0x38, 0x2b, 0x62, 0x36, 0x87, 0x4c, 0x08, 0x52, 0x1d, 0xf7, 0xe9, 0x7d, 0xd3, 0xa2, 0xde, 0x5d,
    0x64, 0xc3, 0xb3, 0x6d, 0x2b, 0xc8, 0x48, 0x44, 0xb7, 0xb3, 0x13, 0xda, 0xbf, 0x2c, 0xbf, 0x2c,
    0xb2, 0x7b, 0xb8, 0xaa, 0xda, 0x7d, 0xe3, 0xd8, 0x3f, 0x85, 0x66, 0x95, 0x58, 0x5d, 0x9d, 0xf1,
    0xaf, 0x83, 0xf4, 0x88, 0xc2, 0x78, 0x8d, 0xba, 0x79, 0xb7, 0x97, 0x9e, 0x65, 0x69, 0xf2, 0xdd,
    0x83, 0x80, 0xab, 0x57, 0x44, 0x98, 0xe4, 0xe6, 0x60, 0x9e, 0xdc, 0x7c, 0x47, 0x29, 0x89, 0xfe,
    0xd9, 0x36, 0xe3, 0x96, 0x52, 0xd3, 0x27, 0x85, 0x04, 0xf1, 0x7d, 0xce, 0xea, 0xc2, 0xe2, 0x82,
    0x8b, 0xb7, 0xc7, 0x3b, 0xbb, 0xc2, 0xec, 0xa8, 0x51, 0x92, 0xc1, 0x83, 0xba, 0x2d, 0xd8, 0xa3,
    0xb5, 0xea, 0xdd, 0x27, 0xc4, 0xfc, 0x9d, 0xf7, 0x6e, 0x93, 0xff, 0x8d, 0x88, 0x5d, 0x4a, 0x81,
    0xa1, 0x67, 0xcf, 0x57, 0x10, 0x64, 0x5e, 0x54, 0x99, 0x4d, 0xee, 0xd0, 0x71, 0x4d, 0x82, 0xe7,
    0x48, 0xe9, 0x12, 0xc1, 0x22, 0xd3, 0x2d, 0xda, 0xdc, 0xc6, 0x5e, 0x41, 0xbe, 0x5e, 0xd6, 0x01,
    0x4c, 0xca, 0xc1, 0x14, 0xaf, 0xf3, 0x8f, 0xb5, 0x6f, 0x15, 0xd8, 0xb2, 0xf3, 0x68, 0x73, 0x04,
    0xf2, 0x32, 0xfb, 0x01, 0x04, 0x92, 0x0c, 0xa4, 0x00, 0x99, 0xde, 0x58, 0x38, 0xda, 0xb7, 0x9e,
    0x10, 0x33, 0x4a, 0x7a, 0x7a, 0x68, 0x30, 0x6d, 0x91, 0xa9, 0xe8, 0x3c, 0x82, 0x5f, 0xff, 0x82,
    0x36, 0x30, 0xe4, 0xde, 0x41, 0x7b, 0x94, 0xbc, 0xb0, 0x2f, 0x16, 0xe7, 0xe3, 0xd2, 0x32, 0xd0,
    0xee, 0x86, 0x72, 0xe6, 0xf4, 0x4d, 0xe1, 0x53, 0xcb, 0x7b, 0xe2, 0x18, 0xe3, 0xaf, 0xba, 0xe1,
    0x6d, 0x7e, 0x72, 0xe0, 0x95, 0xb2, 0x8b, 0x2a, 0x37, 0xb2, 0x1c, 0x2a, 0x79, 0x6d, 0x7c, 0x86,
    0xd9, 0x35, 0x1d, 0x6a, 0xa7, 0x94, 0x55, 0x84, 0x4d, 0x92, 0x4d, 0xf4, 0x88, 0x35, 0x2f, 0x0f,
    0xb6, 0xc1, 0x1e, 0x1e, 0xca, 0xdb, 0x45, 0x2b, 0x83, 0xc8, 0x86, 0xf0, 0x64, 0x00, 0x16, 0x60,
    0x43, 0x7c, 0x71, 0xae, 0xde, 0x91, 0x31, 0x27, 0x21, 0x36, 0xf0, 0x88, 0x7e, 0x00, 0xf7, 0xa0,
    0x60, 0xb5, 0xfa, 0x54, 0xbf, 0xd9, 0x17, 0xde, 0x0b, 0xab, 0x62, 0x0e, 0x07, 0x49, 0x8e, 0x31,
    0xab, 0x9e, 0x44, 0x49, 0xfe, 0x44, 0xe3, 0xea, 0x06, 0xfd, 0xf2, 0x07, 0x11, 0x57, 0x46, 0xb1,
    0x54, 0x3a, 0x74, 0x90, 0x45, 0x4d, 0x54, 0x5e, 0x0f, 0x2e, 0xfb, 0x2d, 0x16, 0xe0, 0xdb, 0x82,
    0xcb, 0x07, 0xff, 0x2a, 0x1d, 0x5f, 0x40, 0xa9, 0x80, 0x0b, 0x53, 0x98, 0xf9, 0xe8, 0x5b, 0x4b,
    0x8e, 0x72, 0x90, 0xb1, 0x1b, 0x05, 0x37, 0x03, 0x39, 0x58, 0xc7, 0xf5, 0xa0, 0x32, 0xe4, 0xbd,
    0x39, 0x12, 0x62, 0x36, 0x5a, 0xd2, 0x23, 0x61, 0x10, 0xb8, 0x2f, 0x26, 0xad, 0xd4, 0x2b, 0x30,
    0x97, 0x5e, 0x61, 0xa1, 0xca, 0x5b, 0x76, 0x3e, 0xa5, 0x04, 0x08, 0x5b, 0x96, 0xbf, 0x86, 0x71,
    0x68, 0x3d, 0xac, 0x18, 0x5f, 0xd5, 0xda, 0x88, 0x6f, 0xd8, 0x8f, 0xf2, 0xe1, 0xc4, 0x24, 0x52,
    0xc6, 0x4c, 0xbf, 0x87, 0xdf, 0x60, 0x97, 0x29, 0x25, 0x36, 0x76, 0x7f, 0xb7, 0x39, 0x8d, 0xfd,
    0xcc, 0xa0, 0xf4, 0x00, 0xa1, 0x16, 0xe2, 0xf2, 0x99, 0xdf, 0x17, 0x20, 0xa7, 0x3d, 0xcd, 0xe0,
    0xc3, 0x39, 0xde, 0xb8, 0xe0, 0x42, 0xdc, 0x02, 0x86, 0xe1, 0x29, 0x86, 0xfd, 0x83, 0xb8, 0xd0,
    0xdc, 0x21, 0x41, 0xef, 0x08, 0xcf, 0x6c, 0x95, 0xee, 0x4a, 0xe0, 0xd3, 0xdb, 0xd9, 0xa4, 0x69,
    0x14, 0x71, 0xcf, 0xcf, 0x61, 0x20, 0x7c, 0x00, 0xbe, 0x12, 0x72, 0x04, 0xcc, 0x20, 0x28, 0xfc,
    0x32, 0x19, 0x58, 0x3e, 0x71, 0x46, 0x66, 0xf3, 0xbf, 0xcb, 0x03, 0x53, 0x83, 0xb8, 0xa3, 0xf3,
    0x52, 0x7c, 0x73, 0xc4, 0x16, 0x4f, 0x07, 0x0a, 0xa4, 0xd8, 0x2e, 0xcd, 0xb2, 0x6f, 0xe5, 0xfc,
    0x7a, 0x3d, 0x49, 0x02, 0xe1, 0xae, 0x36, 0x15, 0xc0, 0x2d, 0xb4, 0x53, 0xca, 0x58, 0x9f, 0x00,
    0x72, 0x8e, 0x2f, 0xf3, 0xcd, 0x4b, 0x5c, 0xf6, 0x28, 0xf8, 0x65, 0xea, 0xa3, 0xc9, 0xd4, 0xc4,
    0x45, 0xd6, 0x3f, 0xe0, 0x8b, 0xdd, 0x4d, 0x40, 0xe0, 0x22, 0xba, 0xff, 0xc5, 0x87, 0x76, 0x48,
    0xb0, 0xe5, 0x32, 0x6a, 0x31, 0x6d, 0x14, 0xfa, 0x11, 0x8a, 0x10, 0xc2, 0x69, 0x23, 0xfa, 0x9f,
    0x67, 0x4e, 0xc3, 0x18, 0x13, 0xf2, 0x87, 0x2d, 0x8b, 0x98, 0xf9, 0x71, 0xbc, 0xbb, 0xe3, 0xe8,
    0x76, 0xe2, 0x96, 0xcc, 0xde, 0x75, 0xb4, 0x67, 0xcc, 0xcb, 0x79, 0xd3, 0x00, 0xc0, 0xb7, 0xeb,
    0x98, 0x0c, 0x66, 0x15, 0x63, 0x1c, 0x5d, 0xdc, 0xff, 0x88, 0x3a, 0x35, 0xd8, 0x25, 0xd6, 0x4b,
    0xba, 0xb2, 0x55, 0x4a, 0xf3, 0xd3, 0x20, 0x37, 0xd0, 0x58, 0xd3, 0xd2, 0xd7, 0xbc, 0xf5, 0xe1,
    0x0c, 0xcb, 0x57, 0xee, 0xde, 0xf4, 0x43, 0x6f, 0x22, 0xfe, 0xcf, 0xf1, 0x43, 0xcf, 0x71, 0xb2,
    0x70, 0x28, 0xc9, 0x30, 0xc7, 0xb5, 0x2d, 0xcd, 0x1f, 0x18, 0xbb, 0x6f, 0x9e, 0x04, 0x50, 0xa3,
    0x1e, 0xc7, 0x89, 0xf0, 0xcd, 0x90, 0xa3, 0xc6, 0x34, 0x2d, 0x58, 0x2c, 0xdf, 0xbb, 0x55, 0x4a,
    0x52, 0x20, 0xd0, 0x5b, 0xfe, 0xf4, 0xf6, 0x61, 0x84, 0xea, 0x59, 0x3d, 0x83, 0xfd, 0x07, 0xce,
    0x44, 0x9b, 0x58, 0x90, 0x96, 0x5f, 0xf9, 0xa6, 0xe2, 0x70, 0x02, 0x40, 0x04, 0x5a, 0xcd, 0xd7,
    0xc2, 0x16, 0x90, 0x22, 0x43, 0x1c, 0xe2, 0xc6, 0x86, 0x37, 0x83, 0x0d, 0xbb, 0x14, 0xf0, 0xc8,
    0x5c, 0x72, 0x99, 0xad, 0x18, 0x90, 0x5d, 0x23, 0x35, 0x60, 0x04, 0x6b, 0xe5, 0x80, 0xc3, 0x17,
    0xd8, 0xb7, 0xd1, 0xa2, 0x08, 0x54, 0x72, 0x1c, 0x59, 0xe5, 0x21, 0xfd, 0x17, 0xa2, 0x82, 0xe7,
    0xd1, 0x03, 0xbc, 0x6a, 0xce, 0xaa, 0x1a, 0x5b, 0xb2, 0x91, 0x53, 0xfa, 0x5e, 0xad, 0x74, 0x9c,
    0x7d, 0x57, 0xd1, 0xb9, 0xbc, 0xee, 0xec, 0x3f, 0xd8, 0x21, 0x7e, 0x15, 0x69, 0x2c, 0x7f, 0x3a,
    0x09, 0xa6, 0x27, 0xe9, 0xaf, 0x3f, 0x71, 0xeb, 0xb7, 0x1f, 0x10, 0xf2, 0x49, 0x7d, 0xcf, 0xc1,
    0x68, 0x34, 0x20, 0xf8, 0xcc, 0x05, 0xd2, 0x9e, 0xee, 0xc0, 0xf7, 0xc2, 0xc9, 0xf5, 0xa9, 0x7a,
    0x90, 0x61, 0x9a, 0x2b, 0x5c, 0x52, 0x84, 0xbe, 0xfb, 0x11, 0x32, 0x40, 0xa7, 0xe3, 0x3d, 0x98,
    0x59, 0xcd, 0xd0, 0x3c, 0xb1, 0x63, 0x7f, 0x91, 0x00, 0xb2, 0x87, 0xb2, 0x8c, 0x73, 0xa2, 0x25,
    0xc3, 0xec, 0xa2, 0x67, 0x70, 0xa2, 0xa4, 0x31, 0x1b, 0x28, 0x08, 0x1d, 0xb0, 0x5f, 0xab, 0x85,
    0xba, 0x95, 0xdc, 0xc8, 0x04, 0x3f, 0x9d, 0x9b, 0x61, 0x71, 0xf3, 0x84, 0x04, 0x1d, 0x61, 0x5f,
    0x2e, 0xcc, 0x22, 0x30, 0x56, 0x14, 0xb6, 0x1b, 0xf7, 0x6a, 0x92, 0xdb, 0xba, 0xc7, 0x9b, 0xdc,
    0xa0, 0xa3, 0x96, 0x13, 0x90, 0xbc, 0x41, 0xc8, 0x32, 0x2d, 0xf5, 0x89, 0x1c, 0xe4, 0x27, 0xa4,
    0x9d, 0xb5, 0xf5, 0x95, 0x84, 0x23, 0x29, 0x74, 0x99, 0x0f, 0x71, 0x7c, 0x9e, 0xe5, 0x55, 0x1f,
    0xc5, 0x77, 0x5e, 0x55, 0x7e, 0x52, 0x1c, 0x6d, 0x90, 0x0c, 0xcb, 0xce, 0xd0, 0x71, 0x83, 0xfc,
    0xfe, 0xb0, 0xce, 0xcf, 0x92, 0x59, 0x81, 0x92, 0xf2, 0x20, 0xb7, 0xbf, 0x6f, 0x83, 0xe2, 0x36,
    0x04, 0x56, 0xa9, 0x70, 0x22, 0x3e, 0x39, 0x35, 0x73, 0x72, 0x3a, 0xb8, 0xb2, 0xda, 0x5b, 0x35,
    0x3e, 0x11, 0x0c, 0xe2, 0x7f, 0x6e, 0xbe, 0xc1, 0x0c, 0xe6, 0x1e, 0x70, 0x68, 0x9f, 0x09, 0x9e,
    0x38, 0xfe, 0x4c, 0xe8, 0xe6, 0xd5, 0x06, 0x0e, 0xea, 0xbe, 0x06, 0x26, 0xab, 0x15, 0x25, 0x72,
    0x6d, 0xf5, 0xf2, 0x6b, 0xba, 0xde, 0xae, 0xc5, 0x75, 0x0f, 0xff, 0x2a, 0xfc, 0x5e, 0xd0, 0x26,
    0x14, 0x1e, 0xd2, 0x8e, 0x6f, 0x1f, 0x8f, 0xad, 0xf9, 0xc4, 0xc1, 0xa1, 0x0b, 0x69, 0x62, 0x84,
    0x52, 0xc6, 0xbe, 0x99, 0x38, 0x31, 0x69, 0xdf, 0x2f, 0x9d, 0x50, 0xc3, 0x55, 0x82, 0x64, 0x0a,
    0x62, 0x9c, 0x57, 0x43, 0x9e, 0x4b, 0xc8, 0xe0, 0x08, 0xcb, 0x27, 0xa4, 0xdd, 0x83, 0xcb, 0xd7,
    0x84, 0xb9, 0x71, 0x4b, 0xdd, 0x18, 0x77, 0x68, 0xd8, 0x80, 0xd9, 0xfe, 0x18, 0x7a, 0x75, 0x80,
    0x07, 0x7e, 0x52, 0xf1, 0x8a, 0x44, 0xce, 0x68, 0x6f, 0x0a, 0xa9, 0x8a, 0x86, 0x66, 0xb4, 0xfe,
    0x08, 0xd5, 0x06, 0xda, 0xed, 0x4c, 0x36, 0x51, 0x6e, 0x69, 0xb1, 0x8b, 0x99, 0x39, 0xdb, 0x08,
    0xdc, 0x4e, 0x68, 0xf6, 0x77, 0x1c, 0x22, 0xcc, 0xfa, 0xa2, 0x83, 0x27, 0x1e, 0x34, 0x6c, 0x18,
    0x08, 0x88, 0x4b, 0x48, 0x19, 0xd0, 0x23, 0xc3, 0x7c, 0x92, 0xb8, 0x6b, 0x15, 0xe1, 0x85, 0x42,
    0x69, 0xaf, 0x3f, 0x4f, 0x0b, 0x5d, 0x06, 0xf4, 0x98, 0xfb, 0x5f, 0xb0, 0xb5, 0x9a, 0x2e, 0x2c,
    0x19, 0xd1, 0xc5, 0xf4, 0xe4, 0xe0, 0x60, 0x3d, 0x38, 0x42, 0xbf, 0x82, 0xe4, 0xc6, 0x98, 0x67,
    0x91, 0xcc, 0xb3, 0x90, 0x52, 0xd1, 0xac, 0x03, 0x65, 0x49, 0xd9, 0xbb, 0xcc, 0xc3, 0xfe, 0x9d,
    0x31, 0x3c, 0x54, 0x01, 0x1c, 0xa5, 0x24, 0x1f, 0x2b, 0x06, 0x55, 0xd9, 0x2b, 0xfa, 0x0a, 0x10,
    0xec, 0x2a, 0x03, 0x0a, 0xf4, 0x51, 0x5b, 0xbe, 0xa1, 0x7b, 0x23, 0x8b, 0xa1, 0x57, 0xe7, 0x06,
    0xf2, 0x02, 0xb0, 0x4d, 0x05, 0x13, 0x1f, 0x86, 0xe0, 0xb1, 0xf6, 0x6f, 0x5e, 0x7e, 0x49, 0x94,
    0x72, 0x1a, 0xce, 0x07, 0x91, 0x31, 0xd5, 0x51, 0x9b, 0xab, 0x3b, 0x78, 0x38, 0x8c, 0x70, 0xfa,
    0xab, 0x3b, 0x3d, 0xb0, 0xda, 0x0e, 0xa4, 0xa0, 0x8d, 0x0a, 0x76, 0x93, 0xed, 0x30, 0x92, 0xb3,
    0x8f, 0x25, 0x96, 0x66, 0xf7, 0x8e, 0xdd, 0xf6, 0xac, 0x1e, 0xae, 0xc2, 0x50, 0x1e, 0x05, 0xd7,
    0x11, 0x9f, 0xf9, 0x0a, 0xce, 0x5a, 0x2a, 0x3f, 0xb0, 0xa1, 0x90, 0x3f, 0x64, 0x22, 0x52, 0xc2,
    0x07, 0x4e, 0x11, 0xcc, 0xaa, 0x3e, 0x9e, 0xa2, 0xef, 0x83, 0x8a, 0x43, 0xdf, 0x3f, 0xb1, 0x6a,
    0x18, 0xb0, 0x65, 0x8d, 0xa8, 0xdd, 0x8b, 0xc3, 0x52, 0x31, 0x18, 0xfa, 0x7f, 0x69, 0xfe, 0x6f,
    0x00, 0xb6, 0x2d, 0x52, 0x40, 0xc4, 0x56, 0xee, 0x72, 0x9b, 0xba, 0xf0, 0xa5, 0x87, 0xab, 0xf0,
    0x97, 0x3a, 0x73, 0x3d, 0x9a, 0x62, 0xb1, 0x41, 0x67, 0x67, 0xdc, 0x0f, 0x3c, 0x69, 0x1c, 0x4a,
    0x76, 0xf3, 0x3d, 0x0f, 0x11, 0x09, 0x0c, 0xeb, 0xf5, 0x0f, 0x96, 0xa3, 0x95, 0x95, 0x19, 0x5c,
    0xca, 0x33, 0x36, 0x45, 0x87, 0x63, 0x4b, 0x82, 0x80, 0x13, 0x33, 0x27, 0x34, 0x12, 0x9b, 0x22,
    0x52, 0x3f, 0x80, 0x1b, 0x3c, 0x03, 0x87, 0xdf, 0xce, 0x76, 0x8f, 0xb2, 0x93, 0x5e, 0x98, 0x52,
    0xdc, 0x6e, 0x64, 0x3a, 0x5d, 0xb8, 0xed, 0xf7, 0xed, 0xe6, 0x90, 0x40, 0x0b, 0x1d, 0xa1, 0x67,
    0x70, 0x53, 0x75, 0x56, 0x59, 0xd2, 0xdc, 0xd6, 0xda, 0x92, 0x7d, 0x26, 0x1c, 0x31, 0x1a, 0xde,
    0x8d, 0x9e, 0x92, 0xdc, 0x07, 0xb0, 0x73, 0x82, 0xd3, 0xc0, 0xd3, 0x73, 0xa8, 0x94, 0xca, 0xb1,
    0xfb, 0xff, 0xe2, 0xf4, 0xfe, 0xa4, 0x62, 0x37, 0xb4, 0x9a, 0x34, 0x74, 0x63, 0x4d, 0xbc, 0x7d,
    0xc7, 0x14, 0x05, 0x60, 0xea, 0x42, 0xf3, 0xdd, 0xc5, 0x68, 0x9b, 0x4d, 0x8b, 0x51, 0xb5, 0x9a,
    0xfe, 0x30, 0x4b, 0x90, 0x0e, 0xdd, 0x9b, 0x45, 0x79, 0x55, 0xce, 0xcc, 0x45, 0xa0, 0x17, 0x84,
    0xce, 0x33, 0x2c, 0x79, 0x57, 0xf7, 0x2b, 0xe3, 0x58, 0x32, 0x31, 0xfa, 0x80, 0x5d, 0xdb, 0xc3,
    0x08, 0xc9, 0x24, 0x37, 0x97, 0x7b, 0x3d, 0x95, 0xb9, 0x5f, 0xfb, 0xeb, 0x00, 0x93, 0xe9, 0x84,
    0x22, 0x7c, 0x8e, 0x15, 0x56, 0x38, 0x20, 0xe9, 0x66, 0xf9, 0x2c, 0x1b, 0x36, 0x25, 0xd6, 0xcf,
    0xfe, 0x4f, 0x8d, 0xfb, 0x42, 0xeb, 0x5c, 0x72, 0x21, 0xe2, 0x83, 0x9e, 0x1e, 0x65, 0x5e, 0x0b,
    0xe0, 0x79, 0x7a, 0xc8, 0xd1, 0x37, 0xee, 0x83, 0xa8, 0x3b, 0xd7, 0x1d, 0x75, 0xc9, 0x50, 0x9b,
    0xd7, 0xe5, 0x2d, 0x2c, 0x59, 0x46, 0x0e, 0x56, 0x10, 0xd7, 0xf5, 0xca, 0x00, 0x22, 0x41, 0xb5,
    0x34, 0x4a, 0x4a, 0xe9, 0x80, 0x4d, 0x24, 0x01, 0xef, 0xbe, 0xbf, 0xb8, 0x1b, 0xcf, 0xd7, 0x9b,
    0x48, 0xb6, 0x4a, 0xbc, 0x11, 0x7f, 0xf1, 0xa4, 0xca, 0xa8, 0xda, 0xf6, 0xdc, 0xdb, 0xb7, 0xe6,
    0xc6, 0x5e, 0x2b, 0x08, 0xae, 0xb9, 0xfe, 0x09, 0xef, 0x82, 0x37, 0x38, 0xc2, 0x25, 0x3a, 0x1f,
    0x43, 0x33, 0x8d, 0xaf, 0x33, 0xee, 0xf9, 0x80, 0x8f, 0xb2, 0x62, 0xfa, 0x81, 0x2d, 0xd5, 0x79,
    0x30, 0xf9, 0x13, 0xcb, 0x83, 0x2e, 0x10, 0x8a, 0xc2, 0xcb, 0x43, 0x4e, 0x17, 0x11, 0x29, 0xc6,
    0x1a, 0x23, 0xcd, 0x1a, 0xfc, 0x21, 0x7a, 0x34, 0x63, 0x37, 0x4b, 0x61, 0x8a, 0x5d, 0xfa, 0x62,
    0x20, 0x6d, 0xaf, 0x93, 0x01, 0xfa, 0xff, 0x45, 0x43, 0x12, 0xe7, 0x92, 0x6c, 0xff, 0xb9, 0xc8,
    0x02, 0x98, 0x8f, 0xc5, 0x97, 0x3d, 0x45, 0x2b, 0x6f, 0x11, 0x1e, 0x0e, 0x0e, 0x4c, 0x42, 0x13,
    0x16, 0x8e, 0xff, 0xa9, 0x84, 0x3b, 0x47, 0x4e, 0x06, 0x45, 0xc3, 0xe3, 0xde, 0x5b, 0x78, 0x8a,
    0x56, 0x38, 0x05, 0xf8, 0x04, 0xfc, 0xae, 0x3c, 0x35, 0xc3, 0x16, 0xa6, 0xa2, 0xdb, 0x9e, 0x47,
    0xca, 0x29, 0x1b, 0x6e, 0x19, 0xf0, 0x1f, 0x2e, 0x74, 0x6e, 0x29, 0x3a, 0x77, 0x16, 0xf6, 0x65,
    0x78, 0x1e, 0x27, 0xdc, 0x22, 0x38, 0xb2, 0x6a, 0x89, 0x05, 0x88, 0x77, 0x6e, 0x5b, 0xbd, 0x86,
    0x83, 0x18, 0x15, 0x80, 0x21, 0x99, 0x75, 0xfc, 0x95, 0xa7, 0x00, 0xba, 0x5f, 0x53, 0x82, 0xab,
    0xf5, 0x74, 0x02, 0xe2, 0x9b, 0xf0, 0xa2, 0x30, 0xb8, 0x2c, 0xff, 0x61, 0xb6, 0x61, 0x93, 0xf8,
    0x84, 0x47, 0x07, 0x09, 0x36, 0xac, 0x63, 0xdf, 0xd0, 0x35, 0xaa, 0x7c, 0x40, 0xe6, 0x2f, 0x74,
    0x58, 0xa2, 0x43, 0x69, 0x2d, 0x37, 0x2a, 0xe8, 0x0e, 0xd4, 0x33, 0x87, 0xe6, 0xf7, 0xce, 0x54,
    0x96, 0x2b, 0xcd, 0x7e, 0xde, 0x5d, 0xaf, 0xf4, 0x00, 0x1b, 0x0d, 0xf1, 0x5b, 0x36, 0x14, 0x06,
    0xc6, 0x52, 0x84, 0x48, 0x1c, 0xaa, 0xb4, 0xf7, 0x03, 0x9f, 0x4c, 0x0b, 0x68, 0x88, 0x5c, 0x9d,
    0x1f, 0x1b, 0x69, 0x7e, 0x2d, 0x24, 0x89, 0x91, 0x98, 0xb7, 0x78, 0xe1, 0x32, 0x81, 0x53, 0xb1,
    0xb4, 0xa9, 0x4a, 0x09, 0xbc, 0xb1, 0x5f, 0xfe, 0x68, 0x5c, 0x22, 0x4d, 0x71, 0x9d, 0xfe, 0x83,
    0x6b, 0x18, 0x4e, 0xb0, 0x59, 0xc5, 0x15, 0x3b, 0x3a, 0xe4, 0x84, 0xba, 0xa7, 0x07, 0x42, 0x54,
    0x11, 0xbe, 0x54, 0x26, 0xa7, 0x0f, 0xab, 0x43, 0x03, 0x9f, 0xc1, 0x4d, 0xcc, 0xe8, 0xd4, 0x57,
    0x75, 0xe3, 0xc8, 0x3a, 0x0f, 0x95, 0x58, 0x67, 0xcc, 0xf6, 0xf7, 0xdb, 0x27, 0xd5, 0xc8, 0xd6,
    0x09, 0xed, 0x53, 0xef, 0x2f, 0x3c, 0xe0, 0xdf, 0xa4, 0xd6, 0xfb, 0x16, 0x95, 0xf1, 0x48, 0x02,
    0x52, 0x03, 0x91, 0xd4, 0xff, 0x3e, 0x02, 0x31, 0x08, 0x76, 0xa4, 0x96, 0x4e, 0xfa, 0xa9, 0x59,
    0xe6, 0xc1, 0xd7, 0x6b, 0x19, 0x35, 0x60, 0x3e, 0xea, 0x9c, 0x95, 0x38, 0x97, 0x23, 0x7b, 0x08,
    0x51, 0x4f, 0x77, 0x99, 0x82, 0x72, 0x75, 0xb5, 0x50, 0x90, 0x6b, 0xf9, 0x1b, 0x46, 0xb4, 0xb0,
    0xee, 0x62, 0x34, 0xb6, 0xa3, 0x9b, 0xee, 0x4c, 0xa9, 0xcb, 0x36, 0x75, 0x83, 0xc6, 0x42, 0x0d,
    0x19, 0xe1, 0x5e, 0x2e, 0x27, 0xf6, 0xc1, 0x78, 0x4d, 0xff, 0xf4, 0xe0, 0xce, 0xa1, 0xc5, 0xf1,
    0x37, 0xd1, 0x8a, 0x6c, 0x6e, 0x26, 0x45, 0xa2, 0xd4, 0x5d, 0x12, 0xe5, 0x45, 0x35, 0xdd, 0x58,
    0xf6, 0x1d, 0xd3, 0xca, 0xe1, 0x13, 0x9b, 0xb0, 0xd3, 0x85, 0xc5, 0xf2, 0xb7, 0xb3, 0x55, 0xf4,
    0x82, 0xb1, 0xfb, 0xe6, 0x3e, 0x23, 0x54, 0xe0, 0x24, 0x43, 0xb5, 0x4c, 0x0e, 0x49, 0xcc, 0x9b,
    0xbd, 0xb7, 0x4d, 0xdc, 0xfa, 0xe6, 0xd0, 0x76, 0xa9, 0x3c, 0x16, 0x3c, 0xc4, 0xf9, 0x5c, 0xcf,
    0x37, 0xd3, 0xd2, 0x1e, 0xc0, 0x51, 0xb1, 0x96, 0x75, 0xe2, 0xb7, 0x7e, 0x79, 0xca, 0x5c, 0xff,
    0xc3, 0x18, 0xb9, 0x7a, 0x64, 0xfb, 0x91, 0x02, 0xf2, 0x1a, 0xc1, 0x1d, 0x5c, 0x4c, 0x3b, 0x22,
    0x7b, 0x9f, 0x61, 0xe0, 0x97, 0x90, 0x95, 0x02, 0x48, 0xe4, 0xaa, 0xb5, 0x14, 0xb1, 0xd7, 0x35,
    0x34, 0x0f, 0x5b, 0xa3, 0x55, 0x05, 0x0a, 0xcb, 0xce, 0xbe, 0x13, 0x8e, 0xa8, 0x5c, 0xaf, 0xb9,
    0x0a, 0x29, 0x7c, 0x90, 0xb2, 0x6e, 0x1e, 0xf8, 0x00, 0x5a, 0x69, 0x1b, 0xf8, 0x7d, 0xe0, 0x8f,
    0x1b, 0xf5, 0x80, 0x17, 0x50, 0x3d, 0xef, 0x02, 0x2c, 0x56, 0xba, 0x14, 0x54, 0xaf, 0xb0, 0x6e,
    0xe3, 0xf3, 0xd9, 0x65, 0xdb, 0x2e, 0x52, 0xbb, 0x31, 0x63, 0x00, 0xa6, 0xd3, 0x7a, 0x4a, 0xbd,
    0x89, 0x56, 0xff, 0x21, 0xdb, 0x04, 0x95, 0xb8, 0xb4, 0xa0, 0x2c, 0x58, 0xc2, 0x87, 0x68, 0x25,
    0x01, 0xdb, 0xda, 0xee, 0x44, 0x6f, 0x6d, 0xc4, 0x04, 0xd9, 0x97, 0xea, 0xda, 0xc6, 0x4f, 0xce,
    0x55, 0x1c, 0x2b, 0x9d, 0x4b, 0xdd, 0x28, 0x7b, 0xc4, 0x80, 0x0f, 0x07, 0x9d, 0x58, 0xff, 0x0d,
    0x01, 0x4e, 0xe0, 0xff, 0x29, 0x55, 0x91, 0x21, 0x1c, 0x82, 0x9f, 0x6f, 0x3b, 0x37, 0xab, 0x26,
    0x6e, 0x77, 0xec, 0x88, 0x4c, 0x9d, 0xbf, 0x9f, 0xfc, 0xa7, 0xdd, 0xbf, 0xef, 0x2b, 0x33, 0x3c,
    0x33, 0x74, 0x72, 0xbf, 0x76, 0x8b, 0xa4, 0x79, 0x20, 0xe2, 0xd6, 0x33, 0xdb, 0x2d, 0x05, 0x23,
    0x1e, 0xc5, 0xe5, 0x5b, 0xd0, 0xbb, 0x63, 0xc7, 0xcb, 0x04, 0xbf, 0x12, 0x97, 0xe5, 0x5e, 0x5b,
    0x27, 0x25, 0xa9, 0x28, 0x97, 0xd1, 0x55, 0xa7, 0x00, 0x9e, 0x43, 0xaf, 0xc2, 0x18, 0xf1, 0xb4,
    0x1e, 0x05, 0xaa, 0xda, 0x38, 0x2f, 0x23, 0xfe, 0x39, 0x86, 0x8e, 0x35, 0x43, 0x88, 0xe1, 0x5b,
    0xe4, 0xb6, 0xb5, 0xcf, 0x63, 0x74, 0x1d, 0x00, 0x89, 0x24, 0x7c, 0xd6, 0x4b, 0xc2, 0x85, 0x03,
    0x6c, 0x04, 0x73, 0x63, 0xac, 0x72, 0x0f, 0xbf, 0x9e, 0x59, 0xa6, 0xd9, 0x7d, 0x76, 0xa6, 0xda,
    0x83, 0xd7, 0x86, 0xba, 0xd1, 0xd0, 0x5a, 0x90, 0xea, 0xf9, 0xf0, 0x61, 0xf3, 0x01, 0x4f, 0xf6,
    0x54, 0xa0, 0xe8, 0xbb, 0xc3, 0xa2, 0x96, 0x3a, 0xbf, 0xda, 0xba, 0x93, 0x40, 0xd5, 0xda, 0x2a,
    0xdb, 0xe8, 0x6e, 0xa3, 0x58, 0x45, 0x2a, 0x94, 0xa9, 0xfd, 0xf5, 0xf8, 0x9a, 0xea, 0x1d, 0x0a,
    0xfd, 0x8d, 0xfb, 0x86, 0x2b, 0xc8, 0xb4, 0x5b, 0xa7, 0x07, 0x4e, 0xed, 0x2d, 0x09, 0x3f, 0x50,
    0x4b, 0x84, 0x9d, 0x52, 0x8d, 0x72, 0x23, 0x31, 0x33, 0xea, 0x3c, 0x16, 0x61, 0x14, 0xf8, 0xd7,
    0x44, 0x1e, 0x93, 0x93, 0xc6, 0xb6, 0x0d, 0x6b, 0xc2, 0x79, 0x39, 0x3a, 0x48, 0xb3, 0x94, 0xd8,
    0x63, 0xca, 0x45, 0x3a, 0x41, 0xda, 0x6e, 0x8b, 0xc6, 0x2d, 0x1b, 0xd9, 0xb5, 0xf7, 0x70, 0xb5,
    0xea, 0x28, 0x47, 0x05, 0x72, 0xc5, 0x57, 0x71, 0x4f, 0x2c, 0x6f, 0x3c, 0x6c, 0xc7, 0xc6, 0x7e,
    0xd6, 0xd2, 0x7a, 0x08, 0x90, 0x59, 0xb7, 0x20, 0x26, 0xad, 0x0e, 0xdf, 0x0b, 0x3f, 0x47, 0x17,
    0xc8, 0xdb, 0xdc, 0x7e, 0xd8, 0xca, 0x7b, 0xbc, 0xee, 0xb2, 0xd7, 0xaf, 0x59, 0x20, 0xdd, 0x96,
    0x29, 0x68, 0x48, 0x51, 0x19, 0xbd, 0x6f, 0x74, 0xd1, 0x4a, 0x16, 0x0b, 0xef, 0xbc, 0x09, 0xaa,
    0xa4, 0x7d, 0xa4, 0xa7, 0x92, 0xcb, 0x03, 0x2f, 0x58, 0xcf, 0x9c, 0xfc, 0x29, 0x79, 0xd6, 0xbf,
    0xa0, 0xb6, 0xc5, 0x77, 0x3c, 0x24, 0xaa, 0xd1, 0x05, 0xfb, 0x86, 0x8e, 0x5f, 0x6f, 0x48, 0x05,
    0x61, 0xee, 0x94, 0x12, 0xe6, 0xb0, 0x5e, 0x7d, 0xe0, 0x9e, 0x6d, 0x26, 0xd9, 0xdf, 0x89, 0xc9,
    0xa4, 0x84, 0x5d, 0xcc, 0x82, 0xd3, 0x7a, 0x94, 0x97, 0xb7, 0x1c, 0x57, 0xa4, 0x15, 0x83, 0x6f,
    0xfb, 0xec, 0x75, 0xf7, 0x1d, 0x5e, 0x42, 0x54, 0x45, 0x2d, 0x0c, 0xe5, 0x9a, 0x57, 0xaa, 0x7f,
    0x27, 0xf9, 0x55, 0x13, 0xbb, 0x74, 0xc9, 0xd7, 0xec, 0xc4, 0x56, 0x9c, 0x58, 0x96, 0x16, 0xaf,
    0x5a, 0x7a, 0x3d, 0x19, 0x52, 0x16, 0x96, 0x8d, 0xfc, 0x4b, 0x0c, 0xe6, 0x0a, 0x05, 0x23, 0x1e,
    0x0e, 0x54, 0x62, 0x6e, 0x6f, 0xbc, 0x92, 0x89, 0xc1, 0xb4, 0xba, 0xf9, 0x26, 0x56, 0x46, 0xe7,
    0x24, 0xc0, 0xdc, 0x44, 0x9a, 0x8b, 0xb8, 0xcf, 0x56, 0x5c, 0x4b, 0x75, 0x57, 0x26, 0x75, 0x5c,
    0xe7, 0x99, 0x21, 0x11, 0x55, 0x38, 0xf5, 0xea, 0xf7, 0xb3, 0xab, 0x88, 0xea, 0x8f, 0xb9, 0xf3,
    0xf5, 0x47, 0xed, 0x75, 0x97, 0x1e, 0xab, 0xb8, 0x4b, 0x63, 0x4a, 0x57, 0x97, 0xb8, 0xb9, 0x09,
    0x92, 0xcd, 0x89, 0x18, 0x58, 0xcc, 0x7a, 0xd2, 0x97, 0xc7, 0x60, 0xb4, 0x88, 0xe0, 0x56, 0xf5,
    0x9c, 0x53, 0xbf, 0xff, 0x8c, 0x5a, 0x3a, 0x16, 0x17, 0x51, 0x09, 0x1d, 0xb2, 0x46, 0xe0, 0x48,
    0x96, 0xaa, 0x90, 0x35, 0x3c, 0x62, 0xfd, 0x6a, 0x39, 0xe2, 0x56, 0x2e, 0x56, 0x37, 0xc9, 0x18,
    0xc4, 0xa2, 0xe1, 0xa5, 0x39, 0x3d, 0xcd, 0x1d, 0xc1, 0x27, 0x32, 0xf6, 0xd3, 0x19, 0x96, 0xf3,
    0x15, 0xca, 0x4d, 0x74, 0x5d, 0x50, 0x2f, 0x1a, 0x59, 0x4b, 0x57, 0xe7, 0x7b, 0xf5, 0x0d, 0x77,
    0xde, 0xb6, 0x22, 0xc2, 0x64, 0xce, 0xa0, 0x47, 0xdb, 0xb8, 0xc6, 0x47, 0x7f, 0xf3, 0x53, 0x71,
    0x39, 0x85, 0xf4, 0xfe, 0xd9, 0x1b, 0x55, 0xd0, 0xb0, 0x64, 0x13, 0x2b, 0x14, 0x0d, 0x69, 0x31,
    0x4d, 0xad, 0xb0, 0xa2, 0xf6, 0x2d, 0x55, 0x8c, 0xaa, 0xcc, 0x1e, 0x1c, 0xe5, 0x80, 0xd5, 0x8c,
    0x19, 0x45, 0x23, 0x9c, 0x2e, 0xd4, 0x89, 0x09, 0xb3, 0x1b, 0x43, 0x26, 0x4f, 0x1a, 0xfc, 0xfb,
    0x36, 0x07, 0x35, 0xc9, 0x35, 0xb0, 0xb8, 0x6e, 0x89, 0x6f, 0xde, 0x6a, 0xd0, 0x56, 0xc5, 0xd0,
    0x5d, 0x11, 0x84, 0xe9, 0xa5, 0xf0, 0xd7, 0xf8, 0xe3, 0x72, 0xf4, 0x56, 0xdc, 0xe9, 0x9e, 0x55,
    0xf1, 0xb4, 0xba, 0xbc, 0x7f, 0xf5, 0xf2, 0x6b, 0xcc, 0x36, 0xf3, 0xe3, 0x28, 0x3c, 0xda, 0x41,
    0xce, 0x29, 0xed, 0x19, 0x4e, 0x48, 0xdb, 0x5a, 0x9e, 0x39, 0xc7, 0x5e, 0x22, 0x8f, 0x27, 0x25,
    0x81, 0x9e, 0x03, 0xd1, 0x09, 0x3e, 0x05, 0x63, 0x12, 0x44, 0xfd, 0xe2, 0x44, 0x70, 0xcf, 0xb8,
    0xc8, 0x26, 0x91, 0x77, 0x34, 0xfe, 0x28, 0x00, 0x75, 0x78, 0x92, 0x66, 0xa4, 0xb4, 0x8d, 0x00,
    0x4a, 0x99, 0xf9, 0xba, 0x94, 0x70, 0xe8, 0xa0, 0x33, 0x69, 0x75, 0x83, 0x1c, 0x6f, 0x73, 0x10,
    0xb0, 0xdc, 0xda, 0x93, 0x14, 0x2d, 0xe5, 0x48, 0x1b, 0x1e, 0x6e, 0xe1, 0x43, 0xf7, 0xad, 0x9c,
    0xf5, 0xce, 0xe6, 0xab, 0x02, 0x8f, 0xed, 0xe5, 0xb3, 0x45, 0xcb, 0xd7, 0x60, 0x73, 0x9f, 0x69,
    0xe1, 0x57, 0xab, 0x69, 0x4f, 0x45, 0x73, 0xd8, 0x96, 0x33, 0x47, 0x4b, 0x8a, 0xb6, 0x3b, 0x3e,
    0x55, 0x78, 0xbe, 0x2d, 0xe2, 0x83, 0x73, 0x56, 0x14, 0x3d, 0xea, 0x1c, 0x29, 0xd3, 0x65, 0x48,
    0xdc, 0x9a, 0x88, 0xa4, 0xe6, 0xfe, 0x07, 0x51, 0x1c, 0x16, 0x57, 0xe2, 0x42, 0xe4, 0xbe, 0x5b,
    0x39, 0x82, 0x86, 0xe1, 0x57, 0x51, 0xe8, 0x9f, 0x49, 0x36, 0xce, 0xdc, 0x0a, 0xc6, 0x8a, 0x98,
    0x23, 0xc7, 0x8e, 0x9d, 0x0d, 0xbe, 0x79, 0x47, 0x1f, 0xae, 0x2a, 0xac, 0x66, 0xc2, 0x4f, 0xf6,
    0x7f, 0x56, 0x32, 0x2a, 0x0e, 0xde, 0x4b, 0x3c, 0x82, 0x75, 0xf3, 0xe0, 0x97, 0xb7, 0x86, 0x65,
    0xe5, 0x4c, 0x6d, 0xf6, 0xe2, 0x3e, 0x52, 0x6e, 0xe3, 0x2d, 0x2f, 0xfd, 0xfb, 0xc6, 0xf4, 0x11,
    0x68, 0xef, 0x45, 0xa7, 0xc8, 0xa8, 0xb2, 0xda, 0x20, 0x93, 0x65, 0x20, 0x25, 0x30, 0x0f, 0x9b,
    0x44, 0x4f, 0x58, 0x68, 0xf7, 0xef, 0xe1, 0x90, 0xec, 0xfe, 0xee, 0x01, 0x70, 0x73, 0x83, 0xcd,
    0x9e, 0x86, 0xf2, 0x85, 0xfd, 0x16, 0x93, 0x7c, 0x82, 0x26, 0x06, 0x44, 0xdc, 0x33, 0xcc, 0x4a,
    0xfa, 0x3c, 0x7c, 0xc6, 0x34, 0x0e, 0xfe, 0xbb, 0xd8, 0x40, 0x97, 0x4d, 0xe7, 0x42, 0x75, 0x7f,
    0x5d, 0x7a, 0x31, 0x58, 0xf7, 0xd7, 0x24, 0x32, 0x6d, 0xc7, 0x14, 0xe8, 0x67, 0x36, 0xbe, 0x45,
    0x63, 0xdc, 0x7f, 0xf9, 0x15, 0x38, 0xe4, 0xfc, 0x9e, 0xa6, 0x78, 0x61, 0x4b, 0x51, 0xfc, 0x31,
    0xab, 0x3d, 0xd2, 0x7c, 0xa6, 0xdc, 0x10, 0xf6, 0x89, 0x3c, 0xf5, 0xec, 0x52, 0x2a, 0x4b, 0xa4,
    0x9f, 0xb3, 0xf9, 0x2f, 0x76, 0x21, 0xc0, 0x44, 0xb9, 0xde, 0xf2, 0xc4, 0xc0, 0xd6, 0x70, 0x15,
    0xf7, 0x2d, 0x98, 0xe3, 0x6a, 0x92, 0x47, 0x16, 0xd6, 0xc2, 0xbd, 0x59, 0x09, 0xa8, 0xce, 0xad,
    0x1c, 0xab, 0x55, 0xee, 0x8a, 0x7a, 0xb6, 0x0e, 0xb6, 0x79, 0x1c, 0x47, 0xf2, 0x4f, 0x6f, 0x18,
    0xff, 0xab, 0x73, 0x24, 0x2c, 0x4c, 0x71, 0x8d, 0xc4, 0x7f, 0x34, 0x61, 0x0b, 0x2e, 0x0b, 0x7f,
    0x66, 0x7f, 0xe3, 0xbb, 0x4e, 0x93, 0x5c, 0x3f, 0x24, 0x5c, 0x26, 0xa9, 0x57, 0xc0, 0x08, 0x5c,
    0xd2, 0xb9, 0x88, 0xdd, 0xe4, 0x49, 0xfc, 0xab, 0x8c, 0x4c, 0xed, 0x17, 0x1e, 0x1d, 0xdd, 0x62,
    0x7c, 0x6f, 0x31, 0x9a, 0x38, 0x3a, 0xca, 0x94, 0x5b, 0xe3, 0x2d, 0x47, 0xef, 0x9a, 0x14, 0xc6,
    0x76, 0x30, 0x46, 0x62, 0x56, 0x32, 0x85, 0x02, 0x59, 0x22, 0x5e, 0xa3, 0x9b, 0xbf, 0x1c, 0x10,
    0x34, 0x74, 0x23, 0xd4, 0x0b, 0x92, 0x8c, 0xe3, 0x49, 0x84, 0x12, 0x60, 0xd1, 0x60, 0xf5, 0xe4,
    0x2e, 0x3d, 0x98, 0x2c, 0x59, 0xe3, 0x92, 0x40, 0xef, 0xda, 0x6f, 0xb4, 0x8a, 0xb8, 0xd4, 0x65,
    0xd2, 0x8b, 0x62, 0x67, 0x3c, 0xe7, 0x74, 0x87, 0x7f, 0x0f, 0x69, 0x51, 0x4a, 0x87, 0xfb, 0x10,
    0xec, 0x7f, 0x9f, 0xd0, 0x80, 0x82, 0x3e, 0x2a, 0xa9, 0x11, 0xc9, 0xd3, 0x8a, 0xe1, 0x22, 0x4b,
    0x5e, 0xdb, 0xbb, 0x8d, 0x7a, 0x29, 0xf1, 0x04, 0xcb, 0xf8, 0xae, 0xe0, 0xe9, 0x09, 0x1e, 0xe7,
    0x84, 0x8d, 0xa6, 0xce, 0xbd, 0xde, 0x9e, 0x5d, 0x4f, 0x01, 0x6e, 0x96, 0xc6, 0xab, 0x6c, 0x17,
    0x6c, 0xab, 0x41, 0xb5, 0xbf, 0xf0, 0xcc, 0xb9, 0x0c, 0xe0, 0xe3, 0x7b, 0xfa, 0xc8, 0xbb, 0x8b,
    0xa7, 0x15, 0x36, 0x9a, 0x45, 0x18, 0x6d, 0x88, 0xa9, 0x26, 0x1e, 0x4e, 0x50, 0x37, 0xcf, 0x28,
    0x0e, 0x7f, 0xbb, 0xe4, 0xc8, 0xa4, 0x91, 0x00, 0xdb, 0x8e, 0x0f, 0xa8, 0x59, 0x93, 0x8b, 0x15,
    0x41, 0x39, 0xa9, 0xfb, 0x54, 0x2c, 0x00, 0xdb, 0xe2, 0xd4, 0x03, 0xfb, 0x2c, 0xb1, 0x09, 0xd3,
    0xf1, 0xf1, 0x1a, 0x8f, 0xe6, 0x9d, 0xae, 0xc6, 0xd1, 0xd5, 0xa6, 0xba, 0x90, 0x5f, 0x12, 0x4c,
    0xed, 0xa3, 0x15, 0xc0, 0x51, 0x2e, 0x73, 0xfd, 0x14, 0x0c, 0x76, 0xff, 0x01, 0x3a, 0xfd, 0x2a,
    0xe1, 0x4b, 0xac, 0x5a, 0x76, 0xe8, 0xd9, 0x6c, 0xdc, 0x65, 0x8d, 0xf4, 0x02, 0x59, 0xdb, 0xdf,
    0xd7, 0xa9, 0xc0, 0xb3, 0x4c, 0x87, 0x81, 0xfb, 0x7f, 0x17, 0xba, 0xbb, 0x7a, 0x43, 0xc0, 0xd1,
    0xbb, 0x56, 0x02, 0xea, 0xce, 0x23, 0x4f, 0x95, 0xa8, 0x02, 0xb4, 0x70, 0x22, 0xf4, 0xf3, 0x1a,
    0x35, 0xf9, 0x68, 0xf5, 0xcc, 0xfc, 0x99, 0x53, 0x75, 0xc7, 0x72, 0xac, 0x7f, 0xc3, 0xa8, 0x99,
    0x6a, 0xec, 0x44, 0x89, 0x54, 0x00, 0x55, 0xf0, 0x74, 0xde, 0xea, 0x8e, 0x06, 0x7d, 0xbf, 0x3b,
    0xca, 0x6f, 0x81, 0x49, 0x85, 0x23, 0xbb, 0xb9, 0x24, 0x25, 0x82, 0x31, 0xa1, 0xcb, 0xa2, 0xcb,
    0x9c, 0xf4, 0xe4, 0xef, 0x76, 0x94, 0x25, 0x58, 0x40, 0x15, 0x67, 0x79, 0xf7, 0x3f, 0xed, 0x70,
    0xfe, 0xb5, 0xb5, 0x46, 0x30, 0x17, 0x54, 0xb3, 0x0e, 0x54, 0xf4, 0x95, 0xd6, 0x31, 0x1a, 0x5e,
    0x6e, 0xbd, 0x5c, 0x85, 0x06, 0x47, 0x45, 0xda, 0x0e, 0x08, 0xe4, 0x48, 0x73, 0xb9, 0xa9, 0x2c,
    0xbd, 0x4b, 0x90, 0x87, 0x82, 0xc5, 0x17, 0x90, 0x09, 0xad, 0x3c, 0x7c, 0x7b, 0x5a, 0x1d, 0x24,
];

#[test]
fn test_known_piece() {
    let expanded_iv = [3u8; 32];
    let piece = [5u8; 4096];

    let layers = 1;
    let mut encoding = piece.clone();
    encode(&mut encoding, expanded_iv, layers).unwrap();
    assert_eq!(encoding, CORRECT_ENCODING);
    let mut decoding = encoding.clone();
    decode(&mut decoding, expanded_iv, layers);

    assert_eq!(piece.to_vec(), decoding.to_vec());
}

#[cfg(feature = "cuda")]
#[test]
fn test_cuda_single_piece() {
    if check_cuda() {
        let expanded_iv = vec![3u8; 32];
        let mut piece = vec![5u8; 4096];

        gpu_test_single_piece(&mut piece, expanded_iv, 1);
        assert_eq!(piece, CORRECT_ENCODING);
    } else {
        panic!("no Nvidia card detected, skip test_cuda_single_piece");
    }
}

#[cfg(feature = "cuda")]
#[test]
fn test_cuda_batch() {
    if check_cuda() {
        let expanded_ivs = vec![3u8; 1024 * 32]; // 1024 expanded_ivs
        let mut pieces = vec![5u8; 1024 * 4096]; // 1024 pieces

        gpu_encode(&mut pieces, expanded_ivs, 1).unwrap();
        for i in 0..1024 {
            assert_eq!(pieces[i * 4096..(i + 1) * 4096], CORRECT_ENCODING);
        }
    } else {
        panic!("no Nvidia card detected, skip test_cuda_single_piece");
    }
}
