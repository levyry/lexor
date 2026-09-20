import { useEffect, useState } from 'preact/hooks';
import { GoldenLayout } from 'golden-layout';
import { useStore } from '../store';
import { getRedexColors } from '../graphUtils';

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

export const WorksheetPanel = ({ sourceId, layout }: { sourceId: number, layout: GoldenLayout }) => {
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
        const titles: Record<string, string> = { AstGraph: 'AST Graph', PathsGraph: 'Reduction Paths', Converter: 'Conversion' };
        layout.addComponent(val, { sourceId, panelId: Math.random().toString(36).slice(2) }, `${titles[val]} (Worksheet #${sourceId})`);
        setIsAddMenuOpen(false);
    };

    const renderStepTokens = (stepTokens: any[]) => {
        const chunks: any[] = [];
        stepTokens.forEach(t => {
            const key = typeof t.style === 'object' ? ('RedexHead' in t.style ? `H-${t.style.RedexHead}` : `B-${JSON.stringify(t.style.RedexBody)}`) : 'Normal';
            const last = chunks[chunks.length - 1];
            if (last && last.key === key) last.tokens.push(t);
            else chunks.push({ key, style: t.style, tokens: [t] });
        });

        return chunks.map((c, i) => {
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
            return <span key={i} style={{ backgroundColor: bg, color, border: `1px solid ${border}`, padding: `0 ${px}`, borderRadius: '2px' }}>{text}</span>;
        });
    };

    const handleMenuBlur = (e: any, closeFn: () => void) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node)) closeFn();
    };

    return (
        <div className="flex flex-col h-full bg-[#1e1e1e] text-gray-300 text-base">
            <div className="@container flex flex-col shrink-0 border-b border-[#333] p-3 bg-[#252526] gap-3">

                {/* Toolbar */}
                <div className="flex items-center gap-2 w-full">

                    {/* Font size */}
                    <div className="relative" onBlur={e => handleMenuBlur(e, () => setIsFontMenuOpen(false))}>
                        <button onClick={() => setIsFontMenuOpen(!isFontMenuOpen)} className={`${btnBase} px-3 font-serif italic`} title="Change Font Size">
                            <span className="font-bold text-lg">A</span><span className="text-sm leading-none">a</span>
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
                    <div className="relative" onBlur={e => handleMenuBlur(e, () => setIsAddMenuOpen(false))}>
                        <button onClick={() => setIsAddMenuOpen(!isAddMenuOpen)} className={`${btnBase} px-3 flex items-center gap-1.5`} title="Add a new pane">
                            <svg className="w-4 h-4 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                            <span className="hidden @md:inline whitespace-nowrap">Add new...</span>
                        </button>
                        {isAddMenuOpen && (
                            <div className="absolute top-full left-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-1 min-w-[150px] whitespace-nowrap">
                                {isSki ? (
                                    <>
                                        <div className="px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer text-gray-300" onMouseDown={(e) => { e.preventDefault(); handleAddView('AstGraph'); }}>AST graph</div>
                                        <div className="px-3 py-1.5 hover:bg-[#094771] hover:text-white cursor-pointer text-gray-300" onMouseDown={(e) => { e.preventDefault(); handleAddView('PathsGraph'); }}>State space graph</div>
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
                    <div className="relative" onBlur={e => handleMenuBlur(e, () => setIsSettingsOpen(false))}>
                        <button onClick={() => setIsSettingsOpen(!isSettingsOpen)} className={`${btnBase} px-3 flex items-center gap-1.5`} title="Configuration">
                            <svg className="w-4 h-4 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
                            <span className="hidden @md:inline whitespace-nowrap">Settings</span>
                        </button>
                        {isSettingsOpen && (
                            <div className="absolute top-full right-0 mt-1 bg-[#252526] border border-[#444] rounded shadow-xl z-50 py-3 px-3 min-w-[200px] flex flex-col gap-3">
                                {isSki && (
                                    <label className="flex items-center gap-2 cursor-pointer hover:text-white text-gray-300">
                                        <input type="checkbox" checked={source.nodeSharing} onChange={e => updateSource(sourceId, { nodeSharing: e.currentTarget.checked })} />
                                        Share Subtrees
                                    </label>
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
                        <option value="Ski">SKI</option>
                        <option value="Lambda">λ</option>
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