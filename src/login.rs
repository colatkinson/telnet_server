//! Code related to logging users in.

use std;
use util::*;

#[derive(PartialEq)]
pub enum LoginActions {
    Login,
    New,
    Quit
}

/// Prompt the user for a password and return the result.
///
/// The newline is automatically stripped from the result.
///
/// # Examples
///
/// ```norun
/// use telnet_server::login::get_password;
/// assert_eq!(get_password(), "");
/// ```
fn get_password() -> String {
    last_line();
    print!("Password: ");
    print!("\x1b[3{}m\x1b[4{}m", BG, BG);
    let _ = flush_out();
    let mut pwd = String::new();
    std::io::stdin().read_line(&mut pwd).unwrap();

    pwd.trim().to_string()
}

/// Prompt the user for a password and return the result.
///
/// The newline is automatically stripped from the result.
///
/// # Examples
///
/// ```norun
/// use telnet_server::login::get_username;
/// assert_eq!(get_username(), "");
/// ```
fn get_username() -> String {
    last_line();
    set_def_colors();
    print!("Username: ");
    let _ = flush_out();

    let mut user = String::new();
    std::io::stdin().read_line(&mut user).unwrap();

    user.trim().to_string()
}

pub fn action_select() -> LoginActions {
    clear_screen();
    print_banner();
    let mut num = 0;
    let action_vec = ["Login", "Create Account", "Quit"];
    for opt in &action_vec {
        num += 1;
        println!("{}) {}", num, opt);
    }
    //println!("{}) {}", num + 1, "Quit");
    last_line();
    print!("Select an action: ");
    let _ = flush_out();

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    let parsed = choice.trim().parse().unwrap();

    let res = match parsed {
        1 => LoginActions::Login,
        2 => LoginActions::New,
        3 => LoginActions::Quit,
        _ => action_select()
    };
    res
}

/// Display a login screen and return the username and password given.
///
/// # Examples
///
/// ```
/// use telnet_server::login::login_screen;
/// assert_eq!(login_screen(), ("".to_string(), "".to_string()));
/// ```
pub fn login_screen() -> (String, String) {
    set_def_colors();
    clear_screen();
    print_banner();
    let user = get_username();
    clear_screen();
    print_banner();
    let pwd = get_password();

    (user, pwd)
}
