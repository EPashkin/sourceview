// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
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
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Style(Object<ffi::GtkSourceStyle, ffi::GtkSourceStyleClass>);

    match fn {
        get_type => || ffi::gtk_source_style_get_type(),
    }
}

pub trait StyleExt {
    fn get_property_background(&self) -> Option<String>;

    fn get_property_background_set(&self) -> bool;

    fn get_property_bold(&self) -> bool;

    fn get_property_bold_set(&self) -> bool;

    fn get_property_foreground(&self) -> Option<String>;

    fn get_property_foreground_set(&self) -> bool;

    fn get_property_italic(&self) -> bool;

    fn get_property_italic_set(&self) -> bool;

    fn get_property_line_background(&self) -> Option<String>;

    fn get_property_line_background_set(&self) -> bool;

    fn get_property_pango_underline(&self) -> pango::Underline;

    fn get_property_scale(&self) -> Option<String>;

    fn get_property_scale_set(&self) -> bool;

    fn get_property_strikethrough(&self) -> bool;

    fn get_property_strikethrough_set(&self) -> bool;

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn get_property_underline(&self) -> bool;

    fn get_property_underline_color(&self) -> Option<String>;

    fn get_property_underline_color_set(&self) -> bool;

    fn get_property_underline_set(&self) -> bool;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bold_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bold_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_foreground_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_italic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_italic_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pango_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_strikethrough_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_strikethrough_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn connect_property_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_underline_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_underline_color_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_underline_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Style> + IsA<glib::object::Object>> StyleExt for O {
    fn get_property_background(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_bold(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_bold_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_foreground(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_foreground_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_italic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_italic_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_line_background(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_line_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_pango_underline(&self) -> pango::Underline {
        unsafe {
            let mut value = Value::from_type(<pango::Underline as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pango-underline".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_scale(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_scale_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_strikethrough(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_strikethrough_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline_color(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_underline_color_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background",
                transmute(notify_background_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-set",
                transmute(notify_background_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bold_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bold",
                transmute(notify_bold_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bold_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bold-set",
                transmute(notify_bold_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::foreground",
                transmute(notify_foreground_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_foreground_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::foreground-set",
                transmute(notify_foreground_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_italic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::italic",
                transmute(notify_italic_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_italic_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::italic-set",
                transmute(notify_italic_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_line_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::line-background",
                transmute(notify_line_background_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_line_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::line-background-set",
                transmute(notify_line_background_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pango_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pango-underline",
                transmute(notify_pango_underline_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scale",
                transmute(notify_scale_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scale_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scale-set",
                transmute(notify_scale_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_strikethrough_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::strikethrough",
                transmute(notify_strikethrough_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_strikethrough_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::strikethrough-set",
                transmute(notify_strikethrough_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::underline",
                transmute(notify_underline_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_underline_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::underline-color",
                transmute(notify_underline_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_underline_color_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::underline-color-set",
                transmute(notify_underline_color_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_underline_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::underline-set",
                transmute(notify_underline_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_background_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_background_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bold_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bold_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_foreground_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_foreground_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_italic_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_italic_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_line_background_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_line_background_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pango_underline_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scale_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scale_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_strikethrough_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_strikethrough_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_underline_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_underline_color_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_underline_color_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_underline_set_trampoline<P>(this: *mut ffi::GtkSourceStyle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Style> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Style::from_glib_borrow(this).downcast_unchecked())
}
