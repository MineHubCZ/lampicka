// epic rust programming
// blazingly fast
// parek

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{time::Duration, io::Write, thread};

use serialport::SerialPort;

fn scan() -> Option<Box<dyn SerialPort>> {
    // TODO return None instead of .expect()
    let ports = serialport::available_ports().expect("No ports found!");
    println!("{:?}", ports);
    for p in ports {
        println!("{}", p.port_name);
        let raw_port = serialport::new(p.port_name.as_str(), 9600)
            .timeout(Duration::from_millis(2000))
            .open()
        ;

        if raw_port.is_err() {
            println!("AHA JE ERROR PRI HLEDANI PORTU!!! {:?}", raw_port.err());
            continue;
        }

        let mut port = raw_port.unwrap();

        port
            .write("csmenor".as_bytes()).expect("Unable to write to port")
        ;

        thread::sleep(Duration::from_millis(1000));

        let mut result = vec![0; 9];
      
        let reading = port.read(result.as_mut_slice());

        if reading.is_err() {
            println!("AHA JE ERROR PRI POKUSU O CTENI");
            continue;
        }
       
        let message = String::from_utf8(result).expect("menor cos to poslal");
        println!("{:?}", message);

        if message == "csmoravak" {
            println!("tak sem curak no");
            return Some(port);
        }
    }
    return None;
}

#[tauri::command]
fn write(setting: String) -> bool {
    println!("WRITE SE PUSTI!!!!");
    let raw_port = scan();
    if raw_port.is_none() {
        println!("AHA ALE NEPRIPOJILI JSME SE TO JE SPATNY");
        return false;
    }

    let mut port = raw_port.unwrap();

    port.write(setting.as_bytes()).expect("Unable to write");
    return true;
}

#[tauri::command]
fn connect() -> Option<Vec<String>> {
    let raw_port = scan();

    if raw_port.is_none() {
        return None;
    }

    let mut port = raw_port.unwrap();

    if port.write("profiles".as_bytes()).is_err() {
        return None;
    }

    let mut result: Vec<String> = Vec::new();

    for _ in 0..=5 {
        let mut buffer = vec![0; 100];
        if port.read(&mut buffer).is_err() {
            return None;
        }
        if let Ok(profile) = String::from_utf8(buffer) {
            result.push(profile.trim_matches('\0').to_string());
        } else {
            return None;
        }
    }
   
    Some(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, write])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
