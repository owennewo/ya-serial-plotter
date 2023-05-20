<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { selectedSerialPort } from "../stores/selection";

    export let device = null;

    const connect = () => {
        console.log("connecting to:", device.port_name);
        invoke("connect", { portName: device.port_name, baudRate: 115200 });
        selectedSerialPort.set(device);
    };
</script>

<div class="card w-64 bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex justify-between items-center">
            <h2 class="card-title">
                {device.port_type?.UsbPort?.product
                    ? device.port_type?.UsbPort?.product
                    : device?.port_name}
            </h2>
            <div class="relative group inline-block">
                <span class="material-icons md-36 md-dark pointer-events-none"
                    >info</span
                >
                <div
                    class="absolute left-1/2 transform -translate-x-1/2 mt-2 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 ease-in-out z-10"
                >
                    <div
                        class="rounded p-2 bg-white shadow-lg text-sm text-gray-700"
                    >
                        <p>Port Name: {device.port_name}</p>
                        <p>Product: {device.port_type?.UsbPort?.product}</p>
                        <p>
                            Manufacturer: {device.port_type?.UsbPort
                                ?.manufacturer}
                        </p>
                        <p>
                            Vid/Pid: {device.port_type?.UsbPort?.vid}/{device
                                .port_type?.UsbPort?.vid}
                        </p>
                        <p>
                            Serial #: {device.port_type?.UsbPort?.serial_number}
                        </p>
                    </div>
                </div>
            </div>
        </div>
        <button class="btn" on:click={connect}>Connect</button>
    </div>
</div>

<style>
</style>
