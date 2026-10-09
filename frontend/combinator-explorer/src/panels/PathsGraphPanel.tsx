import { useEffect, useRef, useMemo, useState, useLayoutEffect } from 'preact/hooks';
import cytoscape from 'cytoscape';
import { useStore } from '../store';
import { cytoStylesheet } from '../graphUtils';
import { RenderStepTokens } from './WorksheetPanel';
import { Codicon } from '../components/Codicon';

export const PathsGraphPanel = ({ sourceId, panelId: initialPanelId }: { sourceId: number, panelId?: string }) => {
    const source = useStore(state => state.sources[sourceId]);
    const [panelId] = useState(() => initialPanelId || Math.random().toString(36).slice(2));
    const isFocused = useStore(state => state.globalActiveGraph === panelId);

    const selectedNodeId = source?.selectedNodeId ?? 0;
    const setSelectedNodeId = (id: number) => {
        useStore.getState().updateSource(sourceId, { selectedNodeId: id });
    };

    const [hoveredOverlayEdgeId, setHoveredOverlayEdgeId] = useState<string | null>(null);
    const [hoveredCytoEdgeId, setHoveredCytoEdgeId] = useState<string | null>(null);
    const [showNormalOrder, setShowNormalOrder] = useState(true);
    const [enableHighlighting, setEnableHighlighting] = useState(true);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);
    const [expansionMode, setExpansionMode] = useState<'none' | 'normal' | 'all'>('none');
    const [nodePos, setNodePos] = useState({ x: -1000, y: -1000 });
    const [_, setMousePos] = useState({ x: 0, y: 0 });

    const cyRef = useRef<cytoscape.Core | null>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const boxRef = useRef<HTMLDivElement>(null);
    const headRef = useRef<HTMLElement>(null);
    const idleWidthRef = useRef<number>(0);
    const idleHeadOffsetsRef = useRef<Map<number, number>>(new Map());

    const activeEdgeId = hoveredOverlayEdgeId || hoveredCytoEdgeId;

    useEffect(() => { useStore.getState().setActiveGraphForSource(sourceId, panelId); }, [panelId, sourceId]);

    const handleMenuBlur = (e: any, closeFn: () => void) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node)) closeFn();
    };

    const updateOverlayPos = () => {
        if (!cyRef.current) return;
        const cy = cyRef.current;
        const currentSelectedId = useStore.getState().sources[sourceId]?.selectedNodeId ?? 0;
        const node = cy.getElementById(`n${currentSelectedId}`);
        if (node.length > 0) {
            const pos = node.renderedPosition();
            setNodePos({ x: pos.x, y: pos.y });
        }
    };

    const normalOrderNodeIds = useMemo(() => {
        if (!source?.stateSpace) return new Set<number>();
        const { edges } = source.stateSpace;
        const set = new Set<number>([0]);
        let current = 0;
        while (true) {
            const nextEdge = edges.find((e: any) => e.from === current && e.is_normal_order);
            if (!nextEdge || set.has(nextEdge.to)) break;
            set.add(nextEdge.to);
            current = nextEdge.to;
        }
        return set;
    }, [source?.stateSpace]);

    const elements = useMemo(() => {
        if (!source?.stateSpace) return [];
        const { nodes, edges } = source.stateSpace;

        const cyNodes = nodes.map((n: any) => {
            const isSelected = n.id === selectedNodeId;
            const isNormal = normalOrderNodeIds.has(n.id);
            const isExpanded = expansionMode === 'all' ||
                (expansionMode === 'normal' && isNormal) ||
                isSelected;

            let bgColor = '#444';
            let borderColor = '#666';
            let borderWidth = 2;
            let opacity = 1;
            let textColor = '#e5e7eb';

            if (isSelected) {
                if (enableHighlighting) {
                    bgColor = 'rgba(0,0,0,0)';
                    borderColor = 'rgba(0,0,0,0)';
                    borderWidth = 0;
                    opacity = 0;
                    textColor = 'rgba(0,0,0,0)';
                } else {
                    bgColor = '#1e1e1e';
                    borderColor = '#3b82f6';
                    borderWidth = 2.5;
                    opacity = 1;
                    textColor = '#e5e7eb';
                }
            } else if (isExpanded) {
                bgColor = '#1e1e1e';
                borderColor = isNormal && showNormalOrder ? '#ef4444' : '#555';
                borderWidth = isNormal && showNormalOrder ? 2.5 : 1.5;
            } else {
                bgColor = isNormal && showNormalOrder ? '#ef4444' : '#444';
                borderColor = isNormal && showNormalOrder ? '#b91c1c' : '#666';
            }

            return {
                group: 'nodes',
                data: {
                    id: `n${n.id}`,
                    label: isExpanded ? ` ${n.expression} ` : '',
                    bgColor,
                    borderColor,
                    borderWidth,
                    textColor,
                    opacity,
                    nodeShape: isExpanded ? 'round-rectangle' : 'ellipse',
                    width: isExpanded ? `${Math.max(40, n.expression.length * 9.5 + 20)}px` : '12px',
                    height: isExpanded ? '30px' : '12px',
                    padding: isExpanded ? '6px' : '0px',
                }
            };
        });

        // Group edges by `from` node and reverse each group so Dagre orders left-to-right correctly
        const edgesByFrom = new Map<number, any[]>();
        edges.forEach((e: any, i: number) => {
            const list = edgesByFrom.get(e.from) || [];
            list.push({ edge: e, originalIndex: i });
            edgesByFrom.set(e.from, list);
        });

        const cyEdges: any[] = [];
        edgesByFrom.forEach(group => {
            // Reverse outgoing edges for this node so leftmost redex target is placed on the left by Dagre
            group.slice().reverse().forEach(({ edge: e, originalIndex: i }) => {
                cyEdges.push({
                    group: 'edges',
                    data: {
                        id: `e${i}`,
                        source: `n${e.from}`,
                        target: `n${e.to}`,
                        isNormalOrder: e.is_normal_order,
                        toId: e.to,
                    }
                });
            });
        });

        return [...cyNodes, ...cyEdges];
    }, [source?.stateSpace, selectedNodeId, expansionMode, showNormalOrder, normalOrderNodeIds, enableHighlighting]);

    const customStylesheet = useMemo(() => {
        return [
            ...cytoStylesheet,
            {
                selector: 'node',
                style: {
                    'width': 'data(width)',
                    'height': 'data(height)',
                    'label': 'data(label)',
                    'color': 'data(textColor)',
                    'background-color': 'data(bgColor)',
                    'border-color': 'data(borderColor)',
                    'border-width': 'data(borderWidth)' as any,
                    'opacity': 'data(opacity)' as any,
                    'padding': 'data(padding)' as any,
                    'font-family': 'monospace',
                    'font-size': '15px',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'shape': 'data(nodeShape)' as any,
                }
            },
            {
                selector: 'edge',
                style: {
                    'width': (ele: any) => (showNormalOrder && ele.data('isNormalOrder')) ? 3 : 2,
                    'curve-style': 'bezier',
                    'arrow-scale': 1.2,
                }
            }
        ] as cytoscape.StylesheetStyle[];
    }, [showNormalOrder]);

    useEffect(() => {
        if (!containerRef.current) return;

        const cy = cytoscape({
            container: containerRef.current,
            style: customStylesheet,
            elements: [],
            wheelSensitivity: 0.2,
        });
        cyRef.current = cy;

        cy.on('pan zoom position', updateOverlayPos);

        cy.on('mouseover', 'edge', (event) => {
            setHoveredCytoEdgeId(event.target.id());
        });

        cy.on('mouseout', 'edge', () => {
            setHoveredCytoEdgeId(null);
        });

        cy.on('tap', 'edge', (event) => {
            setSelectedNodeId(event.target.data('toId'));
        });

        cy.on('tap', 'node', (event) => {
            setSelectedNodeId(parseInt(event.target.id().replace('n', ''), 10));
        });

        const observer = new ResizeObserver(() => {
            cy.resize();
            cy.fit(undefined, 50);
            updateOverlayPos();
        });
        observer.observe(containerRef.current);

        return () => {
            observer.disconnect();
            cy.destroy();
            cyRef.current = null;
        };
    }, []);

    useEffect(() => {
        if (cyRef.current) cyRef.current.style(customStylesheet);
    }, [customStylesheet]);

    useEffect(() => {
        if (!cyRef.current || elements.length === 0) return;
        const cy = cyRef.current;

        cy.elements().remove();
        cy.add(elements);

        cy.layout({
            name: 'dagre',
            rankDir: 'TB',
            ranker: 'tight-tree',
            nodeSep: 50,
            edgeSep: 40,
            rankSep: 60,
            animate: false,
            stop: () => {
                cy.fit(undefined, 50);
                updateOverlayPos();
            }
        } as any).run();
    }, [elements]);

    useEffect(() => {
        if (!cyRef.current) return;
        const cy = cyRef.current;
        cy.edges().forEach(edge => {
            const isNorm = showNormalOrder && edge.data('isNormalOrder');
            const isHovered = activeEdgeId !== null && edge.id() === activeEdgeId;
            edge.style('line-color', isHovered ? '#3b82f6' : (isNorm ? '#ef4444' : '#555'));
            edge.style('target-arrow-color', isHovered ? '#3b82f6' : (isNorm ? '#ef4444' : '#777'));
        });
    }, [activeEdgeId, showNormalOrder, elements]);

    const { displayTokens, activeTargetId } = useMemo(() => {
        if (!source?.stateSpace) return { displayTokens: null, activeTargetId: null };
        const { nodes, edges } = source.stateSpace;

        if (!enableHighlighting) {
            const node = nodes.find((n: any) => n.id === selectedNodeId);
            const toks = [{ text: node?.expression || '', style: 'Normal', edgeId: null }];
            return { displayTokens: toks, activeTargetId: null };
        }

        const outgoingEdges = edges.filter((e: any) => e.from === selectedNodeId);

        if (outgoingEdges.length === 0) {
            const node = nodes.find((n: any) => n.id === selectedNodeId);
            const toks = [{ text: node?.expression || '', style: 'Normal', edgeId: null }];
            return { displayTokens: toks, activeTargetId: null };
        }

        const firstTokens = outgoingEdges[0].transition_tokens;
        const idleTokens = firstTokens.map((t: any, i: number) => {
            const headEdge = outgoingEdges.find((e: any) => {
                const s = e.transition_tokens[i].style;
                return typeof s === 'object' && 'RedexHead' in s;
            });
            return {
                ...t,
                style: headEdge ? headEdge.transition_tokens[i].style : 'Normal',
                edgeId: headEdge ? headEdge.to : null,
                isActive: false,
            };
        });

        let activeTargetId: number | null = null;
        if (activeEdgeId !== null) {
            const edgeIndex = parseInt(activeEdgeId.replace('e', ''), 10);
            const edge = edges[edgeIndex];
            if (edge && edge.from === selectedNodeId) {
                activeTargetId = edge.to;
                const tokensWithEdge = edge.transition_tokens.map((t: any, i: number) => {
                    const isPartOfActiveRedex = typeof t.style === 'object';
                    if (isPartOfActiveRedex) {
                        return { ...t, edgeId: edge.to, isActive: true };
                    }
                    const idleToken = idleTokens[i];
                    return {
                        ...t,
                        style: 'Normal',
                        edgeId: idleToken ? idleToken.edgeId : null,
                        isActive: false,
                    };
                });
                return { displayTokens: tokensWithEdge, activeTargetId };
            }
        }

        return { displayTokens: idleTokens, activeTargetId };
    }, [source?.stateSpace, selectedNodeId, activeEdgeId, enableHighlighting]);

    useLayoutEffect(() => {
        if (!boxRef.current) return;
        const box = boxRef.current;

        if (activeTargetId === null) {
            const currentWidth = box.offsetWidth;
            idleWidthRef.current = currentWidth;

            const offsets = new Map<number, number>();
            const headElements = box.querySelectorAll<HTMLElement>('[data-edge-id]');
            headElements.forEach(el => {
                const idStr = el.getAttribute('data-edge-id');
                if (idStr) {
                    const id = parseInt(idStr, 10);
                    if (!isNaN(id)) {
                        offsets.set(id, el.offsetLeft);
                    }
                }
            });
            idleHeadOffsetsRef.current = offsets;

            box.style.left = `${nodePos.x - currentWidth / 2}px`;
        } else {
            const idleWidth = idleWidthRef.current || box.offsetWidth;
            const idleLeft = nodePos.x - idleWidth / 2;

            if (headRef.current && idleHeadOffsetsRef.current.has(activeTargetId)) {
                const idleHeadOffset = idleHeadOffsetsRef.current.get(activeTargetId)!;
                const activeHeadOffset = headRef.current.offsetLeft;
                const shift = idleHeadOffset - activeHeadOffset;
                box.style.left = `${idleLeft + shift}px`;
            } else {
                box.style.left = `${nodePos.x - box.offsetWidth / 2}px`;
            }
        }
    });

    const handleTokenHover = (toId: number | null) => {
        if (toId === null) {
            setHoveredOverlayEdgeId(null);
        } else if (source?.stateSpace) {
            const edgeIndex = source.stateSpace.edges.findIndex((e: any) => e.from === selectedNodeId && e.to === toId);
            if (edgeIndex !== -1) setHoveredOverlayEdgeId(`e${edgeIndex}`);
        }
    };

    if (!source) return <div className="p-2 text-red-400 text-base">Source deleted.</div>;

    return (
        <div
            className="flex flex-col h-full w-full bg-[#1e1e1e]"
            onMouseDown={() => useStore.getState().setActiveGraphForSource(sourceId, panelId)}
            onMouseMove={e => setMousePos({ x: e.clientX, y: e.clientY })}
        >
            <div className={`@container flex items-center justify-between p-2 border-b shrink-0 transition-colors ${isFocused ? 'border-blue-500 bg-[#2d2d2d]' : 'border-[#333] bg-[#252526]'}`}>
                {/* Generate Graph Button */}
                <button
                    onClick={() => useStore.getState().generateStateSpace(sourceId)}
                    disabled={source?.isGeneratingStateSpace}
                    className={`bg-[#333] hover:bg-[#444] border border-[#555] px-3 h-[30px] rounded text-white text-sm font-medium transition-colors flex items-center gap-1.5 ${source?.isGeneratingStateSpace ? 'opacity-70 cursor-wait' : 'cursor-pointer'}`}
                    title="Generate State Space Graph"
                >
                    {source?.isGeneratingStateSpace ? (
                        <>
                            <Codicon name="loading" className="text-gray-400 shrink-0 codicon-modifier-spin" />
                            <span className="hidden @md:inline whitespace-nowrap">Generating...</span>
                        </>
                    ) : (
                        <>
                            <Codicon name="type-hierarchy" className="text-gray-400 shrink-0" />
                            <span className="hidden @md:inline whitespace-nowrap">Generate Graph</span>
                        </>
                    )}
                </button>

                {/* Space in between */}
                <div className="flex-1" />

                {/* Settings Dropdown */}
                <div className="relative" onBlur={e => handleMenuBlur(e, () => setIsSettingsOpen(false))}>
                    <button
                        onClick={() => setIsSettingsOpen(!isSettingsOpen)}
                        className="bg-[#333] hover:bg-[#444] border border-[#555] px-3 h-[30px] rounded text-white text-sm font-medium transition-colors cursor-pointer flex items-center gap-1.5"
                        title="Configuration"
                    >
                        <Codicon name="settings-gear" className="text-gray-400 shrink-0" />
                        <span className="hidden @md:inline whitespace-nowrap">Settings</span>
                    </button>
                    {isSettingsOpen && (
                        <div className="absolute top-full right-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-3 px-3 min-w-[220px] flex flex-col gap-2.5">
                            <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 text-sm select-none">
                                <input
                                    type="checkbox"
                                    checked={showNormalOrder}
                                    onChange={e => setShowNormalOrder(e.currentTarget.checked)}
                                />
                                Normal Order Path
                            </label>

                            <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 text-sm select-none">
                                <input
                                    type="checkbox"
                                    checked={enableHighlighting}
                                    onChange={e => setEnableHighlighting(e.currentTarget.checked)}
                                />
                                Redex Highlighting
                            </label>

                            <div className="h-px bg-[#333] my-1" />

                            <span className="text-xs text-gray-500 uppercase font-bold tracking-wider">Node Expansion</span>

                            <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 text-sm select-none">
                                <input
                                    type="radio"
                                    name="nodeExpansion"
                                    checked={expansionMode === 'none'}
                                    onChange={() => setExpansionMode('none')}
                                />
                                Selected Only
                            </label>

                            <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 text-sm select-none">
                                <input
                                    type="radio"
                                    name="nodeExpansion"
                                    checked={expansionMode === 'normal'}
                                    onChange={() => setExpansionMode('normal')}
                                />
                                Expand Normal Order Path
                            </label>

                            <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 text-sm select-none">
                                <input
                                    type="radio"
                                    name="nodeExpansion"
                                    checked={expansionMode === 'all'}
                                    onChange={() => setExpansionMode('all')}
                                />
                                Expand All Nodes
                            </label>
                        </div>
                    )}
                </div>
            </div>

            <div className="flex-1 w-full relative min-h-[1px]">
                <div ref={containerRef} style={{ width: '100%', height: '100%', visibility: source.stateSpace ? 'visible' : 'hidden' }} />

                {source?.stateSpace?.truncated && (
                    <div className="absolute top-3 left-1/2 -translate-x-1/2 z-10 flex items-center gap-1.5 text-xs bg-[#332b00]/95 border border-[#f59e0b] text-[#fef08a] px-3 py-1.5 rounded shadow-lg select-none whitespace-nowrap">
                        <Codicon name="warning" className="text-[#f59e0b]" />
                        <span>State space exceeded limit ({source.stateSpace.nodes?.length ?? 150} nodes) and was truncated</span>
                    </div>
                )}

                {source?.stateSpace && showNormalOrder && (
                    <div className="absolute bottom-3 left-3 z-10 flex items-center gap-2 text-xs font-mono bg-[#1e1e1e]/90 border border-[#333] px-2.5 py-1.5 rounded shadow pointer-events-none select-none">
                        <div className="w-4 h-0.5 bg-[#ef4444]"></div>
                        <span className="text-gray-400">Normal Order Path</span>
                    </div>
                )}

                {!source.stateSpace && (
                    <div className="absolute inset-0 flex items-center justify-center flex-col text-gray-500 p-8 text-center pointer-events-none">
                        {source?.isGeneratingStateSpace ? (
                            <>
                                <Codicon name="loading" className="text-[48px] mb-4 opacity-50 codicon-modifier-spin text-blue-400" />
                                <h2 className="text-lg font-bold text-gray-300 mb-2">Generating State Space...</h2>
                                <p className="text-sm max-w-sm">Exploring reduction graph in background worker...</p>
                            </>
                        ) : (
                            <>
                                <Codicon name="type-hierarchy" className="text-[48px] mb-4 opacity-30" />
                                <h2 className="text-lg font-bold text-gray-400 mb-2">No State Space Generated</h2>
                                <p className="text-sm max-w-sm">Click the button above to generate a graph of all possible reduction paths for the current expression.</p>
                                {source?.error && <p className="mt-4 text-red-400 text-xs break-all">{source.error}</p>}
                            </>
                        )}
                    </div>
                )}

                {source.stateSpace && enableHighlighting && nodePos.x > -500 && displayTokens && (
                    <div
                        ref={boxRef}
                        onMouseLeave={() => handleTokenHover(null)}
                        className="absolute z-10 pointer-events-auto font-mono flex items-center bg-[#1e1e1e] border border-blue-500 rounded p-3 text-white text-lg shadow-2xl whitespace-nowrap gap-x-[2px]"
                        style={{
                            top: nodePos.y,
                            transform: 'translateY(-50%)',
                        }}
                    >
                        <RenderStepTokens
                            stepTokens={displayTokens}
                            activeTargetId={activeTargetId}
                            headRef={headRef}
                            isHoverable={true}
                            onTokenHover={handleTokenHover}
                            onTokenClick={(id) => { if (id !== null) setSelectedNodeId(id); }}
                        />
                    </div>
                )}
            </div>
        </div>
    );
};