# ZIC
A work in progress image format which is both fast, and small.
Focus intended towards decompression speed.

This is my first Rust project, so there will be many fixes of every line of code upcoming.

## Compression
So pixels are basically 24 bits, 8 bits for each of channel of `rgb`.
(To be added the alpha channel, making it 32 bits)

An image should include a header, in which it contains information such as:
- Width
- Height
- RGB-RGBA

Followed by the data

And at the end, the huffman tree (Or maybe straight after the header)

The idea is to implement somewhat of an `lzss` compression

## Tags
The current tags for each pixel type may be changed:
- 0 | Huffman input
- 1 | 0 Difference previous px of -2..1
- 1 | 10 (May change) At index 0..255, and repeat 1..32 (2bit)
- 1 | 11 Pixel


## TODO:
- [ ] Finish the encoder
  - [ ] Make the lzss compression
    - [ ] Search buffer
    - [ ] Compare all possible items
    - [ ] Select most useful for the repeat algorithm
  - [ ] Make a way to store the file
    - [ ] Store the header, tree and data
- [ ] Make a decoder
- [ ] Make a translation layer to PNG


### Notes:
- The project may not be a reliable way of image compression
- The `bitwise_buffer` is designed so that the items of the image are not forced to be 8 bit, as it will manage to move the overflow of items to the next bit. It will also fill out (TO BE DONE) the remaining characters to make the image final size be `8*k`

### Why Zic?
`Zic` stands for Zick image compression format.

