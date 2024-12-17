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

pub fn releaseToolchain<'a>(toolchainPath: &'a str, desDir: &'a str) -> (&'a str, Vec<&'a str>) {
    if cfg!(windows) {
        return ("busybox", vec![
            "tar",
            "xvf",
            toolchainPath,
            "-C",
            desDir
        ]);
    }

    ("bash", vec![
        "tar",
        "xvf",
        toolchainPath,
        "-C",
        desDir
    ])
}