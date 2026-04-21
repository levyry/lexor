use egui::{
    Color32, FontFamily, FontId, Pos2, Shape, Stroke, Vec2,
    epaint::{CircleShape, TextShape},
};
use egui_graphs::{DisplayNode, DrawContext, NodeProps};
use petgraph::{EdgeType, stable_graph::IndexType};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomNodeShape {
    pub pos: Pos2,
    pub selected: bool,
    pub dragged: bool,
    pub hovered: bool,
    pub color: Option<Color32>,
    pub label_text: String,
    pub radius: f32,
}

impl<N: Clone> From<NodeProps<N>> for CustomNodeShape {
    fn from(node_props: NodeProps<N>) -> Self {
        Self {
            pos: node_props.location(),
            selected: node_props.selected,
            dragged: node_props.dragged,
            hovered: node_props.hovered,
            label_text: node_props.label.clone(),
            color: node_props.color(),
            radius: 10.0,
        }
    }
}

// SAFETY: This is the same code used by egui_graphs, and there haven't
//         been any cases where that overflowed, so this Should Be Good...
#[expect(clippy::arithmetic_side_effects)]
impl<N: Clone, E: Clone, Ty: EdgeType, Ix: IndexType> DisplayNode<N, E, Ty, Ix>
    for CustomNodeShape
{
    fn is_inside(&self, pos: Pos2) -> bool {
        let dir = pos - self.pos;
        dir.length() <= self.radius
    }

    fn closest_boundary_point(&self, dir: Vec2) -> Pos2 {
        self.pos + dir.normalized() * self.radius
    }

    fn shapes(&mut self, ctx: &DrawContext) -> Vec<Shape> {
        let mut res = Vec::with_capacity(3);

        let circle_center = ctx.meta.canvas_to_screen_pos(self.pos);
        let circle_radius = ctx.meta.canvas_to_screen_size(self.radius);
        let color = self.effective_color(ctx);
        let stroke = Stroke::default();

        res.push(
            CircleShape {
                center: circle_center,
                radius: circle_radius,
                fill: color,
                stroke,
            }
            .into(),
        );

        let font_size = circle_radius * 1.2;

        let text_color = Color32::WHITE;

        let galley = ctx.ctx.fonts_mut(|f| {
            f.layout_no_wrap(
                self.label_text.clone(),
                FontId::new(font_size, FontFamily::Proportional),
                text_color,
            )
        });

        let label_pos = Pos2::new(
            circle_center.x - galley.size().x / 2.0,
            circle_center.y - galley.size().y / 2.0,
        );

        res.push(TextShape::new(label_pos, galley, text_color).into());

        res
    }

    fn update(&mut self, state: &NodeProps<N>) {
        self.pos = state.location();
        self.selected = state.selected;
        self.dragged = state.dragged;
        self.hovered = state.hovered;
        self.label_text.clone_from(&state.label);
        self.color = state.color();
    }
}

impl CustomNodeShape {
    const fn is_interacted(&self) -> bool {
        self.selected || self.dragged || self.hovered
    }

    fn effective_color(&self, ctx: &DrawContext) -> Color32 {
        if let Some(c) = self.color {
            return c;
        }

        let style = if self.is_interacted() {
            ctx.ctx.style().visuals.widgets.active
        } else {
            ctx.ctx.style().visuals.widgets.inactive
        };

        style.fg_stroke.color
    }
}
