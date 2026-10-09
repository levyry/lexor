import { useEffect, useRef, useState } from 'preact/hooks';
import { ComponentContainer, GoldenLayout } from 'golden-layout';
import { useStore } from '../store';
import { getRedexColors, groupStepTokens } from '../graphUtils';
import { openPanelRight } from '../utils/layoutUtils';
import { Codicon } from '../components/Codicon';

export const RenderStepTokens = ({ stepTokens, onTokenHover, onTokenClick, isHoverable, activeTargetId, headRef }: { stepTokens: any[], onTokenHover?: (edgeId: number | null) => void, onTokenClick?: (edgeId: number | null) => void, isHoverable?: boolean, activeTargetId?: number | null, headRef?: any }) => {
    const chunks = groupStepTokens(stepTokens);

    return <>{chunks.map((c) => {
        const text = c.tokens.map((t: any) => t.text).join('');
        let bg = "transparent", color = "#cccccc", border = "transparent", px = "0";
        if (typeof c.style === 'object') {
            const isHead = 'RedexHead' in c.style;
            const colors = getRedexColors(isHead ? c.style.RedexHead : c.style.RedexBody[0]);
            bg = isHead ? colors.head : colors.body;
            color = isHead ? 'white' : '#cccccc';
            border = colors.border;
            px = "4px";
        }
        const isHead = typeof c.style === 'object' && 'RedexHead' in c.style;
        const edgeId = c.tokens[0]?.edgeId;
        const isTargetHead = isHead && activeTargetId !== undefined && activeTargetId !== null && edgeId === activeTargetId;

        const handleMouseEnter = () => {
            if (!isHoverable || !onTokenHover) return;
            if (edgeId !== undefined && edgeId !== null) {
                onTokenHover(edgeId);
            } else {
                onTokenHover(null);
            }
        };

        return <span
            key={c.startIndex}
            ref={isTargetHead ? headRef : null}
            data-edge-id={isHead && edgeId !== undefined && edgeId !== null ? edgeId : undefined}
            data-active-head={isTargetHead ? "true" : undefined}
            style={{
                position: 'relative',
                display: 'inline-block',
                backgroundColor: bg,
                color,
                border: `1px solid ${border}`,
                padding: `0 ${px}`,
                borderRadius: '2px',
                cursor: isHoverable && (typeof c.style === 'object' || (edgeId !== undefined && edgeId !== null)) ? 'pointer' : 'default',
                transition: 'background-color 0.1s ease, color 0.1s ease, border-color 0.1s ease'
            }}
            onMouseEnter={handleMouseEnter}
            onClick={isHoverable && onTokenClick && edgeId !== undefined && edgeId !== null ? () => onTokenClick(edgeId) : undefined}
        >
            {isHoverable && typeof c.style === 'object' && (
                <span style={{
                    position: 'absolute',
                    top: '-6px', bottom: '-6px',
                    left: '0px',
                    right: '0px',
                    zIndex: 10,
                    pointerEvents: 'none'
                }} />
            )}
            {text}
        </span>;
    })}</>;
};

const LAMBDA_DEFS = [
    { comb: "S", def: "a -> b -> c -> a c(b c)", val: "(\\x.\\y.\\z.x z(y z))" },
    { comb: "K", def: "a -> b -> a", val: "(\\x.\\y.x)" },
    { comb: "I", def: "a -> a", val: "(\\x.x)" },
    { comb: "B", def: "a -> b -> c -> a(b c)", val: "(\\x.\\y.\\z.x(y z))" },
    { comb: "C", def: "a -> b -> c -> a c b", val: "(\\x.\\y.\\z.x z y)" }
];
const FONT_SIZES = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];

const btnBase = "bg-[#333] hover:bg-[#444] border border-[#555] h-[30px] rounded text-white outline-none focus:border-blue-500 transition-colors cursor-pointer flex items-center justify-center shrink-0";
const selectBase = "bg-[#2d2d2d] border border-[#444] px-2 h-[30px] rounded text-white outline-none cursor-pointer shrink-0";

