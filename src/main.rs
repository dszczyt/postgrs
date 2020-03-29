extern crate crc;

mod utils;
mod types;
use utils::init::globals;
use utils::cache::relmapper::RelMapFile;

fn main() {
    println!("PG_DATA: {}", globals::data_dir());

    let relMapFile = RelMapFile::load().unwrap();
    println!("{:X?} {:?}", relMapFile.magic, relMapFile.num_mappings);

    for num_mapping in 0..relMapFile.num_mappings {
        let mapping = relMapFile.mappings.get(num_mapping as usize).unwrap();
        println!("{:?}", mapping);
    }
}
