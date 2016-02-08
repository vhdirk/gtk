// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(gtk_3_10)]
use Entry;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct SearchBar(Object<ffi::GtkSearchBar>): Widget, Container, Bin, Buildable;

    match fn {
        get_type => || ffi::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    #[cfg(gtk_3_10)]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn connect_entry<T: IsA<Entry>>(&self, entry: &T) {
        unsafe {
            ffi::gtk_search_bar_connect_entry(self.to_glib_none().0, entry.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_search_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_search_mode(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn handle_event(&self, event: /*Unknown conversion*//*Unimplemented*/Event) -> bool {
    //    unsafe { TODO: call ffi::gtk_search_bar_handle_event() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            ffi::gtk_search_bar_set_search_mode(self.to_glib_none().0, search_mode.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_show_close_button(&self, visible: bool) {
        unsafe {
            ffi::gtk_search_bar_set_show_close_button(self.to_glib_none().0, visible.to_glib());
        }
    }
}