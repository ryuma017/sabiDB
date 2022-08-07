use std::fs::{File, OpenOptions};
use std::io::{self, SeekFrom, Seek, Read, Write};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: Self = Self(u64::MAX);

    pub fn valid(self) -> Option<Self> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }
}

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;

        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_file_path)?;

        Self::new(heap_file)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.read_exact(data)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.write_all(data)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }
}
