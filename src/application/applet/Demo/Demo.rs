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
        FileUtil::create(fileName.to_string());
    }

    fn test3(&self) {
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new("Hello world!")));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new("世界，你好！".to_string())));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new(12)));
    }

    fn test4(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let size = FileUtil::size(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", Box::new(size)));
    }

    fn test5(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let isFolder = FileUtil::isFolder(fileName.to_string());
        let isFile = FileUtil::isFile(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFolder", Box::new(isFolder)));
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFile", Box::new(isFile)));
    }

    fn test6(&self) {
        // let fileName = "C:\\Users\\admin\\Desktop\\apk";
        let fileName = "C:\\Users\\admin\\Desktop\\api-go";
        // let fileName = "C:\\Users\\Admin\\Desktop\\Database-Backup";
        let folderSize = FileUtil::sizeFolder(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test6", "folderSize", Box::new(folderSize)));
    }

    fn test7(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\rust-test\\rust-demo";
        FileUtil::mkdir(fileName.to_string());
    }

    fn test8(&self) {
        // let fileName = "C:\\Users\\Admin\\Desktop\\busybox-df.json";
        let fileName = "C:\\Users\\Admin\\Desktop\\api-ts.code-workspace";
        let lines = FileUtil::readByLine(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test8", "lines.len()", Box::new(lines.len())));
        // let content = FileUtil::read(fileName.to_string());
        // LogUtil::loggerLine(Log::of("Demo", "test8", "content.len()", Box::new(content.len())));
    }

}

pub fn run() {
    let demo = Demo::new("Demo test.".to_string());

    demo.test8();
    // demo.test7();
    // demo.test6();
    // demo.test5();
    // demo.test4();
    // demo.test3();
    // demo.test2();
    // demo.test1();
    // demo.test();
}
