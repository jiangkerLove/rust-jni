use std::thread;
use rust_native::log_map::{MMAP, write_log};

fn main() {
    let mut handles = vec![];

    for i in 0..100000 {
        let handle = thread::spawn(move || {
            write_log(String::from("example.txt"), String::from(format!("this is origin file from txt: {}", i)));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut guard = MMAP.lock().unwrap();


    if let Some(mmap) = &mut guard.m_map {
        mmap.flush().unwrap()
    }
}