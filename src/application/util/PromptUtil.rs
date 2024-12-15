use crate::application::deploy::pojo::po::Script::Script;

pub fn packageRustScript(crossBuildPath: &str) -> (&str, Vec<&str>) {
    if cfg!(windows) {
        return ("busybox", vec![
            "bash",
            crossBuildPath,
        ]);
    }

    ("bash", vec![
        crossBuildPath,
    ])
}