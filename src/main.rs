use cmds::Command;
use std::io::Write;

mod cmds;

fn clear_screen() {
    print!("\x1b[2J\x1b[;H");
    std::io::stdout().flush().unwrap();
}

fn menu_screen() -> bool {
    clear_screen();
    let command_vec = vec![Command::Message];

    let mut num = 0;
    for opt in &command_vec {
        num += 1;
        println!("{}) {}", num, opt.get_name());
    }
    println!("{}) {}", num + 1, "Quit");
    print!("\x1b[9999;1H");
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
    loop {
        if !menu_screen() {
            break;
        }
    }
    println!("Thank you, come again!");
}
