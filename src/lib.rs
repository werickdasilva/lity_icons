mod icon_data;
mod icon_size;
mod icons;

use druid::{Color, Data, Widget};
pub use icon_data::IconData;
pub use icon_size::IconSize;
pub use icons::*;

pub struct Icon {
    icon: IconData,
    size: IconSize,
    color: Color,
}

impl<T: Data> Widget<T> for Icon {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut T,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &T,
        _env: &druid::Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &druid::Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &T,
        _env: &druid::Env,
    ) -> druid::Size {
        self.size.get_size()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {
        (self.icon.paint)(ctx, &self.color);
    }
}
