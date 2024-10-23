use crate::application::pojo::dto::Log::Log;
use crate::application::util::{FileUtil, LogUtil};

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

    fn test1(&self) {
        let dir = FileUtil::workFolder();
        println!("msg: {}", dir)
    }

    fn test2(&self) {
        let fileName = "C:\\$WINRE_BACKUP_PARTITION.MARKER";
        FileUtil::create(String::from(fileName));
    }

    fn test3(&self) {
        LogUtil::loggerLine::<&str>(Log::of("Demo", "test3", "msg", Box::new("Hello world!")));
        LogUtil::loggerLine::<String>(Log::of("Demo", "test3", "msg", Box::new(String::from("世界，你好！"))));
        LogUtil::loggerLine::<i32>(Log::of("Demo", "test3", "msg", Box::new(12)));
    }
}

pub fn run() {
    let demo = Demo::new(String::from("Demo test."));

    // demo.test3();
    // demo.test2();
    demo.test1();
    // demo.test();
}
