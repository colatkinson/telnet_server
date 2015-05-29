//! The main binary.

#![allow(dead_code)]
extern crate telnet_server;

use telnet_server::cmds::Command;
use telnet_server::util::*;
use telnet_server::login;
use std::io::Write;
use telnet_server::user;

fn menu_screen(cxt: &Context) -> bool {
    set_def_colors();
    clear_screen();
    let command_vec = vec![Command::Message];

    println!("{:?}", cxt);

    let mut num = 0;
    for opt in &command_vec {
        num += 1;
        println!("{}) {}", num, opt.get_name());
    }
    println!("{}) {}", num + 1, "Quit");
    last_line();
    print!("Choose a command: ");
    std::io::stdout().flush().unwrap();

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    let parse_res = choice.trim().parse();
    let choice: usize = match parse_res {
        Ok(v) => v,
        Err(_) => {
            return true;
        }
    };

    if choice == num + 1 {
        false
    } else {
        println!("Zippity zoo!");
        let _ = command_vec[choice - 1].run(cxt);
        true
    }
}

fn main() {
    set_def_colors();
    clear_screen();
    let act = login::action_select();
    let mut login_prompt: bool;
    if act == login::LoginActions::Quit {
        return;
    }
    let (mut user, mut pwd): (String, String);
    loop {
        //clear_screen();
        let x = login::login_screen();
        user = x.0;
        pwd = x.1;
        match act {
            login::LoginActions::New => {
                let _ = user::add_user(&user, &pwd);
                login_prompt = false;
            },
            login::LoginActions::Quit =>  {
                login_prompt = false;
            },
            login::LoginActions::Login => {
                login_prompt = !user::check_user(&user, &pwd);
            }
        }

        if !login_prompt {
            break;
        }
    }

    let cxt = Context {user: user};
    loop {
        if !menu_screen(&cxt) {
            break;
        }
    }
    println!("Thank you, come again!");
}
