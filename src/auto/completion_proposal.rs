// This file was generated by gir (d8a605d) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use gdk_pixbuf;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use gio;
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
    pub struct CompletionProposal(Object<ffi::GtkSourceCompletionProposal, ffi::GtkSourceCompletionProposalIface>);

    match fn {
        get_type => || ffi::gtk_source_completion_proposal_get_type(),
    }
}

pub trait CompletionProposalExt {
    fn changed(&self);

    fn equal<P: IsA<CompletionProposal>>(&self, other: &P) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_icon_name(&self) -> Option<String>;

    fn get_info(&self) -> Option<String>;

    fn get_label(&self) -> Option<String>;

    fn get_markup(&self) -> Option<String>;

    fn get_text(&self) -> Option<String>;

    fn hash(&self) -> u32;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_changed(&self);
}

impl<O: IsA<CompletionProposal> + IsA<glib::object::Object> + glib::object::ObjectExt> CompletionProposalExt for O {
    fn changed(&self) {
        unsafe {
            ffi::gtk_source_completion_proposal_changed(self.to_glib_none().0);
        }
    }

    fn equal<P: IsA<CompletionProposal>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_proposal_equal(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_proposal_get_gicon(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_proposal_get_icon(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_proposal_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_info(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_proposal_get_info(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_proposal_get_label(self.to_glib_none().0))
        }
    }

    fn get_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_proposal_get_markup(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_proposal_get_text(self.to_glib_none().0))
        }
    }

    fn hash(&self) -> u32 {
        unsafe {
            ffi::gtk_source_completion_proposal_hash(self.to_glib_none().0)
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_changed(&self) {
        let _ = self.emit("changed", &[]).unwrap();
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkSourceCompletionProposal, f: glib_ffi::gpointer)
where P: IsA<CompletionProposal> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionProposal::from_glib_borrow(this).downcast_unchecked())
}
