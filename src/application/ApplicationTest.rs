struct ApplicationTest {
    msg: String,
}

impl ApplicationTest {
    fn new(msg: String) -> Self {
        Self { msg }
    }

    fn test(&self) {
        println!("msg: {}", self.msg)
    }
}


pub fn run() {
    let msg = String::from("Hello world!");
    let app_test = ApplicationTest::new(msg);
    app_test.test()
}