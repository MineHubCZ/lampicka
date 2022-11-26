#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::SerialPort;

struct Port {
    port: Box<dyn SerialPort>
}



#[tauri::command]
fn scan() -> bool {
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
        
        if String::from_utf8(result).expect("Bad string") == "csmoravak" {
            return true;
        }
    }
    return false;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