export const WorksheetPanel = ({ sourceId, layout, container }: { sourceId: number, layout: GoldenLayout, container?: ComponentContainer }) => {
    const source = useStore(state => state.sources[sourceId]);
    const updateSource = useStore(state => state.updateSource);
    const evaluate = useStore(state => state.evaluate);

    const [chainStep, setChainStep] = useState(0);
    const [isFontMenuOpen, setIsFontMenuOpen] = useState(false);
    const [isAddMenuOpen, setIsAddMenuOpen] = useState(false);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);

    useEffect(() => {
        if (!source) return;
        const timer = setTimeout(() => evaluate(sourceId), 500);
        return () => clearTimeout(timer);
    }, [source?.skiInput, source?.lambdaInput, source?.kind, source?.skiStrategy, source?.lambdaStrategy, source?.nodeSharing, evaluate, sourceId]);

    useEffect(() => {
        if (source && chainStep >= source.reductionSteps.length) {
            setChainStep(Math.max(0, source.reductionSteps.length - 1));
        }
    }, [source?.reductionSteps.length]);

    if (!source) return <div className="p-2 text-red-400 text-base">Source deleted.</div>;
    const isSki = source.kind === 'Ski';

    const handleAddView = (val: string) => {
        const titles: Record<string, string> = { AstGraph: 'AST Graph', PathsGraph: 'State Space Graph', Converter: 'Conversion' };
        openPanelRight(layout, val, { sourceId, panelId: Math.random().toString(36).slice(2) }, `${titles[val]} (Worksheet #${sourceId})`, container);
        setIsAddMenuOpen(false);
    };

    const renderStepTokens = (stepTokens: any[]) => <RenderStepTokens stepTokens={stepTokens} />;

    const fontMenuRef = useRef<HTMLDivElement>(null);
    const addMenuRef = useRef<HTMLDivElement>(null);
    const settingsMenuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!isFontMenuOpen && !isAddMenuOpen && !isSettingsOpen) return;
        const handleOutsideClick = (e: MouseEvent) => {
            const target = e.target as Node;
            if (isFontMenuOpen && fontMenuRef.current && !fontMenuRef.current.contains(target)) {
                setIsFontMenuOpen(false);
            }
            if (isAddMenuOpen && addMenuRef.current && !addMenuRef.current.contains(target)) {
                setIsAddMenuOpen(false);
            }
            if (isSettingsOpen && settingsMenuRef.current && !settingsMenuRef.current.contains(target)) {
                setIsSettingsOpen(false);
            }
        };
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === 'Escape') {
                setIsFontMenuOpen(false);
                setIsAddMenuOpen(false);
                setIsSettingsOpen(false);
            }
        };
        document.addEventListener('mousedown', handleOutsideClick);
        document.addEventListener('keydown', handleKeyDown);
        return () => {
            document.removeEventListener('mousedown', handleOutsideClick);
            document.removeEventListener('keydown', handleKeyDown);
        };
    }, [isFontMenuOpen, isAddMenuOpen, isSettingsOpen]);

    const handleMenuBlur = (e: any, closeFn: () => void) => {
        if (e.relatedTarget && !e.currentTarget.contains(e.relatedTarget as Node)) closeFn();
    };

    return (
        <div className="flex flex-col h-full bg-[#1e1e1e] text-gray-300 text-base">
            <div className="@container flex flex-col shrink-0 border-b border-[#333] p-3 bg-[#252526] gap-3">

                {/* Toolbar */}
                <div className="flex items-center gap-2 w-full">

                    {/* Font size */}
                    <div className="relative" ref={fontMenuRef} onBlur={e => handleMenuBlur(e, () => setIsFontMenuOpen(false))}>
                        <button onClick={() => setIsFontMenuOpen(!isFontMenuOpen)} className={`${btnBase} px-3 flex items-center justify-center`} title="Change Font Size">
                            <Codicon name="text-size" className="text-gray-400 shrink-0" />
                        </button>
                        {isFontMenuOpen && (
                            <div className="absolute top-full left-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-1 min-w-[70px]">
                                {FONT_SIZES.map(sz => (
                                    <div
                                        key={sz}
                                        className={`px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer ${source.fontSize === sz ? 'text-white font-bold' : 'text-gray-300'}`}
                                        // Use onMouseDown and preventDefault to stop the menu from blurring/closing before the state updates
                                        onMouseDown={(e) => {
                                            e.preventDefault();
                                            updateSource(sourceId, { fontSize: sz });
                                            setIsFontMenuOpen(false);
                                        }}
                                    >
                                        {sz}px
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>

                    {/* Add new... */}
                    <div className="relative" ref={addMenuRef} onBlur={e => handleMenuBlur(e, () => setIsAddMenuOpen(false))}>
                        <button onClick={() => setIsAddMenuOpen(!isAddMenuOpen)} className={`${btnBase} px-3 flex items-center gap-1.5`} title="Add a new pane">
                            <Codicon name="add" className="text-gray-400 shrink-0" />
                            <span className="hidden @md:inline whitespace-nowrap">Add new...</span>
                        </button>
                        {isAddMenuOpen && (
                            <div className="absolute top-full left-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-1 min-w-[150px] whitespace-nowrap">
                                {isSki ? (
                                    <>
                                        <div className="px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer text-gray-300" onMouseDown={(e) => { e.preventDefault(); handleAddView('AstGraph'); }}>AST Graph</div>
                                        <div className="px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer text-gray-300" onMouseDown={(e) => { e.preventDefault(); handleAddView('PathsGraph'); }}>State Space Graph</div>
                                        <div className="px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer text-gray-300" onMouseDown={(e) => { e.preventDefault(); handleAddView('Converter'); }}>Conversion</div>
                                    </>
                                ) : (
                                    <div className="px-3 py-1.5 text-gray-500 italic">No views available</div>
                                )}
                            </div>
                        )}
                    </div>

                    <div className="flex-1" />

                    {/* Settings dropdown */}
                    <div className="relative" ref={settingsMenuRef} onBlur={e => handleMenuBlur(e, () => setIsSettingsOpen(false))}>
                        <button onClick={() => setIsSettingsOpen(!isSettingsOpen)} className={`${btnBase} px-3 flex items-center gap-1.5`} title="Configuration">
                            <Codicon name="settings-gear" className="text-gray-400 shrink-0" />
                            <span className="hidden @md:inline whitespace-nowrap">Settings</span>
                        </button>
                        {isSettingsOpen && (
                            <div className="absolute top-full right-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-3 px-3 min-w-[220px] flex flex-col gap-3">
                                {isSki && (
                                    <>
                                        <label htmlFor={`worksheet-share-subtrees-${sourceId}`} className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300 select-none">
                                            <input
                                                id={`worksheet-share-subtrees-${sourceId}`}
                                                type="checkbox"
                                                checked={source.nodeSharing}
                                                onChange={e => updateSource(sourceId, { nodeSharing: e.currentTarget.checked })}
                                                className="cursor-pointer"
                                            />
                                            Share subtrees
                                        </label>
                                        <button
                                            type="button"
                                            onMouseDown={(e) => {
                                                e.preventDefault();
                                                useStore.getState().desugarSkiInput(sourceId);
                                                setIsSettingsOpen(false);
                                            }}
                                            className="w-full text-center px-3 py-1.5 rounded bg-[#333] hover:bg-[#094771] hover:text-white text-gray-200 text-xs font-medium cursor-pointer transition-colors border border-[#444]"
                                        >
                                            Desugar to SKI
                                        </button>
                                    </>
                                )}
                                <div className="flex flex-col gap-1.5">
                                    <span className="text-xs text-gray-500 uppercase font-bold tracking-wider">Strategy</span>
                                    <select value={isSki ? source.skiStrategy : source.lambdaStrategy} onChange={e => updateSource(sourceId, isSki ? { skiStrategy: e.currentTarget.value } : { lambdaStrategy: e.currentTarget.value })} className={`${selectBase} w-full`}>
                                        {isSki ? <><option value="WeakHeadNormalForm">WHNF</option><option value="NormalForm">NF</option></> : <><option value="CallByName">CBN</option><option value="CallByValue">CBV</option><option value="NormalOrder">Normal</option></>}
                                    </select>
                                </div>
                            </div>
                        )}
                    </div>

                    {/* Language selector */}
                    <select value={source.kind} onChange={e => updateSource(sourceId, { kind: e.currentTarget.value as any })} className={`${selectBase} font-bold text-center px-1`}>
                        <option value="Ski">SKI-calculus</option>
                        <option value="Lambda">λ-calculus</option>
                    </select>
                </div>

                {/* Lambda shortcuts */}
                {!isSki && (
                    <div className="flex gap-2">
                        {LAMBDA_DEFS.map(d => (
                            <button key={d.comb} onClick={() => useStore.getState().appendLambdaComb(sourceId, d.val)} className={`${btnBase} w-7 font-bold text-base`}>{d.comb}</button>
                        ))}
                    </div>
                )}

                {/* Error alert */}
                {source.error && <div className="text-red-400 font-mono bg-red-900/20 p-2 rounded border border-red-900">Error: {source.error}</div>}

                {/* Code input */}
                <textarea
                    value={isSki ? source.skiInput : source.lambdaInput}
                    onChange={(e) => updateSource(sourceId, { [isSki ? 'skiInput' : 'lambdaInput']: e.currentTarget.value })}
                    className="w-full bg-[#1e1e1e] border border-[#444] text-[#d4d4d4] p-2 font-mono resize-none focus:outline-none focus:border-blue-500 rounded"
                    style={{ fontSize: `${source.fontSize}px` }}
                    spellcheck={false} rows={2}
                />
            </div>

            {/* Output */}
            <div className="flex-1 overflow-auto p-4 font-mono leading-relaxed" style={{ fontSize: `${source.fontSize}px` }}>
                {!source.reductionSteps.length && <div className="text-gray-500 text-center mt-10">Waiting for input...</div>}
                {source.reductionSteps.map((stepTokens, idx) => (
                    <div key={idx}
                        onClick={() => {
                            setChainStep(idx);
                            const active = useStore.getState().activeGraphBySource[sourceId];
                            if (active) useStore.getState().setGraphStep(active, idx);
                        }}
                        className={`flex flex-wrap items-center gap-x-[2px] mb-1 p-1 rounded cursor-pointer ${chainStep === idx ? 'bg-[#2a2d2e]' : 'hover:bg-[#2a2d2e]/50'}`}>
                        <span className="text-gray-500 mr-3 select-none w-8 text-right text-base">{(idx + 1).toString().padStart(2, '0')}.</span>
                        {renderStepTokens(stepTokens)}
                    </div>
                ))}
            </div>
        </div>
    );
};