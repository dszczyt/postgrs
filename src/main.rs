// #![feature(bool_to_option)]

extern crate crc;
#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate bitfield;

// #[macro_use]
// extern crate pest_derive;

// #![feature(plugin)]
// #![plugin(rustlex)]
// #[allow(plugin_as_library)] extern crate rustlex;

mod pg_config;

mod backend;

mod access;
mod parser;
mod storage;
mod types;
mod utils;
// use parser::sql::{SQLParser, Rule};
use parser::parser::raw_parser;
use std::{
    fs::{self, File},
    mem::size_of,
};
use storage::page::bufpage::PageHeaderData;
use utils::cache::relmapper::RelMapFile;
use utils::init::globals;

fn main() {
    println!("PG_DATA: {}", globals::data_dir().to_str().unwrap());

    let rel_map_file = RelMapFile::load().unwrap();
    println!("{:X?} {:?}", rel_map_file.magic, rel_map_file.num_mappings);

    for num_mapping in 0..rel_map_file.num_mappings {
        let mapping = rel_map_file.mappings.get(num_mapping as usize).unwrap();
        let path = mapping.get_path().unwrap();

        println!("{:?} {}", mapping, path.to_str().unwrap());

        let file_length = fs::metadata(&path).unwrap().len() as usize;
        if file_length < size_of::<PageHeaderData>() {
            continue;
        }
        // let mut file = File::open(&path).unwrap();
        println!("{:?}", PageHeaderData::from_file(path));
    }

    raw_parser("🙈🙉🙊💥ÿ".to_owned());
    println!("{:?}", raw_parser(" Select  ; ".to_owned()));

    // let parser = SQLParser::parse(Rule::statement, "SeLeCt -- XXX\n  -12;");
    // println!("{:?}", parser);
}
