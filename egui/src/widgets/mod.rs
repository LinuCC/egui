//! Widgets are pieces of GUI such as [`Label`], [`Button`], [`Slider`] etc.
//!
//! Example widget uses:
//! * `ui.add(Label::new("Text").text_color(color::red));`
//! * `if ui.add(Button::new("Click me")).clicked { ... }`

#![allow(clippy::new_without_default)]

use crate::*;

mod button;
pub mod color_picker;
mod drag_value;
mod hyperlink;
mod image;
mod label;
mod selected_label;
mod separator;
mod slider;
pub(crate) mod text_edit;

pub use hyperlink::*;
pub use label::*;
pub use selected_label::*;
pub use separator::*;
pub use {button::*, drag_value::DragValue, image::Image, slider::*, text_edit::*};

// ----------------------------------------------------------------------------

/// Anything implementing Widget can be added to a [`Ui`] with [`Ui::add`].
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub trait Widget {
    /// Allocate space, interact, paint, and return a [`Response`].
    fn ui(self, ui: &mut Ui) -> Response;
}

// ----------------------------------------------------------------------------

/// Show a button to reset a value to its default.
/// The button is only enabled if the value does not already have its original value.
pub fn reset_button<T: Default + PartialEq>(ui: &mut Ui, value: &mut T) {
    let def = T::default();
    if ui.add(Button::new("Reset").enabled(*value != def)).clicked {
        *value = def;
    }
}
