// This file was generated by gir (e43d6c3) from gir-files (db49619)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct CompletionActivation: u32 {
        const COMPLETION_ACTIVATION_NONE = 0;
        const COMPLETION_ACTIVATION_INTERACTIVE = 1;
        const COMPLETION_ACTIVATION_USER_REQUESTED = 2;
    }
}

#[doc(hidden)]
impl ToGlib for CompletionActivation {
    type GlibType = ffi::GtkSourceCompletionActivation;

    fn to_glib(&self) -> ffi::GtkSourceCompletionActivation {
        ffi::GtkSourceCompletionActivation::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompletionActivation> for CompletionActivation {
    fn from_glib(value: ffi::GtkSourceCompletionActivation) -> CompletionActivation {
        CompletionActivation::from_bits_truncate(value.bits())
    }
}

impl StaticType for CompletionActivation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_completion_activation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CompletionActivation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CompletionActivation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GtkSourceCompletionActivation::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for CompletionActivation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct DrawSpacesFlags: u32 {
        const DRAW_SPACES_SPACE = 1;
        const DRAW_SPACES_TAB = 2;
        const DRAW_SPACES_NEWLINE = 4;
        const DRAW_SPACES_NBSP = 8;
        const DRAW_SPACES_LEADING = 16;
        const DRAW_SPACES_TEXT = 32;
        const DRAW_SPACES_TRAILING = 64;
        const DRAW_SPACES_ALL = 127;
    }
}

#[doc(hidden)]
impl ToGlib for DrawSpacesFlags {
    type GlibType = ffi::GtkSourceDrawSpacesFlags;

    fn to_glib(&self) -> ffi::GtkSourceDrawSpacesFlags {
        ffi::GtkSourceDrawSpacesFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceDrawSpacesFlags> for DrawSpacesFlags {
    fn from_glib(value: ffi::GtkSourceDrawSpacesFlags) -> DrawSpacesFlags {
        DrawSpacesFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for DrawSpacesFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_draw_spaces_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DrawSpacesFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DrawSpacesFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GtkSourceDrawSpacesFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for DrawSpacesFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

#[cfg(feature = "v3_14")]
bitflags! {
    pub struct FileSaverFlags: u32 {
        const FILE_SAVER_FLAGS_NONE = 0;
        const FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS = 1;
        const FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME = 2;
        const FILE_SAVER_FLAGS_CREATE_BACKUP = 4;
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for FileSaverFlags {
    type GlibType = ffi::GtkSourceFileSaverFlags;

    fn to_glib(&self) -> ffi::GtkSourceFileSaverFlags {
        ffi::GtkSourceFileSaverFlags::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceFileSaverFlags> for FileSaverFlags {
    fn from_glib(value: ffi::GtkSourceFileSaverFlags) -> FileSaverFlags {
        FileSaverFlags::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_14")]
impl StaticType for FileSaverFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_file_saver_flags_get_type()) }
    }
}

#[cfg(feature = "v3_14")]
impl<'a> FromValueOptional<'a> for FileSaverFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(feature = "v3_14")]
impl<'a> FromValue<'a> for FileSaverFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GtkSourceFileSaverFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

#[cfg(feature = "v3_14")]
impl SetValue for FileSaverFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct GutterRendererState: u32 {
        const GUTTER_RENDERER_STATE_NORMAL = 0;
        const GUTTER_RENDERER_STATE_CURSOR = 1;
        const GUTTER_RENDERER_STATE_PRELIT = 2;
        const GUTTER_RENDERER_STATE_SELECTED = 4;
    }
}

#[doc(hidden)]
impl ToGlib for GutterRendererState {
    type GlibType = ffi::GtkSourceGutterRendererState;

    fn to_glib(&self) -> ffi::GtkSourceGutterRendererState {
        ffi::GtkSourceGutterRendererState::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceGutterRendererState> for GutterRendererState {
    fn from_glib(value: ffi::GtkSourceGutterRendererState) -> GutterRendererState {
        GutterRendererState::from_bits_truncate(value.bits())
    }
}

impl StaticType for GutterRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_gutter_renderer_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GutterRendererState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GutterRendererState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GtkSourceGutterRendererState::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for GutterRendererState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

#[cfg(feature = "v3_18")]
bitflags! {
    pub struct SortFlags: u32 {
        const SORT_FLAGS_NONE = 0;
        const SORT_FLAGS_CASE_SENSITIVE = 1;
        const SORT_FLAGS_REVERSE_ORDER = 2;
        const SORT_FLAGS_REMOVE_DUPLICATES = 4;
    }
}

#[cfg(feature = "v3_18")]
#[doc(hidden)]
impl ToGlib for SortFlags {
    type GlibType = ffi::GtkSourceSortFlags;

    fn to_glib(&self) -> ffi::GtkSourceSortFlags {
        ffi::GtkSourceSortFlags::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_18")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSortFlags> for SortFlags {
    fn from_glib(value: ffi::GtkSourceSortFlags) -> SortFlags {
        SortFlags::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_18")]
impl StaticType for SortFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_sort_flags_get_type()) }
    }
}

#[cfg(feature = "v3_18")]
impl<'a> FromValueOptional<'a> for SortFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(feature = "v3_18")]
impl<'a> FromValue<'a> for SortFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GtkSourceSortFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

#[cfg(feature = "v3_18")]
impl SetValue for SortFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

