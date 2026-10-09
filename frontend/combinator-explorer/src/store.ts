import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { desugar_ski_church } from './wasm/lexor_api.js';
import { runWorkerTask } from './workers/workerClient';

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
    isGeneratingStateSpace?: boolean;
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
    evaluate: (id: number) => Promise<void> | void;
    convertSkiToLambda: (id: number) => Promise<void> | void;
    desugarSkiInput: (id: number) => void;
    appendLambdaComb: (id: number, combDef: string) => void;
    generateStateSpace: (id: number) => Promise<void> | void;
}

const createDefaultSource = (id: number, kind: 'Ski' | 'Lambda'): Source => ({
    id, kind, skiInput: "SKISKI", lambdaInput: "\\x.\\y.\\z.y(x z)",
    skiStrategy: "NormalForm", lambdaStrategy: "NormalOrder",
    fontSize: 20, nodeSharing: true, desugarChurch: false, reductionSteps: [], graphHistory: [],
    convertedLambda: "", stateSpace: null, error: null, selectedNodeId: 0, isGeneratingStateSpace: false
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
    convertSkiToLambda: async (id) => {
        try {
            const src = get().sources[id];
            if (!src) return;
            const result = await runWorkerTask({ type: "Conversion", source_id: id, input: src.skiInput });
            get().updateSource(id, { convertedLambda: result.result });
        } catch (e) { console.error(e); }
    },
    evaluate: async (id) => {
        const src = get().sources[id];
        if (!src) return;
        const isSki = src.kind === 'Ski';

        try {
            const result = await runWorkerTask({
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
            });

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
    generateStateSpace: async (id) => {
        const src = get().sources[id];
        if (!src || src.kind !== 'Ski') return;
        get().updateSource(id, { isGeneratingStateSpace: true, error: null });
        try {
            const result = await runWorkerTask({
                type: "StateSpace", source_id: id, input: src.skiInput
            });
            if (result.error) throw new Error(result.error);
            if (result.result?.Ok) {
                get().updateSource(id, {
                    stateSpace: result.result.Ok,
                    error: null,
                    selectedNodeId: 0,
                    isGeneratingStateSpace: false
                });
            } else if (result.result?.Err) {
                throw new Error(result.result.Err);
            }
        } catch (e) {
            get().updateSource(id, { error: String(e), isGeneratingStateSpace: false });
        }
    }
}), {
    name: 'lexor-state-storage',
    partialize: (state) => ({
        sources: Object.fromEntries(
            Object.entries(state.sources).map(([id, s]) => [
                id,
                {
                    id: s.id,
                    kind: s.kind,
                    skiInput: s.skiInput,
                    lambdaInput: s.lambdaInput,
                    skiStrategy: s.skiStrategy,
                    lambdaStrategy: s.lambdaStrategy,
                    fontSize: s.fontSize,
                    nodeSharing: s.nodeSharing,
                    desugarChurch: s.desugarChurch,
                    reductionSteps: s.reductionSteps?.slice(0, 50) || [],
                    graphHistory: [],
                    convertedLambda: s.convertedLambda,
                    stateSpace: null,
                    selectedNodeId: 0,
                    error: null,
                    isGeneratingStateSpace: false,
                }
            ])
        ),
        graphSteps: state.graphSteps,
        activeGraphBySource: state.activeGraphBySource,
        globalActiveGraph: state.globalActiveGraph,
    }),
}));