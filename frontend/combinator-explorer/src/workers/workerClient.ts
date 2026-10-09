let worker: Worker | null = null;
const pendingTasks = new Map<number, { resolve: (val: any) => void; reject: (err: any) => void }>();
let nextTaskId = 1;

function getWorker(): Worker {
    if (!worker) {
        worker = new Worker(new URL('./reductionWorker.ts', import.meta.url), { type: 'module' });
        worker.onmessage = (e: MessageEvent) => {
            const { id, result, error } = e.data;
            const pending = pendingTasks.get(id);
            if (pending) {
                pendingTasks.delete(id);
                if (error) {
                    pending.reject(new Error(error));
                } else {
                    pending.resolve(result);
                }
            }
        };
        worker.onerror = (err) => {
            console.error('Reduction worker error:', err);
        };
    }
    return worker;
}

export function runWorkerTask<T = any>(task: any): Promise<T> {
    return new Promise((resolve, reject) => {
        const id = nextTaskId++;
        pendingTasks.set(id, { resolve, reject });
        getWorker().postMessage({ id, task });
    });
}
