export const TopBar = ({
    onAddWorksheet,
    onOpenGuide,
}: {
    onAddWorksheet: () => void;
    onOpenGuide: () => void;
}) => {
    return (
        <div className="h-8 bg-[#2d2d2d] flex items-center px-4 gap-3 text-xs border-b border-[#1e1e1e] shadow-sm z-10 shrink-0">
            <a
                href="/"
                className="font-bold text-white tracking-wide mr-2 text-sm select-none cursor-pointer hover:text-gray-200 transition-colors no-underline"
                title="Combinator Explorer"
            >
                Combinator Explorer
            </a>

            <button
                onClick={onAddWorksheet}
                className="bg-[#333] hover:bg-[#444] px-3 py-1 rounded text-white border border-[#555] transition-colors flex items-center gap-1.5 cursor-pointer"
            >
                <svg className="w-3.5 h-3.5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                New Worksheet
            </button>

            <button
                onClick={onOpenGuide}
                className="bg-[#333] hover:bg-[#444] px-3 py-1 rounded text-white border border-[#555] transition-colors flex items-center gap-1.5 cursor-pointer"
            >
                Guide
            </button>
        </div>
    );
};