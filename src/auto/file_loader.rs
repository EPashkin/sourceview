// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use Buffer;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use CompressionType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use File;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use NewlineType;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gio;
use glib;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileLoader(Object<ffi::GtkSourceFileLoader, ffi::GtkSourceFileLoaderClass>);

    match fn {
        get_type => || ffi::gtk_source_file_loader_get_type(),
    }
}

impl FileLoader {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new(buffer: &Buffer, file: &File) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_loader_new(buffer.to_glib_none().0, file.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new_from_stream<P: IsA<gio::InputStream>>(buffer: &Buffer, file: &File, stream: &P) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_loader_new_from_stream(buffer.to_glib_none().0, file.to_glib_none().0, stream.to_glib_none().0))
        }
    }
}

pub trait FileLoaderExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_input_stream(&self) -> Option<gio::InputStream>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async_future<'b, 'c, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileLoader> + IsA<glib::object::Object>> FileLoaderExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_compression_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_encoding(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_file(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_input_stream(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_location(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_newline_type(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T) {
    //    unsafe { TODO: call ffi::gtk_source_file_loader_load_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async_future<'b, 'c, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use send_cell::SendCell;

        //let progress_callback = progress_callback.into();
        //let progress_callback = progress_callback.map(ToOwned::to_owned);
        //let progress_callback_data = progress_callback_data.into();
        //let progress_callback_data = progress_callback_data.map(ToOwned::to_owned);
        //let progress_callback_notify = progress_callback_notify.into();
        //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = SendCell::new(send);
        //    let obj_clone = SendCell::new(obj.clone());
        //    obj.load_async(
        //         io_priority,
        //         Some(&cancellable),
        //         progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
        //         progress_callback_data.as_ref().map(::std::borrow::Borrow::borrow),
        //         progress_callback_notify.as_ref().map(::std::borrow::Borrow::borrow),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]) {
        unsafe {
            ffi::gtk_source_file_loader_set_candidate_encodings(self.to_glib_none().0, candidate_encodings.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-stream",
                transmute(notify_input_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::location",
                transmute(notify_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_input_stream_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}
