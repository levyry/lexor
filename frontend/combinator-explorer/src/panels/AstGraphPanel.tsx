import { useEffect, useRef, useMemo, useState } from 'preact/hooks';
import CytoscapeComponent from 'react-cytoscapejs';
import cytoscape from 'cytoscape';
import { useStore } from '../store';
import { buildCytoscapeElements, cytoStylesheet } from '../graphUtils';
import { Codicon } from '../components/Codicon';

export const AstGraphPanel = ({ sourceId, panelId: initialPanelId }: { sourceId: number, panelId?: string }) => {
    const source = useStore(state => state.sources[sourceId]);
    const [panelId] = useState(() => initialPanelId || Math.random().toString(36).slice(2));
    const step = useStore(state => state.graphSteps[panelId] || 0);

    const isFocused = useStore(state => state.globalActiveGraph === panelId);

    const [isReady, setIsReady] = useState(false);
    const cyRef = useRef<cytoscape.Core | null>(null);
    const wrapperRef = useRef<HTMLDivElement>(null);

    useEffect(() => { useStore.getState().setActiveGraphForSource(sourceId, panelId); }, [panelId, sourceId]);

    if (!source) return <div className="p-2 text-red-400 text-base">Source deleted.</div>;

    const maxStep = Math.max(0, source.graphHistory.length - 1);
    const elements = useMemo(() => buildCytoscapeElements(source.graphHistory[step] || []), [source.graphHistory, step]);

    useEffect(() => { if (step > maxStep) useStore.getState().setGraphStep(panelId, maxStep); }, [maxStep, step, panelId]);

    useEffect(() => {
        if (!wrapperRef.current) return;
        const observer = new ResizeObserver(([entry]) => {
            if (entry.contentRect.width > 0 && entry.contentRect.height > 0) {
                setIsReady(true);
                cyRef.current?.resize();
                cyRef.current?.fit(undefined, 50);
            }
        });
        observer.observe(wrapperRef.current);
        return () => observer.disconnect();
    }, []);

    useEffect(() => {
        if (cyRef.current && elements.length > 0 && isReady) {
            const cy = cyRef.current;
            cy.elements().remove();
            cy.add(elements);
            cy.layout({
                name: 'dagre',
                rankDir: 'TB',
                ranker: 'network-simplex',
                nodeSep: 60,
                edgeSep: 30,
                rankSep: 60,
                animate: false,
                stop: () => {
                    cy.fit(undefined, 50);
                    if (cy.zoom() > 1.5) { cy.zoom(1.5); cy.center(); }
                }
            } as any).run();
        }
    }, [elements, isReady]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (/input|textarea|select/i.test(document.activeElement?.tagName || '')) return;

            if (useStore.getState().globalActiveGraph !== panelId || !source.graphHistory.length) return;

            if (['ArrowLeft', 'ArrowUp', 'ArrowRight', 'ArrowDown'].includes(e.key)) {
                e.preventDefault();
                const cur = useStore.getState().graphSteps[panelId] || 0;
                useStore.getState().setGraphStep(panelId, Math.max(0, Math.min(maxStep, cur + (['ArrowLeft', 'ArrowUp'].includes(e.key) ? -1 : 1))));
            }
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [source.graphHistory.length, maxStep, panelId]);

    const handleStep = (delta: number) => useStore.getState().setGraphStep(panelId, Math.max(0, Math.min(maxStep, step + delta)));

    const handleExportSvg = () => {
        if (!cyRef.current || !elements.length) return;
        const cy = cyRef.current;
        try {
            const svgContent = (cy as any).svg({
                full: true,
                bg: '#1e1e1e',
            });
            const safeName = (source.skiInput || 'ast')
                .replace(/[^a-zA-Z0-9_-]/g, '_')
                .slice(0, 30);
            const blob = new Blob([svgContent], { type: 'image/svg+xml;charset=utf-8' });
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.download = `ast-step-${step + 1}-${safeName}.svg`;
            link.href = url;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
        } catch (err) {
            console.error('Failed to export AST SVG:', err);
        }
    };

    return (
        <div className="flex flex-col h-full w-full bg-[#1e1e1e]" onMouseDown={() => useStore.getState().setActiveGraphForSource(sourceId, panelId)}>
            <div className={`flex items-center justify-between p-2 border-b shrink-0 transition-colors ${isFocused ? 'border-blue-500 bg-[#2d2d2d]' : 'border-[#333] bg-[#252526]'}`}>
                <div className="flex items-center gap-2">
                    <button disabled={step === 0} onClick={() => handleStep(-1)} className="bg-[#333] hover:bg-[#444] disabled:opacity-50 px-4 py-1 rounded text-white text-base cursor-pointer">←</button>
                    <span className="text-base font-mono text-gray-400 select-none">Step {elements.length ? step + 1 : 0} / {source.graphHistory.length}</span>
                    <button disabled={step >= maxStep} onClick={() => handleStep(1)} className="bg-[#333] hover:bg-[#444] disabled:opacity-50 px-4 py-1 rounded text-white text-base cursor-pointer">→</button>
                </div>
                {elements.length > 0 && (
                    <button
                        onClick={handleExportSvg}
                        className="bg-[#333] hover:bg-[#444] border border-[#555] px-2.5 h-[30px] rounded text-white text-sm font-medium transition-colors cursor-pointer flex items-center gap-1.5"
                        title="Export AST Graph as Vector SVG"
                    >
                        <Codicon name="export" className="text-gray-400 shrink-0" />
                        <span className="hidden sm:inline whitespace-nowrap">Export SVG</span>
                    </button>
                )}
            </div>
            <div ref={wrapperRef} className="flex-1 w-full relative min-h-[1px]">
                {!isReady ? <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-base">Initializing...</div>
                    : elements.length ? <CytoscapeComponent cy={cy => cyRef.current = cy} elements={elements} stylesheet={cytoStylesheet} style={{ width: '100%', height: '100%' }} layout={{ name: 'preset' }} />
                        : <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-base">No Graph Compiled</div>}
            </div>
        </div>
    );
};