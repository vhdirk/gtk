// This file was generated by gir (9e3f4cc) from gir-files (71d73f0)
// DO NOT EDIT

use Error;
use Unit;
use ffi;
use glib;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct PaperSize(Boxed<ffi::GtkPaperSize>);

    match fn {
        copy => |ptr| ffi::gtk_paper_size_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_paper_size_free(ptr),
    }
}

impl PaperSize {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P) -> PaperSize {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new(name.0))
        }
    }

    pub fn new_custom(name: &str, display_name: &str, width: f64, height: f64, unit: Unit) -> PaperSize {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_custom(name.to_glib_none().0, display_name.to_glib_none().0, width, height, unit.to_glib()))
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn new_from_gvariant(variant: &glib::Variant) -> PaperSize {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_from_gvariant(variant.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn new_from_ipp(ipp_name: &str, width: f64, height: f64) -> PaperSize {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_from_ipp(ipp_name.to_glib_none().0, width, height))
        }
    }

    pub fn new_from_key_file(key_file: &glib::KeyFile, group_name: &str) -> Result<PaperSize, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_paper_size_new_from_key_file(key_file.to_glib_none().0, group_name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_ppd(ppd_name: &str, ppd_display_name: &str, width: f64, height: f64) -> PaperSize {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_from_ppd(ppd_name.to_glib_none().0, ppd_display_name.to_glib_none().0, width, height))
        }
    }

    pub fn get_default_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_bottom_margin(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn get_default_left_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_left_margin(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn get_default_right_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_right_margin(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn get_default_top_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_top_margin(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_display_name(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_height(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_name(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_ppd_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_ppd_name(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_width(mut_override(self.to_glib_none().0), unit.to_glib())
        }
    }

    pub fn is_custom(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_custom(mut_override(self.to_glib_none().0)))
        }
    }

    fn is_equal(&self, size2: &PaperSize) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_equal(mut_override(self.to_glib_none().0), mut_override(size2.to_glib_none().0)))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn is_ipp(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_ipp(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64, unit: Unit) {
        unsafe {
            ffi::gtk_paper_size_set_size(self.to_glib_none_mut().0, width, height, unit.to_glib());
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn to_gvariant(&mut self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_to_gvariant(self.to_glib_none_mut().0))
        }
    }

    pub fn to_key_file(&mut self, key_file: &glib::KeyFile, group_name: &str) {
        unsafe {
            ffi::gtk_paper_size_to_key_file(self.to_glib_none_mut().0, key_file.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    pub fn get_default() -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_default())
        }
    }

    pub fn get_paper_sizes(include_custom: bool) -> Vec<PaperSize> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_paper_size_get_paper_sizes(include_custom.to_glib()))
        }
    }
}

impl PartialEq for PaperSize {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl Eq for PaperSize {}
