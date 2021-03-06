// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use StyleScheme;
use ffi;
use glib;
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
    pub struct StyleSchemeManager(Object<ffi::GtkSourceStyleSchemeManager, ffi::GtkSourceStyleSchemeManagerClass>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_manager_get_type(),
    }
}

impl StyleSchemeManager {
    pub fn new() -> StyleSchemeManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_style_scheme_manager_new())
        }
    }

    pub fn get_default() -> Option<StyleSchemeManager> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_default())
        }
    }
}

impl Default for StyleSchemeManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StyleSchemeManagerExt {
    fn append_search_path(&self, path: &str);

    fn force_rescan(&self);

    fn get_scheme(&self, scheme_id: &str) -> Option<StyleScheme>;

    fn get_scheme_ids(&self) -> Vec<String>;

    fn get_search_path(&self) -> Vec<String>;

    fn prepend_search_path(&self, path: &str);

    fn set_search_path(&self, path: &[&str]);

    fn connect_property_scheme_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleSchemeManager> + IsA<glib::object::Object>> StyleSchemeManagerExt for O {
    fn append_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_append_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn force_rescan(&self) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_force_rescan(self.to_glib_none().0);
        }
    }

    fn get_scheme(&self, scheme_id: &str) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme(self.to_glib_none().0, scheme_id.to_glib_none().0))
        }
    }

    fn get_scheme_ids(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme_ids(self.to_glib_none().0))
        }
    }

    fn get_search_path(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_manager_get_search_path(self.to_glib_none().0))
        }
    }

    fn prepend_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_prepend_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn set_search_path(&self, path: &[&str]) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_set_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn connect_property_scheme_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scheme-ids",
                transmute(notify_scheme_ids_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search-path",
                transmute(notify_search_path_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_scheme_ids_trampoline<P>(this: *mut ffi::GtkSourceStyleSchemeManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleSchemeManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleSchemeManager::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_search_path_trampoline<P>(this: *mut ffi::GtkSourceStyleSchemeManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleSchemeManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleSchemeManager::from_glib_borrow(this).downcast_unchecked())
}
