#![doc = "Sidecar module for class [`ENetConnection`][crate::engine::ENetConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetConnection` enums](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetConnection.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`e_net_connection`][crate::engine::e_net_connection]: sidecar module with related enum/flag types\n* [`IENetConnection`][crate::engine::IENetConnection]: virtual methods\n\n\nSee also [Godot docs for `ENetConnection`](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetConnection {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ENetConnection`][crate::engine::ENetConnection].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetConnection` methods](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetConnection: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetConnection {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_host_bound_full(&mut self, bind_address: GString, bind_port: i32, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32, i32, i32, i32, i32);
            let args = (bind_address, bind_port, max_peers, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_host_bound", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_host_bound(&mut self, bind_address: GString, bind_port: i32,) -> crate::engine::global::Error {
            self.create_host_bound_ex(bind_address, bind_port,) . done()
        }
        #[inline]
        pub fn create_host_bound_ex(&mut self, bind_address: GString, bind_port: i32,) -> ExCreateHostBound < '_ > {
            ExCreateHostBound::new(self, bind_address, bind_port,)
        }
        pub(crate) fn create_host_full(&mut self, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, i32, i32, i32);
            let args = (max_peers, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_host(&mut self,) -> crate::engine::global::Error {
            self.create_host_ex() . done()
        }
        #[inline]
        pub fn create_host_ex(&mut self,) -> ExCreateHost < '_ > {
            ExCreateHost::new(self,)
        }
        pub fn destroy(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "destroy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_to_host_full(&mut self, address: GString, port: i32, channels: i32, data: i32,) -> Option < Gd < crate::engine::ENetPacketPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ENetPacketPeer > >;
            type CallSig = (Option < Gd < crate::engine::ENetPacketPeer > >, GString, i32, i32, i32);
            let args = (address, port, channels, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_to_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect_to_host(&mut self, address: GString, port: i32,) -> Option < Gd < crate::engine::ENetPacketPeer > > {
            self.connect_to_host_ex(address, port,) . done()
        }
        #[inline]
        pub fn connect_to_host_ex(&mut self, address: GString, port: i32,) -> ExConnectToHost < '_ > {
            ExConnectToHost::new(self, address, port,)
        }
        pub(crate) fn service_full(&mut self, timeout: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "service", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn service(&mut self,) -> VariantArray {
            self.service_ex() . done()
        }
        #[inline]
        pub fn service_ex(&mut self,) -> ExService < '_ > {
            ExService::new(self,)
        }
        pub fn flush(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flush", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bandwidth_limit_full(&mut self, in_bandwidth: i32, out_bandwidth: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bandwidth_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bandwidth_limit(&mut self,) {
            self.bandwidth_limit_ex() . done()
        }
        #[inline]
        pub fn bandwidth_limit_ex(&mut self,) -> ExBandwidthLimit < '_ > {
            ExBandwidthLimit::new(self,)
        }
        pub fn channel_limit(&mut self, limit: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "channel_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn broadcast(&mut self, channel: i32, packet: PackedByteArray, flags: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, PackedByteArray, i32);
            let args = (channel, packet, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "broadcast", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compress(&mut self, mode: crate::engine::e_net_connection::CompressionMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::e_net_connection::CompressionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dtls_server_setup(&mut self, server_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::TlsOptions >);
            let args = (server_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dtls_server_setup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn dtls_client_setup_full(&mut self, hostname: GString, client_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, Gd < crate::engine::TlsOptions >);
            let args = (hostname, client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dtls_client_setup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn dtls_client_setup(&mut self, hostname: GString,) -> crate::engine::global::Error {
            self.dtls_client_setup_ex(hostname,) . done()
        }
        #[inline]
        pub fn dtls_client_setup_ex(&mut self, hostname: GString,) -> ExDtlsClientSetup < '_ > {
            ExDtlsClientSetup::new(self, hostname,)
        }
        pub fn refuse_new_connections(&mut self, refuse: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (refuse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "refuse_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop_statistic(&mut self, statistic: crate::engine::e_net_connection::HostStatistic,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, crate::engine::e_net_connection::HostStatistic);
            let args = (statistic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pop_statistic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_channels(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_port(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peers(&mut self,) -> Array < Gd < crate::engine::ENetPacketPeer > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::ENetPacketPeer > > >;
            type CallSig = (Array < Gd < crate::engine::ENetPacketPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn socket_send(&mut self, destination_address: GString, destination_port: i32, packet: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, PackedByteArray);
            let args = (destination_address, destination_port, packet,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "socket_send", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ENetConnection {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ENetConnection\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetConnection {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ENetConnection {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ENetConnection {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ENetConnection {
        
    }
    impl crate::obj::cap::GodotDefault for ENetConnection {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ENetConnection {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ENetConnection {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ENetConnection > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ENetConnection::create_host_bound_ex`][super::ENetConnection::create_host_bound_ex]."]
#[must_use]
pub struct ExCreateHostBound < 'a > {
    surround_object: &'a mut re_export::ENetConnection, bind_address: GString, bind_port: i32, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHostBound < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, bind_address: GString, bind_port: i32,) -> Self {
        Self {
            surround_object, bind_address, bind_port, max_peers: 32i32, max_channels: 0i32, in_bandwidth: 0i32, out_bandwidth: 0i32,
        }
    }
    #[inline]
    pub fn max_peers(self, value: i32) -> Self {
        Self {
            max_peers: value, .. self
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
        re_export::ENetConnection::create_host_bound_full(self.surround_object, self.bind_address, self.bind_port, self.max_peers, self.max_channels, self.in_bandwidth, self.out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::create_host_ex`][super::ENetConnection::create_host_ex]."]
#[must_use]
pub struct ExCreateHost < 'a > {
    surround_object: &'a mut re_export::ENetConnection, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHost < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        Self {
            surround_object, max_peers: 32i32, max_channels: 0i32, in_bandwidth: 0i32, out_bandwidth: 0i32,
        }
    }
    #[inline]
    pub fn max_peers(self, value: i32) -> Self {
        Self {
            max_peers: value, .. self
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
        re_export::ENetConnection::create_host_full(self.surround_object, self.max_peers, self.max_channels, self.in_bandwidth, self.out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::connect_to_host_ex`][super::ENetConnection::connect_to_host_ex]."]
#[must_use]
pub struct ExConnectToHost < 'a > {
    surround_object: &'a mut re_export::ENetConnection, address: GString, port: i32, channels: i32, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToHost < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, address: GString, port: i32,) -> Self {
        Self {
            surround_object, address, port, channels: 0i32, data: 0i32,
        }
    }
    #[inline]
    pub fn channels(self, value: i32) -> Self {
        Self {
            channels: value, .. self
        }
    }
    #[inline]
    pub fn data(self, value: i32) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::ENetPacketPeer > > {
        re_export::ENetConnection::connect_to_host_full(self.surround_object, self.address, self.port, self.channels, self.data,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::service_ex`][super::ENetConnection::service_ex]."]
#[must_use]
pub struct ExService < 'a > {
    surround_object: &'a mut re_export::ENetConnection, timeout: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExService < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        Self {
            surround_object, timeout: 0i32,
        }
    }
    #[inline]
    pub fn timeout(self, value: i32) -> Self {
        Self {
            timeout: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VariantArray {
        re_export::ENetConnection::service_full(self.surround_object, self.timeout,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::bandwidth_limit_ex`][super::ENetConnection::bandwidth_limit_ex]."]
#[must_use]
pub struct ExBandwidthLimit < 'a > {
    surround_object: &'a mut re_export::ENetConnection, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBandwidthLimit < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        Self {
            surround_object, in_bandwidth: 0i32, out_bandwidth: 0i32,
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
    pub fn done(self) {
        re_export::ENetConnection::bandwidth_limit_full(self.surround_object, self.in_bandwidth, self.out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::dtls_client_setup_ex`][super::ENetConnection::dtls_client_setup_ex]."]
#[must_use]
pub struct ExDtlsClientSetup < 'a > {
    surround_object: &'a mut re_export::ENetConnection, hostname: GString, client_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDtlsClientSetup < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, hostname: GString,) -> Self {
        Self {
            surround_object, hostname, client_options: unimplemented !("see #156"),
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
        re_export::ENetConnection::dtls_client_setup_full(self.surround_object, self.hostname, self.client_options,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    pub const COMPRESS_NONE: Self = Self {
        ord: 0i32
    };
    pub const COMPRESS_RANGE_CODER: Self = Self {
        ord: 1i32
    };
    pub const COMPRESS_FASTLZ: Self = Self {
        ord: 2i32
    };
    pub const COMPRESS_ZLIB: Self = Self {
        ord: 3i32
    };
    pub const COMPRESS_ZSTD: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for CompressionMode {
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
impl crate::builtin::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompressionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EventType {
    ord: i32
}
impl EventType {
    pub const EVENT_ERROR: Self = Self {
        ord: - 1i32
    };
    pub const EVENT_NONE: Self = Self {
        ord: 0i32
    };
    pub const EVENT_CONNECT: Self = Self {
        ord: 1i32
    };
    pub const EVENT_DISCONNECT: Self = Self {
        ord: 2i32
    };
    pub const EVENT_RECEIVE: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EventType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for EventType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EventType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EventType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HostStatistic {
    ord: i32
}
impl HostStatistic {
    pub const HOST_TOTAL_SENT_DATA: Self = Self {
        ord: 0i32
    };
    pub const HOST_TOTAL_SENT_PACKETS: Self = Self {
        ord: 1i32
    };
    pub const HOST_TOTAL_RECEIVED_DATA: Self = Self {
        ord: 2i32
    };
    pub const HOST_TOTAL_RECEIVED_PACKETS: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for HostStatistic {
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
impl crate::builtin::meta::GodotConvert for HostStatistic {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HostStatistic {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HostStatistic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}