import "./styles.css";
import App from "./App.svelte";
import './lib/stores/mux.ts';


const app = new App({
  target: document.getElementById("app"),
});

export default app;
