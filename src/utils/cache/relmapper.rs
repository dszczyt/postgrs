use crate::types::oid::{DEFAULTTABLESPACE_OID, Oid};
use std::{mem::{size_of, transmute}, fs::File, io::{Read, BufReader}};
use super::inval::get_database_path;
use crate::utils::init::globals::data_dir;

const FILENAME: &str = r#"pg_filenode.map"#;
const MAGIC: i32 = 0x592717; // version ID value
const MAX_MAPPINGS: usize = 62; // 62 * 8 + 16 = 512

pub type PgCrc32c = u32;

#[derive(Debug)]
pub struct RelMapping {
    pub mapoid: Oid, // OID of a catalog
    pub mapfilenode: Oid, // its filenode number
}

// #[repr(C)]
// #[derive(Debug)]
pub struct RelMapFile {
    pub magic: i32,			/* always RELMAPPER_FILEMAGIC */
	pub num_mappings: i32,	/* number of valid RelMapping entries */
    pub mappings: [RelMapping; MAX_MAPPINGS],
	pub crc: PgCrc32c,			/* CRC of all above */
	pub pad: i32,			/* to make the struct size be 512 exactly */
}

impl RelMapFile {
    pub fn load(/*shared: bool*/) -> Result<Self, String> {
        let mut r = [0u8; size_of::<RelMapFile>()];
        let mapfilename = format!(
            "{}/{}/{}",
            data_dir(),
            get_database_path(1, DEFAULTTABLESPACE_OID)?, 
            FILENAME
        );
        let file = File::open(&mapfilename).unwrap();
        let mut reader = BufReader::new(file);
        reader.read_exact(&mut r).unwrap();
        
        let relMapFile: Self = unsafe {transmute(r)};
        if relMapFile.magic != MAGIC ||
            relMapFile.num_mappings < 0 ||
            relMapFile.num_mappings > MAX_MAPPINGS as i32
        {
            return Err(format!("Relation mapping file \"{}\" contains invalid data", mapfilename))
        }
        Ok(relMapFile)
    }
}