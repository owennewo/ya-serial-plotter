// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialport::{SerialPort, SerialPortInfo};
use std::collections::HashMap;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{CustomMenuItem, Menu, Submenu};

#[tauri::command]
fn list_ports() -> Vec<SerialPortInfo> {
    let ports = serialport::available_ports().expect("No ports found!");
    return ports;
}

#[tauri::command]
fn send(
    ports: tauri::State<Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>>>,
    port_name: &str,
    buf: Vec<u8>,
) {
    let ports = ports.lock().unwrap();
    if let Some(port) = ports.get(port_name) {
        let mut port = port.lock().unwrap();
        port.write(&buf).unwrap();
    } else {
        println!("Port not found");
    }
}

#[tauri::command]
fn disconnect(
    ports: tauri::State<Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>>>,
    port_name: &str,
) {
    ports.lock().unwrap().remove(port_name);
    println!("Port {} disconnected", port_name);
}

#[tauri::command]
fn dummy(window: tauri::Window, numStreams: u32, samples_per_second: u64, period_in_millis: u64) {
    // declare variable startTime and set it to the current time
    let mut startTime = std::time::Instant::now();

    std::thread::spawn(move || loop {
        // sleep duration based on samples_per_second
        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / samples_per_second,
        ));
        // declare variable now and set it to the current time
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(startTime);

        // create an array to store the sine values
        let mut values = vec![0.0; numStreams as usize];
        for i in 0..numStreams {
            // generate sine waves with a phase shift proportional to the index
            let period_in_secs = period_in_millis as f32 / 1000.0;
            values[i as usize] = ((elapsed.as_secs_f32() * 2.0 * std::f32::consts::PI
                / period_in_secs)
                + ((i as f32) * 2.0 * std::f32::consts::PI / (numStreams as f32)))
                .sin();
        }

        // create a string that contains the values of the sine waves to 2 decimal places that is terminated with a newline
        let s = format!(
            "{}\n",
            values
                .iter()
                .map(|&x| format!("{:.2}", x))
                .collect::<Vec<_>>()
                .join(",")
        );
        // emit this string to bus-receive
        window.emit("bus-receive", Some(s.as_bytes())).unwrap();
    });
}

#[tauri::command]
fn connect(
    ports: tauri::State<Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>>>,
    window: tauri::Window,
    port_name: &str,
    baud_rate: u32,
) -> bool {
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(port) => {
            let port_name = port_name.to_owned();

            let port = Arc::new(Mutex::new(port));

            ports
                .lock()
                .unwrap()
                .insert(port_name.to_string(), Arc::clone(&port));

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

                    let mut port_lock = port.lock().unwrap();

                    match port_lock.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            window.emit("bus-receive", Some(&serial_buf[..t])).unwrap();
                        }
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
    println!("main");

    let ports: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        // .add_native_item(MenuItem::Copy)
        // .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    println!("start");
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .manage(ports)
        .invoke_handler(tauri::generate_handler![
            list_ports, connect, disconnect, send, dummy
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("done");
}
