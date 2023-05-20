// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialport::SerialPortInfo;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::io::{self};
use std::time::Duration;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};


#[tauri::command]
fn list_ports() -> Vec<SerialPortInfo> {
    // println!("RUST: list_ports");
    let ports = serialport::available_ports().expect("No ports found!");
    return ports;
}

#[tauri::command]
fn disconnect(ports: tauri::State<Arc<Mutex<HashMap<String, bool>>>>, port_name: &str) {
    ports.lock().unwrap().remove(port_name);
    println!("Port {} disconnected", port_name);
}

#[tauri::command]
fn connect(ports: tauri::State<Arc<Mutex<HashMap<String, bool>>>>, window: tauri::Window, port_name: &str) -> bool { 
    let baud_rate:u32 = 115_200;
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();
    

    match port {
        Ok(mut port) => {

            let port_name = port_name.to_owned();

            ports.lock().unwrap().insert(port_name.to_string(), true);

            let ports_clone = Arc::clone(&ports);

            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, baud_rate);
            std::thread::spawn(move || {
            loop {
                // if ports does not have this port_name, drop mem and exit thread
                if !ports_clone.lock().unwrap().contains_key(&port_name) {
                    // simply breaking loop will cause port to be dropped
                    break;
                }

                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        window.emit("my-event", Some(&serial_buf[..t])).unwrap();
                        // io::stdout().write_all(&serial_buf[..t]).unwrap();
                        // print serial_buf.len() to stdout
                        // println!("serial_buf.len(): {}", serial_buf.len());
                        
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            });
            return true;
            // Ok("good"); // Ok(port)
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            //
            //Err(Box::new(e))
            return false;
        }
    }
}


fn main() {
    let ports: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    println!("start");
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "quit" => {
                std::process::exit(0);
              }
              "close" => {
                event.window().close().unwrap();
              }
              _ => {}
            }
          })
        .manage(ports)
        .invoke_handler(tauri::generate_handler![list_ports, connect, disconnect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        println!("done");
}

