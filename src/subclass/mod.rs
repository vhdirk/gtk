// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]

pub mod application;
pub mod application_window;
pub mod bin;
pub mod box_;
pub mod container;
pub mod dialog;
pub mod event_box;
pub mod header_bar;
pub mod stack;
pub mod widget;
pub mod window;
pub mod cell_renderer;

pub mod prelude {
    pub use super::application::GtkApplicationImpl;
    pub use super::application_window::ApplicationWindowImpl;
    pub use super::bin::BinImpl;
    pub use super::box_::BoxImpl;
    pub use super::container::ContainerImpl;
    pub use super::dialog::DialogImpl;
    pub use super::event_box::EventBoxImpl;
    pub use super::header_bar::HeaderBarImpl;
    pub use super::stack::StackImpl;
    pub use super::widget::WidgetImpl;
    pub use super::window::WindowImpl;
    pub use super::cell_renderer::CellRendererImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
}

use self::prelude::*;