mod application;

use crate::application::{Application, ApplicationTest};

fn main() {
    Application::run();
    ApplicationTest::run();
}
