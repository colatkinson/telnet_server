pub mod message;

pub trait Cmd {
    fn get_name() -> String;
    fn run();
}