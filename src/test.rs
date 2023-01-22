#[cfg(test)]
mod tests {
    use std::fmt::format;
    use std::ops::Add;
    use crate::{encode, ImageDesc};
    use crate::huffman::{build_huffman_tree, NodeTypes};
    use crate::pixel::Pixel;
    use super::*;

    #[test]
    fn it_works() {
        encode(vec![Pixel { r: 0, g: 0, b: 255, }, Pixel { r: 0, g: 0, b: 255, }, Pixel { r: 0, g: 2, b: 255, }], ImageDesc { width: 0, height: 0 });
        if let Some(i) = diff_to_pixel(&Pixel {
            r: 0,
            g: 0,
            b: 3,
        }, &Pixel{
            r: 2,
            g: 2,
            b: 1,
        }) {
            println!("{}:{}:{}", i.0, i.1, i.2)
        }
    }
    fn diff_to_pixel(from: &Pixel, to: &Pixel) -> Option<(u8, u8, u8)> {
        let r = (from.r as i16) - (to.r as i16) + 2;
        // As to not waste time making the next two items, because it will be the same testing all together
        if r < 0 || r > 3 { return None; }
        let g = (from.g as i16) - (to.g as i16) + 2;
        if g < 0 || g > 3 { return None; }
        let b = (from.b as i16) - (to.b as i16) + 2;
        if b < 0 || b > 3 { return None; }
        Some(((r) as u8, (g) as u8, (b) as u8))
    }
    /*#[test]
    fn huffman_tree() {
        let pixels = vec![
            Pixel { r: 1, g: 2, b: 255 },

            Pixel { r: 0, g: 2, b: 255 }, Pixel { r: 0, g: 2, b: 255 },

            Pixel { r: 255, g: 0, b: 0 }, Pixel { r: 255, g: 0, b: 0 }, Pixel { r: 255, g: 0, b: 0 }, Pixel { r: 255, g: 0, b: 0 },

            Pixel { r: 0, g: 255, b: 0 },  Pixel { r: 0, g: 255, b: 0 }, Pixel { r: 0, g: 255, b: 0 }, Pixel { r: 0, g: 255, b: 0 },
            Pixel { r: 0, g: 255, b: 0 },  Pixel { r: 0, g: 255, b: 0 }, Pixel { r: 0, g: 255, b: 0 }, Pixel { r: 0, g: 255, b: 0 },

            Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 },
            Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 },
            Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 },
            Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 }, Pixel { r: 0, g: 225, b: 0 },
        ];
        let huffman_tree = build_huffman_tree(&pixels);

        // Read and log the tree
        let treeVals: String = String::new();
        read_tree(&treeVals, NodeTypes::Switch(Box::new(huffman_tree)));

    }

    fn read_tree(tree_vals: &String, t: NodeTypes) {
        match t {
            NodeTypes::Node(node) => {
                //tree_vals.push_str(format!("{}:{}/{}/{};", node.code, node.pixel.r, node.pixel.g, node.pixel.b));
                println!("{};{} - {}: Pixel {}/{}/{}", node.code, format!("{:b}", node.code), node.freq, node.pixel.r, node.pixel.g, node.pixel.b);
            },
            NodeTypes::Switch(switch) => {
                read_tree(&tree_vals,switch.left);
                read_tree(&tree_vals, switch.right);
            },
        }
    }*/
}