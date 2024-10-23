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
        FileUtil::create(fileName);
    }

    fn test3(&self) {
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new("Hello world!")));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new(String::from("世界，你好！"))));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new(12)));
    }

    fn test4(&self) {
        let size = FileUtil::size("C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx");
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new(size)));
    }
}

pub fn run() {
    let demo = Demo::new(String::from("Demo test."));

    demo.test4();
    // demo.test3();
    // demo.test2();
    // demo.test1();
    // demo.test();
}
