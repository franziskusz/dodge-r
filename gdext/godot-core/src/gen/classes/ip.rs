#![doc = "Sidecar module for class [`Ip`][crate::engine::Ip].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `IP` enums](https://docs.godotengine.org/en/stable/classes/class_ip.html#enumerations).\n\n"]
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
    #[doc = "Godot class `IP.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`ip`][crate::engine::ip]: sidecar module with related enum/flag types\n* [`IIp`][crate::engine::IIp]: virtual methods\n\n\nSee also [Godot docs for `IP`](https://docs.godotengine.org/en/stable/classes/class_ip.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ip {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Ip`][crate::engine::Ip].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `IP` methods](https://docs.godotengine.org/en/stable/classes/class_ip.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IIp: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Ip {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"IP\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn resolve_hostname_full(&mut self, host: GString, ip_type: crate::engine::ip::Type,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, crate::engine::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resolve_hostname", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resolve_hostname(&mut self, host: GString,) -> GString {
            self.resolve_hostname_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_ex(&mut self, host: GString,) -> ExResolveHostname < '_ > {
            ExResolveHostname::new(self, host,)
        }
        pub(crate) fn resolve_hostname_addresses_full(&mut self, host: GString, ip_type: crate::engine::ip::Type,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString, crate::engine::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resolve_hostname_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resolve_hostname_addresses(&mut self, host: GString,) -> PackedStringArray {
            self.resolve_hostname_addresses_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_addresses_ex(&mut self, host: GString,) -> ExResolveHostnameAddresses < '_ > {
            ExResolveHostnameAddresses::new(self, host,)
        }
        pub(crate) fn resolve_hostname_queue_item_full(&mut self, host: GString, ip_type: crate::engine::ip::Type,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, crate::engine::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resolve_hostname_queue_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resolve_hostname_queue_item(&mut self, host: GString,) -> i32 {
            self.resolve_hostname_queue_item_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_queue_item_ex(&mut self, host: GString,) -> ExResolveHostnameQueueItem < '_ > {
            ExResolveHostnameQueueItem::new(self, host,)
        }
        pub fn get_resolve_item_status(&self, id: i32,) -> crate::engine::ip::ResolverStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::ip::ResolverStatus >;
            type CallSig = (crate::engine::ip::ResolverStatus, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resolve_item_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolve_item_address(&self, id: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resolve_item_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolve_item_addresses(&self, id: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resolve_item_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_resolve_item(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_resolve_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_addresses(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_interfaces(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_interfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn clear_cache_full(&mut self, hostname: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (hostname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn clear_cache(&mut self,) {
            self.clear_cache_ex() . done()
        }
        #[inline]
        pub fn clear_cache_ex(&mut self,) -> ExClearCache < '_ > {
            ExClearCache::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const RESOLVER_MAX_QUERIES: i32 = 256i32;
        pub const RESOLVER_INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for Ip {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"IP\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Ip {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Ip {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Ip {
        
    }
    impl std::ops::Deref for Ip {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Ip {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Ip {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Ip > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_ex`][super::Ip::resolve_hostname_ex]."]
#[must_use]
pub struct ExResolveHostname < 'a > {
    surround_object: &'a mut re_export::Ip, host: GString, ip_type: crate::engine::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostname < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: GString,) -> Self {
        Self {
            surround_object, host, ip_type: crate::obj::EngineEnum::from_ord(3),
        }
    }
    #[inline]
    pub fn ip_type(self, value: crate::engine::ip::Type) -> Self {
        Self {
            ip_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Ip::resolve_hostname_full(self.surround_object, self.host, self.ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_addresses_ex`][super::Ip::resolve_hostname_addresses_ex]."]
#[must_use]
pub struct ExResolveHostnameAddresses < 'a > {
    surround_object: &'a mut re_export::Ip, host: GString, ip_type: crate::engine::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameAddresses < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: GString,) -> Self {
        Self {
            surround_object, host, ip_type: crate::obj::EngineEnum::from_ord(3),
        }
    }
    #[inline]
    pub fn ip_type(self, value: crate::engine::ip::Type) -> Self {
        Self {
            ip_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::Ip::resolve_hostname_addresses_full(self.surround_object, self.host, self.ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_queue_item_ex`][super::Ip::resolve_hostname_queue_item_ex]."]
#[must_use]
pub struct ExResolveHostnameQueueItem < 'a > {
    surround_object: &'a mut re_export::Ip, host: GString, ip_type: crate::engine::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameQueueItem < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: GString,) -> Self {
        Self {
            surround_object, host, ip_type: crate::obj::EngineEnum::from_ord(3),
        }
    }
    #[inline]
    pub fn ip_type(self, value: crate::engine::ip::Type) -> Self {
        Self {
            ip_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Ip::resolve_hostname_queue_item_full(self.surround_object, self.host, self.ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::clear_cache_ex`][super::Ip::clear_cache_ex]."]
#[must_use]
pub struct ExClearCache < 'a > {
    surround_object: &'a mut re_export::Ip, hostname: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearCache < 'a > {
    fn new(surround_object: &'a mut re_export::Ip,) -> Self {
        Self {
            surround_object, hostname: GString::from(""),
        }
    }
    #[inline]
    pub fn hostname(self, value: GString) -> Self {
        Self {
            hostname: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Ip::clear_cache_full(self.surround_object, self.hostname,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ResolverStatus {
    ord: i32
}
impl ResolverStatus {
    pub const RESOLVER_STATUS_NONE: Self = Self {
        ord: 0i32
    };
    pub const RESOLVER_STATUS_WAITING: Self = Self {
        ord: 1i32
    };
    pub const RESOLVER_STATUS_DONE: Self = Self {
        ord: 2i32
    };
    pub const RESOLVER_STATUS_ERROR: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ResolverStatus {
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
impl crate::builtin::meta::GodotConvert for ResolverStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ResolverStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ResolverStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Type {
    ord: i32
}
impl Type {
    pub const TYPE_NONE: Self = Self {
        ord: 0i32
    };
    pub const TYPE_IPV4: Self = Self {
        ord: 1i32
    };
    pub const TYPE_IPV6: Self = Self {
        ord: 2i32
    };
    pub const TYPE_ANY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for Type {
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
impl crate::builtin::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Type {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}