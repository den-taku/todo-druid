//! Composed widget

use crate::data::*;
use druid::{
    widget::{Checkbox, Flex, Label, List},
    Widget, WidgetExt,
};

fn todo_item() -> impl Widget<TodoItem> {
    let checkbox = Checkbox::new("").lens(TodoItem::done);
    let label = Label::raw().lens(TodoItem::text);

    Flex::row().with_child(checkbox).with_flex_child(label, 1.)
}

pub fn build_ui() -> impl Widget<AppState> {
    List::new(todo_item).lens(AppState::todos)
}
