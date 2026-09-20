import { useStore } from '../store';

export const ConverterPanel = ({ sourceId }: { sourceId: number }) => {
    const source = useStore(state => state.sources[sourceId]);

    if (!source) return <div className="p-2 text-red-400 text-xs">Source deleted.</div>;

    return (
        <div className="flex flex-col h-full w-full bg-[#1e1e1e] p-3 text-xs overflow-auto">
            <div className="mb-4">
                <div className="text-gray-400 mb-1 font-bold">Converted Output:</div>
                <div className="bg-[#252526] border border-[#444] p-2 rounded text-gray-300 font-mono break-all min-h-[34px]">
                    {source.convertedLambda || <span className="text-gray-600 italic">No conversion available.</span>}
                </div>
            </div>

            <div className="text-gray-400 mb-2 border-b border-[#333] pb-1 font-bold">AST Metadata (Pending Backend)</div>

            <div className="grid grid-cols-2 gap-2 opacity-50 select-none">
                {[
                    { label: 'Node Count', val: '--' },
                    { label: 'Tree Depth', val: '--' }
                ].map(stat => (
                    <div key={stat.label} className="bg-[#252526] border border-[#333] p-2 rounded">
                        <div className="text-gray-500 uppercase text-[10px]">{stat.label}</div>
                        <div className="text-lg text-white font-mono">{stat.val}</div>
                    </div>
                ))}

                <div className="bg-[#252526] border border-[#333] p-2 rounded col-span-2">
                    <div className="text-gray-500 uppercase text-[10px] mb-1">Combinator Distribution</div>
                    <div className="flex gap-2 font-mono">
                        <span className="bg-[#963232] text-white px-2 py-0.5 rounded">S: --</span>
                        <span className="bg-[#326496] text-white px-2 py-0.5 rounded">K: --</span>
                        <span className="bg-[#329632] text-white px-2 py-0.5 rounded">I: --</span>
                    </div>
                </div>
            </div>
        </div>
    );
};