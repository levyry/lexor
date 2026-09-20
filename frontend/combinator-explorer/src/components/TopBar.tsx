export const TopBar = ({ onAddWorksheet }: { onAddWorksheet: () => void }) => {
    return (
        <div className="h-8 bg-[#2d2d2d] flex items-center px-4 gap-4 text-xs border-b border-[#1e1e1e] shadow-sm z-10 shrink-0">
            <span className="font-bold text-white tracking-wide mr-2 text-sm">Combinator Explorer</span>

            <button
                onClick={onAddWorksheet}
                className="bg-[#333] hover:bg-[#444] px-3 py-1 rounded text-white border border-[#555] transition-colors flex items-center gap-1.5 cursor-pointer"
            >
                <svg className="w-3.5 h-3.5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                New Worksheet
            </button>
        </div>
    );
};