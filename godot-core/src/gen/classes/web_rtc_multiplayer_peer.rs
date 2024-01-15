#![doc = "Sidecar module for class [`WebRtcMultiplayerPeer`][crate::engine::WebRtcMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCMultiplayerPeer.`\n\nInherits [`MultiplayerPeer`][crate::engine::MultiplayerPeer].\n\nRelated symbols:\n\n* [`web_rtc_multiplayer_peer`][crate::engine::web_rtc_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IWebRtcMultiplayerPeer`][crate::engine::IWebRtcMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `WebRTCMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebRtcMultiplayerPeer`][crate::engine::WebRtcMultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebRTCMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcMultiplayerPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcMultiplayerPeer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_server_full(&mut self, channels_config: VariantArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, VariantArray);
            let args = (channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_server(&mut self,) -> crate::engine::global::Error {
            self.create_server_ex() . done()
        }
        #[inline]
        pub fn create_server_ex(&mut self,) -> ExCreateServer < '_ > {
            ExCreateServer::new(self,)
        }
        pub(crate) fn create_client_full(&mut self, peer_id: i32, channels_config: VariantArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, VariantArray);
            let args = (peer_id, channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_client", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_client(&mut self, peer_id: i32,) -> crate::engine::global::Error {
            self.create_client_ex(peer_id,) . done()
        }
        #[inline]
        pub fn create_client_ex(&mut self, peer_id: i32,) -> ExCreateClient < '_ > {
            ExCreateClient::new(self, peer_id,)
        }
        pub(crate) fn create_mesh_full(&mut self, peer_id: i32, channels_config: VariantArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, VariantArray);
            let args = (peer_id, channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_mesh(&mut self, peer_id: i32,) -> crate::engine::global::Error {
            self.create_mesh_ex(peer_id,) . done()
        }
        #[inline]
        pub fn create_mesh_ex(&mut self, peer_id: i32,) -> ExCreateMesh < '_ > {
            ExCreateMesh::new(self, peer_id,)
        }
        pub(crate) fn add_peer_full(&mut self, peer: Gd < crate::engine::WebRtcPeerConnection >, peer_id: i32, unreliable_lifetime: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::WebRtcPeerConnection >, i32, i32);
            let args = (peer, peer_id, unreliable_lifetime,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_peer(&mut self, peer: Gd < crate::engine::WebRtcPeerConnection >, peer_id: i32,) -> crate::engine::global::Error {
            self.add_peer_ex(peer, peer_id,) . done()
        }
        #[inline]
        pub fn add_peer_ex(&mut self, peer: Gd < crate::engine::WebRtcPeerConnection >, peer_id: i32,) -> ExAddPeer < '_ > {
            ExAddPeer::new(self, peer, peer_id,)
        }
        pub fn remove_peer(&mut self, peer_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_peer(&mut self, peer_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peer(&mut self, peer_id: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peers(&mut self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peers", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebRtcMultiplayerPeer {
        type Base = crate::engine::MultiplayerPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WebRTCMultiplayerPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WebRtcMultiplayerPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::MultiplayerPeer > for WebRtcMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for WebRtcMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for WebRtcMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for WebRtcMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebRtcMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebRtcMultiplayerPeer {
        type Target = crate::engine::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WebRtcMultiplayerPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WebRtcMultiplayerPeer > for $Class {
                
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
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_server_ex`][super::WebRtcMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'a > {
    surround_object: &'a mut re_export::WebRtcMultiplayerPeer, channels_config: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateServer < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer,) -> Self {
        Self {
            surround_object, channels_config: Array::new(),
        }
    }
    #[inline]
    pub fn channels_config(self, value: VariantArray) -> Self {
        Self {
            channels_config: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebRtcMultiplayerPeer::create_server_full(self.surround_object, self.channels_config,)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_client_ex`][super::WebRtcMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'a > {
    surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32, channels_config: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateClient < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32,) -> Self {
        Self {
            surround_object, peer_id, channels_config: Array::new(),
        }
    }
    #[inline]
    pub fn channels_config(self, value: VariantArray) -> Self {
        Self {
            channels_config: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebRtcMultiplayerPeer::create_client_full(self.surround_object, self.peer_id, self.channels_config,)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_mesh_ex`][super::WebRtcMultiplayerPeer::create_mesh_ex]."]
#[must_use]
pub struct ExCreateMesh < 'a > {
    surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32, channels_config: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMesh < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32,) -> Self {
        Self {
            surround_object, peer_id, channels_config: Array::new(),
        }
    }
    #[inline]
    pub fn channels_config(self, value: VariantArray) -> Self {
        Self {
            channels_config: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebRtcMultiplayerPeer::create_mesh_full(self.surround_object, self.peer_id, self.channels_config,)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::add_peer_ex`][super::WebRtcMultiplayerPeer::add_peer_ex]."]
#[must_use]
pub struct ExAddPeer < 'a > {
    surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer: Gd < crate::engine::WebRtcPeerConnection >, peer_id: i32, unreliable_lifetime: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPeer < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer: Gd < crate::engine::WebRtcPeerConnection >, peer_id: i32,) -> Self {
        Self {
            surround_object, peer, peer_id, unreliable_lifetime: 1i32,
        }
    }
    #[inline]
    pub fn unreliable_lifetime(self, value: i32) -> Self {
        Self {
            unreliable_lifetime: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebRtcMultiplayerPeer::add_peer_full(self.surround_object, self.peer, self.peer_id, self.unreliable_lifetime,)
    }
}