#![doc = "Sidecar module for class [`ENetMultiplayerPeer`][crate::engine::ENetMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetMultiplayerPeer.`\n\nInherits [`MultiplayerPeer`][crate::engine::MultiplayerPeer].\n\nRelated symbols:\n\n* [`e_net_multiplayer_peer`][crate::engine::e_net_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IENetMultiplayerPeer`][crate::engine::IENetMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `ENetMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ENetMultiplayerPeer`][crate::engine::ENetMultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetMultiplayerPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetMultiplayerPeer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_server_full(&mut self, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, i32, i32, i32, i32);
            let args = (port, max_clients, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2541usize);
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
        pub(crate) fn create_client_full(&mut self, address: GString, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32, i32, i32, i32, i32);
            let args = (address, port, channel_count, in_bandwidth, out_bandwidth, local_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_client", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_client(&mut self, address: GString, port: i32,) -> crate::engine::global::Error {
            self.create_client_ex(address, port,) . done()
        }
        #[inline]
        pub fn create_client_ex(&mut self, address: GString, port: i32,) -> ExCreateClient < '_ > {
            ExCreateClient::new(self, address, port,)
        }
        pub fn create_mesh(&mut self, unique_id: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32);
            let args = (unique_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_mesh_peer(&mut self, peer_id: i32, host: Gd < crate::engine::ENetConnection >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, Gd < crate::engine::ENetConnection >);
            let args = (peer_id, host,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_mesh_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bind_ip(&mut self, ip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (ip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bind_ip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_host(&self,) -> Option < Gd < crate::engine::ENetConnection > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ENetConnection > >;
            type CallSig = (Option < Gd < crate::engine::ENetConnection > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peer(&self, id: i32,) -> Option < Gd < crate::engine::ENetPacketPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ENetPacketPeer > >;
            type CallSig = (Option < Gd < crate::engine::ENetPacketPeer > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peer", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ENetMultiplayerPeer {
        type Base = crate::engine::MultiplayerPeer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ENetMultiplayerPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ENetMultiplayerPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::MultiplayerPeer > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::PacketPeer > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for ENetMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ENetMultiplayerPeer {
        type Target = crate::engine::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ENetMultiplayerPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ENetMultiplayerPeer > for $Class {
                
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
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_server_ex`][super::ENetMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'a > {
    surround_object: &'a mut re_export::ENetMultiplayerPeer, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateServer < 'a > {
    fn new(surround_object: &'a mut re_export::ENetMultiplayerPeer, port: i32,) -> Self {
        Self {
            surround_object, port, max_clients: 32i32, max_channels: 0i32, in_bandwidth: 0i32, out_bandwidth: 0i32,
        }
    }
    #[inline]
    pub fn max_clients(self, value: i32) -> Self {
        Self {
            max_clients: value, .. self
        }
    }
    #[inline]
    pub fn max_channels(self, value: i32) -> Self {
        Self {
            max_channels: value, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, value: i32) -> Self {
        Self {
            in_bandwidth: value, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, value: i32) -> Self {
        Self {
            out_bandwidth: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::ENetMultiplayerPeer::create_server_full(self.surround_object, self.port, self.max_clients, self.max_channels, self.in_bandwidth, self.out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_client_ex`][super::ENetMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'a > {
    surround_object: &'a mut re_export::ENetMultiplayerPeer, address: GString, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateClient < 'a > {
    fn new(surround_object: &'a mut re_export::ENetMultiplayerPeer, address: GString, port: i32,) -> Self {
        Self {
            surround_object, address, port, channel_count: 0i32, in_bandwidth: 0i32, out_bandwidth: 0i32, local_port: 0i32,
        }
    }
    #[inline]
    pub fn channel_count(self, value: i32) -> Self {
        Self {
            channel_count: value, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, value: i32) -> Self {
        Self {
            in_bandwidth: value, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, value: i32) -> Self {
        Self {
            out_bandwidth: value, .. self
        }
    }
    #[inline]
    pub fn local_port(self, value: i32) -> Self {
        Self {
            local_port: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::ENetMultiplayerPeer::create_client_full(self.surround_object, self.address, self.port, self.channel_count, self.in_bandwidth, self.out_bandwidth, self.local_port,)
    }
}