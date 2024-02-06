#![doc = "Sidecar module for class [`MultiplayerPeer`][crate::engine::MultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerPeer.`\n\nInherits [`PacketPeer`][crate::engine::PacketPeer].\n\nRelated symbols:\n\n* [`multiplayer_peer`][crate::engine::multiplayer_peer]: sidecar module with related enum/flag types\n* [`IMultiplayerPeer`][crate::engine::IMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiplayerPeer`][crate::engine::MultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerPeer {
        pub fn set_transfer_channel(&mut self, channel: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transfer_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transfer_channel(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transfer_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transfer_mode(&mut self, mode: crate::engine::multiplayer_peer::TransferMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::multiplayer_peer::TransferMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transfer_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transfer_mode(&self,) -> crate::engine::multiplayer_peer::TransferMode {
            type RetMarshal = PtrcallReturnT < crate::engine::multiplayer_peer::TransferMode >;
            type CallSig = (crate::engine::multiplayer_peer::TransferMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transfer_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_peer(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_target_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_peer(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_channel(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_mode(&self,) -> crate::engine::multiplayer_peer::TransferMode {
            type RetMarshal = PtrcallReturnT < crate::engine::multiplayer_peer::TransferMode >;
            type CallSig = (crate::engine::multiplayer_peer::TransferMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn disconnect_peer_full(&mut self, peer: i32, force: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (peer, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn disconnect_peer(&mut self, peer: i32,) {
            self.disconnect_peer_ex(peer,) . done()
        }
        #[inline]
        pub fn disconnect_peer_ex(&mut self, peer: i32,) -> ExDisconnectPeer < '_ > {
            ExDisconnectPeer::new(self, peer,)
        }
        pub fn get_connection_status(&self,) -> crate::engine::multiplayer_peer::ConnectionStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::multiplayer_peer::ConnectionStatus >;
            type CallSig = (crate::engine::multiplayer_peer::ConnectionStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connection_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_unique_id(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refuse_new_connections(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_refuse_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_refusing_new_connections(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_refusing_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_server_relay_supported(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_server_relay_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const TARGET_PEER_BROADCAST: i32 = 0i32;
        pub const TARGET_PEER_SERVER: i32 = 1i32;
        
    }
    impl crate::obj::GodotClass for MultiplayerPeer {
        type Base = crate::engine::PacketPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MultiplayerPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MultiplayerPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for MultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MultiplayerPeer {
        
    }
    impl std::ops::Deref for MultiplayerPeer {
        type Target = crate::engine::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MultiplayerPeer {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`MultiplayerPeer::disconnect_peer_ex`][super::MultiplayerPeer::disconnect_peer_ex]."]
#[must_use]
pub struct ExDisconnectPeer < 'a > {
    surround_object: &'a mut re_export::MultiplayerPeer, peer: i32, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDisconnectPeer < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerPeer, peer: i32,) -> Self {
        Self {
            surround_object, peer, force: false,
        }
    }
    #[inline]
    pub fn force(self, value: bool) -> Self {
        Self {
            force: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::MultiplayerPeer::disconnect_peer_full(self.surround_object, self.peer, self.force,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ConnectionStatus {
    ord: i32
}
impl ConnectionStatus {
    pub const CONNECTION_DISCONNECTED: Self = Self {
        ord: 0i32
    };
    pub const CONNECTION_CONNECTING: Self = Self {
        ord: 1i32
    };
    pub const CONNECTION_CONNECTED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ConnectionStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ConnectionStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ConnectionStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ConnectionStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TransferMode {
    ord: i32
}
impl TransferMode {
    pub const TRANSFER_MODE_UNRELIABLE: Self = Self {
        ord: 0i32
    };
    pub const TRANSFER_MODE_UNRELIABLE_ORDERED: Self = Self {
        ord: 1i32
    };
    pub const TRANSFER_MODE_RELIABLE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TransferMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TransferMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TransferMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TransferMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}