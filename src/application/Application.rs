use crate::application::deploy::service::BuildScriptService::BuildScriptService;

struct Application {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        // Demo::run()
        BuildScriptService::run()
    }
}


pub fn run() {
    Application::new().run();
}
