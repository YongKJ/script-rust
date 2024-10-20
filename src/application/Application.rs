use crate::application::applet::Demo::Demo;

struct Application {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        Demo::run()
    }
}


pub fn run() {
    Application::new().run();
}
