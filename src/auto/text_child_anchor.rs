// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TextChildAnchor(Object<ffi::GtkTextChildAnchor>);

    match fn {
        get_type => || ffi::gtk_text_child_anchor_get_type(),
    }
}

impl TextChildAnchor {
    pub fn new() -> TextChildAnchor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_child_anchor_new())
        }
    }
}

impl Default for TextChildAnchor {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TextChildAnchorExt {
    fn get_deleted(&self) -> bool;

    fn get_widgets(&self) -> Vec<Widget>;
}

impl<O: IsA<TextChildAnchor>> TextChildAnchorExt for O {
    fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_child_anchor_get_deleted(self.to_glib_none().0))
        }
    }

    fn get_widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_child_anchor_get_widgets(self.to_glib_none().0))
        }
    }
}
