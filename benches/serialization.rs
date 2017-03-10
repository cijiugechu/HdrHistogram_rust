#![feature(test)]

extern crate hdrsample;
extern crate rand;
extern crate test;

use hdrsample::*;
use hdrsample::serialization::*;
use self::rand::distributions::range::Range;
use self::rand::distributions::IndependentSample;
use self::test::Bencher;
use std::io::Cursor;

#[bench]
fn serialize_tiny_dense(b: &mut Bencher) {
    // 256 + 3 * 128 = 640 counts
    do_serialize_bench(b, 1, 2047, 2, 1.5)
}

#[bench]
fn serialize_tiny_sparse(b: &mut Bencher) {
    // 256 + 3 * 128 = 640 counts
    do_serialize_bench(b, 1, 2047, 2, 0.1)
}

#[bench]
fn serialize_small_dense(b: &mut Bencher) {
    // 2048 counts
    do_serialize_bench(b, 1, 2047, 3, 1.5)
}

#[bench]
fn serialize_small_sparse(b: &mut Bencher) {
    // 2048 counts
    do_serialize_bench(b, 1, 2047, 3, 0.1)
}

#[bench]
fn serialize_medium_dense(b: &mut Bencher) {
    // 56320 counts
    do_serialize_bench(b, 1, u64::max_value(), 3, 1.5)
}

#[bench]
fn serialize_medium_sparse(b: &mut Bencher) {
    // 56320 counts
    do_serialize_bench(b, 1, u64::max_value(), 3, 0.1)
}

#[bench]
fn serialize_large_dense(b: &mut Bencher) {
    // 6291456 buckets
    do_serialize_bench(b, 1, u64::max_value(), 5, 1.5)
}

#[bench]
fn serialize_large_sparse(b: &mut Bencher) {
    // 6291456 buckets
    do_serialize_bench(b, 1, u64::max_value(), 5, 0.1)
}

#[bench]
fn deserialize_tiny_dense(b: &mut Bencher) {
    // 256 + 3 * 128 = 640 counts
    do_deserialize_bench(b, 1, 2047, 2, 1.5)
}

#[bench]
fn deserialize_tiny_sparse(b: &mut Bencher) {
    // 256 + 3 * 128 = 640 counts
    do_deserialize_bench(b, 1, 2047, 2, 0.1)
}

#[bench]
fn deserialize_small_dense(b: &mut Bencher) {
    // 2048 counts
    do_deserialize_bench(b, 1, 2047, 3, 1.5)
}

#[bench]
fn deserialize_small_sparse(b: &mut Bencher) {
    // 2048 counts
    do_deserialize_bench(b, 1, 2047, 3, 0.1)
}

#[bench]
fn deserialize_medium_dense(b: &mut Bencher) {
    // 56320 counts
    do_deserialize_bench(b, 1, u64::max_value(), 3, 1.5)
}

#[bench]
fn deserialize_medium_sparse(b: &mut Bencher) {
    // 56320 counts
    do_deserialize_bench(b, 1, u64::max_value(), 3, 0.1)
}

#[bench]
fn deserialize_large_dense(b: &mut Bencher) {
    // 6291456 buckets
    do_deserialize_bench(b, 1, u64::max_value(), 5, 1.5)
}

#[bench]
fn deserialize_large_sparse(b: &mut Bencher) {
    // 6291456 buckets
    do_deserialize_bench(b, 1, u64::max_value(), 5, 0.1)
}


fn do_serialize_bench(b: &mut Bencher, low: u64, high: u64, digits: u8, fraction_of_counts_len: f64) {
    let mut s = V2Serializer::new();
    let mut h = Histogram::<u64>::new_with_bounds(low, high, digits).unwrap();
    let random_counts = (fraction_of_counts_len * h.len() as f64) as usize;
    let mut vec = Vec::with_capacity(random_counts);

    let range = Range::new(low, high);

    let mut rng = rand::weak_rng();
    for _ in 0..random_counts {
        h.record(range.ind_sample(&mut rng)).unwrap();
    };

    b.iter(|| {
        vec.clear();

        let _ = s.serialize(&h, &mut vec).unwrap();
    });
}

fn do_deserialize_bench(b: &mut Bencher, low: u64, high: u64, digits: u8, fraction_of_counts_len: f64) {
    let mut s = V2Serializer::new();
    let mut h = Histogram::<u64>::new_with_bounds(low, high, digits).unwrap();
    let random_counts = (fraction_of_counts_len * h.len() as f64) as usize;
    let mut vec = Vec::with_capacity(random_counts);

    let range = Range::new(low, high);

    let mut rng = rand::weak_rng();
    for _ in 0..random_counts {
        h.record(range.ind_sample(&mut rng)).unwrap();
    };

    let _ = s.serialize(&h, &mut vec).unwrap();

    let mut d = Deserializer::new();
    b.iter(|| {
        let mut cursor = Cursor::new(&vec);
        let _: Histogram<u64> = d.deserialize(&mut cursor).unwrap();
    });
}
