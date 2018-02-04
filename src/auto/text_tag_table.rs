// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use Buildable;
use TextTag;
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
    pub struct TextTagTable(Object<ffi::GtkTextTagTable, ffi::GtkTextTagTableClass>): Buildable;

    match fn {
        get_type => || ffi::gtk_text_tag_table_get_type(),
    }
}

impl TextTagTable {
    pub fn new() -> TextTagTable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_tag_table_new())
        }
    }
}

impl Default for TextTagTable {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TextTagTableExt {
    fn add(&self, tag: &TextTag) -> bool;

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TextTagTableForeach, data: P);

    fn get_size(&self) -> i32;

    fn lookup(&self, name: &str) -> Option<TextTag>;

    fn remove(&self, tag: &TextTag);

    fn connect_tag_added<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tag_changed<F: Fn(&Self, &TextTag, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tag_removed<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TextTagTable> + IsA<glib::object::Object>> TextTagTableExt for O {
    fn add(&self, tag: &TextTag) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_tag_table_add(self.to_glib_none().0, tag.to_glib_none().0))
        }
    }

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TextTagTableForeach, data: P) {
    //    unsafe { TODO: call ffi::gtk_text_tag_table_foreach() }
    //}

    fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_text_tag_table_get_size(self.to_glib_none().0)
        }
    }

    fn lookup(&self, name: &str) -> Option<TextTag> {
        unsafe {
            from_glib_none(ffi::gtk_text_tag_table_lookup(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove(&self, tag: &TextTag) {
        unsafe {
            ffi::gtk_text_tag_table_remove(self.to_glib_none().0, tag.to_glib_none().0);
        }
    }

    fn connect_tag_added<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextTag) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-added",
                transmute(tag_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_tag_changed<F: Fn(&Self, &TextTag, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextTag, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-changed",
                transmute(tag_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_tag_removed<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextTag) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-removed",
                transmute(tag_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn tag_added_trampoline<P>(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, f: glib_ffi::gpointer)
where P: IsA<TextTagTable> {
    callback_guard!();
    let f: &&(Fn(&P, &TextTag) + 'static) = transmute(f);
    f(&TextTagTable::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tag))
}

unsafe extern "C" fn tag_changed_trampoline<P>(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, size_changed: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<TextTagTable> {
    callback_guard!();
    let f: &&(Fn(&P, &TextTag, bool) + 'static) = transmute(f);
    f(&TextTagTable::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tag), from_glib(size_changed))
}

unsafe extern "C" fn tag_removed_trampoline<P>(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, f: glib_ffi::gpointer)
where P: IsA<TextTagTable> {
    callback_guard!();
    let f: &&(Fn(&P, &TextTag) + 'static) = transmute(f);
    f(&TextTagTable::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tag))
}
