//! Composed widget

use crate::data::*;
use druid::{widget::Label, Widget};

pub fn build_ui() -> impl Widget<AppState> {
    Label::new("Hello")
}
