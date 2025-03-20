use libm::log;
use std::io::Cursor;
use murmur3::murmur3_x64_128;
use std::fmt;
use num_traits::pow;

/// Inserts and does some lookups into our custom BloomFilter implementation
pub fn run_bloom_filter() {
    let mut bf = BloomFilter::new(10, 0.01);
    bf.insert("1");
    bf.insert("2");
    bf.insert("42");
    println!("lookup: 1? {}", bf.lookup("1"));
    println!("lookup: 2? {}", bf.lookup("2"));
    println!("lookup: 3? {}", bf.lookup("3"));
}

/// BitSet of size n + k hash function (based on the desired false positive rate (f)), no false negatives
#[derive(Debug)]
pub struct BloomFilter {
    m: u128,
    k: u32,
    bit_array: BitArray,
}

impl BloomFilter {
    pub fn new(n: u32, f: f64) -> Self {
        let m = calculate_m(n as f64, f);
        let k = calculate_k(m, n as f64);
        Self {
            m: m,
            k: k,
            bit_array: BitArray::new(m as usize),
        }
    }

    pub fn insert(&mut self, key: &str) {
        for i in 0..self.k {
            let hash_result = murmur3_x64_128(&mut Cursor::new(key), i);
            if hash_result.is_ok() {
                let index_k = hash_result.ok().unwrap() % self.m;
                let result = self.bit_array.set(index_k as usize, true);
                if result.is_err() {
                    eprintln!("Failed to insert key {} due to: {}", key, result.err().unwrap());
                }
            }
        }
    }

    pub fn lookup(&mut self, key: &str) -> bool {
        for i in 0..self.k {
            let hash_result = murmur3_x64_128(&mut Cursor::new(key), i);
            if hash_result.is_ok() {
                let index_k = hash_result.ok().unwrap() % self.m;
                if !self.bit_array.get(index_k as usize).unwrap() {
                    return false;
                }
            }
        }
        true
    }
}

fn calculate_m(n: f64, f: f64) -> u128 {
    let denom = pow(log(2 as f64), 2);
    (-log(f)*n/denom) as u128
}

fn calculate_k(m: u128, n: f64) -> u32 {
    let kf = (m as f64)*log(2 as f64)/n;
    kf as u32
}

/// A BitArray that efficiently stores a sequence of bits
#[derive(Debug)]
struct BitArray {
    /// The underlying storage for our bits
    data: Vec<u8>,
    /// The number of bits in the array
    size: usize,
}

impl BitArray {
    /// Create a new BitArray with the specified number of bits, all initialized to zero
    pub fn new(size: usize) -> Self {
        // Calculate how many bytes we need to store the bits
        let num_bytes = (size + 7) / 8;
        BitArray {
            data: vec![0; num_bytes],
            size,
        }
    }

    /// Get the value of the bit at the specified index
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.size {
            return None;
        }

        let byte_index = index / 8;
        let bit_offset = index % 8;
        let mask = 1 << bit_offset;

        Some((self.data[byte_index] & mask) != 0)
    }

    /// Set the value of the bit at the specified index
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }

        let byte_index = index / 8;
        let bit_offset = index % 8;
        let mask = 1 << bit_offset;

        if value {
            // Set the bit to 1
            self.data[byte_index] |= mask;
        } else {
            // Set the bit to 0
            self.data[byte_index] &= !mask;
        }

        Ok(())
    }
}

impl fmt::Display for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            if let Some(bit) = self.get(i) {
                write!(f, "{}", if bit { "1" } else { "0" })?;
            }
        }
        Ok(())
    }
}
