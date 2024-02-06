#![doc = "Sidecar module for class [`Time`][crate::engine::Time].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Time` enums](https://docs.godotengine.org/en/stable/classes/class_time.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Time.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`time`][crate::engine::time]: sidecar module with related enum/flag types\n* [`ITime`][crate::engine::ITime]: virtual methods\n\n\nSee also [Godot docs for `Time`](https://docs.godotengine.org/en/stable/classes/class_time.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Time {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Time`][crate::engine::Time].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Time` methods](https://docs.godotengine.org/en/stable/classes/class_time.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITime: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Time {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Time\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_datetime_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_date_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_date_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_datetime_string_from_unix_time_full(&self, unix_time_val: i64, use_space: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i64, bool);
            let args = (unix_time_val, use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_datetime_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            self.get_datetime_string_from_unix_time_ex(unix_time_val,) . done()
        }
        #[inline]
        pub fn get_datetime_string_from_unix_time_ex(&self, unix_time_val: i64,) -> ExGetDatetimeStringFromUnixTime < '_ > {
            ExGetDatetimeStringFromUnixTime::new(self, unix_time_val,)
        }
        pub fn get_date_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_date_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_datetime_dict_from_datetime_string(&self, datetime: GString, weekday: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, bool);
            let args = (datetime, weekday,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_dict_from_datetime_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_datetime_string_from_datetime_dict(&self, datetime: Dictionary, use_space: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Dictionary, bool);
            let args = (datetime, use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_string_from_datetime_dict", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_datetime_dict(&self, datetime: Dictionary,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Dictionary);
            let args = (datetime,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unix_time_from_datetime_dict", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_datetime_string(&self, datetime: GString,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, GString);
            let args = (datetime,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unix_time_from_datetime_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset_string_from_offset_minutes(&self, offset_minutes: i64,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i64);
            let args = (offset_minutes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset_string_from_offset_minutes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_datetime_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_datetime_dict_from_system(&self,) -> Dictionary {
            self.get_datetime_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_dict_from_system_ex(&self,) -> ExGetDatetimeDictFromSystem < '_ > {
            ExGetDatetimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_date_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_date_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_date_dict_from_system(&self,) -> Dictionary {
            self.get_date_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_dict_from_system_ex(&self,) -> ExGetDateDictFromSystem < '_ > {
            ExGetDateDictFromSystem::new(self,)
        }
        pub(crate) fn get_time_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_time_dict_from_system(&self,) -> Dictionary {
            self.get_time_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_dict_from_system_ex(&self,) -> ExGetTimeDictFromSystem < '_ > {
            ExGetTimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_datetime_string_from_system_full(&self, utc: bool, use_space: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool, bool);
            let args = (utc, use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_datetime_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_datetime_string_from_system(&self,) -> GString {
            self.get_datetime_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_string_from_system_ex(&self,) -> ExGetDatetimeStringFromSystem < '_ > {
            ExGetDatetimeStringFromSystem::new(self,)
        }
        pub(crate) fn get_date_string_from_system_full(&self, utc: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_date_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_date_string_from_system(&self,) -> GString {
            self.get_date_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_string_from_system_ex(&self,) -> ExGetDateStringFromSystem < '_ > {
            ExGetDateStringFromSystem::new(self,)
        }
        pub(crate) fn get_time_string_from_system_full(&self, utc: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_time_string_from_system(&self,) -> GString {
            self.get_time_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_string_from_system_ex(&self,) -> ExGetTimeStringFromSystem < '_ > {
            ExGetTimeStringFromSystem::new(self,)
        }
        pub fn get_time_zone_from_system(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_zone_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_system(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unix_time_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks_msec(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ticks_msec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks_usec(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ticks_usec", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Time {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Time\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Time {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Time {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Time {
        
    }
    impl std::ops::Deref for Time {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Time {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Time {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Time > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_unix_time_ex`][super::Time::get_datetime_string_from_unix_time_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromUnixTime < 'a > {
    surround_object: &'a re_export::Time, unix_time_val: i64, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromUnixTime < 'a > {
    fn new(surround_object: &'a re_export::Time, unix_time_val: i64,) -> Self {
        Self {
            surround_object, unix_time_val, use_space: false,
        }
    }
    #[inline]
    pub fn use_space(self, value: bool) -> Self {
        Self {
            use_space: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Time::get_datetime_string_from_unix_time_full(self.surround_object, self.unix_time_val, self.use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_dict_from_system_ex`][super::Time::get_datetime_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeDictFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        re_export::Time::get_datetime_dict_from_system_full(self.surround_object, self.utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_dict_from_system_ex`][super::Time::get_date_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDateDictFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        re_export::Time::get_date_dict_from_system_full(self.surround_object, self.utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_dict_from_system_ex`][super::Time::get_time_dict_from_system_ex]."]
#[must_use]
pub struct ExGetTimeDictFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        re_export::Time::get_time_dict_from_system_full(self.surround_object, self.utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_system_ex`][super::Time::get_datetime_string_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false, use_space: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn use_space(self, value: bool) -> Self {
        Self {
            use_space: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Time::get_datetime_string_from_system_full(self.surround_object, self.utc, self.use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_string_from_system_ex`][super::Time::get_date_string_from_system_ex]."]
#[must_use]
pub struct ExGetDateStringFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Time::get_date_string_from_system_full(self.surround_object, self.utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_string_from_system_ex`][super::Time::get_time_string_from_system_ex]."]
#[must_use]
pub struct ExGetTimeStringFromSystem < 'a > {
    surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        Self {
            surround_object, utc: false,
        }
    }
    #[inline]
    pub fn utc(self, value: bool) -> Self {
        Self {
            utc: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Time::get_time_string_from_system_full(self.surround_object, self.utc,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Month {
    ord: i32
}
impl Month {
    pub const MONTH_JANUARY: Self = Self {
        ord: 1i32
    };
    pub const MONTH_FEBRUARY: Self = Self {
        ord: 2i32
    };
    pub const MONTH_MARCH: Self = Self {
        ord: 3i32
    };
    pub const MONTH_APRIL: Self = Self {
        ord: 4i32
    };
    pub const MONTH_MAY: Self = Self {
        ord: 5i32
    };
    pub const MONTH_JUNE: Self = Self {
        ord: 6i32
    };
    pub const MONTH_JULY: Self = Self {
        ord: 7i32
    };
    pub const MONTH_AUGUST: Self = Self {
        ord: 8i32
    };
    pub const MONTH_SEPTEMBER: Self = Self {
        ord: 9i32
    };
    pub const MONTH_OCTOBER: Self = Self {
        ord: 10i32
    };
    pub const MONTH_NOVEMBER: Self = Self {
        ord: 11i32
    };
    pub const MONTH_DECEMBER: Self = Self {
        ord: 12i32
    };
    
}
impl crate::obj::EngineEnum for Month {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Month {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Month {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Month {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Weekday {
    ord: i32
}
impl Weekday {
    pub const WEEKDAY_SUNDAY: Self = Self {
        ord: 0i32
    };
    pub const WEEKDAY_MONDAY: Self = Self {
        ord: 1i32
    };
    pub const WEEKDAY_TUESDAY: Self = Self {
        ord: 2i32
    };
    pub const WEEKDAY_WEDNESDAY: Self = Self {
        ord: 3i32
    };
    pub const WEEKDAY_THURSDAY: Self = Self {
        ord: 4i32
    };
    pub const WEEKDAY_FRIDAY: Self = Self {
        ord: 5i32
    };
    pub const WEEKDAY_SATURDAY: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for Weekday {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Weekday {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Weekday {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Weekday {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}