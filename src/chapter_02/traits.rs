use std::ops::Add;

trait Sum<T> {
    fn sum(self) -> T;
}

struct Foo<T> {
    a: T,
    b: T,
}

impl<T> Sum<T> for Foo<T>
where
    T: Add<Output = T>,
{
    fn sum(self) -> T {
        self.a + self.b
    }
}

struct Bar<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Sum<T> for Bar<T>
where
    T: Add<Output = T>,
{
    fn sum(self) -> T {
        self.a + self.b + self.c
    }
}

pub fn run() {
    let foo = Foo { a: 2u32, b: 4u32 };
    let bar = Bar {
        a: 3u64,
        b: 7u64,
        c: 8u64,
    };

    println!("{}", foo.sum());
    println!("{}", bar.sum());
}
