use std;
use std::io::Write;

pub const BG: u8 = 4;
pub const FG: u8 = 7;

pub struct Context {
    pub user: String
}

pub fn flush_out() {
    std::io::stdout().flush().unwrap();
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[;H");
    flush_out();
}

pub fn last_line() {
    print!("\x1b[9999;1H");
    flush_out();
}

pub fn set_def_colors() {
    print!("\x1b[3{}m\x1b[4{}m", FG, BG);
    flush_out();
}

pub fn print_banner() {
    let logo = " 
    
                     _.---._     .---.
            __...---' .---. `---'-.   `.
  ~ -~ -.-''__.--' _.'( | )`.  `.  `._ :
 -.~~ .'__-'_ .--'' ._`---'_.-.  `.   `-`.
  ~ ~_~-~-~_ ~ -._ -._``---. -.    `-._   `.
    ~- ~ ~ -_ -~ ~ -.._ _ _ _ ..-_ `.  `-._``--.._
     ~~-~ ~-_ _~ ~-~ ~ -~ _~~_-~ -._  `-.  -. `-._``--.._.--''. ~ -~_
         ~~ -~_-~ _~- _~~ _~-_~ ~-_~~ ~-.___    -._  `-.__   `. `. ~ -_~
             ~~ _~- ~~- -_~  ~- ~ - _~~- _~~ ~---...__ _    ._ .` `. ~-_~
                ~ ~- _~~- _-_~ ~-_ ~-~ ~_-~ _~- ~_~-_~  ~--.....--~ -~_ ~
                     ~ ~ - ~  ~ ~~ - ~~-  ~~- ~-  ~ -~ ~ ~ -~~-  ~- ~-~

                     install gentoo";
    println!("{}", logo);
}