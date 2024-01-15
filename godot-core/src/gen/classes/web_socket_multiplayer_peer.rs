#![doc = "Sidecar module for class [`WebSocketMultiplayerPeer`][crate::engine::WebSocketMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebSocketMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebSocketMultiplayerPeer.`\n\nInherits [`MultiplayerPeer`][crate::engine::MultiplayerPeer].\n\nRelated symbols:\n\n* [`web_socket_multiplayer_peer`][crate::engine::web_socket_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IWebSocketMultiplayerPeer`][crate::engine::IWebSocketMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `WebSocketMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebSocketMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebSocketMultiplayerPeer`][crate::engine::WebSocketMultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebSocketMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebSocketMultiplayerPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebSocketMultiplayerPeer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_client_full(&mut self, url: GString, tls_client_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, Gd < crate::engine::TlsOptions >);
            let args = (url, tls_client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_client", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_client(&mut self, url: GString,) -> crate::engine::global::Error {
            self.create_client_ex(url,) . done()
        }
        #[inline]
        pub fn create_client_ex(&mut self, url: GString,) -> ExCreateClient < '_ > {
            ExCreateClient::new(self, url,)
        }
        pub(crate) fn create_server_full(&mut self, port: i32, bind_address: GString, tls_server_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, GString, Gd < crate::engine::TlsOptions >);
            let args = (port, bind_address, tls_server_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_server(&mut self, port: i32,) -> crate::engine::global::Error {
            self.create_server_ex(port,) . done()
        }
        #[inline]
        pub fn create_server_ex(&mut self, port: i32,) -> ExCreateServer < '_ > {
            ExCreateServer::new(self, port,)
        }
        pub fn get_peer(&self, peer_id: i32,) -> Option < Gd < crate::engine::WebSocketPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::WebSocketPeer > >;
            type CallSig = (Option < Gd < crate::engine::WebSocketPeer > >, i32);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peer_address(&self, id: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peer_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peer_port(&self, id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peer_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_protocols(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_protocols", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_supported_protocols(&mut self, protocols: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (protocols,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_supported_protocols", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handshake_headers(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_handshake_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handshake_headers(&mut self, protocols: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (protocols,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handshake_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inbound_buffer_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inbound_buffer_size(&mut self, buffer_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outbound_buffer_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outbound_buffer_size(&mut self, buffer_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handshake_timeout(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_handshake_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handshake_timeout(&mut self, timeout: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handshake_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_queued_packets(&mut self, max_queued_packets: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_queued_packets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_queued_packets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_queued_packets(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_queued_packets", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebSocketMultiplayerPeer {
        type Base = crate::engine::MultiplayerPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WebSocketMultiplayerPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebSocketMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WebSocketMultiplayerPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::MultiplayerPeer > for WebSocketMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for WebSocketMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for WebSocketMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for WebSocketMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebSocketMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebSocketMultiplayerPeer {
        type Target = crate::engine::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebSocketMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WebSocketMultiplayerPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WebSocketMultiplayerPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::MultiplayerPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PacketPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WebSocketMultiplayerPeer::create_client_ex`][super::WebSocketMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'a > {
    surround_object: &'a mut re_export::WebSocketMultiplayerPeer, url: GString, tls_client_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateClient < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketMultiplayerPeer, url: GString,) -> Self {
        Self {
            surround_object, url, tls_client_options: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn tls_client_options(self, value: Gd < crate::engine::TlsOptions >) -> Self {
        Self {
            tls_client_options: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebSocketMultiplayerPeer::create_client_full(self.surround_object, self.url, self.tls_client_options,)
    }
}
#[doc = "Default-param extender for [`WebSocketMultiplayerPeer::create_server_ex`][super::WebSocketMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'a > {
    surround_object: &'a mut re_export::WebSocketMultiplayerPeer, port: i32, bind_address: GString, tls_server_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateServer < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketMultiplayerPeer, port: i32,) -> Self {
        Self {
            surround_object, port, bind_address: GString::from("*"), tls_server_options: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn bind_address(self, value: GString) -> Self {
        Self {
            bind_address: value, .. self
        }
    }
    #[inline]
    pub fn tls_server_options(self, value: Gd < crate::engine::TlsOptions >) -> Self {
        Self {
            tls_server_options: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebSocketMultiplayerPeer::create_server_full(self.surround_object, self.port, self.bind_address, self.tls_server_options,)
    }
}