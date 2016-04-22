// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use Container;
use ReliefStyle;
use ToolItem;
use ToolShell;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ToolItemGroup(Object<ffi::GtkToolItemGroup>): Container, Widget, ToolShell;

    match fn {
        get_type => || ffi::gtk_tool_item_group_get_type(),
    }
}

impl ToolItemGroup {
    pub fn new(label: &str) -> ToolItemGroup {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_item_group_new(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_collapsed(self.to_glib_none().0))
        }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_drop_item(self.to_glib_none().0, x, y))
        }
    }

    //pub fn get_ellipsize(&self) -> /*Ignored*/pango::EllipsizeMode {
    //    unsafe { TODO: call ffi::gtk_tool_item_group_get_ellipsize() }
    //}

    pub fn get_header_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_header_relief(self.to_glib_none().0))
        }
    }

    pub fn get_item_position<T: IsA<ToolItem>>(&self, item: &T) -> i32 {
        unsafe {
            ffi::gtk_tool_item_group_get_item_position(self.to_glib_none().0, item.to_glib_none().0)
        }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label(self.to_glib_none().0))
        }
    }

    pub fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label_widget(self.to_glib_none().0))
        }
    }

    pub fn get_n_items(&self) -> u32 {
        unsafe {
            ffi::gtk_tool_item_group_get_n_items(self.to_glib_none().0)
        }
    }

    pub fn get_nth_item(&self, index: u32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_nth_item(self.to_glib_none().0, index))
        }
    }

    pub fn insert<T: IsA<ToolItem>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_insert(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    pub fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            ffi::gtk_tool_item_group_set_collapsed(self.to_glib_none().0, collapsed.to_glib());
        }
    }

    //pub fn set_ellipsize(&self, ellipsize: /*Ignored*/pango::EllipsizeMode) {
    //    unsafe { TODO: call ffi::gtk_tool_item_group_set_ellipsize() }
    //}

    pub fn set_header_relief(&self, style: ReliefStyle) {
        unsafe {
            ffi::gtk_tool_item_group_set_header_relief(self.to_glib_none().0, style.to_glib());
        }
    }

    pub fn set_item_position<T: IsA<ToolItem>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_set_item_position(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_tool_item_group_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    pub fn set_label_widget<T: IsA<Widget>>(&self, label_widget: &T) {
        unsafe {
            ffi::gtk_tool_item_group_set_label_widget(self.to_glib_none().0, label_widget.to_glib_none().0);
        }
    }
}
