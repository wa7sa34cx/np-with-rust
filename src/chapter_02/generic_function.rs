use std::ops::Add;

struct Foo<T> {
    a: T,
    b: T,
}

impl<T> Foo<T>
where
    T: Add<Output = T>,
{
    fn sum(self) -> T {
        self.a + self.b
    }
}

pub fn run() {
    let one = Foo { a: 2u32, b: 4u32 };
    let two = Foo { a: 3u64, b: 7u64 };

    println!("{}", one.sum());
    println!("{}", two.sum());
}
