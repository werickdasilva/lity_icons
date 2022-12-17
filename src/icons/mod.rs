use druid::{kurbo::Line, Color, RenderContext};

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

    pub const MENU: IconData = IconData {
        paint: |ctx, color| {
            let size = ctx.size().width;
            let tree_line = 3.;
            let line_width = 3.;

            let current_size_of_all_rows = tree_line * line_width;
            let free_size = size - current_size_of_all_rows;
            let space_between_lines = free_size / tree_line;

            println!("{space_between_lines}");
            let position_secound_line = (space_between_lines * 2.) + line_width;
            let position_third_line = (space_between_lines * 3.) + (line_width * 2.);

            let primary_line = Line::new((0., space_between_lines), (size, space_between_lines));
            let secound_line =
                Line::new((0., (position_secound_line)), (size, position_secound_line));
            let third_line = Line::new((0., position_third_line), (size, position_third_line));

            ctx.stroke(primary_line, color, line_width);
            ctx.stroke(secound_line, color, line_width);
            ctx.stroke(third_line, color, line_width);
        },
    };
}
