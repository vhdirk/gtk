use std::convert;
use std::ffi::OsString;
use std::fmt;
use std::ops;
use std::ptr;

use libc;

use gtk_sys;

use super::prelude::*;
use glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use CellRenderer;
use CellRendererClass;
use Widget;
use Rectangle;
use CellRendererState;

pub trait CellRendererImpl: ObjectImpl + 'static {

    fn render(
        &self,
        cell_renderer: &::CellRenderer,
        cr: &cairo::Context,
        widget: &::Widget,
        background_area: &::Rectangle,
        cell_area: &::Rectangle,
        flags: ::CellRendererState,
    ) {
        self.parent_render(cell_renderer, cr, widget, background_area, cell_area, flags)
    }

    fn parent_render(
        &self,
        cell_renderer: &::CellRenderer,
        cr: &cairo::Context,
        widget: &::Widget,
        background_area: &::Rectangle,
        cell_area: &::Rectangle,
        flags: ::CellRendererState,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            (*parent_class)
                .render
                .map(|f| {
                    f(
                        cell_renderer.to_glib_none().0,
                        cr.to_glib_none().0,
                        widget.to_glib_none().0,
                        background_area.to_glib_none().0,
                        cell_area.to_glib_none().0,
                        flags.to_glib(),
                    )
                })
                .unwrap_or(())
        }
    }



}

pub unsafe trait CellRendererClassSubclassExt: Sized + 'static {}

unsafe impl CellRendererClassSubclassExt for CellRendererClass {}

unsafe impl<T: ObjectSubclass + CellRendererImpl> IsSubclassable<T> for CellRendererClass
{
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *const Self as *mut gtk_sys::GtkCellRendererClass);
            klass.render = Some(cell_renderer_render::<T>);
        }
    }
}

unsafe extern "C" fn cell_renderer_render<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    cr: *mut cairo_sys::cairo_t,
    widget: *mut gtk_sys::GtkWidget,
    background_area: *const gdk_sys::GdkRectangle,
    cell_area: *const gdk_sys::GdkRectangle,
    flags: gtk_sys::GtkCellRendererState,
) where
    T: CellRendererImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: CellRenderer = from_glib_borrow(ptr);

    imp.render(
        &wrap,
        &from_glib_borrow(cr),
        &from_glib_borrow(widget),
        &from_glib_borrow(background_area),
        &from_glib_borrow(cell_area),
        from_glib(flags),
    )
}

