use druid::{kurbo::Line, RenderContext};

use crate::IconData;

pub struct Icons;

impl Icons {
    pub const CLOSE: IconData = IconData {
        paint: |ctx, color| {
            let size = ctx.size().width;
            let line_width = 3.;
            let line_one = Line::new((0., 0.), (size, size));
            let line_two = Line::new((0., size), (size, 0.));

            ctx.stroke(line_one, color, line_width);
            ctx.stroke(line_two, color, line_width);
        },
    };
}
