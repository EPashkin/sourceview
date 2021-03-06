// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use gtk;

use Completion;
use CompletionContext;

impl Completion {
    pub fn create_context<'a, P: Into<Option<&'a mut gtk::TextIter>>>(&self, position: P) -> Option<CompletionContext> {
        let mut position = position.into();
        let position = position.to_glib_none_mut();
        unsafe {
            from_glib_none(ffi::gtk_source_completion_create_context(self.to_glib_none().0, position.0))
        }
    }
}
