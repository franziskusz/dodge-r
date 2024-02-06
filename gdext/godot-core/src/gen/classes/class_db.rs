#![doc = "Sidecar module for class [`ClassDb`][crate::engine::ClassDb].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ClassDB` enums](https://docs.godotengine.org/en/stable/classes/class_classdb.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ClassDB.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`class_db`][crate::engine::class_db]: sidecar module with related enum/flag types\n* [`IClassDb`][crate::engine::IClassDb]: virtual methods\n\n\nSee also [Godot docs for `ClassDB`](https://docs.godotengine.org/en/stable/classes/class_classdb.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ClassDb {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ClassDb`][crate::engine::ClassDb].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ClassDB` methods](https://docs.godotengine.org/en/stable/classes/class_classdb.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IClassDb: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ClassDb {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"ClassDB\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_class_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_class_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inheriters_from_class(&self, class: StringName,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inheriters_from_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_class(&self, class: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_exists(&self, class: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_parent_class(&self, class: StringName, inherits: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (class, inherits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_parent_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_instantiate(&self, class: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instantiate(&self, class: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_has_signal(&self, class: StringName, signal: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (class, signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_has_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_get_signal(&self, class: StringName, signal: StringName,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, StringName, StringName);
            let args = (class, signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_get_signal_list_full(&self, class: StringName, no_inheritance: bool,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, StringName, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_signal_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_signal_list(&self, class: StringName,) -> Array < Dictionary > {
            self.class_get_signal_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_signal_list_ex(&self, class: StringName,) -> ExClassGetSignalList < '_ > {
            ExClassGetSignalList::new(self, class,)
        }
        pub(crate) fn class_get_property_list_full(&self, class: StringName, no_inheritance: bool,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, StringName, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_property_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_property_list(&self, class: StringName,) -> Array < Dictionary > {
            self.class_get_property_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_property_list_ex(&self, class: StringName,) -> ExClassGetPropertyList < '_ > {
            ExClassGetPropertyList::new(self, class,)
        }
        pub fn class_get_property(&self, object: Gd < crate::engine::Object >, property: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Gd < crate::engine::Object >, StringName);
            let args = (object, property,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_set_property(&self, object: Gd < crate::engine::Object >, property: StringName, value: Variant,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Object >, StringName, Variant);
            let args = (object, property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_set_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_has_method_full(&self, class: StringName, method: StringName, no_inheritance: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName, bool);
            let args = (class, method, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_has_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_has_method(&self, class: StringName, method: StringName,) -> bool {
            self.class_has_method_ex(class, method,) . done()
        }
        #[inline]
        pub fn class_has_method_ex(&self, class: StringName, method: StringName,) -> ExClassHasMethod < '_ > {
            ExClassHasMethod::new(self, class, method,)
        }
        pub(crate) fn class_get_method_list_full(&self, class: StringName, no_inheritance: bool,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, StringName, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_method_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_method_list(&self, class: StringName,) -> Array < Dictionary > {
            self.class_get_method_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_method_list_ex(&self, class: StringName,) -> ExClassGetMethodList < '_ > {
            ExClassGetMethodList::new(self, class,)
        }
        pub(crate) fn class_get_integer_constant_list_full(&self, class: StringName, no_inheritance: bool,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, StringName, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_integer_constant_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_integer_constant_list(&self, class: StringName,) -> PackedStringArray {
            self.class_get_integer_constant_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_list_ex(&self, class: StringName,) -> ExClassGetIntegerConstantList < '_ > {
            ExClassGetIntegerConstantList::new(self, class,)
        }
        pub fn class_has_integer_constant(&self, class: StringName, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (class, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_has_integer_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_get_integer_constant(&self, class: StringName, name: StringName,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, StringName, StringName);
            let args = (class, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_integer_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_has_enum_full(&self, class: StringName, name: StringName, no_inheritance: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName, bool);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_has_enum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_has_enum(&self, class: StringName, name: StringName,) -> bool {
            self.class_has_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_has_enum_ex(&self, class: StringName, name: StringName,) -> ExClassHasEnum < '_ > {
            ExClassHasEnum::new(self, class, name,)
        }
        pub(crate) fn class_get_enum_list_full(&self, class: StringName, no_inheritance: bool,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, StringName, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_enum_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_enum_list(&self, class: StringName,) -> PackedStringArray {
            self.class_get_enum_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_enum_list_ex(&self, class: StringName,) -> ExClassGetEnumList < '_ > {
            ExClassGetEnumList::new(self, class,)
        }
        pub(crate) fn class_get_enum_constants_full(&self, class: StringName, enum_: StringName, no_inheritance: bool,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, StringName, StringName, bool);
            let args = (class, enum_, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_enum_constants", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_enum_constants(&self, class: StringName, enum_: StringName,) -> PackedStringArray {
            self.class_get_enum_constants_ex(class, enum_,) . done()
        }
        #[inline]
        pub fn class_get_enum_constants_ex(&self, class: StringName, enum_: StringName,) -> ExClassGetEnumConstants < '_ > {
            ExClassGetEnumConstants::new(self, class, enum_,)
        }
        pub(crate) fn class_get_integer_constant_enum_full(&self, class: StringName, name: StringName, no_inheritance: bool,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName, StringName, bool);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "class_get_integer_constant_enum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn class_get_integer_constant_enum(&self, class: StringName, name: StringName,) -> StringName {
            self.class_get_integer_constant_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_enum_ex(&self, class: StringName, name: StringName,) -> ExClassGetIntegerConstantEnum < '_ > {
            ExClassGetIntegerConstantEnum::new(self, class, name,)
        }
        pub fn is_class_enabled(&self, class: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_class_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ClassDb {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ClassDB\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ClassDb {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ClassDb {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for ClassDb {
        
    }
    impl std::ops::Deref for ClassDb {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ClassDb {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ClassDb {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ClassDb > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_signal_list_ex`][super::ClassDb::class_get_signal_list_ex]."]
#[must_use]
pub struct ExClassGetSignalList < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetSignalList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName,) -> Self {
        Self {
            surround_object, class, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        re_export::ClassDb::class_get_signal_list_full(self.surround_object, self.class, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_property_list_ex`][super::ClassDb::class_get_property_list_ex]."]
#[must_use]
pub struct ExClassGetPropertyList < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetPropertyList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName,) -> Self {
        Self {
            surround_object, class, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        re_export::ClassDb::class_get_property_list_full(self.surround_object, self.class, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_method_ex`][super::ClassDb::class_has_method_ex]."]
#[must_use]
pub struct ExClassHasMethod < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, method: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasMethod < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName, method: StringName,) -> Self {
        Self {
            surround_object, class, method, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::ClassDb::class_has_method_full(self.surround_object, self.class, self.method, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_method_list_ex`][super::ClassDb::class_get_method_list_ex]."]
#[must_use]
pub struct ExClassGetMethodList < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetMethodList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName,) -> Self {
        Self {
            surround_object, class, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        re_export::ClassDb::class_get_method_list_full(self.surround_object, self.class, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_list_ex`][super::ClassDb::class_get_integer_constant_list_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantList < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName,) -> Self {
        Self {
            surround_object, class, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::ClassDb::class_get_integer_constant_list_full(self.surround_object, self.class, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_enum_ex`][super::ClassDb::class_has_enum_ex]."]
#[must_use]
pub struct ExClassHasEnum < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, name: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName, name: StringName,) -> Self {
        Self {
            surround_object, class, name, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::ClassDb::class_has_enum_full(self.surround_object, self.class, self.name, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_list_ex`][super::ClassDb::class_get_enum_list_ex]."]
#[must_use]
pub struct ExClassGetEnumList < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName,) -> Self {
        Self {
            surround_object, class, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::ClassDb::class_get_enum_list_full(self.surround_object, self.class, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_constants_ex`][super::ClassDb::class_get_enum_constants_ex]."]
#[must_use]
pub struct ExClassGetEnumConstants < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, enum_: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumConstants < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName, enum_: StringName,) -> Self {
        Self {
            surround_object, class, enum_, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::ClassDb::class_get_enum_constants_full(self.surround_object, self.class, self.enum_, self.no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_enum_ex`][super::ClassDb::class_get_integer_constant_enum_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantEnum < 'a > {
    surround_object: &'a re_export::ClassDb, class: StringName, name: StringName, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: StringName, name: StringName,) -> Self {
        Self {
            surround_object, class, name, no_inheritance: false,
        }
    }
    #[inline]
    pub fn no_inheritance(self, value: bool) -> Self {
        Self {
            no_inheritance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        re_export::ClassDb::class_get_integer_constant_enum_full(self.surround_object, self.class, self.name, self.no_inheritance,)
    }
}