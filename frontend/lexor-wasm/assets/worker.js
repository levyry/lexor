import init, { process_worker_job } from './lexor-wasm.js';

const initPromise = init();

self.onmessage = async (event) => {
    console.log("Worker received a job request!");

    try {
        await initPromise;

        const requestJson = event.data;
        const responseJson = process_worker_job(requestJson);

        self.postMessage(responseJson);

    } catch (error) {
        console.error("The Rust WASM module panicked or failed:", error);
    }
};