use std::{env::args, process::exit};
mod jsonreading;
mod xmlreading;
fn main() {
    let args: Vec<String> = args().collect();
    let model = if args.len() < 2 {
        None
    } else {
        Some(args[1].as_str())
    };
    let res = match model {
        None => {
            println!("nothing",);
            Ok(())
        }
        Some("xml") => xmlreading::run(),
        Some("json") => jsonreading::run(),
        Some(_) => {
            println!("only json and xml allowed right now");
            Ok(())
        }
    };
    exit(match res {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })
}
