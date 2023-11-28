import initPlayground, { run_program } from "./pkg/playground.js";

await initPlayground();

self.onmessage = function (e) {
  self.postMessage(run_program("1 + 1"));
}

