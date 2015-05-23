use std;
use util::*;

fn get_password() -> String {
    last_line();
    print!("Password: ");
    print!("\x1b[3{}m\x1b[4{}m", BG, BG);
    flush_out();
    let mut pwd = String::new();
    std::io::stdin().read_line(&mut pwd).unwrap();

    pwd
}

fn get_username() -> String {
    last_line();
    set_def_colors();
    print!("Username: ");
    flush_out();

    let mut user = String::new();
    std::io::stdin().read_line(&mut user).unwrap();

    user
}

pub fn login_screen() -> (String, String) {
    clear_screen();
    print_banner();
    let user = get_username();
    clear_screen();
    print_banner();
    let pwd = get_password();

    (user, pwd)
}