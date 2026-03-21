import init, { process_worker_job } from './lexor-wasm.js';

async function start() {
    await init();
    self.onmessage = (event) => {
        const requestJson = event.data;
        const responseJson = process_worker_job(requestJson);

        self.postMessage(responseJson);
    };
}
start();