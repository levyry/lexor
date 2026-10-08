import cytoscape from 'cytoscape';
import dagre from 'cytoscape-dagre';

cytoscape.use(dagre);

export const getRedexColors = (label: string) => {
    const colors: Record<string, any> = {
        S: { head: '#963232', body: '#321414', border: '#ff6464' },
        K: { head: '#326496', body: '#14283c', border: '#6496ff' },
        I: { head: '#329632', body: '#143c14', border: '#64ff64' },
        B: { head: '#966432', body: '#3c2814', border: '#ffb464' },
        C: { head: '#963296', body: '#3c143c', border: '#ff64ff' },
        // Operators (cyan/teal)
        '+': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '-': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '*': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '/': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '^': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '=': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '<': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
        '>': { head: '#0e7490', body: '#164e63', border: '#22d3ee' },
    };

    if (colors[label]) return colors[label];
    // Numbers (amber/gold)
    if (/^-?\d+$/.test(label)) {
        return { head: '#92400e', body: '#451a03', border: '#f59e0b' };
    }
    return { head: '#3c3c3c', body: '#2d2d2d', border: '#555555' };
};

export const groupStepTokens = (stepTokens: any[]) => {
    const chunks: any[] = [];
    let currentIndex = 0;
    stepTokens.forEach(t => {
        const key = typeof t.style === 'object' 
            ? ('RedexHead' in t.style ? `H-${t.style.RedexHead}-${t.edgeId ?? ''}` : `B-${JSON.stringify(t.style.RedexBody)}`) 
            : (t.edgeId ? `Normal-${t.edgeId}` : 'Normal');
        const last = chunks[chunks.length - 1];
        if (last && last.key === key) {
            last.tokens.push(t);
        } else {
            chunks.push({ key, style: t.style, tokens: [t], startIndex: currentIndex });
        }
        currentIndex += (t.text?.length || 1);
    });
    return chunks;
};

export const buildCytoscapeElements = (apiNodes: any[]) => apiNodes.flatMap(node => {
    const isApp = node.kind === 'App';
    let label = '@';
    let nodeType: 'app' | 'comb' | 'num' | 'op' = 'app';

    if (!isApp && typeof node.kind === 'object' && node.kind !== null) {
        if ('Comb' in node.kind) {
            label = String(node.kind.Comb);
            nodeType = 'comb';
        } else if ('Num' in node.kind) {
            label = String(node.kind.Num);
            nodeType = 'num';
        } else if ('Op' in node.kind) {
            label = String(node.kind.Op);
            nodeType = 'op';
        } else if ('Prim' in node.kind) {
            const prim = node.kind.Prim;
            if (typeof prim === 'object' && prim !== null) {
                if ('Num' in prim) {
                    label = String(prim.Num);
                    nodeType = 'num';
                } else if ('Op' in prim) {
                    label = String(prim.Op);
                    nodeType = 'op';
                }
            }
        }
    }

    const colors = isApp ? { head: '#3c3c3c', border: '#555555' } : getRedexColors(label);
    const shape = isApp ? 'ellipse' : (nodeType === 'op' ? 'diamond' : 'round-rectangle');

    return [
        {
            data: {
                id: node.id.toString(),
                label,
                bgColor: colors.head,
                borderColor: colors.border,
                nodeShape: shape,
                isApp,
            }
        },
        ...[...node.children].reverse().map((childId: number, revIndex: number) => {
            const origIndex = node.children.length - 1 - revIndex;
            return {
                data: {
                    id: `${node.id}->${childId}-${origIndex}`,
                    source: node.id.toString(),
                    target: childId.toString(),
                    isLeft: origIndex === 0,
                }
            };
        })
    ];
});

export const cytoStylesheet: cytoscape.StylesheetStyle[] = [
    {
        selector: 'node',
        style: {
            'label': 'data(label)',
            'background-color': 'data(bgColor)',
            'border-width': 2,
            'border-color': 'data(borderColor)',
            'color': '#ffffff',
            'text-valign': 'center',
            'text-halign': 'center',
            'font-family': 'monospace',
            'font-size': '12px',
            'font-weight': 'bold',
            'shape': 'data(nodeShape)' as any,
            'width': '34px',
            'height': '34px'
        }
    },
    {
        selector: 'edge',
        style: {
            'width': 2,
            'line-color': '#555',
            'target-arrow-color': '#777',
            'target-arrow-shape': 'triangle',
            'arrow-scale': 1.0,
            'curve-style': 'bezier',
            'control-point-step-size': 28,
            'opacity': 0.85
        }
    }
];