<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let initialTime = undefined;

    let mux_data = {};

    listen("bus-receive2", (event) => {
        if (initialTime === undefined) {
            initialTime = Date.now();
        }
        const payload = event.payload as number[];

        const dataString = payload
            .map((item) => String.fromCharCode(item))
            .join("");
        const frame = dataString.split(",").map((item) => Number(item));

        frame.forEach((data, index) => {
            let series_id = `csv_${index}_${frame.length}`;
            if (mux_data[mux_id] === undefined) {
                mux_data[mux_id] = {
                    columns: [...Array(series_set.length)].map(
                        (_, i) => `${mux_id}_${i}}`
                    ),
                    data: [],
                    timestamps: [],
                    fps: 0,
                };
            }
            mux_data[mux_id].data.push(data);
            mux_data[mux_id].timestamps.push(Date.now() - initialTime);
        });

        data.unshift(Date.now() - initialTime);
    });
</script>

<main class="container">
    <h3>Confog</h3>
    <div class="overflow-x-auto">
        <table class="table">
            <!-- head -->
            <thead>
                <tr>
                    <th>Id</th>
                    <th>Type</th>
                    <th>Details</th>
                    <th>Aliases</th>
                    <th>RPS</th>
                </tr>
            </thead>
            <tbody>
                <!-- row 1 -->
                <tr>
                    <th>csv_4</th>
                    <td>csv</td>
                    <td>4args, comma separate, crlf</td>
                    <td>arg1, arg2, arg3, arg4</td>
                    <td>12.2</td>
                </tr>
            </tbody>
        </table>
    </div>
</main>

<style global>
</style>
