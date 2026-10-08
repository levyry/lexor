import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { process_worker_job, desugar_ski_church } from './wasm/lexor_api.js';

export interface Source {
    id: number;
    kind: 'Ski' | 'Lambda';
    skiInput: string;
    lambdaInput: string;
    skiStrategy: string;
    lambdaStrategy: string;
    fontSize: number;
    nodeSharing: boolean;
    desugarChurch: boolean;
    reductionSteps: any[];
    graphHistory: any[][];
    convertedLambda: string;
    stateSpace: any | null;
    selectedNodeId: number;
    error: string | null;
}

interface AppState {
    sources: Record<number, Source>;
    graphSteps: Record<string, number>;
    setGraphStep: (id: string, step: number) => void;
    activeGraphBySource: Record<number, string>;
    setActiveGraphForSource: (sourceId: number, panelId: string) => void;
    globalActiveGraph: string | null;
    addSource: (kind: 'Ski' | 'Lambda') => number;
    removeSource: (id: number) => void;
    updateSource: (id: number, updater: Partial<Source> | ((s: Source) => Partial<Source>)) => void;
    evaluate: (id: number) => void;
    convertSkiToLambda: (id: number) => void;
    desugarSkiInput: (id: number) => void;
    appendLambdaComb: (id: number, combDef: string) => void;
    generateStateSpace: (id: number) => void;
}

const createDefaultSource = (id: number, kind: 'Ski' | 'Lambda'): Source => ({
    id, kind, skiInput: "SKISKI", lambdaInput: "\\x.\\y.\\z.y(x z)",
    skiStrategy: "NormalForm", lambdaStrategy: "NormalOrder",
    fontSize: 20, nodeSharing: true, desugarChurch: false, reductionSteps: [], graphHistory: [],
    convertedLambda: "", stateSpace: null, error: null, selectedNodeId: 0
});

export const useStore = create<AppState>()(persist((set, get) => ({
    sources: { 1: createDefaultSource(1, 'Ski') },
    graphSteps: {},
    setGraphStep: (id, step) => set(s => ({ graphSteps: { ...s.graphSteps, [id]: step } })),
    activeGraphBySource: {},
    globalActiveGraph: null,
    setActiveGraphForSource: (sourceId, panelId) => set(s => ({
        activeGraphBySource: { ...s.activeGraphBySource, [sourceId]: panelId },
        globalActiveGraph: panelId
    })),

    addSource: (kind) => {
        const { sources } = get();
        const id = Math.max(0, ...Object.keys(sources).map(Number)) + 1;
        set({ sources: { ...sources, [id]: createDefaultSource(id, kind) } });
        return id;
    },
    removeSource: (id) => set(s => {
        const { [id]: _, ...sources } = s.sources;
        return { sources };
    }),
    updateSource: (id, updater) => set(s => {
        const src = s.sources[id];
        if (!src) return s;
        return { sources: { ...s.sources, [id]: { ...src, ...(typeof updater === 'function' ? updater(src) : updater) } } };
    }),
    appendLambdaComb: (id, combDef) => {
        const src = get().sources[id];
        if (src) get().updateSource(id, { lambdaInput: src.lambdaInput + combDef });
    },
    desugarSkiInput: (id) => {
        try {
            const src = get().sources[id];
            if (!src) return;
            const desugared = desugar_ski_church(src.skiInput);
            get().updateSource(id, { skiInput: desugared });
        } catch (e) {
            console.error('Failed to desugar Church numerals:', e);
        }
    },
    convertSkiToLambda: (id) => {
        try {
            const src = get().sources[id];
            if (!src) return;
            const result = JSON.parse(process_worker_job(JSON.stringify({ type: "Conversion", source_id: id, input: src.skiInput })));
            get().updateSource(id, { convertedLambda: result.result });
        } catch (e) { console.error(e); }
    },
    evaluate: (id) => {
        const src = get().sources[id];
        if (!src) return;
        const isSki = src.kind === 'Ski';

        try {
            const result = JSON.parse(process_worker_job(JSON.stringify({
                type: "Reduction", source_id: id,
                strategy: isSki ? { Ski: src.skiStrategy } : { Lambda: src.lambdaStrategy },
                state: isSki ? {
                    Ski: {
                        wants_steps: true,
                        wants_graph: true,
                        desugar_church: src.desugarChurch ?? false,
                        node_sharing: src.nodeSharing
                    }
                } : { Lambda: { placeholder: false } },
                input: isSki ? src.skiInput : src.lambdaInput
            })));

            if (result.error) throw new Error(result.error);

            get().updateSource(id, isSki ? {
                error: null,
                reductionSteps: result.state.Ski.steps || [],
                graphHistory: result.state.Ski.graph_nodes || []
            } : {
                error: null,
                reductionSteps: [[{ text: result.state.Lambda.output, style: "Normal" }]],
                graphHistory: []
            });
        } catch (e) { get().updateSource(id, { error: String(e) }); }
    },
    generateStateSpace: (id) => {
        const src = get().sources[id];
        if (!src || src.kind !== 'Ski') return;
        try {
            const result = JSON.parse(process_worker_job(JSON.stringify({
                type: "StateSpace", source_id: id, input: src.skiInput
            })));
            if (result.error) throw new Error(result.error);
            if (result.result?.Ok) {
                get().updateSource(id, { stateSpace: result.result.Ok, error: null, selectedNodeId: 0 });
            } else if (result.result?.Err) {
                throw new Error(result.result.Err);
            }
        } catch (e) { get().updateSource(id, { error: String(e) }); }
    }
}), { name: 'lexor-state-storage' }));