use druid::{Color, PaintCtx};

type FunctionPaint = fn(&mut PaintCtx, &Color);

pub struct IconData {
    pub paint: FunctionPaint,
}

impl IconData {
    pub fn new(paint: FunctionPaint) -> Self {
        Self { paint }
    }
}
