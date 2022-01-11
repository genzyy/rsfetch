use crate::functions::get_distro;

mod functions;

fn main() {
    println!("My system fetch");

    let username: String = functions::get_username();
    println!("{}", username);
    let hostname: String = functions::get_hostname();
    println!("{}", hostname);

    get_distro();
}
