import { LayoutConfig } from 'golden-layout';

export const defaultLayoutConfig: LayoutConfig = {
    header: {
        popout: false,
    },
    root: {
        type: 'row',
        content: [
            {
                type: 'component',
                componentType: 'Worksheet',
                title: 'Worksheet #1',
                componentState: { sourceId: 1 }
            },
            {
                type: 'component',
                componentType: 'AstGraph',
                title: 'AST Graph (Worksheet #1)',
                componentState: { sourceId: 1, panelId: 'default-ast-1' }
            }
        ]
    }
};

export const createPresetLayout = (sourceId: number, includePathsGraph: boolean): LayoutConfig => {
    const content: any[] = [
        {
            type: 'component',
            componentType: 'Worksheet',
            title: `Worksheet #${sourceId}`,
            componentState: { sourceId }
        }
    ];

    if (includePathsGraph) {
        content.push({
            type: 'component',
            componentType: 'PathsGraph',
            title: `State Space Graph (Worksheet #${sourceId})`,
            componentState: { sourceId, panelId: Math.random().toString(36).slice(2) }
        });
    }

    return {
        header: {
            popout: false,
        },
        root: {
            type: 'row',
            content
        }
    };
};