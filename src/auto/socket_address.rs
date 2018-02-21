// This file was generated by gir (https://github.com/gtk-rs/gir @ 72ba992)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use SocketConnectable;
use SocketFamily;
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
    pub struct SocketAddress(Object<ffi::GSocketAddress, ffi::GSocketAddressClass>): SocketConnectable;

    match fn {
        get_type => || ffi::g_socket_address_get_type(),
    }
}

impl SocketAddress {
    //pub fn new_from_native(native: /*Unimplemented*/Fundamental: Pointer, len: usize) -> SocketAddress {
    //    unsafe { TODO: call ffi::g_socket_address_new_from_native() }
    //}
}

pub trait SocketAddressExt {
    fn get_family(&self) -> SocketFamily;

    fn get_native_size(&self) -> isize;

    //fn to_native<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, dest: P, destlen: usize) -> Result<(), Error>;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketAddress> + IsA<glib::object::Object>> SocketAddressExt for O {
    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_address_get_family(self.to_glib_none().0))
        }
    }

    fn get_native_size(&self) -> isize {
        unsafe {
            ffi::g_socket_address_get_native_size(self.to_glib_none().0)
        }
    }

    //fn to_native<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, dest: P, destlen: usize) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_socket_address_to_native() }
    //}

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::family",
                transmute(notify_family_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_family_trampoline<P>(this: *mut ffi::GSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketAddress> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketAddress::from_glib_borrow(this).downcast_unchecked())
}
