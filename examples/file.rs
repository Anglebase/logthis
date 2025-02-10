use logthis::*;

fn main() {
    Log::set_file(Some(String::from("log.txt")));

    info!("Hello, world!");
    info!("This is a info message!");
    error!("Why are you doing this?");
}