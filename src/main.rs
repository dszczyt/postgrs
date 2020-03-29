extern crate crc;
#[macro_use] extern crate memoffset;

mod utils;
mod types;
use utils::init::globals;
use utils::cache::relmapper::RelMapFile;

fn main() {
    println!("PG_DATA: {}", globals::data_dir());

    let rel_map_file = RelMapFile::load().unwrap();
    println!("{:X?} {:?}", rel_map_file.magic, rel_map_file.num_mappings);

    for num_mapping in 0..rel_map_file.num_mappings {
        let mapping = rel_map_file.mappings.get(num_mapping as usize).unwrap();
        println!("{:?}", mapping);
    }
}
