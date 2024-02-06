#![doc = "Sidecar module for class [`StreamPeerTls`][crate::engine::StreamPeerTls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeerTLS` enums](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::engine::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `StreamPeerTLS.`\n\nInherits [`StreamPeer`][crate::engine::StreamPeer].\n\nRelated symbols:\n\n* [`stream_peer_tls`][crate::engine::stream_peer_tls]: sidecar module with related enum/flag types\n* [`IStreamPeerTls`][crate::engine::IStreamPeerTls]: virtual methods\n\n\nSee also [Godot docs for `StreamPeerTLS`](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeerTls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StreamPeerTls`][crate::engine::StreamPeerTls].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StreamPeerTLS` methods](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeerTls: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `#[base]` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::engine::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
    }
    impl StreamPeerTls {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn poll(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn accept_stream(&mut self, stream: Gd < crate::engine::StreamPeer >, server_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::StreamPeer >, Gd < crate::engine::TlsOptions >);
            let args = (stream, server_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "accept_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_to_stream_full(&mut self, stream: Gd < crate::engine::StreamPeer >, common_name: GString, client_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::StreamPeer >, GString, Gd < crate::engine::TlsOptions >);
            let args = (stream, common_name, client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_to_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect_to_stream(&mut self, stream: Gd < crate::engine::StreamPeer >, common_name: GString,) -> crate::engine::global::Error {
            self.connect_to_stream_ex(stream, common_name,) . done()
        }
        #[inline]
        pub fn connect_to_stream_ex(&mut self, stream: Gd < crate::engine::StreamPeer >, common_name: GString,) -> ExConnectToStream < '_ > {
            ExConnectToStream::new(self, stream, common_name,)
        }
        pub fn get_status(&self,) -> crate::engine::stream_peer_tls::Status {
            type RetMarshal = PtrcallReturnT < crate::engine::stream_peer_tls::Status >;
            type CallSig = (crate::engine::stream_peer_tls::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::engine::StreamPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::StreamPeer > >;
            type CallSig = (Option < Gd < crate::engine::StreamPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_from_stream(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_from_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for StreamPeerTls {
        type Base = crate::engine::StreamPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"StreamPeerTLS\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeerTls {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for StreamPeerTls {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::StreamPeer > for StreamPeerTls {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for StreamPeerTls {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for StreamPeerTls {
        
    }
    impl crate::obj::cap::GodotDefault for StreamPeerTls {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StreamPeerTls {
        type Target = crate::engine::StreamPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeerTls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_StreamPeerTls {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::StreamPeerTls > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::StreamPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeerTls::connect_to_stream_ex`][super::StreamPeerTls::connect_to_stream_ex]."]
#[must_use]
pub struct ExConnectToStream < 'a > {
    surround_object: &'a mut re_export::StreamPeerTls, stream: Gd < crate::engine::StreamPeer >, common_name: GString, client_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToStream < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeerTls, stream: Gd < crate::engine::StreamPeer >, common_name: GString,) -> Self {
        Self {
            surround_object, stream, common_name, client_options: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn client_options(self, value: Gd < crate::engine::TlsOptions >) -> Self {
        Self {
            client_options: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::StreamPeerTls::connect_to_stream_full(self.surround_object, self.stream, self.common_name, self.client_options,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Status {
    ord: i32
}
impl Status {
    pub const STATUS_DISCONNECTED: Self = Self {
        ord: 0i32
    };
    pub const STATUS_HANDSHAKING: Self = Self {
        ord: 1i32
    };
    pub const STATUS_CONNECTED: Self = Self {
        ord: 2i32
    };
    pub const STATUS_ERROR: Self = Self {
        ord: 3i32
    };
    pub const STATUS_ERROR_HOSTNAME_MISMATCH: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Status {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Status {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Status {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Status {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}