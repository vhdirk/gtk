// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct EventBox(Object<ffi::GtkEventBox>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_event_box_get_type(),
    }
}

impl EventBox {
    pub fn new() -> EventBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_event_box_new()).downcast_unchecked()
        }
    }
}

impl Default for EventBox {
    fn default() -> Self {
        Self::new()
    }
}

pub trait EventBoxExt {
    fn get_above_child(&self) -> bool;

    fn get_visible_window(&self) -> bool;

    fn set_above_child(&self, above_child: bool);

    fn set_visible_window(&self, visible_window: bool);

    fn connect_property_above_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_visible_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<EventBox> + IsA<glib::object::Object>> EventBoxExt for O {
    fn get_above_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_above_child(self.to_glib_none().0))
        }
    }

    fn get_visible_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_visible_window(self.to_glib_none().0))
        }
    }

    fn set_above_child(&self, above_child: bool) {
        unsafe {
            ffi::gtk_event_box_set_above_child(self.to_glib_none().0, above_child.to_glib());
        }
    }

    fn set_visible_window(&self, visible_window: bool) {
        unsafe {
            ffi::gtk_event_box_set_visible_window(self.to_glib_none().0, visible_window.to_glib());
        }
    }

    fn connect_property_above_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::above-child",
                transmute(notify_above_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-window",
                transmute(notify_visible_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_above_child_trampoline<P>(this: *mut ffi::GtkEventBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EventBox::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_window_trampoline<P>(this: *mut ffi::GtkEventBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EventBox::from_glib_none(this).downcast_unchecked())
}
