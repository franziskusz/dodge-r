#![doc = "Sidecar module for class [`Marshalls`][crate::engine::Marshalls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Marshalls` enums](https://docs.godotengine.org/en/stable/classes/class_marshalls.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Marshalls.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`marshalls`][crate::engine::marshalls]: sidecar module with related enum/flag types\n* [`IMarshalls`][crate::engine::IMarshalls]: virtual methods\n\n\nSee also [Godot docs for `Marshalls`](https://docs.godotengine.org/en/stable/classes/class_marshalls.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Marshalls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Marshalls`][crate::engine::Marshalls].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Marshalls` methods](https://docs.godotengine.org/en/stable/classes/class_marshalls.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMarshalls: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Marshalls {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Marshalls\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn variant_to_base64_full(&mut self, variant: Variant, full_objects: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Variant, bool);
            let args = (variant, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "variant_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn variant_to_base64(&mut self, variant: Variant,) -> GString {
            self.variant_to_base64_ex(variant,) . done()
        }
        #[inline]
        pub fn variant_to_base64_ex(&mut self, variant: Variant,) -> ExVariantToBase64 < '_ > {
            ExVariantToBase64::new(self, variant,)
        }
        pub(crate) fn base64_to_variant_full(&mut self, base64_str: GString, allow_objects: bool,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString, bool);
            let args = (base64_str, allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "base64_to_variant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn base64_to_variant(&mut self, base64_str: GString,) -> Variant {
            self.base64_to_variant_ex(base64_str,) . done()
        }
        #[inline]
        pub fn base64_to_variant_ex(&mut self, base64_str: GString,) -> ExBase64ToVariant < '_ > {
            ExBase64ToVariant::new(self, base64_str,)
        }
        pub fn raw_to_base64(&mut self, array: PackedByteArray,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, PackedByteArray);
            let args = (array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "raw_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn base64_to_raw(&mut self, base64_str: GString,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, GString);
            let args = (base64_str,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "base64_to_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn utf8_to_base64(&mut self, utf8_str: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (utf8_str,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "utf8_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn base64_to_utf8(&mut self, base64_str: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (base64_str,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "base64_to_utf8", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Marshalls {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Marshalls\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Marshalls {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Marshalls {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Marshalls {
        
    }
    impl std::ops::Deref for Marshalls {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Marshalls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Marshalls {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Marshalls > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Marshalls::variant_to_base64_ex`][super::Marshalls::variant_to_base64_ex]."]
#[must_use]
pub struct ExVariantToBase64 < 'a > {
    surround_object: &'a mut re_export::Marshalls, variant: Variant, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVariantToBase64 < 'a > {
    fn new(surround_object: &'a mut re_export::Marshalls, variant: Variant,) -> Self {
        Self {
            surround_object, variant, full_objects: false,
        }
    }
    #[inline]
    pub fn full_objects(self, value: bool) -> Self {
        Self {
            full_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Marshalls::variant_to_base64_full(self.surround_object, self.variant, self.full_objects,)
    }
}
#[doc = "Default-param extender for [`Marshalls::base64_to_variant_ex`][super::Marshalls::base64_to_variant_ex]."]
#[must_use]
pub struct ExBase64ToVariant < 'a > {
    surround_object: &'a mut re_export::Marshalls, base64_str: GString, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBase64ToVariant < 'a > {
    fn new(surround_object: &'a mut re_export::Marshalls, base64_str: GString,) -> Self {
        Self {
            surround_object, base64_str, allow_objects: false,
        }
    }
    #[inline]
    pub fn allow_objects(self, value: bool) -> Self {
        Self {
            allow_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::Marshalls::base64_to_variant_full(self.surround_object, self.base64_str, self.allow_objects,)
    }
}