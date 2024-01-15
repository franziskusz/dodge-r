#![doc = "Sidecar module for class [`WebSocketPeer`][crate::engine::WebSocketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebSocketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebSocketPeer.`\n\nInherits [`PacketPeer`][crate::engine::PacketPeer].\n\nRelated symbols:\n\n* [`web_socket_peer`][crate::engine::web_socket_peer]: sidecar module with related enum/flag types\n* [`IWebSocketPeer`][crate::engine::IWebSocketPeer]: virtual methods\n\n\nSee also [Godot docs for `WebSocketPeer`](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebSocketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebSocketPeer`][crate::engine::WebSocketPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebSocketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebSocketPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebSocketPeer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn connect_to_url_full(&mut self, url: GString, tls_client_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, Gd < crate::engine::TlsOptions >);
            let args = (url, tls_client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_to_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect_to_url(&mut self, url: GString,) -> crate::engine::global::Error {
            self.connect_to_url_ex(url,) . done()
        }
        #[inline]
        pub fn connect_to_url_ex(&mut self, url: GString,) -> ExConnectToUrl < '_ > {
            ExConnectToUrl::new(self, url,)
        }
        pub fn accept_stream(&mut self, stream: Gd < crate::engine::StreamPeer >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::StreamPeer >);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "accept_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn send_full(&mut self, message: PackedByteArray, write_mode: crate::engine::web_socket_peer::WriteMode,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray, crate::engine::web_socket_peer::WriteMode);
            let args = (message, write_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn send(&mut self, message: PackedByteArray,) -> crate::engine::global::Error {
            self.send_ex(message,) . done()
        }
        #[inline]
        pub fn send_ex(&mut self, message: PackedByteArray,) -> ExSend < '_ > {
            ExSend::new(self, message,)
        }
        pub fn send_text(&mut self, message: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (message,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn was_string_packet(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "was_string_packet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn close_full(&mut self, code: i32, reason: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (code, reason,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn close(&mut self,) {
            self.close_ex() . done()
        }
        #[inline]
        pub fn close_ex(&mut self,) -> ExClose < '_ > {
            ExClose::new(self,)
        }
        pub fn get_connected_host(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connected_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connected_port(&self,) -> u16 {
            type RetMarshal = PtrcallReturnT < u16 >;
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connected_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_protocol(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_protocol", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_requested_url(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_requested_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_no_delay(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_no_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_outbound_buffered_amount(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_outbound_buffered_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ready_state(&self,) -> crate::engine::web_socket_peer::State {
            type RetMarshal = PtrcallReturnT < crate::engine::web_socket_peer::State >;
            type CallSig = (crate::engine::web_socket_peer::State,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ready_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_close_code(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_close_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_close_reason(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_close_reason", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_protocols(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_protocols", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_supported_protocols(&mut self, protocols: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (protocols,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_supported_protocols", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handshake_headers(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_handshake_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handshake_headers(&mut self, protocols: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (protocols,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handshake_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inbound_buffer_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inbound_buffer_size(&mut self, buffer_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outbound_buffer_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outbound_buffer_size(&mut self, buffer_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outbound_buffer_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_queued_packets(&mut self, buffer_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_queued_packets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_queued_packets(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9782usize);
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
    impl crate::obj::GodotClass for WebSocketPeer {
        type Base = crate::engine::PacketPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WebSocketPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebSocketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WebSocketPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for WebSocketPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for WebSocketPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for WebSocketPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebSocketPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebSocketPeer {
        type Target = crate::engine::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebSocketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WebSocketPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WebSocketPeer > for $Class {
                
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
#[doc = "Default-param extender for [`WebSocketPeer::connect_to_url_ex`][super::WebSocketPeer::connect_to_url_ex]."]
#[must_use]
pub struct ExConnectToUrl < 'a > {
    surround_object: &'a mut re_export::WebSocketPeer, url: GString, tls_client_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToUrl < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer, url: GString,) -> Self {
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
        re_export::WebSocketPeer::connect_to_url_full(self.surround_object, self.url, self.tls_client_options,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::send_ex`][super::WebSocketPeer::send_ex]."]
#[must_use]
pub struct ExSend < 'a > {
    surround_object: &'a mut re_export::WebSocketPeer, message: PackedByteArray, write_mode: crate::engine::web_socket_peer::WriteMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSend < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer, message: PackedByteArray,) -> Self {
        Self {
            surround_object, message, write_mode: crate::obj::EngineEnum::from_ord(1),
        }
    }
    #[inline]
    pub fn write_mode(self, value: crate::engine::web_socket_peer::WriteMode) -> Self {
        Self {
            write_mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebSocketPeer::send_full(self.surround_object, self.message, self.write_mode,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::close_ex`][super::WebSocketPeer::close_ex]."]
#[must_use]
pub struct ExClose < 'a > {
    surround_object: &'a mut re_export::WebSocketPeer, code: i32, reason: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClose < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer,) -> Self {
        Self {
            surround_object, code: 1000i32, reason: GString::from(""),
        }
    }
    #[inline]
    pub fn code(self, value: i32) -> Self {
        Self {
            code: value, .. self
        }
    }
    #[inline]
    pub fn reason(self, value: GString) -> Self {
        Self {
            reason: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::WebSocketPeer::close_full(self.surround_object, self.code, self.reason,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WriteMode {
    ord: i32
}
impl WriteMode {
    pub const WRITE_MODE_TEXT: Self = Self {
        ord: 0i32
    };
    pub const WRITE_MODE_BINARY: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for WriteMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for WriteMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for WriteMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for WriteMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct State {
    ord: i32
}
impl State {
    pub const STATE_CONNECTING: Self = Self {
        ord: 0i32
    };
    pub const STATE_OPEN: Self = Self {
        ord: 1i32
    };
    pub const STATE_CLOSING: Self = Self {
        ord: 2i32
    };
    pub const STATE_CLOSED: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for State {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for State {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for State {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for State {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}