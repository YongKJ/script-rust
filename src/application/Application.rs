struct Application {
    msg: String,
}

impl Application {
    fn new(msg: String) -> Self {
        Self { msg }
    }

    fn test(&self) {
        println!("msg: {}", self.msg)
    }
}


pub fn run() {
    let msg = String::from("Hello world!");
    let app = Application::new(msg);
    app.test()
}
