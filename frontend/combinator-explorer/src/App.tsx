import { render } from 'preact';
import { useEffect, useRef, useState } from 'preact/hooks';
import { GoldenLayout, LayoutConfig } from 'golden-layout';
import 'golden-layout/dist/css/goldenlayout-base.css';
import 'golden-layout/dist/css/themes/goldenlayout-dark-theme.css';
import init from './wasm/lexor_api.js';
import { useStore } from './store';
import { defaultLayoutConfig } from './config/layoutConfig';
import { TopBar } from './components/TopBar';
import { IntroModal } from './components/IntroModal';
import { WorksheetPanel } from './panels/WorksheetPanel';
import { AstGraphPanel } from './panels/AstGraphPanel';
import { PathsGraphPanel } from './panels/PathsGraphPanel';
import { ConverterPanel } from './panels/ConverterPanel';

const LAYOUT_STORAGE_KEY = 'lexor-layout-storage';

export function App() {
    const containerRef = useRef<HTMLDivElement>(null);
    const layoutRef = useRef<GoldenLayout | null>(null);
    const [isGuideOpen, setIsGuideOpen] = useState(false);

    useEffect(() => { init(); }, []);

    useEffect(() => {
        if (!containerRef.current) return;
        const layout = new GoldenLayout(containerRef.current);

        layout.registerComponentFactoryFunction('Worksheet', (container) => {
            const sourceId = (container.initialState as any)?.sourceId ?? 1;
            render(<WorksheetPanel sourceId={sourceId} layout={layout} />, container.element);
            container.on('destroy', () => {
                render(null, container.element);
                useStore.getState().removeSource(sourceId);
            });
        });

        const OTHER_PANELS = { AstGraph: AstGraphPanel, PathsGraph: PathsGraphPanel, Converter: ConverterPanel };

        Object.entries(OTHER_PANELS).forEach(([name, Component]) => {
            layout.registerComponentFactoryFunction(name, (container) => {
                const state = container.initialState as any;
                const sourceId = state?.sourceId ?? 1;
                const panelId = state?.panelId ?? Math.random().toString(36).slice(2);

                const Comp = Component as any;

                render(<Comp sourceId={sourceId} panelId={panelId} />, container.element);
                container.on('destroy', () => render(null, container.element));
            });
        });
        let configToLoad: LayoutConfig = defaultLayoutConfig;
        const savedLayout = localStorage.getItem(LAYOUT_STORAGE_KEY);

        if (savedLayout) {
            try {
                configToLoad = LayoutConfig.fromResolved(JSON.parse(savedLayout));
            } catch (e) {
                console.warn('Failed to parse saved layout configuration', e);
            }
        }

        try {
            layout.loadLayout(configToLoad);
        } catch (e) {
            console.error('Failed to load saved layout, falling back to default.', e);
            localStorage.removeItem(LAYOUT_STORAGE_KEY);
            layout.loadLayout(defaultLayoutConfig);
        }

        layoutRef.current = layout;

        layout.on('stateChanged', () => {
            if (layout.isInitialised) {
                localStorage.setItem(LAYOUT_STORAGE_KEY, JSON.stringify(layout.saveLayout()));
            }
        });

        const handleResize = () => layout.updateRootSize();
        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);
            layout.destroy();
        };
    }, []);

    return (
        <div className="flex flex-col w-screen h-screen bg-[#1e1e1e] overflow-hidden">
            <TopBar
                onAddWorksheet={() => {
                    const id = useStore.getState().addSource('Ski');
                    layoutRef.current?.addComponent('Worksheet', { sourceId: id }, `Worksheet #${id}`);
                }}
                onOpenGuide={() => setIsGuideOpen(true)}
            />
            <div ref={containerRef} className="flex-1 w-full relative min-h-0" />
            <IntroModal isOpen={isGuideOpen} onClose={() => setIsGuideOpen(false)} />
        </div>
    );
}