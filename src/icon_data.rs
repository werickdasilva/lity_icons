use druid::PaintCtx;

type FunctionPaint = fn(&mut PaintCtx);

pub struct IconData {
    pub paint: FunctionPaint,
}

impl IconData {
    pub fn new(paint: FunctionPaint) -> Self {
        Self { paint }
    }
}
