/////////////////////////////////////////////////////
extern crate tokio;
use tokio::net::TcpStream;

use text_io::read;

extern crate winapi;

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;

use std::process::Command;
/////////////////////////////////////////////////////

fn main() {
    let new_title = "port@scanner:~$: nico.dev | exit with ctrl+c";
    let new_title_wide: Vec<u16> = OsStr::new(new_title).encode_wide().chain(once(0)).collect();
    unsafe {
        SetConsoleTitleW(new_title_wide.as_ptr());
    }

    loop {
        print!("\x1B[2J\x1B[H"); // clear screen
        println!("port@scanner:~$: target ip: ");
        let input: String = read!();
        println!("");
        println!("port@scanner:~$: port: ");
        let input2: String = read!();
        println!("");
        port_scanner(input, input2);
    }

}

#[tokio::main]
async fn port_scanner(address: String, portinput: String){
    let addr = address;
    let port = portinput.parse().unwrap();
    match TcpStream::connect((addr, port)).await {
        Ok(_) => {
            println!("port {} is open", port);
            println!("");

            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

        }
        Err(_) => {
            println!("port {} is closed", port);
            println!("");

            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

        }
    }
}
