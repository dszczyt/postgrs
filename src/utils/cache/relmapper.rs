use crate::types::oid::{DEFAULTTABLESPACE_OID, Oid};
use crate::utils::init::globals::data_dir;
use crc::crc32;
use std::{mem::{size_of, transmute}, fs::File, io::{Read, BufReader}};
use super::inval::get_database_path;
use std::path::PathBuf;

const FILENAME: &str = "pg_filenode.map";
const MAGIC: i32 = 0x592717; // version ID value
const MAX_MAPPINGS: usize = 62; // 62 * 8 + 16 = 512

pub type PgCrc32c = u32;

#[repr(C)]
#[derive(Debug)]
pub struct RelMapping {
    pub mapoid: Oid, // OID of a catalog
    pub mapfilenode: Oid, // its filenode number
}

impl RelMapping {
    pub fn get_path(&self) -> Result<PathBuf, String> {
        let mut filepath = PathBuf::from(data_dir());
        filepath.push(get_database_path(1, DEFAULTTABLESPACE_OID)?);
        filepath.push(self.mapfilenode.to_string());
        Ok(filepath)
    }
}

#[repr(C)]
// #[derive(Debug)]
pub struct RelMapFile {
    pub magic: i32,			/* always RELMAPPER_FILEMAGIC */
	pub num_mappings: i32,	/* number of valid RelMapping entries */
    pub mappings: [RelMapping; MAX_MAPPINGS],
	pub crc: PgCrc32c,			/* CRC of all above */
	pub pad: i32,			/* to make the struct size be 512 exactly */
}

#[test]
fn rel_map_file_is_512_bytes() {
    assert_eq!(size_of::<RelMapFile>(), 512);
}

impl RelMapFile {
    pub fn load(/*shared: bool*/) -> Result<Self, String> {
        let mut r = [0u8; size_of::<RelMapFile>()];
        let mut mapfilename = data_dir();
        mapfilename.push(get_database_path(1, DEFAULTTABLESPACE_OID)?);
        mapfilename.push(FILENAME);
        let file = File::open(&mapfilename).unwrap();
        let mut reader = BufReader::new(file);
        reader.read_exact(&mut r).unwrap();
        
        let rel_map_file: Self = unsafe {transmute(r)};
        if rel_map_file.magic != MAGIC ||
            rel_map_file.num_mappings < 0 ||
            rel_map_file.num_mappings > MAX_MAPPINGS as i32
        {
            return Err(format!("Relation mapping file \"{:?}\" contains invalid data", mapfilename));
        }

        if crc32::checksum_castagnoli(&r[..offset_of!(RelMapFile, crc)]) != rel_map_file.crc {
            return Err(format!("relation mapping file \"{:?}\" contains incorrect checksum", mapfilename))
        }

        Ok(rel_map_file)
    }
}