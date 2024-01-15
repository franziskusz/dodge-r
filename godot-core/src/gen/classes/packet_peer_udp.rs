#![doc = "Sidecar module for class [`PacketPeerUdp`][crate::engine::PacketPeerUdp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PacketPeerUDP` enums](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PacketPeerUDP.`\n\nInherits [`PacketPeer`][crate::engine::PacketPeer].\n\nRelated symbols:\n\n* [`packet_peer_udp`][crate::engine::packet_peer_udp]: sidecar module with related enum/flag types\n* [`IPacketPeerUdp`][crate::engine::IPacketPeerUdp]: virtual methods\n\n\nSee also [Godot docs for `PacketPeerUDP`](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PacketPeerUdp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PacketPeerUdp`][crate::engine::PacketPeerUdp].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PacketPeerUDP` methods](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPacketPeerUdp: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PacketPeerUdp {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn bind_full(&mut self, port: i32, bind_address: GString, recv_buf_size: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, GString, i32);
            let args = (port, bind_address, recv_buf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bind", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bind(&mut self, port: i32,) -> crate::engine::global::Error {
            self.bind_ex(port,) . done()
        }
        #[inline]
        pub fn bind_ex(&mut self, port: i32,) -> ExBind < '_ > {
            ExBind::new(self, port,)
        }
        pub fn close(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "wait", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bound(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bound", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_to_host(&mut self, host: GString, port: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_to_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_socket_connected(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_socket_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_ip(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_ip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_port(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_port(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dest_address(&mut self, host: GString, port: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_dest_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_broadcast_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_broadcast_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn join_multicast_group(&mut self, multicast_address: GString, interface_name: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (multicast_address, interface_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "join_multicast_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn leave_multicast_group(&mut self, multicast_address: GString, interface_name: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (multicast_address, interface_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "leave_multicast_group", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PacketPeerUdp {
        type Base = crate::engine::PacketPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PacketPeerUDP\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PacketPeerUdp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PacketPeerUdp {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for PacketPeerUdp {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PacketPeerUdp {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PacketPeerUdp {
        
    }
    impl crate::obj::cap::GodotDefault for PacketPeerUdp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PacketPeerUdp {
        type Target = crate::engine::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PacketPeerUdp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PacketPeerUdp {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PacketPeerUdp > for $Class {
                
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
#[doc = "Default-param extender for [`PacketPeerUdp::bind_ex`][super::PacketPeerUdp::bind_ex]."]
#[must_use]
pub struct ExBind < 'a > {
    surround_object: &'a mut re_export::PacketPeerUdp, port: i32, bind_address: GString, recv_buf_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBind < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeerUdp, port: i32,) -> Self {
        Self {
            surround_object, port, bind_address: GString::from("*"), recv_buf_size: 65536i32,
        }
    }
    #[inline]
    pub fn bind_address(self, value: GString) -> Self {
        Self {
            bind_address: value, .. self
        }
    }
    #[inline]
    pub fn recv_buf_size(self, value: i32) -> Self {
        Self {
            recv_buf_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::PacketPeerUdp::bind_full(self.surround_object, self.port, self.bind_address, self.recv_buf_size,)
    }
}