export const PathsGraphPanel = () => (
    <div className="flex flex-col h-full w-full bg-[#1e1e1e] items-center justify-center p-6 text-center">
        <div className="text-gray-400 mb-2">
            <svg className="w-10 h-10 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
            </svg>
            <h2 className="text-sm font-bold text-white mb-1">State Space Graph</h2>
            <p className="text-xs max-w-md mx-auto leading-relaxed text-gray-500">
                State space generation is pending backend implementation.
                Future versions will display the complete graph of all possible WHNF and NF reduction paths here.
            </p>
        </div>
    </div>
);