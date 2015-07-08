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
