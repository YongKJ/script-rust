use crate::application::util::FileUtil;

struct Demo {
    msg: String,
}

impl Demo {
    fn new(msg: String) -> Self {
        Self { msg }
    }

    fn test1(&self) {
        let dir = FileUtil::workFolder();
        println!("msg: {}", dir)
    }

    fn test(&self) {
        println!("msg: {}", self.msg)
    }
}

pub fn run() {
    let demo = Demo::new(String::from("Demo test."));

    demo.test1();
    // demo.test();
}
