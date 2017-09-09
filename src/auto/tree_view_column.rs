// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use CellArea;
use CellLayout;
use CellRenderer;
use SortType;
use TreeIter;
use TreeModel;
use TreeViewColumnSizing;
use Widget;
use ffi;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TreeViewColumn(Object<ffi::GtkTreeViewColumn>): CellLayout;

    match fn {
        get_type => || ffi::gtk_tree_view_column_get_type(),
    }
}

impl TreeViewColumn {
    pub fn new() -> TreeViewColumn {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_new())
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> TreeViewColumn {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_new_with_area(area.to_glib_none().0))
        }
    }

    //pub fn new_with_attributes<P: IsA<CellRenderer>>(title: &str, cell: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeViewColumn {
    //    unsafe { TODO: call ffi::gtk_tree_view_column_new_with_attributes() }
    //}
}

impl Default for TreeViewColumn {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TreeViewColumnExt {
    fn cell_get_position<P: IsA<CellRenderer>>(&self, cell_renderer: &P) -> Option<(i32, i32)>;

    fn cell_get_size<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, cell_area: P) -> (i32, i32, i32, i32);

    fn cell_is_visible(&self) -> bool;

    fn cell_set_cell_data<P: IsA<TreeModel>>(&self, tree_model: &P, iter: &TreeIter, is_expander: bool, is_expanded: bool);

    fn clicked(&self);

    fn focus_cell<P: IsA<CellRenderer>>(&self, cell: &P);

    fn get_alignment(&self) -> f32;

    fn get_button(&self) -> Option<Widget>;

    fn get_clickable(&self) -> bool;

    fn get_expand(&self) -> bool;

    fn get_fixed_width(&self) -> i32;

    fn get_max_width(&self) -> i32;

    fn get_min_width(&self) -> i32;

    fn get_reorderable(&self) -> bool;

    fn get_resizable(&self) -> bool;

    fn get_sizing(&self) -> TreeViewColumnSizing;

    fn get_sort_column_id(&self) -> i32;

    fn get_sort_indicator(&self) -> bool;

    fn get_sort_order(&self) -> SortType;

    fn get_spacing(&self) -> i32;

    fn get_title(&self) -> Option<String>;

    fn get_tree_view(&self) -> Option<Widget>;

    fn get_visible(&self) -> bool;

    fn get_widget(&self) -> Option<Widget>;

    fn get_width(&self) -> i32;

    fn get_x_offset(&self) -> i32;

    fn queue_resize(&self);

    fn set_alignment(&self, xalign: f32);

    //fn set_cell_data_func<'a, P: IsA<CellRenderer>, Q: Into<Option<&'a /*Unimplemented*/TreeCellDataFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cell_renderer: &P, func: Q, func_data: R, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_clickable(&self, clickable: bool);

    fn set_expand(&self, expand: bool);

    fn set_fixed_width(&self, fixed_width: i32);

    fn set_max_width(&self, max_width: i32);

    fn set_min_width(&self, min_width: i32);

    fn set_reorderable(&self, reorderable: bool);

    fn set_resizable(&self, resizable: bool);

    fn set_sizing(&self, type_: TreeViewColumnSizing);

    fn set_sort_column_id(&self, sort_column_id: i32);

    fn set_sort_indicator(&self, setting: bool);

    fn set_sort_order(&self, order: SortType);

    fn set_spacing(&self, spacing: i32);

    fn set_title(&self, title: &str);

    fn set_visible(&self, visible: bool);

    fn set_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clickable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resizable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sizing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sort_column_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sort_indicator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeViewColumn> + IsA<glib::object::Object>> TreeViewColumnExt for O {
    fn cell_get_position<P: IsA<CellRenderer>>(&self, cell_renderer: &P) -> Option<(i32, i32)> {
        unsafe {
            let mut x_offset = mem::uninitialized();
            let mut width = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_column_cell_get_position(self.to_glib_none().0, cell_renderer.to_glib_none().0, &mut x_offset, &mut width));
            if ret { Some((x_offset, width)) } else { None }
        }
    }

