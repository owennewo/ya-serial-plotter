# (Yet Another) Serial Plotter

Until I get to v0.5, this tool won't be useful. Ignore 
From v0.5 to 1.0 it might work, but won't have feature parity with Arduino Serial Monitor/Ploter
After 1.0 it'll have features not available in Arduino Serial Monitor/Ploter
 - auto scan devices
 - hex (wireshark like) output
 - auto plots supporting a number of formats
 - manual connection, device, dashboard settings some of these sharable

# Tech
This project uses [Tauri](https://tauri.app/) - which supports multiple OSs (Mac/Linux/Win).  The frontend runs in the OSs native web renderer and the backend is rust.

The frontend is written as a [svelte](https://svelte.dev/) app (i.e. typescript/html/css) which can easily communicate with backend rust using 'actions' and 'events'.  Rust is small, fast and seems to have a good cross platform [serialport](https://docs.rs/serialport/latest/serialport/) library.  

The frontend svelte app will be supported by [tailwindcss](https://tailwindcss.com/) and [daisy.ui](https://daisyui.com/) for layout and components.

