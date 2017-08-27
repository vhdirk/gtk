// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Container;
use IconSize;
use Orientable;
use Orientation;
use ToolItem;
use ToolShell;
use ToolbarStyle;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Toolbar(Object<ffi::GtkToolbar>): Container, Widget, Orientable, ToolShell;

    match fn {
        get_type => || ffi::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toolbar_new()).downcast_unchecked()
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToolbarExt {
    fn get_drop_index(&self, x: i32, y: i32) -> i32;

    fn get_icon_size(&self) -> IconSize;

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_n_items(&self) -> i32;

    fn get_nth_item(&self, n: i32) -> Option<ToolItem>;

    fn get_show_arrow(&self) -> bool;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32);

    fn set_drop_highlight_item<'a, P: IsA<ToolItem> + 'a, Q: Into<Option<&'a P>>>(&self, tool_item: Q, index_: i32);

    fn set_icon_size(&self, icon_size: IconSize);

    fn set_show_arrow(&self, show_arrow: bool);

    fn set_style(&self, style: ToolbarStyle);

    fn unset_icon_size(&self);

    fn unset_style(&self);

    fn get_property_icon_size_set(&self) -> bool;

    fn set_property_icon_size_set(&self, icon_size_set: bool);

    fn get_property_toolbar_style(&self) -> ToolbarStyle;

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle);

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool);

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool);

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(&self, f: F) -> u64;

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(&self, f: F) -> u64;

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Toolbar> + IsA<Container> + IsA<glib::object::Object>> ToolbarExt for O {
    fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(self.to_glib_none().0, x, y)
        }
    }

    fn get_icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_icon_size(self.to_glib_none().0))
        }
    }

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(self.to_glib_none().0, item.to_glib_none().0)
        }
    }

    fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(self.to_glib_none().0)
        }
    }

    fn get_nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_toolbar_get_nth_item(self.to_glib_none().0, n))
        }
    }

    fn get_show_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_show_arrow(self.to_glib_none().0))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32) {
        unsafe {
            ffi::gtk_toolbar_insert(self.to_glib_none().0, item.to_glib_none().0, pos);
        }
    }

    fn set_drop_highlight_item<'a, P: IsA<ToolItem> + 'a, Q: Into<Option<&'a P>>>(&self, tool_item: Q, index_: i32) {
        let tool_item = tool_item.into();
        let tool_item = tool_item.to_glib_none();
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(self.to_glib_none().0, tool_item.0, index_);
        }
    }

    fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(self.to_glib_none().0, icon_size.to_glib());
        }
    }

    fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_toolbar_set_show_arrow(self.to_glib_none().0, show_arrow.to_glib());
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_toolbar_set_style(self.to_glib_none().0, style.to_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(self.to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_style(self.to_glib_none().0);
        }
    }

    fn get_property_icon_size_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, Value::from(&icon_size_set).to_glib_none().0);
        }
    }

    fn get_property_toolbar_style(&self) -> ToolbarStyle {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        let toolbar_style = toolbar_style.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, Value::from(&toolbar_style).to_glib_none().0);
        }
    }

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, Value::from(&expand).to_glib_none().0);
        }
    }

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "focus-home-or-end",
                transmute(focus_home_or_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, Orientation) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "orientation-changed",
                transmute(orientation_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, i32) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup-context-menu",
                transmute(popup_context_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ToolbarStyle) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "style-changed",
                transmute(style_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size",
                transmute(notify_icon_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size-set",
                transmute(notify_icon_size_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-arrow",
                transmute(notify_show_arrow_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::toolbar-style",
                transmute(notify_toolbar_style_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn focus_home_or_end_trampoline<P>(this: *mut ffi::GtkToolbar, focus_home: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P, bool) -> bool + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked(), from_glib(focus_home)).to_glib()
}

unsafe extern "C" fn orientation_changed_trampoline<P>(this: *mut ffi::GtkToolbar, orientation: ffi::GtkOrientation, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P, Orientation) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked(), from_glib(orientation))
}

unsafe extern "C" fn popup_context_menu_trampoline<P>(this: *mut ffi::GtkToolbar, x: libc::c_int, y: libc::c_int, button: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P, i32, i32, i32) -> Inhibit + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked(), x, y, button).to_glib()
}

unsafe extern "C" fn style_changed_trampoline<P>(this: *mut ffi::GtkToolbar, style: ffi::GtkToolbarStyle, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P, ToolbarStyle) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked(), from_glib(style))
}

unsafe extern "C" fn notify_icon_size_trampoline<P>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_size_set_trampoline<P>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_arrow_trampoline<P>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_toolbar_style_trampoline<P>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Toolbar::from_glib_none(this).downcast_unchecked())
}
