use cmds::Cmd;

pub struct Message;

impl Cmd for Message {
    fn get_name() -> String {
        "Helpful Message".to_string()
    }

    fn run() {
        println!("(Most) computers use binary!");
    }
}