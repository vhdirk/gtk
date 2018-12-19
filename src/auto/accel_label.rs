// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Label;
use Misc;
use Widget;
use ffi;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use gdk;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AccelLabel(Object<ffi::GtkAccelLabel, ffi::GtkAccelLabelClass>): Label, Misc, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_accel_label_get_type(),
    }
}

impl AccelLabel {
    pub fn new(string: &str) -> AccelLabel {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_accel_label_new(string.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait AccelLabelExt {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_accel(&self) -> (u32, gdk::ModifierType);

    fn get_accel_widget(&self) -> Option<Widget>;

    fn get_accel_width(&self) -> u32;

    fn refetch(&self) -> bool;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType);

    fn set_accel_closure<'a, P: Into<Option<&'a glib::Closure>>>(&self, accel_closure: P);

    fn set_accel_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, accel_widget: Q);

    fn get_property_accel_closure(&self) -> Option<glib::Closure>;

    fn connect_property_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AccelLabel> + IsA<glib::object::Object>> AccelLabelExt for O {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_accel(&self) -> (u32, gdk::ModifierType) {
        unsafe {
            let mut accelerator_key = mem::uninitialized();
            let mut accelerator_mods = mem::uninitialized();
            ffi::gtk_accel_label_get_accel(self.to_glib_none().0, &mut accelerator_key, &mut accelerator_mods);
            (accelerator_key, from_glib(accelerator_mods))
        }
    }

    fn get_accel_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_accel_label_get_accel_widget(self.to_glib_none().0))
        }
    }

    fn get_accel_width(&self) -> u32 {
        unsafe {
            ffi::gtk_accel_label_get_accel_width(self.to_glib_none().0)
        }
    }

    fn refetch(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_label_refetch(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType) {
        unsafe {
            ffi::gtk_accel_label_set_accel(self.to_glib_none().0, accelerator_key, accelerator_mods.to_glib());
        }
    }

    fn set_accel_closure<'a, P: Into<Option<&'a glib::Closure>>>(&self, accel_closure: P) {
        let accel_closure = accel_closure.into();
        let accel_closure = accel_closure.to_glib_none();
        unsafe {
            ffi::gtk_accel_label_set_accel_closure(self.to_glib_none().0, accel_closure.0);
        }
    }

    fn set_accel_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, accel_widget: Q) {
        let accel_widget = accel_widget.into();
        let accel_widget = accel_widget.to_glib_none();
        unsafe {
            ffi::gtk_accel_label_set_accel_widget(self.to_glib_none().0, accel_widget.0);
        }
    }

    fn get_property_accel_closure(&self) -> Option<glib::Closure> {
        unsafe {
            let mut value = Value::from_type(<glib::Closure as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-closure".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-closure",
                transmute(notify_accel_closure_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-widget",
                transmute(notify_accel_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accel_closure_trampoline<P>(this: *mut ffi::GtkAccelLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccelLabel> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccelLabel::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accel_widget_trampoline<P>(this: *mut ffi::GtkAccelLabel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccelLabel> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccelLabel::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for AccelLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelLabel")
    }
}