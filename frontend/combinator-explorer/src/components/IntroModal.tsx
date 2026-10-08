import { useEffect } from 'preact/hooks';
import { useStore } from '../store';
import { createPresetLayout } from '../config/layoutConfig';

interface IntroModalProps {
    isOpen: boolean;
    onClose: () => void;
    layoutRef: any;
}

const EXAMPLES = [

    {
        expr: '(6 * 7) + (20 - 5)',
        desc: 'Basic arithmetic: 42 + 15 = 57',
    },
    {
        expr: 'K (10 + 20) (99 * 99)',
        desc: 'K combinator discarding second argument',
    },
    {
        expr: 'SKIS(KSSKSI)KSKI',
        desc: 'Explore the state space graph for expressions with multiple possible redexes',
        generateStateSpace: true,
    },
    {
        expr: '(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)I)',
        desc: 'Church exponentiation of 2^3',
    },
    {
        expr: '(S(S(KS)K)(S(S(KS)K)I))(S(S(KS)K)I)(S(S(KS)K)I)',
        desc: 'Small Ackermann function',
    },
];

export const IntroModal = ({ isOpen, onClose, layoutRef }: IntroModalProps) => {
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onClose();
        };
        if (isOpen) {
            window.addEventListener('keydown', handleKeyDown);
        }
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [isOpen, onClose]);

    if (!isOpen) return null;

    const handleLoadExample = async (example: typeof EXAMPLES[0]) => {
        const layout = layoutRef.current;
        if (!layout) return;

        const newSourceId = useStore.getState().addSource('Ski');

        useStore.getState().updateSource(newSourceId, {
            kind: 'Ski',
            skiInput: example.expr,
        });
        await useStore.getState().evaluate(newSourceId);

        let needsPathGraph = 'generateStateSpace' in example && example.generateStateSpace;
        if (needsPathGraph) {
            await useStore.getState().generateStateSpace(newSourceId);
        }

        const presetLayout = createPresetLayout(newSourceId, needsPathGraph);

        layout.loadLayout(presetLayout);

        onClose();
    };

    return (
        <div
            className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50"
            onClick={onClose}
        >
            <div
                className="w-full max-w-2xl bg-[#252526] border border-[#444] rounded shadow-lg flex flex-col text-[#cccccc] text-xs font-mono"
                onClick={e => e.stopPropagation()}
            >
                <div className="flex items-center justify-between px-4 py-2 border-b border-[#333] bg-[#2d2d2d] text-white">
                    <span>Combinator Explorer guide</span>
                    <button
                        onClick={onClose}
                        className="text-[#888] hover:text-white cursor-pointer px-1"
                        title="Close"
                    >
                        ✕
                    </button>
                </div>

                <div className="p-4 space-y-4 overflow-y-auto max-h-[80vh] leading-relaxed">
                    <div>
                        SKI combinator calculus is a turing complete, variable-free computational model equivalent to lambda calculus.
                        Expressions are built using function applications and primitive combinators.
                    </div>

                    Reduction rules:
                    <div className="border border-[#333] bg-[#1e1e1e] p-3 space-y-1">
                        <div>I x       -&gt; x</div>
                        <div>K x y     -&gt; x</div>
                        <div>S x y z   -&gt; x z (y z)</div>
                        <div>B x y z   -&gt; x (y z)</div>
                        <div>C x y z   -&gt; x z y</div>
                    </div>

                    <div>
                        Reduction strategies:
                        <div>* WHNF (Weak Head Normal Form): Reduces the outermost redex until the head is an unapplied combinator or primitive.</div>
                        <div>* NF (Normal Form): Reduces all sub-expressions until no reducible expressions remain.</div>
                    </div>

                    <div>
                        Church numerals and operators:
                        <div>* Numbers can be written as regular integers (0, 1, 2, ...) and reduced with operators (+, -, *, /, ^, =, &lt;, &gt;).</div>
                        <div>* Alternatively, use "Desugar to SKI" to expand numbers into Church numeral combinators (0 -&gt; K I, 1 -&gt; I, 2 -&gt; S(S(KS)K)I).</div>
                    </div>

                    <div className="space-y-2 pt-2 border-t border-[#333]">
                        <div>Examples:</div>
                        <div className="space-y-1.5">
                            {EXAMPLES.map((ex, idx) => (
                                <div key={idx} className="flex items-center gap-3 bg-[#1e1e1e] p-2 border border-[#333]">
                                    <button
                                        onClick={() => handleLoadExample(ex)}
                                        className="bg-[#333] hover:bg-[#444] border border-[#555] px-2 py-0.5 text-white cursor-pointer shrink-0"
                                    >
                                        Load
                                    </button>
                                    <div className="flex-1 min-w-0">
                                        <div className="text-white truncate">{ex.expr}</div>
                                        <div className="text-[#888]">{ex.desc}</div>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* Footer */}
                <div className="flex justify-end px-4 py-2 border-t border-[#333] bg-[#2d2d2d]">
                    <button
                        onClick={onClose}
                        className="bg-[#333] hover:bg-[#444] border border-[#555] px-4 py-1 text-white cursor-pointer"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
    );
};
