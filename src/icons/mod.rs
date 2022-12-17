use druid::{kurbo::Line, RenderContext};

use crate::IconData;

pub struct Icons;

impl Icons {
    pub const ARROW_BACK: IconData = IconData {
        paint: |ctx, color| {
            let size = ctx.size().width;
            let line_width = 2.;
            let half_the_width_of_the_line = line_width / 2.;
            let center_position = size / 2.;

            let curved_line_to_top =
                Line::new((0., center_position), (center_position + line_width, 0.0));
            let curved_line_to_bottom = Line::new(
                (0.0, center_position - half_the_width_of_the_line),
                (center_position + line_width, size),
            );
            let center_line = Line::new(
                (half_the_width_of_the_line, center_position),
                (size, center_position),
            );

            ctx.stroke(curved_line_to_top, color, line_width);
            ctx.stroke(curved_line_to_bottom, color, line_width);
            ctx.stroke(center_line, color, line_width);
        },
    };

    pub const CLOSE: IconData = IconData {
        paint: |ctx, color| {
            let size = ctx.size().width;
            let line_width = 2.;
            let line_one = Line::new((0., 0.), (size, size));
            let line_two = Line::new((0., size), (size, 0.));

            ctx.stroke(line_one, color, line_width);
            ctx.stroke(line_two, color, line_width);
        },
    };

    pub const MENU: IconData = IconData {
        paint: |ctx, color| {
            let size = ctx.size().width;
            let tree_line = 2.;
            let line_width = 2.;

            let current_size_of_all_rows = tree_line * line_width;
            let free_size = size - current_size_of_all_rows;
            let space_between_lines = free_size / tree_line;

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
