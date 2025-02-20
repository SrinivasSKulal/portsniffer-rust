use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;


struct Arguements{
    flag :String,
    ipaddr :IpAddr,
    threads : u16,
}
impl Arguements{
    fn new(args) -> Result<Arguements , &'static str>{
        if args.len() < 2{
            return Err("Not enough arguements");
        }else if args.len() >4 {
            return Err("Too many arguements");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(
                Arguements{
                    flag:String::from(""),
                    ipaddr,
                    threads : 4,
                }
            );
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") &&args.len() == 2{
                println!("Usage: -j to select how many threads you want \r\n  -h or -help to show this help message")
            }else if flag.contains("-h") || flag.contains("-help"){
                return Err("To many arguements");
            }else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) =>s,
                    Err(_) =>return Err("Not a valid IP address must be IPV4 or IPV6.")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse number of threads.")
                };
                return Ok(Arguements {
                    threads,
                    flag,
                    ipaddr
                });
            }else {
                return Err("Invalid Syntax");
            }
        }
        

    }
}
fn main(){
    let args:Vec<String> = env::args().collect();

    let program = args[0].clone();

    let arguements =    Arguements::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
               process::exit(0); 
            }
            else{
                eprintln!("{} problem parsing arguements : {} " , program , err);
                process::exit(0);
            }
        }
    );
}
