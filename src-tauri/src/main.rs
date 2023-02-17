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
    let ports = serialport::available_ports().expect("No ports found!");
    println!("[LOG] Ports: {:?}", ports);
    for p in ports {
        println!("[LOG] Scanning port {}", p.port_name);
        let raw_port = serialport::new(p.port_name.as_str(), 9600)
            .timeout(Duration::from_millis(2000))
            .open()
        ;

        if raw_port.is_err() {
            println!("[LOG] There was an error: {:?}", raw_port.err());
            continue;
        }

        let mut port = raw_port.unwrap();

        port
            .write("csmenor".as_bytes()).expect("[LOG] Unable to write to port")
        ;

        thread::sleep(Duration::from_millis(1000));

        let mut result = vec![0; 9];
      
        let reading = port.read(result.as_mut_slice());

        if reading.is_err() {
            println!("[LOG] Error while reading {:?}", reading.err());
            continue;
        }
       
        let message = String::from_utf8(result).expect("[LOG] Menor poslal nejakou bejkarnu");

        if message == "csmoravak" {
            return Some(port);
        }
    }

    println!("[LOG] Unable to connect");
    return None;
}

#[tauri::command]
fn write(setting: String) -> bool {
    println!("[LOG] Starting to write");
    let raw_port = scan();
    if raw_port.is_none() {
        return false;
    }

    let mut port = raw_port.unwrap();

    port.write(setting.as_bytes()).expect("[LOG] Unable to write");
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
        println!("pokus o cteni profilu minus");
        return None;
    }

    let mut result: Vec<String> = Vec::new();

    for i in 0..=5 {
        let mut buffer = vec![0; 100];
        if let Err(e) = port.read(&mut buffer) {
            println!("pokus o cteni profilu {} minus: {:?}", i, e);
            return None;
        }
        if let Ok(profile) = String::from_utf8(buffer) {
            result.push(profile.trim_matches('\0').to_string());
        } else {
            println!("pokus o parsovani profilu {} minus", i);
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
