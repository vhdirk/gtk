// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use PlacesOpenFlags;
use ScrolledWindow;
use Widget;
use ffi;
#[cfg(feature = "v3_18")]
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_10")]
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PlacesSidebar(Object<ffi::GtkPlacesSidebar>): ScrolledWindow, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_places_sidebar_get_type(),
    }
}

impl PlacesSidebar {
    #[cfg(feature = "v3_10")]
    pub fn new() -> PlacesSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_places_sidebar_new()).downcast_unchecked()
        }
    }
}

#[cfg(feature = "v3_10")]
impl Default for PlacesSidebar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PlacesSidebarExt {
    //#[cfg(feature = "v3_10")]
    //fn add_shortcut<P: IsA</*Ignored*/gio::File>>(&self, location: &P);

    #[cfg(feature = "v3_12")]
    fn get_local_only(&self) -> bool;

    //#[cfg(feature = "v3_10")]
    //fn get_location(&self) -> /*Ignored*/Option<gio::File>;

    //#[cfg(feature = "v3_10")]
    //fn get_nth_bookmark(&self, n: i32) -> /*Ignored*/Option<gio::File>;

    #[cfg(feature = "v3_10")]
    fn get_open_flags(&self) -> PlacesOpenFlags;

    fn get_show_connect_to_server(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_show_desktop(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn get_show_enter_location(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn get_show_other_locations(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn get_show_recent(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn get_show_trash(&self) -> bool;

    //#[cfg(feature = "v3_10")]
    //fn list_shortcuts(&self) -> /*Ignored*/Vec<gio::File>;

    //#[cfg(feature = "v3_10")]
    //fn remove_shortcut<P: IsA</*Ignored*/gio::File>>(&self, location: &P);

    #[cfg(feature = "v3_18")]
    fn set_drop_targets_visible(&self, visible: bool, context: &gdk::DragContext);

    #[cfg(feature = "v3_12")]
    fn set_local_only(&self, local_only: bool);

    //#[cfg(feature = "v3_10")]
    //fn set_location<'a, P: IsA</*Ignored*/gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q);

    #[cfg(feature = "v3_10")]
    fn set_open_flags(&self, flags: PlacesOpenFlags);

    #[cfg(feature = "v3_10")]
    fn set_show_connect_to_server(&self, show_connect_to_server: bool);

    #[cfg(feature = "v3_10")]
    fn set_show_desktop(&self, show_desktop: bool);

    #[cfg(feature = "v3_14")]
    fn set_show_enter_location(&self, show_enter_location: bool);

    #[cfg(feature = "v3_18")]
    fn set_show_other_locations(&self, show_other_locations: bool);

    #[cfg(feature = "v3_18")]
    fn set_show_recent(&self, show_recent: bool);

    #[cfg(feature = "v3_18")]
    fn set_show_trash(&self, show_trash: bool);

    fn get_property_local_only(&self) -> bool;

    fn set_property_local_only(&self, local_only: bool);

    //fn get_property_location(&self) -> /*Ignored*/Option<gio::File>;

    //fn set_property_location<P: IsA</*Ignored*/gio::File> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, location: Option<&P>);

    fn get_property_open_flags(&self) -> PlacesOpenFlags;

    fn set_property_open_flags(&self, open_flags: PlacesOpenFlags);

    #[cfg(feature = "v3_18")]
    fn get_property_populate_all(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn set_property_populate_all(&self, populate_all: bool);

    fn set_property_show_connect_to_server(&self, show_connect_to_server: bool);

    fn get_property_show_desktop(&self) -> bool;

    fn set_property_show_desktop(&self, show_desktop: bool);

    fn get_property_show_enter_location(&self) -> bool;

    fn set_property_show_enter_location(&self, show_enter_location: bool);

    fn get_property_show_other_locations(&self) -> bool;

    fn set_property_show_other_locations(&self, show_other_locations: bool);

    fn get_property_show_recent(&self) -> bool;

    fn set_property_show_recent(&self, show_recent: bool);

    fn get_property_show_trash(&self) -> bool;

    fn set_property_show_trash(&self, show_trash: bool);

    #[cfg(feature = "v3_10")]
    fn connect_drag_action_ask<F: Fn(&Self, i32) -> i32 + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_10")]
    //fn connect_drag_action_requested<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_10")]
    //fn connect_drag_perform_drop<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_20")]
    //fn connect_mount<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_10")]
    //fn connect_open_location<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_10")]
    //fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_show_connect_to_server<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_14")]
    fn connect_show_enter_location<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_10")]
    fn connect_show_error_message<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_18")]
    fn connect_show_other_locations<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_20")]
    fn connect_show_other_locations_with_flags<F: Fn(&Self, PlacesOpenFlags) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(feature = "v3_20")]
    //fn connect_unmount<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_open_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_18")]
    fn connect_property_populate_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_connect_to_server_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_desktop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_enter_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_other_locations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_recent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_trash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PlacesSidebar> + IsA<glib::object::Object>> PlacesSidebarExt for O {
    //#[cfg(feature = "v3_10")]
    //fn add_shortcut<P: IsA</*Ignored*/gio::File>>(&self, location: &P) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_add_shortcut() }
    //}

    #[cfg(feature = "v3_12")]
    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_local_only(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn get_location(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_location() }
    //}

    //#[cfg(feature = "v3_10")]
    //fn get_nth_bookmark(&self, n: i32) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_nth_bookmark() }
    //}

    #[cfg(feature = "v3_10")]
    fn get_open_flags(&self) -> PlacesOpenFlags {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_open_flags(self.to_glib_none().0))
        }
    }

    fn get_show_connect_to_server(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_connect_to_server(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_show_desktop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_desktop(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_show_enter_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_enter_location(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_show_other_locations(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_other_locations(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_show_recent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_recent(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_show_trash(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_trash(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn list_shortcuts(&self) -> /*Ignored*/Vec<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_list_shortcuts() }
    //}

    //#[cfg(feature = "v3_10")]
    //fn remove_shortcut<P: IsA</*Ignored*/gio::File>>(&self, location: &P) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_remove_shortcut() }
    //}

    #[cfg(feature = "v3_18")]
    fn set_drop_targets_visible(&self, visible: bool, context: &gdk::DragContext) {
        unsafe {
            ffi::gtk_places_sidebar_set_drop_targets_visible(self.to_glib_none().0, visible.to_glib(), context.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn set_location<'a, P: IsA</*Ignored*/gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_set_location() }
    //}

    #[cfg(feature = "v3_10")]
    fn set_open_flags(&self, flags: PlacesOpenFlags) {
        unsafe {
            ffi::gtk_places_sidebar_set_open_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_connect_to_server(self.to_glib_none().0, show_connect_to_server.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_show_desktop(&self, show_desktop: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_desktop(self.to_glib_none().0, show_desktop.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_enter_location(self.to_glib_none().0, show_enter_location.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_show_other_locations(&self, show_other_locations: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_other_locations(self.to_glib_none().0, show_other_locations.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_show_recent(&self, show_recent: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_recent(self.to_glib_none().0, show_recent.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_show_trash(&self, show_trash: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_trash(self.to_glib_none().0, show_trash.to_glib());
        }
    }

    fn get_property_local_only(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "local-only".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_local_only(&self, local_only: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "local-only".to_glib_none().0, Value::from(&local_only).to_glib_none().0);
        }
    }

    //fn get_property_location(&self) -> /*Ignored*/Option<gio::File> {
    //    let mut value = Value::from(None::<&/*Ignored*/gio::File>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "location".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}

    //fn set_property_location<P: IsA</*Ignored*/gio::File> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, location: Option<&P>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "location".to_glib_none().0, Value::from(location).to_glib_none().0);
    //    }
    //}

    fn get_property_open_flags(&self) -> PlacesOpenFlags {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "open-flags".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    fn set_property_open_flags(&self, open_flags: PlacesOpenFlags) {
        let open_flags = open_flags.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "open-flags".to_glib_none().0, Value::from(&open_flags).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_property_populate_all(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "populate-all".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_18")]
    fn set_property_populate_all(&self, populate_all: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "populate-all".to_glib_none().0, Value::from(&populate_all).to_glib_none().0);
        }
    }

    fn set_property_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-connect-to-server".to_glib_none().0, Value::from(&show_connect_to_server).to_glib_none().0);
        }
    }

    fn get_property_show_desktop(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-desktop".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_desktop(&self, show_desktop: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-desktop".to_glib_none().0, Value::from(&show_desktop).to_glib_none().0);
        }
    }

    fn get_property_show_enter_location(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-enter-location".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-enter-location".to_glib_none().0, Value::from(&show_enter_location).to_glib_none().0);
        }
    }

    fn get_property_show_other_locations(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-other-locations".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_other_locations(&self, show_other_locations: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-other-locations".to_glib_none().0, Value::from(&show_other_locations).to_glib_none().0);
        }
    }

    fn get_property_show_recent(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-recent".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_recent(&self, show_recent: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-recent".to_glib_none().0, Value::from(&show_recent).to_glib_none().0);
        }
    }

    fn get_property_show_trash(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-trash".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_trash(&self, show_trash: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-trash".to_glib_none().0, Value::from(&show_trash).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn connect_drag_action_ask<F: Fn(&Self, i32) -> i32 + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) -> i32 + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drag-action-ask",
                transmute(drag_action_ask_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn connect_drag_action_requested<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored dest_file: Gio.File
    //    Ignored source_file_list: *.List TypeId { ns_id: 6, id: 15 }
    //}

    //#[cfg(feature = "v3_10")]
    //fn connect_drag_perform_drop<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored dest_file: Gio.File
    //    Ignored source_file_list: *.List TypeId { ns_id: 6, id: 15 }
    //}

    //#[cfg(feature = "v3_20")]
    //fn connect_mount<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored mount_operation: Gio.MountOperation
    //}

    //#[cfg(feature = "v3_10")]
    //fn connect_open_location<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored location: Gio.File
    //}

    //#[cfg(feature = "v3_10")]
    //fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored selected_item: Gio.File
    //    Ignored selected_volume: Gio.Volume
    //}

    fn connect_show_connect_to_server<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-connect-to-server",
                transmute(show_connect_to_server_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_show_enter_location<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-enter-location",
                transmute(show_enter_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    fn connect_show_error_message<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-error-message",
                transmute(show_error_message_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_18")]
    fn connect_show_other_locations<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-other-locations",
                transmute(show_other_locations_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    fn connect_show_other_locations_with_flags<F: Fn(&Self, PlacesOpenFlags) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PlacesOpenFlags) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-other-locations-with-flags",
                transmute(show_other_locations_with_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(feature = "v3_20")]
    //fn connect_unmount<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored mount_operation: Gio.MountOperation
    //}

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-only",
                transmute(notify_local_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::location",
                transmute(notify_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_open_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::open-flags",
                transmute(notify_open_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_18")]
    fn connect_property_populate_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::populate-all",
                transmute(notify_populate_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_connect_to_server_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-connect-to-server",
                transmute(notify_show_connect_to_server_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_desktop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-desktop",
                transmute(notify_show_desktop_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_enter_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-enter-location",
                transmute(notify_show_enter_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_other_locations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-other-locations",
                transmute(notify_show_other_locations_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_recent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-recent",
                transmute(notify_show_recent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_trash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-trash",
                transmute(notify_show_trash_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn drag_action_ask_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, actions: libc::c_int, f: glib_ffi::gpointer) -> libc::c_int
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P, i32) -> i32 + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked(), actions)
}

unsafe extern "C" fn show_connect_to_server_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn show_enter_location_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn show_error_message_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, primary: *mut libc::c_char, secondary: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P, &str, &str) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(primary), &String::from_glib_none(secondary))
}

#[cfg(feature = "v3_18")]
unsafe extern "C" fn show_other_locations_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn show_other_locations_with_flags_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, open_flags: ffi::GtkPlacesOpenFlags, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P, PlacesOpenFlags) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked(), from_glib(open_flags))
}

unsafe extern "C" fn notify_local_only_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_open_flags_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_18")]
unsafe extern "C" fn notify_populate_all_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_connect_to_server_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_desktop_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_enter_location_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_other_locations_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_recent_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_trash_trampoline<P>(this: *mut ffi::GtkPlacesSidebar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PlacesSidebar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PlacesSidebar::from_glib_borrow(this).downcast_unchecked())
}
