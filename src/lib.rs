extern crate bzip2;

#[test]
fn bzip2test() {
    use std::io::prelude::*;
    use bzip2::Compress;
    use bzip2::reader::{BzCompressor, BzDecompressor};

    // Round trip some bytes from a byte source, into a compressor, into a
    // decompressor, and finally into a vector.
    let data = "Hello, World!".as_bytes();
    let compressor = BzCompressor::new(data, Compress::Best);
    let mut decompressor = BzDecompressor::new(compressor);

    let mut contents = String::new();
    decompressor.read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "Hello, World!");
}

pub struct Foo {
    a: u64,
    b: i8,
}

impl Foo {
    pub fn new() -> Foo {
        Foo { a: 42, b: 41 }
    }

    pub fn add(&mut self) {
        self.b += 1;
    }

    pub fn dec(&mut self) {
        self.b -= 1;
    }

    pub fn get(&self) -> u64 {
        self.a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let mut f = Foo::new();
        f.add();
        assert_eq!(f.b, 42);
    }
}
