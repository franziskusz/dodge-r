#![doc = "Sidecar module for class [`UpnpDevice`][crate::engine::UpnpDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNPDevice` enums](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UPNPDevice.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`upnp_device`][crate::engine::upnp_device]: sidecar module with related enum/flag types\n* [`IUpnpDevice`][crate::engine::IUpnpDevice]: virtual methods\n\n\nSee also [Godot docs for `UPNPDevice`](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UpnpDevice {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`UpnpDevice`][crate::engine::UpnpDevice].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UPNPDevice` methods](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUpnpDevice: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl UpnpDevice {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn is_valid_gateway(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_valid_gateway", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn query_external_address(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "query_external_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_port_mapping_full(&self, port: i32, port_internal: i32, desc: GString, proto: GString, duration: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32, GString, GString, i32);
            let args = (port, port_internal, desc, proto, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9185usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9186usize);
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
        pub fn set_description_url(&mut self, url: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (url,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_description_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_description_url(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_description_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_service_type(&mut self, type_: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_service_type(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_control_url(&mut self, url: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (url,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_igd_control_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_control_url(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_igd_control_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_service_type(&mut self, type_: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_igd_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_service_type(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_igd_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_our_addr(&mut self, addr: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (addr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_igd_our_addr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_our_addr(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_igd_our_addr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_status(&mut self, status: crate::engine::upnp_device::IGDStatus,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::upnp_device::IGDStatus);
            let args = (status,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_igd_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_status(&self,) -> crate::engine::upnp_device::IGDStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::upnp_device::IGDStatus >;
            type CallSig = (crate::engine::upnp_device::IGDStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_igd_status", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for UpnpDevice {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"UPNPDevice\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UpnpDevice {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for UpnpDevice {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for UpnpDevice {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for UpnpDevice {
        
    }
    impl crate::obj::cap::GodotDefault for UpnpDevice {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UpnpDevice {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UpnpDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_UpnpDevice {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::UpnpDevice > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UpnpDevice::add_port_mapping_ex`][super::UpnpDevice::add_port_mapping_ex]."]
#[must_use]
pub struct ExAddPortMapping < 'a > {
    surround_object: &'a re_export::UpnpDevice, port: i32, port_internal: i32, desc: GString, proto: GString, duration: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPortMapping < 'a > {
    fn new(surround_object: &'a re_export::UpnpDevice, port: i32,) -> Self {
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
        re_export::UpnpDevice::add_port_mapping_full(self.surround_object, self.port, self.port_internal, self.desc, self.proto, self.duration,)
    }
}
#[doc = "Default-param extender for [`UpnpDevice::delete_port_mapping_ex`][super::UpnpDevice::delete_port_mapping_ex]."]
#[must_use]
pub struct ExDeletePortMapping < 'a > {
    surround_object: &'a re_export::UpnpDevice, port: i32, proto: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeletePortMapping < 'a > {
    fn new(surround_object: &'a re_export::UpnpDevice, port: i32,) -> Self {
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
        re_export::UpnpDevice::delete_port_mapping_full(self.surround_object, self.port, self.proto,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct IGDStatus {
    ord: i32
}
impl IGDStatus {
    pub const IGD_STATUS_OK: Self = Self {
        ord: 0i32
    };
    pub const IGD_STATUS_HTTP_ERROR: Self = Self {
        ord: 1i32
    };
    pub const IGD_STATUS_HTTP_EMPTY: Self = Self {
        ord: 2i32
    };
    pub const IGD_STATUS_NO_URLS: Self = Self {
        ord: 3i32
    };
    pub const IGD_STATUS_NO_IGD: Self = Self {
        ord: 4i32
    };
    pub const IGD_STATUS_DISCONNECTED: Self = Self {
        ord: 5i32
    };
    pub const IGD_STATUS_UNKNOWN_DEVICE: Self = Self {
        ord: 6i32
    };
    pub const IGD_STATUS_INVALID_CONTROL: Self = Self {
        ord: 7i32
    };
    pub const IGD_STATUS_MALLOC_ERROR: Self = Self {
        ord: 8i32
    };
    pub const IGD_STATUS_UNKNOWN_ERROR: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for IGDStatus {
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
impl crate::builtin::meta::GodotConvert for IGDStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for IGDStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for IGDStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}