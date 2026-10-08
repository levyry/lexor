import { ComponentContainer, GoldenLayout, ItemType, ResolvedItemConfig } from 'golden-layout';


export function openPanelRight(
    layout: GoldenLayout,
    componentType: string,
    componentState: any,
    title: string,
    refContainer?: ComponentContainer
) {
    if (!layout.isInitialised) return;

    if (refContainer) {
        try {
            const componentItem = refContainer.parent;
            const currentStack = componentItem?.parent as any;
            if (currentStack) {
                const parent = currentStack.parent;
                if (parent && parent.isRow) {
                    const index = parent.contentItems.indexOf(currentStack);
                    if (index !== -1) {
                        parent.addComponent(componentType, componentState, title, index + 1);
                        parent.updateSize(false);
                        return;
                    }
                } else if (parent) {
                    const rowConfig = ResolvedItemConfig.createDefault(ItemType.row);
                    const row = (layout as any).createContentItem(rowConfig, parent);
                    parent.replaceChild(currentStack, row);
                    row.addChild(currentStack, 0, true);
                    row.addComponent(componentType, componentState, title, 1);
                    currentStack.size = 50;
                    row.updateSize(false);
                    return;
                }
            }
        } catch (e) {
            console.warn('Failed to insert panel to the right of container, falling back', e);
        }
    }

    // If no refContainer was provided (e.g. global "New Worksheet" button),
    // or refContainer couldn't be placed:
    try {
        const root = layout.rootItem as any;
        if (root) {
            if (root.isRow) {
                root.addComponent(componentType, componentState, title);
                root.updateSize(false);
                return;
            } else if (root.isStack) {
                const ground = (layout as any)._groundItem;
                const rowConfig = ResolvedItemConfig.createDefault(ItemType.row);
                const row = (layout as any).createContentItem(rowConfig, ground);
                ground.replaceChild(root, row);
                row.addChild(root, 0, true);
                row.addComponent(componentType, componentState, title, 1);
                root.size = 50;
                row.updateSize(false);
                return;
            }
        }
    } catch (e) {
        console.warn('Failed to append to root row, falling back', e);
    }

    // Fallback to default GoldenLayout addComponent
    layout.addComponent(componentType, componentState, title);
}
