extern crate irc;

use std::default::Default;
use irc::client::prelude::*;
use irc::client::data::Command::PRIVMSG;

fn main() {
    let cfg = Config {
        nickname: Some(format!("the-new-rbot")),
        server: Some(format!("irc.freenode.org")),
        channels: Some(vec![format!("#ayyylmao")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        // Do message processing.
        if message.is_ok() {
            let msg = &message.unwrap();
            let user = msg.source_nickname().unwrap_or("");
            if let PRIVMSG(ref chan, ref cmd) = msg.command {
                println!("{}: {}: {}", chan, user, cmd);
            }
        }
    }
}
