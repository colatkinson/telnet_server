use std;

pub enum Command {
    Message
}

fn message_run() {
    std::thread::sleep_ms(1000);
    println!("(Most) computers use binary!");
    let mut asdf = String::new();
    std::io::stdin().read_line(&mut asdf).unwrap();
    std::io::stdin().read_line(&mut asdf).unwrap();
    println!("[{}]", asdf);
}

impl Command {
    pub fn run(&self) {
        match *self {
            Command::Message => message_run()
        }
    }

    pub fn get_name(&self) -> String {
        match *self {
            Command::Message => "Helpful Message".to_string()
        }
    }
}