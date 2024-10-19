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
    let msg = String::from("Demo test.");
    let demo = Demo::new(msg);
    demo.test();
}