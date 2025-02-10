use std::thread::spawn;

use logthis::*;

struct MyStruct;

impl MyStruct {
    fn new() -> Self {
        info!(Self, "Here is in MyStruct::new!");
        Self
    }
}

fn main() {
    Log::set_level(Level::Debug);
    info!("Hello, world!");
    Log::set_current_thread_name("MainThread");

    warn!("This is a warning!");
    error!("This is an error!");
    MyStruct::new();

    let join = spawn(||{
        debug!("This is a debug message!");
        Log::set_current_thread_name("SpawnThread");
        info!("This is a info message!");
        warn!("This is a warn message!");
        error!("Message","This is a error message!");

        MyStruct::new();
    });

    join.join().unwrap();
}