import init, { process_worker_job } from '../wasm/lexor_api.js';

let isInitialized = false;
let initPromise: Promise<any> | null = null;

async function ensureInit() {
    if (!isInitialized) {
        if (!initPromise) {
            initPromise = init();
        }
        await initPromise;
        isInitialized = true;
    }
}

self.onmessage = async (e: MessageEvent) => {
    const { id, task } = e.data;
    try {
        await ensureInit();
        const resultJson = process_worker_job(JSON.stringify(task));
        const result = JSON.parse(resultJson);
        self.postMessage({ id, result });
    } catch (err: any) {
        self.postMessage({ id, error: err?.message || String(err) });
    }
};
