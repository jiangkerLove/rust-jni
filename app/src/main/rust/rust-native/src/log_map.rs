use std::{u32, usize};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::path::Path;
use std::sync::Mutex;

use memmap2::MmapMut;
use once_cell::sync::Lazy;

const MAX_LEN: u64 = 5 * 1024 * 1024;

pub struct MMapPair {
    pub m_map: Option<MmapMut>,
    pub file_name: String,
}

pub static MMAP: Lazy<Mutex<MMapPair>> = Lazy::new(|| {
    Mutex::new(MMapPair {
        file_name: String::new(),
        m_map: None,
    })
});


pub fn write_log(file_name: String, value: String) {
    let mut guard = MMAP.lock().unwrap();

    if guard.file_name != file_name {
        *guard = create_mmap_pair(file_name)
    }

    if let Some(mmap) = &mut guard.m_map {
        let mut start_index = match mmap[..4].try_into() {
            Ok(arr) => { u32::from_le_bytes(arr) as usize }
            Err(_) => { 0 }
        };

        let len = value.len();

        let end = start_index + len;

        if end < MAX_LEN as usize {
            mmap[start_index..end].copy_from_slice(value.as_bytes());
            mmap[end] = b'\n';
            start_index += len + 1;
            mmap[..4].copy_from_slice(&(start_index as u32).to_le_bytes());
        }
    }
}

fn create_mmap_pair(file_name: String) -> MMapPair {
    let mmap: Result<MmapMut, Error> = unsafe {
        match create_file(&file_name) {
            Ok(file) => {
                // Ensure the file is at least 4 bytes long
                MmapMut::map_mut(&file)
            }
            Err(err) => { Err(err) }
        }
    };
    match mmap {
        Ok(mm) => {
            MMapPair {
                m_map: Some(mm),
                file_name,
            }
        }
        Err(_) => {
            MMapPair {
                m_map: None,
                file_name,
            }
        }
    }
}

fn create_file(file_name: &String) -> Result<File, std::io::Error> {
    let path = Path::new(file_name);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    // Ensure the file is at least 4 bytes long
    file.set_len(MAX_LEN)?;
    Ok(file)
}
