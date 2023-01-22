use std::collections::BTreeMap;
use std::collections::HashMap;
use crate::pixel::Pixel;
use crate::{HUFF_CODE_TAG, RAW_PIXEL_BYTES_SIZE};


#[derive(Clone, Copy)]
pub struct Node {
    //code: u32,
    // There will be a size where its more logical to store the whole pixel than a reference really low in the tree
    freq: u32,
    pixel: Pixel
}
pub struct Switch {
    freq: u32,
    left: NodeTypes,
    right: NodeTypes,
}
pub enum NodeTypes {
    Node(Node),
    Switch(Box<Switch>),
}

// TODO: The current implementation iterates 3 times over the pixels. Once to sort them out by freq, once to build the tree, and once to set the codes. Lower this.
// TODO: If the pixel is too long and its better to just store the whole pixel, skip it from the tree

/// `build_huffman_tree` function takes an vec of pixels as arguments.
/// It iterates over them an sets an order of frequency and then builds a tree
/// where items to the left have an index + 0 and to the right + 1
/// allowing a branch to be called by a single string of bits

pub fn build_huffman_tree(pixels: &Vec<Pixel>) -> HashMap<Pixel, u32> {
    let mut freq_map: HashMap<Pixel, u32> = HashMap::new();

    // Count the frequency of each pixel
    for pixel in pixels {
        *freq_map.entry(pixel.clone()).or_insert(0) += 1;
    }

    // Sort the pixels
    let mut sorted_pixels: Vec<(&Pixel, &u32)> = freq_map.iter().collect();
    sorted_pixels.sort_by_key(|&(_, &frequency)| frequency);

    // Set the base node
    let mut curr_switch = Switch {
        freq: sorted_pixels[0].1 + sorted_pixels[0].1,
        left: NodeTypes::Node(Node {
            //code: 0,
            freq: *sorted_pixels[1].1,
            pixel: *sorted_pixels[1].0,
        }),
        right: NodeTypes::Node(Node {
            //code: 0,
            freq: *sorted_pixels[0].1,
            pixel: *sorted_pixels[0].0,
        }),
    };
    sorted_pixels.drain(..2);

    // Build the Huffman tree
    while sorted_pixels.len() > 0 {
        // So if the second item of sorted_pixels is smaller than the curr_switch item,
        // a new switch will be generated containing all the items smaller than curr_switch
        // then both will be joined, else, the first item of sorted_pixels will be added to a new switch
        if (sorted_pixels.len() > 1) && (curr_switch.freq > *sorted_pixels[1].1) {
            println!("Yeah, so just finish it");
        } else {
            // So we make a new node into the tree containing the sum of the freq and we remove the first item
            curr_switch = Switch {
                freq: curr_switch.freq + sorted_pixels[0].1,
                left: NodeTypes::Node(Node {
                    //code: 0,
                    freq: *sorted_pixels[0].1,
                    pixel: *sorted_pixels[0].0,
                }),
                right: NodeTypes::Switch(Box::new(curr_switch)),
            };
            sorted_pixels.remove(0);
        }
    }
    let mut code_map: HashMap<Pixel, u32> = HashMap::new();
    set_code(&mut code_map, &mut curr_switch.left, 0);
    set_code(&mut code_map, &mut curr_switch.right, 1);

    code_map
}

fn set_code(map: &mut HashMap<Pixel, u32>, t: &mut NodeTypes, c: u32) {
    if c < (RAW_PIXEL_BYTES_SIZE) { // -1 for the huffman tag size
        match t {
            NodeTypes::Node(node) => {
                map.entry(node.pixel).or_insert(c);
                //node.code = c;
            },
            NodeTypes::Switch(switch) => {
                let a = c.clone(); //FIXME
                set_code(map, &mut switch.left, c * 2);
                set_code(map, &mut switch.right, a * 2 + 1);
            },
        }
    }
}