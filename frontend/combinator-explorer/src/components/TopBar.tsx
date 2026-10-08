import { Codicon } from './Codicon';

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
                <Codicon name="add" className="text-gray-400" />
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