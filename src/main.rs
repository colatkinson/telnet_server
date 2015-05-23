extern crate telnet_server;

use telnet_server::cmds::Command;
use telnet_server::util::*;
use telnet_server::login;
use std::io::Write;

fn menu_screen() -> bool {
    set_def_colors();
    clear_screen();
    let command_vec = vec![Command::Message];

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
        command_vec[choice - 1].run();
        true
    }
}

fn main() {
    set_def_colors();
    clear_screen();
    let (user, pass) = login::login_screen();
    let cxt = Context {user: user};
    loop {
        if !menu_screen() {
            break;
        }
    }
    println!("Thank you, come again!");
}
