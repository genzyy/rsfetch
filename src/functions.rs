use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::process::Command;

pub fn get_username() -> String {
    let username_from_env = env::var_os("USER");
    let username: String;

    if username_from_env.is_some() {
        username = username_from_env.unwrap().into_string().unwrap();
    } else {
        username = String::new();
    }

    return username;
}

pub fn get_hostname() -> String {
    let hostname_file: Result<File, Error> = fs::File::open("/etc/hostname");
    let mut hostname: String = String::new();

    let result: Result<usize, Error> = hostname_file.unwrap().read_to_string(&mut hostname);

    if result.is_err() {
        return "error".to_string();
    }

    return hostname;
}

pub fn get_distro() {
    let lsb_release_output = Command::new("sh")
        .args(["lsb_release", "-ds"])
        .output()
        .expect("Error while executing lsb_release -ds.");

    let distro = lsb_release_output.status;
    println!("{:#?}", distro);
}
