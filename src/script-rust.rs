mod application;

use crate::application::{
    Application,
    ApplicationTest,
    applet::Demo::Demo,
};

fn main() {
    Demo::run();
    Application::run();
    ApplicationTest::run();
}