    fn cell_get_size<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, cell_area: P) -> (i32, i32, i32, i32) {
        let cell_area = cell_area.into();
        let cell_area = cell_area.to_glib_none();
        unsafe {
            let mut x_offset = mem::uninitialized();
            let mut y_offset = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_tree_view_column_cell_get_size(self.to_glib_none().0, cell_area.0, &mut x_offset, &mut y_offset, &mut width, &mut height);
            (x_offset, y_offset, width, height)
        }
    }

    fn cell_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_cell_is_visible(self.to_glib_none().0))
        }
    }

    fn cell_set_cell_data<P: IsA<TreeModel>>(&self, tree_model: &P, iter: &TreeIter, is_expander: bool, is_expanded: bool) {
        unsafe {
            ffi::gtk_tree_view_column_cell_set_cell_data(self.to_glib_none().0, tree_model.to_glib_none().0, mut_override(iter.to_glib_none().0), is_expander.to_glib(), is_expanded.to_glib());
        }
    }

    fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.to_glib_none().0);
        }
    }

    fn focus_cell<P: IsA<CellRenderer>>(&self, cell: &P) {
        unsafe {
            ffi::gtk_tree_view_column_focus_cell(self.to_glib_none().0, cell.to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tree_view_column_get_alignment(self.to_glib_none().0)
        }
    }

    fn get_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_button(self.to_glib_none().0))
        }
    }

    fn get_clickable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_clickable(self.to_glib_none().0))
        }
    }

    fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_expand(self.to_glib_none().0))
        }
    }

    fn get_fixed_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_fixed_width(self.to_glib_none().0)
        }
    }

    fn get_max_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_max_width(self.to_glib_none().0)
        }
    }

    fn get_min_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_min_width(self.to_glib_none().0)
        }
    }

    fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_reorderable(self.to_glib_none().0))
        }
    }

    fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_resizable(self.to_glib_none().0))
        }
    }

    fn get_sizing(&self) -> TreeViewColumnSizing {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sizing(self.to_glib_none().0))
        }
    }

    fn get_sort_column_id(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_column_id(self.to_glib_none().0)
        }
    }

    fn get_sort_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_indicator(self.to_glib_none().0))
        }
    }

    fn get_sort_order(&self) -> SortType {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_order(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_spacing(self.to_glib_none().0)
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_title(self.to_glib_none().0))
        }
    }

    fn get_tree_view(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_tree_view(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_visible(self.to_glib_none().0))
        }
    }

    fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_widget(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_width(self.to_glib_none().0)
        }
    }

    fn get_x_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_x_offset(self.to_glib_none().0)
        }
    }

    fn queue_resize(&self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.to_glib_none().0);
        }
    }

    fn set_alignment(&self, xalign: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.to_glib_none().0, xalign);
        }
    }

    //fn set_cell_data_func<'a, P: IsA<CellRenderer>, Q: Into<Option<&'a /*Unimplemented*/TreeCellDataFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cell_renderer: &P, func: Q, func_data: R, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_view_column_set_cell_data_func() }
    //}

    fn set_clickable(&self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.to_glib_none().0, clickable.to_glib());
        }
    }

    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_fixed_width(self.to_glib_none().0, fixed_width);
        }
    }

    fn set_max_width(&self, max_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_max_width(self.to_glib_none().0, max_width);
        }
    }

    fn set_min_width(&self, min_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_min_width(self.to_glib_none().0, min_width);
        }
    }

    fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }

    fn set_sizing(&self, type_: TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_sort_column_id(&self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.to_glib_none().0, sort_column_id);
        }
    }

    fn set_sort_indicator(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_sort_order(&self, order: SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.to_glib_none().0, order.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_tree_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    fn set_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_tree_view_column_set_widget(self.to_glib_none().0, widget.0);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        let mut value = Value::from(None::<&CellArea>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "clicked",
                transmute(clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_alignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alignment",
                transmute(notify_alignment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area",
                transmute(notify_cell_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_clickable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clickable",
                transmute(notify_clickable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::expand",
                transmute(notify_expand_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::fixed-width",
                transmute(notify_fixed_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-width",
                transmute(notify_max_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-width",
                transmute(notify_min_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::reorderable",
                transmute(notify_reorderable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_resizable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::resizable",
                transmute(notify_resizable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sizing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sizing",
                transmute(notify_sizing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sort_column_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sort-column-id",
                transmute(notify_sort_column_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sort_indicator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sort-indicator",
                transmute(notify_sort_indicator_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sort-order",
                transmute(notify_sort_order_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::spacing",
                transmute(notify_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible",
                transmute(notify_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::widget",
                transmute(notify_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_x_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::x-offset",
                transmute(notify_x_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn clicked_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alignment_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_clickable_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_expand_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_fixed_width_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_max_width_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_min_width_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_reorderable_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_resizable_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sizing_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sort_column_id_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sort_indicator_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sort_order_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_spacing_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_widget_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_x_offset_trampoline<P>(this: *mut ffi::GtkTreeViewColumn, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeViewColumn> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeViewColumn::from_glib_borrow(this).downcast_unchecked())
}
