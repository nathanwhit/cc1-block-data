use cc1_block_data::decode_blocks;

fn main() {
    let data = std::fs::read("./blockdata.bincode").unwrap();

    let blocks = decode_blocks(&data).unwrap();

    for b in blocks.iter().take(10) {
        println!("{b:#?}");
    }
}
