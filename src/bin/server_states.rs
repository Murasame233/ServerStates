use server_states::{self, platform::Platform};
use std::{io::{Write, stdout}, thread::sleep, time::Duration};
fn main (){
    let mut p = server_states::platform::get_platform();
    loop {
        sleep(Duration::from_secs(1));
        p.update();
        let v = p.get_all();
        print!("\r");
        for i in 0..v.len(){
            print!("{:>2}% ",v[i]);
        }
        stdout().flush().unwrap();
    }
}