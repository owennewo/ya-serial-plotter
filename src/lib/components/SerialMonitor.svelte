<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/tauri";
    import { selectedSerialPort } from "../stores/selection";

    const encoder: TextEncoder = new TextEncoder();

    let receive_data = "";

    let send_text = "";

    let port_name = undefined;

    selectedSerialPort.subscribe((port) => {
        if (port) {
            port_name = port.port_name;
        } else {
            port_name = undefined;
        }
    });

    listen("bus-receive", (event) => {
        // debugger;
        const payload = event.payload as number[];
        console.log("payload", payload);

        receive_data = receive_data.concat(
            payload.map((value: number) => String.fromCharCode(value)).join("")
        );
    });

    async function send() {
        if (!port_name) {
            console.log("No serial port selected");
            return;
        } else if (!send_text) {
            console.log("No text to send");
            return;
        } else {
            const buf: number[] = Array.from(encoder.encode(send_text));
            invoke("send", { portName: port_name, buf: buf });
        }
    }
</script>

<div class="flex flex-col h-screen">
    <div class="flex items-center h-12">
        <input
            class="flex-grow h-full border rounded px-2"
            type="text"
            id="send-text"
            placeholder="Enter text here..."
            bind:value={send_text}
        />
        <button
            class="btn"
            id="send-button"
            on:click={send}
            disabled={!port_name || !send_text}>Send</button
        >
    </div>
    <div class="flex-grow">
        <textarea
            class="w-full h-full border rounded px-2"
            id="receive-text"
            readonly
            placeholder="Serial Output">{receive_data}</textarea
        >
    </div>
</div>
