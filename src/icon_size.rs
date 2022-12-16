use druid::Size;

pub enum IconSize {
    Size24,
}

impl IconSize {
    pub fn get_size(&self) -> Size {
        match self {
            Self::Size24 => Size::new(24., 24.),
        }
    }
}
