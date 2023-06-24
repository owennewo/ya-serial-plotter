import { emit, listen } from "@tauri-apps/api/event";

let initialTime = undefined;
console.log("mux.ts");

listen("bus-receive", (event) => {
    if (initialTime === undefined) {
        initialTime = Date.now();
    }
    // payload is a Uint8Array
    const payload = event.payload as number[];
    // console.log("payload", payload);
    // convert the Uint8Array to a string
    const dataString = payload
        .map((item) => String.fromCharCode(item))
        .join("");
    // split the string into an array of values
    const data = dataString.split(",").map((item) => Number(item));

    const streamName = `csv_${data.length}`

    emit("mux-stream", {
        name: streamName,
        data: data,
        text: dataString,
        raw: payload,
        timestamp: Date.now() - initialTime
    });


});

