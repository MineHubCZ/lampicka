// epic rust programming
// blazingly fast
// parek

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::SerialPort;

fn scan() -> Option<Box<dyn SerialPort>> {
    // TODO return None instead of .expect()
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        let port = serialport::new(p.port_name.as_str(), 115_200)
            .open().expect("Failed to open port")
        ;
        port
            .write("csmenor".as_bytes()).expect("Unable to write to port")
        ;
        let mut result = vec![0; 32];
        port
            .read(result.as_mut_slice()).expect("Unable to read")
        ;
        
        if String::from_utf8(result).expect("menor cos to poslal") == "csmoravak" {
            return Some(port);
        }
    }
    return None;
}

fn scanProfiles(port: Box<dyn SerialPort>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in [1..5] {
        let mut curent = vec![0; 32];
        port.read(curent.as_mut_slice()).expect("Unable to read");
        result.push(String::from_utf8(curent).expect("menor cos to poslal"));
    }
    return result;
}

#[tauri::command]
fn write(setting: String) -> bool {
    let port = scan();
    if port == None {
        return false;
    }

    scanProfiles(port);

    return port.write(setting.as_bytes()) == None;
}

#[tauri::command]
fn connect() -> Option<Vec<String>> {
    let port = scan();
    if port == None {
        return None;
    }

    return scanProfiles(port);
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
