extern crate disk_bench;
#[macro_use]
extern crate trackable;
use disk_bench::aligned::AlignedBuf;
use disk_bench::file_builder::FileBuilder;
use disk_bench::timer::Timer;
use disk_bench::{track_io, Error};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

fn main() {
    let iter = 500_000;
    let siz = 512;

    let mut file: File = track!(FileBuilder::new().direct_io(true).create("hoge.bin")).unwrap();

    let a = track!(AlignedBuf::new(siz)).unwrap();

    let mut idx: Vec<u64> = (0..iter).collect();
    idx.shuffle(&mut thread_rng());

    {
        let _timer = Timer::new(&format!("start iter={} siz={}", iter, siz));
        for i in idx {
            track_io!(file.seek(SeekFrom::Start(512 * i))).unwrap();
            track_io!(file.write_all(a.as_ref())).unwrap();
        }
    }
}
