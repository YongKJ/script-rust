struct Demo {
    msg: String,
}

impl Demo {
    fn new(msg: String) -> Self {
        Self { msg }
    }

    fn test(&self) {
        println!("msg: {}", self.msg)
    }
}

pub fn run() {
    Demo::new(String::from("Demo test.")).test();
}
