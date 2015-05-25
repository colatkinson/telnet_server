//! Various utility functions that simplify operations for the entire server.

use std;
use std::io::{Write, Result};

/// The ANSI color code of the background color
pub const BG: u8 = 4;
/// The ANSI color code of the foreground color
pub const FG: u8 = 7;

/// A struct that contains session info.
/// It's passed between functions that need it so that they can access the same info.
#[derive(Debug)]
pub struct Context {
    pub user: String
}

impl Context {
    /// Constructs a new `Context`.
    ///
    /// # Examples
    ///
    /// ```
    /// use telnet_server::util::Context;
    /// let cxt = Context::new("john");
    /// ```
    pub fn new(user: &str) -> Context {
        Context {user: user.to_string()}
    }
}

/// Flushes stdout. Useful when printing without a newline.
///
/// # Examples
///
/// ```
/// use telnet_server::util::flush_out;
/// assert!(flush_out().is_ok());
/// ```
pub fn flush_out() -> Result<()> {
    try!(std::io::stdout().flush());
    Ok(())
}

/// Clears the screen and moves to the top left using ANSI control characters.
pub fn clear_screen() {
    print!("\x1b[2J\x1b[;H");
    let _ = flush_out();
}

/// Moves the cursor to the last line of the screen using ANSI control characters.
pub fn last_line() {
    print!("\x1b[9999;1H");
    let _ = flush_out();
}

/// Restores the default terminal colors.
///
/// Uses the values stored in `FG` and `BG`.
pub fn set_def_colors() {
    print!("\x1b[3{}m\x1b[4{}m", FG, BG);
    let _ = flush_out();
}

/// Prints the server banner.
///
/// For some reason, it's a crocodile.
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