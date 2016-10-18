extern crate irc;

use std::default::Default;
use irc::client::prelude::*;
use irc::client::data::Command::PRIVMSG;

fn main() {
    let cfg = Config {
        nickname: Some(format!("the-new-rbot")),
        server: Some(format!("irc.freenode.org")),
        channels: Some(vec![format!("#ayyylmao")]),
        use_ssl: Some(true),
        port: Some(6697),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        // Do message processing.
        if message.is_ok() {
            let msg = &message.unwrap();
            let user = msg.source_nickname().unwrap_or("");
            if let PRIVMSG(ref chan, ref user_msg) = msg.command {
                println!("{}: {}: {}", chan, user, user_msg);

                // Check if first character of the message is the bot character
                if user_msg.as_bytes()[0] == b'!' {
                    let argv: Vec<&str> = user_msg.split_whitespace().collect();
                    let cmd: &str = argv[0];

                    println!("Command: {}", cmd);
                    match cmd {
                        "!test" => println!("You have done the needful."),
                        "!echo" => {
                            server.send_privmsg(chan, &argv.join(" "));
                        },
                        _ => println!("Invalid command!"),
                    }
                }
            }
        }
    }
}
