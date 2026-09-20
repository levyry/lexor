import cytoscape from 'cytoscape';
import dagre from 'cytoscape-dagre';

cytoscape.use(dagre);

export const getRedexColors = (comb: string) => {
    const colors: Record<string, any> = {
        S: { head: '#963232', body: '#321414', border: '#ff6464' },
        K: { head: '#326496', body: '#14283c', border: '#6496ff' },
        I: { head: '#329632', body: '#143c14', border: '#64ff64' },
        B: { head: '#966432', body: '#3c2814', border: '#ffb464' },
        C: { head: '#963296', body: '#3c143c', border: '#ff64ff' }
    };
    return colors[comb] || { head: '#3c3c3c', body: '#2d2d2d', border: '#555555' };
};

export const buildCytoscapeElements = (apiNodes: any[]) => apiNodes.flatMap(node => {
    const isApp = node.kind === 'App';
    const label = isApp ? '@' : node.kind.Comb;
    const colors = isApp ? { head: '#3c3c3c', border: '#555555' } : getRedexColors(label);
    return [
        { data: { id: node.id.toString(), label, bgColor: colors.head, borderColor: colors.border, isApp } },
        ...node.children.map((childId: number) => ({ data: { source: node.id.toString(), target: childId.toString() } }))
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
            'shape': (ele: any) => ele.data('isApp') ? 'ellipse' : 'round-rectangle',
            'width': '30px', 'height': '30px'
        }
    },
    {
        selector: 'edge',
        style: {
            'width': 2,
            'line-color': '#555',
            'target-arrow-color': '#555',
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier'
        }
    }
];