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