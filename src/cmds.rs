//! Code related to the various programs that may be run by users.

use std;
use util::Context;
use std::io::Result;

/// The `Command` type. 
///
/// Essentially a list of the supported commands on the server.
pub enum Command {
    /// Prints out a sample message
    Message
}

/// The main function for running the Helpful Message program.
fn message_run(cxt: &Context) {
    println!("Hi {}! (Most) computers use binary!", cxt.user);
    let mut asdf = String::new();
    std::io::stdin().read_line(&mut asdf).unwrap();
    std::io::stdin().read_line(&mut asdf).unwrap();
    println!("[{}]", asdf);
}

impl Command {
    /// Runs the current command. Returns a `std::io::Result`.
    ///
    /// # Examples
    ///
    /// ```
    /// use telnet_server::cmds::Command;
    /// use telnet_server::util::Context;
    ///
    /// let cxt = Context {user: "".to_string()};
    /// assert!(Command::Message.run(&cxt).is_ok());
    /// ```
    pub fn run(&self, cxt: &Context) -> Result<()> {
        match *self {
            Command::Message => message_run(cxt)
        }
        Ok(())
    }

    /// Returns a descriptive name for the command so that it can be shown in the menu.
    ///
    /// # Examples
    ///
    /// ```
    /// use telnet_server::cmds::Command;
    ///
    /// assert_eq!(Command::Message.get_name(), "Helpful Message");
    /// ```
    pub fn get_name(&self) -> String {
        match *self {
            Command::Message => "Helpful Message".to_string()
        }
    }
}
