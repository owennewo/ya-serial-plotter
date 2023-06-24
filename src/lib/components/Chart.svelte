<script lang="ts">
    import uPlot from "uplot";
    import { onMount } from "svelte";
    import "uplot/dist/uPlot.min.css";

    import { listen } from "@tauri-apps/api/event";

    import { schemeSet1, schemePastel1 } from "d3-scale-chromatic";

    const max_size = 500;

    let seriesCount = -1;

    export const muxData = [];

    listen("mux-stream", (event) => {
        const payload = event.payload as number[];
        const data = payload.data;
        // console.log("payload", payload);

        // data.unshift(Date.now() - initialTime);

        if (muxData.length == 0) {
            muxData.push([payload.timestamp]);
            data.forEach((item, index) => {
                muxData.push([item]);
            });
            //     muxData = [payload.timestamp, ...payload.data]);
        } else {
            muxData[0].push(payload.timestamp);
            data.forEach((item, index) => {
                muxData[index + 1].push(item);
            });
        }
        // muxData.push([payload.timestamp, ...payload.data]);

        if (muxData[0].length > max_size) {
            muxData.forEach((item) => {
                item.shift();
            });
            // muxData.shift();
        }
    });

    // console.log("all_data", muxData);

    let el;
    let fps = 0;

    let now = 0; //Math.floor(Date.now() / 1e3);
    let length = 600;
    let shift = length - 1;

    const opts = {
        title: "6 series x 600 points @ 60fps",
        width: "1000",
        height: 600,
        pxAlign: false,
        scales: {
            y: {
                //	auto: false,
                range: [-2, 2],
            },
            x: {
                time: false,
            },
        },
        axes: [
            {
                space: 300,
            },
        ],
        series: [
            {
                label: "ms",
            },
            {
                label: "arg1",
                stroke: schemeSet1[4],
            },
            {
                label: "arg2",
                stroke: schemeSet1[1],
            },

            {
                label: "arg3",
                stroke: schemeSet1[2],
            },
        ],
    };

    onMount(() => {
        let u = new uPlot(opts, muxData, el);
        // u.series
        let lastTick_ = performance.now();
        function update() {
            // console.log("update", timeStamp);
            let now = performance.now();
            let delta = now - lastTick_;
            fps = 1000 / delta;
            lastTick_ = now;

            shift += 1;
            if (seriesCount != muxData.length) {
                seriesCount = muxData.length;
                while (u.series.length > 2) {
                    u.delSeries(u.series.length - 1);
                }

                for (let i = 0; i < seriesCount - 2; i++) {
                    u.addSeries({
                        label: `arg${i}`,
                        stroke: schemeSet1[i],
                    });
                }
            }
            // console.log(muxData);
            u.setData(muxData, true);
            requestAnimationFrame(update);
        }

        update();
    });
</script>

<main class="container">
    <h3>chart2</h3>
    <div>fps: {fps.toFixed(2)}</div>
    <div bind:this={el} />
</main>

<style global>
</style>
