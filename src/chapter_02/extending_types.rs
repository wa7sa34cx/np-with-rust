trait Sawtooth {
    fn sawtooth(&self) -> Self;
}

impl Sawtooth for f64 {
    fn sawtooth(&self) -> f64 {
        self - self.floor()
    }
}

pub fn run() {
    println!("{}", 2.34_f64.sawtooth());
}
