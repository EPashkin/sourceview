// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_16", feature = "dox"))]
use BackgroundPatternType;
use Buffer;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use ChangeCaseType;
use Completion;
use DrawSpacesFlags;
use Gutter;
use MarkAttributes;
use SmartHomeEndType;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use SpaceDrawer;
use ffi;
use gdk;
use gdk_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct View(Object<ffi::GtkSourceView, ffi::GtkSourceViewClass>): [
        gtk::TextView => gtk_ffi::GtkTextView,
        gtk::Widget => gtk_ffi::GtkWidget,
        gtk::Scrollable => gtk_ffi::GtkScrollable,
    ];

    match fn {
        get_type => || ffi::gtk_source_view_get_type(),
    }
}

impl View {
    pub fn new() -> View {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_buffer(buffer: &Buffer) -> View {
        skip_assert_initialized!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_view_new_with_buffer(buffer.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ViewExt {
    fn get_auto_indent(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_background_pattern(&self) -> BackgroundPatternType;

    fn get_completion(&self) -> Option<Completion>;

    #[cfg_attr(feature = "v3_24", deprecated)]
    fn get_draw_spaces(&self) -> DrawSpacesFlags;

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter>;

    fn get_highlight_current_line(&self) -> bool;

    fn get_indent_on_tab(&self) -> bool;

    fn get_indent_width(&self) -> i32;

    fn get_insert_spaces_instead_of_tabs(&self) -> bool;

    fn get_right_margin_position(&self) -> u32;

    fn get_show_line_marks(&self) -> bool;

    fn get_show_line_numbers(&self) -> bool;

    fn get_show_right_margin(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_smart_backspace(&self) -> bool;

    fn get_smart_home_end(&self) -> SmartHomeEndType;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_space_drawer(&self) -> Option<SpaceDrawer>;

    fn get_tab_width(&self) -> u32;

    fn get_visual_column(&self, iter: &gtk::TextIter) -> u32;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn indent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    fn set_auto_indent(&self, enable: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_background_pattern(&self, background_pattern: BackgroundPatternType);

    #[cfg_attr(feature = "v3_24", deprecated)]
    fn set_draw_spaces(&self, flags: DrawSpacesFlags);

    fn set_highlight_current_line(&self, highlight: bool);

    fn set_indent_on_tab(&self, enable: bool);

    fn set_indent_width(&self, width: i32);

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool);

    fn set_mark_attributes(&self, category: &str, attributes: &MarkAttributes, priority: i32);

    fn set_right_margin_position(&self, pos: u32);

    fn set_show_line_marks(&self, show: bool);

    fn set_show_line_numbers(&self, show: bool);

    fn set_show_right_margin(&self, show: bool);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_smart_backspace(&self, smart_backspace: bool);

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType);

    fn set_tab_width(&self, width: u32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn unindent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_change_case(&self, case_type: ChangeCaseType);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_change_number(&self, count: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_join_lines(&self);

    fn connect_line_mark_activated<F: Fn(&Self, &gtk::TextIter, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_lines<F: Fn(&Self, bool, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_lines(&self, copy: bool, count: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_move_to_matching_bracket(&self, extend_selection: bool);

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_words(&self, count: i32);

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_redo(&self);

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show_completion(&self);

    fn connect_smart_home_end<F: Fn(&Self, &gtk::TextIter, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_undo(&self);

    fn connect_property_auto_indent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_background_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_24", deprecated)]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_highlight_current_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indent_on_tab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indent_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_insert_spaces_instead_of_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_right_margin_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_line_marks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_right_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_smart_backspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_smart_home_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_space_drawer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<View> + IsA<glib::object::Object> + glib::object::ObjectExt> ViewExt for O {
    fn get_auto_indent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_auto_indent(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_background_pattern(&self) -> BackgroundPatternType {
        unsafe {
            from_glib(ffi::gtk_source_view_get_background_pattern(self.to_glib_none().0))
        }
    }

    fn get_completion(&self) -> Option<Completion> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_completion(self.to_glib_none().0))
        }
    }

    fn get_draw_spaces(&self) -> DrawSpacesFlags {
        unsafe {
            from_glib(ffi::gtk_source_view_get_draw_spaces(self.to_glib_none().0))
        }
    }

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_gutter(self.to_glib_none().0, window_type.to_glib()))
        }
    }

    fn get_highlight_current_line(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_highlight_current_line(self.to_glib_none().0))
        }
    }

    fn get_indent_on_tab(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_indent_on_tab(self.to_glib_none().0))
        }
    }

    fn get_indent_width(&self) -> i32 {
        unsafe {
            ffi::gtk_source_view_get_indent_width(self.to_glib_none().0)
        }
    }

    fn get_insert_spaces_instead_of_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_insert_spaces_instead_of_tabs(self.to_glib_none().0))
        }
    }

    fn get_right_margin_position(&self) -> u32 {
        unsafe {
            ffi::gtk_source_view_get_right_margin_position(self.to_glib_none().0)
        }
    }

    fn get_show_line_marks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_line_marks(self.to_glib_none().0))
        }
    }

    fn get_show_line_numbers(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_line_numbers(self.to_glib_none().0))
        }
    }

    fn get_show_right_margin(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_right_margin(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_smart_backspace(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_smart_backspace(self.to_glib_none().0))
        }
    }

    fn get_smart_home_end(&self) -> SmartHomeEndType {
        unsafe {
            from_glib(ffi::gtk_source_view_get_smart_home_end(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_space_drawer(&self) -> Option<SpaceDrawer> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_space_drawer(self.to_glib_none().0))
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe {
            ffi::gtk_source_view_get_tab_width(self.to_glib_none().0)
        }
    }

    fn get_visual_column(&self, iter: &gtk::TextIter) -> u32 {
        unsafe {
            ffi::gtk_source_view_get_visual_column(self.to_glib_none().0, iter.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn indent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_view_indent_lines(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    fn set_auto_indent(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_auto_indent(self.to_glib_none().0, enable.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_background_pattern(&self, background_pattern: BackgroundPatternType) {
        unsafe {
            ffi::gtk_source_view_set_background_pattern(self.to_glib_none().0, background_pattern.to_glib());
        }
    }

    fn set_draw_spaces(&self, flags: DrawSpacesFlags) {
        unsafe {
            ffi::gtk_source_view_set_draw_spaces(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_highlight_current_line(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_view_set_highlight_current_line(self.to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_indent_on_tab(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_indent_on_tab(self.to_glib_none().0, enable.to_glib());
        }
    }

    fn set_indent_width(&self, width: i32) {
        unsafe {
            ffi::gtk_source_view_set_indent_width(self.to_glib_none().0, width);
        }
    }

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_insert_spaces_instead_of_tabs(self.to_glib_none().0, enable.to_glib());
        }
    }

    fn set_mark_attributes(&self, category: &str, attributes: &MarkAttributes, priority: i32) {
        unsafe {
            ffi::gtk_source_view_set_mark_attributes(self.to_glib_none().0, category.to_glib_none().0, attributes.to_glib_none().0, priority);
        }
    }

    fn set_right_margin_position(&self, pos: u32) {
        unsafe {
            ffi::gtk_source_view_set_right_margin_position(self.to_glib_none().0, pos);
        }
    }

    fn set_show_line_marks(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_line_marks(self.to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_line_numbers(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_line_numbers(self.to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_right_margin(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_right_margin(self.to_glib_none().0, show.to_glib());
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_smart_backspace(&self, smart_backspace: bool) {
        unsafe {
            ffi::gtk_source_view_set_smart_backspace(self.to_glib_none().0, smart_backspace.to_glib());
        }
    }

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType) {
        unsafe {
            ffi::gtk_source_view_set_smart_home_end(self.to_glib_none().0, smart_home_end.to_glib());
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            ffi::gtk_source_view_set_tab_width(self.to_glib_none().0, width);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn unindent_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_view_unindent_lines(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ChangeCaseType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-case",
                transmute(change_case_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_change_case(&self, case_type: ChangeCaseType) {
        let _ = self.emit("change-case", &[&case_type]).unwrap();
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-number",
                transmute(change_number_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_change_number(&self, count: i32) {
        let _ = self.emit("change-number", &[&count]).unwrap();
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "join-lines",
                transmute(join_lines_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_join_lines(&self) {
        let _ = self.emit("join-lines", &[]).unwrap();
    }

    fn connect_line_mark_activated<F: Fn(&Self, &gtk::TextIter, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, &gdk::Event) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "line-mark-activated",
                transmute(line_mark_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_lines<F: Fn(&Self, bool, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-lines",
                transmute(move_lines_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_lines(&self, copy: bool, count: i32) {
        let _ = self.emit("move-lines", &[&copy, &count]).unwrap();
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-to-matching-bracket",
                transmute(move_to_matching_bracket_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_move_to_matching_bracket(&self, extend_selection: bool) {
        let _ = self.emit("move-to-matching-bracket", &[&extend_selection]).unwrap();
    }

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-words",
                transmute(move_words_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_words(&self, count: i32) {
        let _ = self.emit("move-words", &[&count]).unwrap();
    }

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "redo",
                transmute(redo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_redo(&self) {
        let _ = self.emit("redo", &[]).unwrap();
    }

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-completion",
                transmute(show_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_show_completion(&self) {
        let _ = self.emit("show-completion", &[]).unwrap();
    }

    fn connect_smart_home_end<F: Fn(&Self, &gtk::TextIter, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "smart-home-end",
                transmute(smart_home_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "undo",
                transmute(undo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_undo(&self) {
        let _ = self.emit("undo", &[]).unwrap();
    }

    fn connect_property_auto_indent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::auto-indent",
                transmute(notify_auto_indent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_background_pattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-pattern",
                transmute(notify_background_pattern_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::completion",
                transmute(notify_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw-spaces",
                transmute(notify_draw_spaces_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_highlight_current_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::highlight-current-line",
                transmute(notify_highlight_current_line_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_indent_on_tab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::indent-on-tab",
                transmute(notify_indent_on_tab_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_indent_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::indent-width",
                transmute(notify_indent_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_insert_spaces_instead_of_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::insert-spaces-instead-of-tabs",
                transmute(notify_insert_spaces_instead_of_tabs_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_right_margin_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::right-margin-position",
                transmute(notify_right_margin_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_line_marks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-line-marks",
                transmute(notify_show_line_marks_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-line-numbers",
                transmute(notify_show_line_numbers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_right_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-right-margin",
                transmute(notify_show_right_margin_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_smart_backspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::smart-backspace",
                transmute(notify_smart_backspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_smart_home_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::smart-home-end",
                transmute(notify_smart_home_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_space_drawer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::space-drawer",
                transmute(notify_space_drawer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tab-width",
                transmute(notify_tab_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn change_case_trampoline<P>(this: *mut ffi::GtkSourceView, case_type: ffi::GtkSourceChangeCaseType, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, ChangeCaseType) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), from_glib(case_type))
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn change_number_trampoline<P>(this: *mut ffi::GtkSourceView, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), count)
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn join_lines_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn line_mark_activated_trampoline<P>(this: *mut ffi::GtkSourceView, iter: *mut gtk_ffi::GtkTextIter, event: *mut gdk_ffi::GdkEvent, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, &gtk::TextIter, &gdk::Event) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(iter), &from_glib_none(event))
}

unsafe extern "C" fn move_lines_trampoline<P>(this: *mut ffi::GtkSourceView, copy: glib_ffi::gboolean, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, bool, i32) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), from_glib(copy), count)
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn move_to_matching_bracket_trampoline<P>(this: *mut ffi::GtkSourceView, extend_selection: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, bool) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), from_glib(extend_selection))
}

unsafe extern "C" fn move_words_trampoline<P>(this: *mut ffi::GtkSourceView, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), count)
}

unsafe extern "C" fn redo_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn show_completion_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn smart_home_end_trampoline<P>(this: *mut ffi::GtkSourceView, iter: *mut gtk_ffi::GtkTextIter, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P, &gtk::TextIter, i32) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(iter), count)
}

unsafe extern "C" fn undo_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_auto_indent_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_background_pattern_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_completion_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_draw_spaces_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_highlight_current_line_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_indent_on_tab_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_indent_width_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_insert_spaces_instead_of_tabs_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_right_margin_position_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_line_marks_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_line_numbers_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_right_margin_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
unsafe extern "C" fn notify_smart_backspace_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_smart_home_end_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
unsafe extern "C" fn notify_space_drawer_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tab_width_trampoline<P>(this: *mut ffi::GtkSourceView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<View> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&View::from_glib_borrow(this).downcast_unchecked())
}
