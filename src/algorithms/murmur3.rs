//! MurmurHash3.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;
use std::io::Cursor;

/// Hash data using MurmurHash3 (x64, 128-bit).
fn murmur3(data: &[u8]) {
    black_box(
        murmur3::murmur3_x64_128(&mut Cursor::new(data), 0)
            .expect("murmur3 on in-memory cursor never fails"),
    );
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "MurmurHash3",
        crate_name: "murmur3",
        output: OutputBits::Fixed(128),
        category: Category::NonCryptographic,
        notes: "x64 128-bit variant; used by Cassandra, Elasticsearch",
        runner: Runner::SingleStream(murmur3),
    }]
}
