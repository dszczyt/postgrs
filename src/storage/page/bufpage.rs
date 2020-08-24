extern crate transmute;

use std::{
    fmt::Debug,
    fs::File,
    io::BufReader,
    io::Read,
    mem::{size_of},
    path::PathBuf, convert::TryInto,
};
use transmute::transmute;

const PG_PAGE_LAYOUT_VERSION: usize = 2;

#[repr(C)]
#[derive(Debug)]
pub struct PageXLogRecPtr {
    pub xlogid: u32,
    pub xrecoff: u32,
}
type LocationIndex = u16;
type TransactionId = u32;

bitfield! {
    pub struct ItemId(u32);
    impl Debug;
    pub get_lp_off, set_lp_off: 31, 17;
    pub get_lp_flags, set_lp_flags: 16, 15;
    pub get_lp_len, set_lp_len: 14, 0;
}

pub struct PageSizeVersion(u16);

impl PageSizeVersion {
    pub fn get_size(&self) -> usize {
        (self.0 & 0xFF00) as usize
    }

    pub fn set_size(&mut self, size: usize) {
        assert_eq!(size % 256, 0);
        self.0 = self.0 & 0x00FF & (size as u16);
    }

    pub fn get_version(&self) -> u16 {
        self.0 & 0x00FF
    }

    pub fn set_version(&mut self, version: usize) {
        assert!(version < 256);
        self.0 = self.0 & 0xFF00 & (version as u16);
    }
}

impl Debug for PageSizeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PageSizeVersion {{ get_size: {}, get_version: {} }}",
            self.get_size(),
            self.get_version()
        )
    }
}

// bitfield! {
//     pub struct PageSizeVersion(u16);
//     impl Debug;
//     pub get_size, _: 15, 8;
//     pub get_version, _: 7, 0;
// }

#[repr(C)]
#[derive(Debug)]
pub struct PageHeaderData {
    pub pd_lsn: PageXLogRecPtr,
    pub pd_checksum: u16,
    pub pd_flags: u16,
    pub pd_lower: LocationIndex,
    pub pd_upper: LocationIndex,
    pub pd_special: LocationIndex,
    pub pd_pagesize_version: PageSizeVersion,
    pub pd_prune_xid: TransactionId,
    // pub item_id_data: [ItemId],
}

impl PageHeaderData {
    pub fn from_file(path: PathBuf) -> Self {
        let mut r = [0u8; size_of::<Self>()];
        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);
        reader.read_exact(&mut r).unwrap();

        unsafe { transmute(r) }
    }
}

#[test]
fn page_header_is_24_bytes() {
    assert_eq!(size_of::<PageHeaderData>(), 24);
}

#[repr(C)]
pub struct Page {
    pub data: Vec<u8>,
    // pub header: PageHeaderData,
}

impl Page {
    // #[inline]
    // fn new_zero(page_size: usize) -> Self {
    //     let mut r = vec![0u8; page_size];
    //     unsafe { transmute(r) }
    // }

    // pub fn init(&mut self, page_size: usize, special_size: usize) {
    //     self.header.pd_lower = size_of::<PageHeaderData>().try_into().unwrap();
    //     self.header.pd_upper = (page_size - special_size) as u16;
    //     self.header.pd_pagesize_version.set_size(page_size);
    //     self.header.pd_pagesize_version.set_version(PG_PAGE_LAYOUT_VERSION);
    // }
    // pub fn get_exact_free_space(&self) -> usize {
    //     let header: PageHeaderData = unsafe { self as PageHeaderData };
    //     (header.pd_upper - header.pd_lower) as usize
    // }

    #[inline]
    pub fn header(&self) -> PageHeaderData {
        let r = &self.data[.. size_of::<PageHeaderData>()];
        unsafe { transmute(r) }
    }

    pub fn items(&self) -> Vec<ItemId> {
        let header = self.header();
        let r = &self.data[size_of::<PageHeaderData>() .. header.pd_lower as usize];
        unsafe { transmute(r) }
    }

    pub fn get_item_data(&self, idx: usize) -> Vec<u8> {
        let items = self.items();
        match items.get(idx) {
            Some(item_id) => self.data[item_id.get_lp_off() as usize .. item_id.get_lp_len() as usize].to_vec(),
            None => vec![]
        }
    }
}
