use crate::application::applet::Demo::Demo;

struct ApplicationTest {}

impl ApplicationTest {
    fn new() -> Self {
        Self {}
    }

    fn test(&self) {
        Demo::run()
    }
}


pub fn run() {
    ApplicationTest::new().test();
}