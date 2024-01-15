#![doc = "Sidecar module for class [`ENetPacketPeer`][crate::engine::ENetPacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetPacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetPacketPeer.`\n\nInherits [`PacketPeer`][crate::engine::PacketPeer].\n\nRelated symbols:\n\n* [`e_net_packet_peer`][crate::engine::e_net_packet_peer]: sidecar module with related enum/flag types\n* [`IENetPacketPeer`][crate::engine::IENetPacketPeer]: virtual methods\n\n\nSee also [Godot docs for `ENetPacketPeer`](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetPacketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ENetPacketPeer`][crate::engine::ENetPacketPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetPacketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetPacketPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetPacketPeer {
        pub(crate) fn peer_disconnect_full(&mut self, data: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "peer_disconnect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn peer_disconnect(&mut self,) {
            self.peer_disconnect_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_ex(&mut self,) -> ExPeerDisconnect < '_ > {
            ExPeerDisconnect::new(self,)
        }
        pub(crate) fn peer_disconnect_later_full(&mut self, data: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "peer_disconnect_later", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn peer_disconnect_later(&mut self,) {
            self.peer_disconnect_later_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_later_ex(&mut self,) -> ExPeerDisconnectLater < '_ > {
            ExPeerDisconnectLater::new(self,)
        }
        pub(crate) fn peer_disconnect_now_full(&mut self, data: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "peer_disconnect_now", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn peer_disconnect_now(&mut self,) {
            self.peer_disconnect_now_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_now_ex(&mut self,) -> ExPeerDisconnectNow < '_ > {
            ExPeerDisconnectNow::new(self,)
        }
        pub fn ping(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ping_interval(&mut self, ping_interval: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (ping_interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ping_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn send(&mut self, channel: i32, packet: PackedByteArray, flags: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, PackedByteArray, i32);
            let args = (channel, packet, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn throttle_configure(&mut self, interval: i32, acceleration: i32, deceleration: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32);
            let args = (interval, acceleration, deceleration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "throttle_configure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: i32, timeout_min: i32, timeout_max: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32);
            let args = (timeout, timeout_min, timeout_max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_address(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_remote_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_port(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_remote_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_statistic(&mut self, statistic: crate::engine::e_net_packet_peer::PeerStatistic,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, crate::engine::e_net_packet_peer::PeerStatistic);
            let args = (statistic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_statistic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state(&self,) -> crate::engine::e_net_packet_peer::PeerState {
            type RetMarshal = PtrcallReturnT < crate::engine::e_net_packet_peer::PeerState >;
            type CallSig = (crate::engine::e_net_packet_peer::PeerState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_channels(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const PACKET_LOSS_SCALE: i32 = 65536i32;
        pub const PACKET_THROTTLE_SCALE: i32 = 32i32;
        pub const FLAG_RELIABLE: i32 = 1i32;
        pub const FLAG_UNSEQUENCED: i32 = 2i32;
        pub const FLAG_UNRELIABLE_FRAGMENT: i32 = 8i32;
        
    }
    impl crate::obj::GodotClass for ENetPacketPeer {
        type Base = crate::engine::PacketPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ENetPacketPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetPacketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ENetPacketPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for ENetPacketPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ENetPacketPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ENetPacketPeer {
        
    }
    impl std::ops::Deref for ENetPacketPeer {
        type Target = crate::engine::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetPacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ENetPacketPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ENetPacketPeer > for $Class {
                
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
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_ex`][super::ENetPacketPeer::peer_disconnect_ex]."]
#[must_use]
pub struct ExPeerDisconnect < 'a > {
    surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnect < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        Self {
            surround_object, data: 0i32,
        }
    }
    #[inline]
    pub fn data(self, value: i32) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ENetPacketPeer::peer_disconnect_full(self.surround_object, self.data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_later_ex`][super::ENetPacketPeer::peer_disconnect_later_ex]."]
#[must_use]
pub struct ExPeerDisconnectLater < 'a > {
    surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectLater < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        Self {
            surround_object, data: 0i32,
        }
    }
    #[inline]
    pub fn data(self, value: i32) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ENetPacketPeer::peer_disconnect_later_full(self.surround_object, self.data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_now_ex`][super::ENetPacketPeer::peer_disconnect_now_ex]."]
#[must_use]
pub struct ExPeerDisconnectNow < 'a > {
    surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectNow < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        Self {
            surround_object, data: 0i32,
        }
    }
    #[inline]
    pub fn data(self, value: i32) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ENetPacketPeer::peer_disconnect_now_full(self.surround_object, self.data,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PeerState {
    ord: i32
}
impl PeerState {
    pub const STATE_DISCONNECTED: Self = Self {
        ord: 0i32
    };
    pub const STATE_CONNECTING: Self = Self {
        ord: 1i32
    };
    pub const STATE_ACKNOWLEDGING_CONNECT: Self = Self {
        ord: 2i32
    };
    pub const STATE_CONNECTION_PENDING: Self = Self {
        ord: 3i32
    };
    pub const STATE_CONNECTION_SUCCEEDED: Self = Self {
        ord: 4i32
    };
    pub const STATE_CONNECTED: Self = Self {
        ord: 5i32
    };
    pub const STATE_DISCONNECT_LATER: Self = Self {
        ord: 6i32
    };
    pub const STATE_DISCONNECTING: Self = Self {
        ord: 7i32
    };
    pub const STATE_ACKNOWLEDGING_DISCONNECT: Self = Self {
        ord: 8i32
    };
    pub const STATE_ZOMBIE: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for PeerState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PeerState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PeerState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PeerState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PeerStatistic {
    ord: i32
}
impl PeerStatistic {
    pub const PEER_PACKET_LOSS: Self = Self {
        ord: 0i32
    };
    pub const PEER_PACKET_LOSS_VARIANCE: Self = Self {
        ord: 1i32
    };
    pub const PEER_PACKET_LOSS_EPOCH: Self = Self {
        ord: 2i32
    };
    pub const PEER_ROUND_TRIP_TIME: Self = Self {
        ord: 3i32
    };
    pub const PEER_ROUND_TRIP_TIME_VARIANCE: Self = Self {
        ord: 4i32
    };
    pub const PEER_LAST_ROUND_TRIP_TIME: Self = Self {
        ord: 5i32
    };
    pub const PEER_LAST_ROUND_TRIP_TIME_VARIANCE: Self = Self {
        ord: 6i32
    };
    pub const PEER_PACKET_THROTTLE: Self = Self {
        ord: 7i32
    };
    pub const PEER_PACKET_THROTTLE_LIMIT: Self = Self {
        ord: 8i32
    };
    pub const PEER_PACKET_THROTTLE_COUNTER: Self = Self {
        ord: 9i32
    };
    pub const PEER_PACKET_THROTTLE_EPOCH: Self = Self {
        ord: 10i32
    };
    pub const PEER_PACKET_THROTTLE_ACCELERATION: Self = Self {
        ord: 11i32
    };
    pub const PEER_PACKET_THROTTLE_DECELERATION: Self = Self {
        ord: 12i32
    };
    pub const PEER_PACKET_THROTTLE_INTERVAL: Self = Self {
        ord: 13i32
    };
    
}
impl crate::obj::EngineEnum for PeerStatistic {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PeerStatistic {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PeerStatistic {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PeerStatistic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}