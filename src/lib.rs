extern crate core;

mod pixel;
mod huffman;
mod bitwise_buffer;
mod test;
mod format_translation;

use crate::bitwise_buffer::{BitwiseBuffer, EncodedBit};
use crate::huffman::build_huffman_tree;
use crate::pixel::Pixel;


pub const RAW_PIXEL_BYTES_SIZE: u32 = 27; // The size that a raw pixel will have in hex 8*3+3
const SEARCH_BUFFER_SIZE: i16 = 12;

const HUFF_CODE_TAG: u8 = 0; // 0 - This one is a tag attached as they won't always be 8bit long
const PREV_PIXEL_DIFF_TAG: u8 = 2; // 10 in binary
const RAW_PIXEL_TAG: u8 = 7; // 111

// Basic information to be store in the header of the file
pub struct ImageDesc {
    width: i8,
    height: i8,
    //rgba?
}

pub struct PrevPixel {
    list: Vec<usize>,
    prev: PrevPixelNode,
}
pub enum PrevPixelNode {
    Node { prev: Box<PrevPixel> },
    End,
}

/**
Encode receives 3
 **/
pub fn encode(mut data: Vec<Pixel>, image_desc: ImageDesc) {
    let (huffman, tree) = build_huffman_tree(&data);
    // So a Pixel will start by searching itself in the huffman code
    // then it will check if it can be store as a diff to the prev px
    // and if it would be larger or smaller
    // For now, (As more optimisation will come) it will otherwise store the whole pixel

    let mut buffer = BitwiseBuffer::new();
    let mut previous_px: Pixel = data[0];

    // TODO: A search buffer which stores all possible combinations of a pixel to be stored as
    //let mut search_buffer: Vec<Pixel> = vec![data[0]];

    // We treat the first px unique as there is no search buffer of previous pixel
    // FIXME: For now, it will check the huffman tree or just store it rawly
    match huffman.get(&previous_px) {
        None => { buffer.push_bits(&px_to_encoded_raw_px(&previous_px)) },
        Some(i) => { buffer.push_bits(i); } //TODO: Not checking if first px is smaller than 25 bits
    };
    data.remove(0);

    for (i, px) in data.iter().enumerate() {

        // So either it is better to store the whole pixel, or store the huffman code
        let huffman_code = huffman.get(px);
        let diff_to_prev_pixel = match diff_to_pixel(&previous_px, &px) {
            None => { 0 }
            Some(i) => { i }
        };

        // So if huffman_code is equal or larger than a bit we'll check if there is a small gap
        // to the previous pixel. If there is one (In which each channel is between -2..1) we use
        // that over the huffman_code as it doesn't need go over the hash function later in decoding
        match huffman_code {
            Some(mut i) => {
                if diff_to_prev_pixel != 0 && i.len() > 1 {
                    buffer.push_bits(&vec![ EncodedBit { data: PREV_PIXEL_DIFF_TAG, bit_len: 2 }, EncodedBit { data: diff_to_prev_pixel, bit_len: 6 } ])
                } else {
                    buffer.push_bits(i);
                }
            }
            None => {
                if diff_to_prev_pixel != 0 {
                    buffer.push_bits(&vec![ EncodedBit { data: PREV_PIXEL_DIFF_TAG, bit_len: 2 }, EncodedBit { data: diff_to_prev_pixel, bit_len: 6 } ])
                } else {
                    buffer.push_bits(&px_to_encoded_raw_px(&previous_px))
                }
            }
        }

        // So it will consider all possible patterns repeated in the last X pixels to get the best
        // possible pixel in terms of off set. Example repeat previous, or repeat Y pattern at N index
        /*let offset_indexes: Vec<usize> = search_buffer.iter().enumerate()
            .filter(|(_, x)| px == *x)
            .map(|(x,_)| x).collect();
        match offset_indexes.len() {
            0 => {

            },
            1 => {
                if offset_indexes[0] < 32 {

                }
            },
            _ => {

            }
        }*/
        //println!("i{}", i);
        //offset_indexes.for_each(|(x)| println!("{},", x));
        // match offset_index {
        //     Some(offset) => {
        //         println!("dupe at index {} ; {}/{}/{}", offset, px.r, px.g, px.b);
        //         if offset == previousOffset {
        //             println!("Two dupes")
        //         }
        //         previousOffset = offset;
        //     },
        //     _ => {
        //
        //     }
        // }
        //search_buffer.remove(0);
        //search_buffer.push(Pixel {r: px.r, g: px.g, b: px.b });
    }
}

fn px_to_encoded_raw_px(px: &Pixel) -> Vec<EncodedBit> {
    vec![
        EncodedBit { data: RAW_PIXEL_TAG, bit_len: 3 },
        EncodedBit { data: px.r, bit_len: 8 },
        EncodedBit { data: px.g, bit_len: 8 },
        EncodedBit { data: px.b, bit_len: 8 },
    ]
}

fn diff_to_pixel(from: &Pixel, to: &Pixel) -> Option<u8> {
    let r = (from.r as i16) - (to.r as i16) + 2;
    // As to not waste time making the next two items, because it will be the same testing all together
    if r < 0 || r > 3 { return None; }
    let g = (from.g as i16) - (to.g as i16) + 2;
    if g < 0 || g > 3 { return None; }
    let b = (from.b as i16) - (to.b as i16) + 2;
    if b < 0 || b > 3 { return None; }
    Some((r as u8) + (g as u8) * 4 + (b as u8) * 8) // 00 (16) 00 (8) 00 (4) - 2^0, 2^2, 2^3
}