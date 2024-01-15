#![doc = "Sidecar module for class [`Upnp`][crate::engine::Upnp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNP` enums](https://docs.godotengine.org/en/stable/classes/class_upnp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UPNP.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`upnp`][crate::engine::upnp]: sidecar module with related enum/flag types\n* [`IUpnp`][crate::engine::IUpnp]: virtual methods\n\n\nSee also [Godot docs for `UPNP`](https://docs.godotengine.org/en/stable/classes/class_upnp.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Upnp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Upnp`][crate::engine::Upnp].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UPNP` methods](https://docs.godotengine.org/en/stable/classes/class_upnp.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUpnp: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Upnp {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_device_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device(&self, index: i32,) -> Option < Gd < crate::engine::UpnpDevice > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::UpnpDevice > >;
            type CallSig = (Option < Gd < crate::engine::UpnpDevice > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_device(&mut self, device: Gd < crate::engine::UpnpDevice >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::UpnpDevice >);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_device(&mut self, index: i32, device: Gd < crate::engine::UpnpDevice >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::UpnpDevice >);
            let args = (index, device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_device(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_devices(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_devices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gateway(&self,) -> Option < Gd < crate::engine::UpnpDevice > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::UpnpDevice > >;
            type CallSig = (Option < Gd < crate::engine::UpnpDevice > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gateway", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn discover_full(&mut self, timeout: i32, ttl: i32, device_filter: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32, GString);
            let args = (timeout, ttl, device_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "discover", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn discover(&mut self,) -> i32 {
            self.discover_ex() . done()
        }
        #[inline]
        pub fn discover_ex(&mut self,) -> ExDiscover < '_ > {
            ExDiscover::new(self,)
        }
        pub fn query_external_address(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "query_external_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_port_mapping_full(&self, port: i32, port_internal: i32, desc: GString, proto: GString, duration: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32, GString, GString, i32);
            let args = (port, port_internal, desc, proto, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_port_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_port_mapping(&self, port: i32,) -> i32 {
            self.add_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn add_port_mapping_ex(&self, port: i32,) -> ExAddPortMapping < '_ > {
            ExAddPortMapping::new(self, port,)
        }
        pub(crate) fn delete_port_mapping_full(&self, port: i32, proto: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, GString);
            let args = (port, proto,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "delete_port_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn delete_port_mapping(&self, port: i32,) -> i32 {
            self.delete_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn delete_port_mapping_ex(&self, port: i32,) -> ExDeletePortMapping < '_ > {
            ExDeletePortMapping::new(self, port,)
        }
        pub fn set_discover_multicast_if(&mut self, m_if: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (m_if,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_discover_multicast_if", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_discover_multicast_if(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_discover_multicast_if", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_discover_local_port(&mut self, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_discover_local_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_discover_local_port(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_discover_local_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_discover_ipv6(&mut self, ipv6: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (ipv6,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_discover_ipv6", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_discover_ipv6(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_discover_ipv6", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Upnp {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"UPNP\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Upnp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Upnp {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Upnp {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Upnp {
        
    }
    impl crate::obj::cap::GodotDefault for Upnp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Upnp {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Upnp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Upnp {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Upnp > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Upnp::discover_ex`][super::Upnp::discover_ex]."]
#[must_use]
pub struct ExDiscover < 'a > {
    surround_object: &'a mut re_export::Upnp, timeout: i32, ttl: i32, device_filter: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDiscover < 'a > {
    fn new(surround_object: &'a mut re_export::Upnp,) -> Self {
        Self {
            surround_object, timeout: 2000i32, ttl: 2i32, device_filter: GString::from("InternetGatewayDevice"),
        }
    }
    #[inline]
    pub fn timeout(self, value: i32) -> Self {
        Self {
            timeout: value, .. self
        }
    }
    #[inline]
    pub fn ttl(self, value: i32) -> Self {
        Self {
            ttl: value, .. self
        }
    }
    #[inline]
    pub fn device_filter(self, value: GString) -> Self {
        Self {
            device_filter: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Upnp::discover_full(self.surround_object, self.timeout, self.ttl, self.device_filter,)
    }
}
#[doc = "Default-param extender for [`Upnp::add_port_mapping_ex`][super::Upnp::add_port_mapping_ex]."]
#[must_use]
pub struct ExAddPortMapping < 'a > {
    surround_object: &'a re_export::Upnp, port: i32, port_internal: i32, desc: GString, proto: GString, duration: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPortMapping < 'a > {
    fn new(surround_object: &'a re_export::Upnp, port: i32,) -> Self {
        Self {
            surround_object, port, port_internal: 0i32, desc: GString::from(""), proto: GString::from("UDP"), duration: 0i32,
        }
    }
    #[inline]
    pub fn port_internal(self, value: i32) -> Self {
        Self {
            port_internal: value, .. self
        }
    }
    #[inline]
    pub fn desc(self, value: GString) -> Self {
        Self {
            desc: value, .. self
        }
    }
    #[inline]
    pub fn proto(self, value: GString) -> Self {
        Self {
            proto: value, .. self
        }
    }
    #[inline]
    pub fn duration(self, value: i32) -> Self {
        Self {
            duration: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Upnp::add_port_mapping_full(self.surround_object, self.port, self.port_internal, self.desc, self.proto, self.duration,)
    }
}
#[doc = "Default-param extender for [`Upnp::delete_port_mapping_ex`][super::Upnp::delete_port_mapping_ex]."]
#[must_use]
pub struct ExDeletePortMapping < 'a > {
    surround_object: &'a re_export::Upnp, port: i32, proto: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeletePortMapping < 'a > {
    fn new(surround_object: &'a re_export::Upnp, port: i32,) -> Self {
        Self {
            surround_object, port, proto: GString::from("UDP"),
        }
    }
    #[inline]
    pub fn proto(self, value: GString) -> Self {
        Self {
            proto: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Upnp::delete_port_mapping_full(self.surround_object, self.port, self.proto,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UPNPResult {
    ord: i32
}
impl UPNPResult {
    pub const UPNP_RESULT_SUCCESS: Self = Self {
        ord: 0i32
    };
    pub const UPNP_RESULT_NOT_AUTHORIZED: Self = Self {
        ord: 1i32
    };
    pub const UPNP_RESULT_PORT_MAPPING_NOT_FOUND: Self = Self {
        ord: 2i32
    };
    pub const UPNP_RESULT_INCONSISTENT_PARAMETERS: Self = Self {
        ord: 3i32
    };
    pub const UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY: Self = Self {
        ord: 4i32
    };
    pub const UPNP_RESULT_ACTION_FAILED: Self = Self {
        ord: 5i32
    };
    pub const UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED: Self = Self {
        ord: 6i32
    };
    pub const UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED: Self = Self {
        ord: 7i32
    };
    pub const UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED: Self = Self {
        ord: 8i32
    };
    pub const UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD: Self = Self {
        ord: 9i32
    };
    pub const UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD: Self = Self {
        ord: 10i32
    };
    pub const UPNP_RESULT_NO_PORT_MAPS_AVAILABLE: Self = Self {
        ord: 11i32
    };
    pub const UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM: Self = Self {
        ord: 12i32
    };
    pub const UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING: Self = Self {
        ord: 13i32
    };
    pub const UPNP_RESULT_SAME_PORT_VALUES_REQUIRED: Self = Self {
        ord: 14i32
    };
    pub const UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED: Self = Self {
        ord: 15i32
    };
    pub const UPNP_RESULT_INVALID_GATEWAY: Self = Self {
        ord: 16i32
    };
    pub const UPNP_RESULT_INVALID_PORT: Self = Self {
        ord: 17i32
    };
    pub const UPNP_RESULT_INVALID_PROTOCOL: Self = Self {
        ord: 18i32
    };
    pub const UPNP_RESULT_INVALID_DURATION: Self = Self {
        ord: 19i32
    };
    pub const UPNP_RESULT_INVALID_ARGS: Self = Self {
        ord: 20i32
    };
    pub const UPNP_RESULT_INVALID_RESPONSE: Self = Self {
        ord: 21i32
    };
    pub const UPNP_RESULT_INVALID_PARAM: Self = Self {
        ord: 22i32
    };
    pub const UPNP_RESULT_HTTP_ERROR: Self = Self {
        ord: 23i32
    };
    pub const UPNP_RESULT_SOCKET_ERROR: Self = Self {
        ord: 24i32
    };
    pub const UPNP_RESULT_MEM_ALLOC_ERROR: Self = Self {
        ord: 25i32
    };
    pub const UPNP_RESULT_NO_GATEWAY: Self = Self {
        ord: 26i32
    };
    pub const UPNP_RESULT_NO_DEVICES: Self = Self {
        ord: 27i32
    };
    pub const UPNP_RESULT_UNKNOWN_ERROR: Self = Self {
        ord: 28i32
    };
    
}
impl crate::obj::EngineEnum for UPNPResult {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for UPNPResult {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for UPNPResult {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UPNPResult {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}