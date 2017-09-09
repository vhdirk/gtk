// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use CellEditable;
use Editable;
use Entry;
use Widget;
use ffi;
#[cfg(feature = "v3_16")]
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_10")]
use glib::signal::SignalHandlerId;
#[cfg(feature = "v3_10")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_10")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_10")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>): Entry, Widget, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(feature = "v3_6")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }
}

#[cfg(feature = "v3_6")]
impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SearchEntryExt {
    #[cfg(feature = "v3_16")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(feature = "v3_16")]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_16")]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_10")]
    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_16")]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchEntry> + IsA<glib::object::Object>> SearchEntryExt for O {
    #[cfg(feature = "v3_16")]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_entry_handle_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-match",
                transmute(next_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "previous-match",
                transmute(previous_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-changed",
                transmute(search_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stop-search",
                transmute(stop_search_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn next_match_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn previous_match_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn search_changed_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn stop_search_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}
