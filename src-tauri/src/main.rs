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
    for p in ports {
        let raw_port = serialport::new(p.port_name.as_str(), 115_200)
            .timeout(Duration::from_millis(2000))
            .open()
        ;

        if raw_port.is_err() {
            continue;
        }

        let mut port = raw_port.unwrap();

        port
            .write("csmenor".as_bytes()).expect("Unable to write to port")
        ;

        thread::sleep(Duration::from_millis(1000));

        let mut result = vec![0; 32];
      
        let reading = port.read(result.as_mut_slice());

        if reading.is_err() {
            continue;
        }
        
        if String::from_utf8(result).expect("menor cos to poslal") == "csmoravak" {
            return Some(port);
        }

    }
    return None;
}

#[tauri::command]
fn write(setting: String) -> bool {
    let raw_port = scan();
    if raw_port.is_some() {
        return false;
    }

    let mut port = raw_port.unwrap();

    port.write(setting.as_bytes()).expect("Unable to write");
    return true;
}

#[tauri::command]
fn connect() -> bool {
    let raw_port = scan();

    return raw_port.is_some();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, write])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
