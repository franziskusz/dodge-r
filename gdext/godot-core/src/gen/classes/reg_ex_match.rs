#![doc = "Sidecar module for class [`RegExMatch`][crate::engine::RegExMatch].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RegExMatch` enums](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RegExMatch.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`reg_ex_match`][crate::engine::reg_ex_match]: sidecar module with related enum/flag types\n* [`IRegExMatch`][crate::engine::IRegExMatch]: virtual methods\n\n\nSee also [Godot docs for `RegExMatch`](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RegExMatch {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RegExMatch`][crate::engine::RegExMatch].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RegExMatch` methods](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRegExMatch: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RegExMatch {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_subject(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subject", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_names(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_strings(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_strings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_full(&self, name: Variant,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Variant);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_string(&self,) -> GString {
            self.get_string_ex() . done()
        }
        #[inline]
        pub fn get_string_ex(&self,) -> ExGetString < '_ > {
            ExGetString::new(self,)
        }
        pub(crate) fn get_start_full(&self, name: Variant,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Variant);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_start(&self,) -> i32 {
            self.get_start_ex() . done()
        }
        #[inline]
        pub fn get_start_ex(&self,) -> ExGetStart < '_ > {
            ExGetStart::new(self,)
        }
        pub(crate) fn get_end_full(&self, name: Variant,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Variant);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_end(&self,) -> i32 {
            self.get_end_ex() . done()
        }
        #[inline]
        pub fn get_end_ex(&self,) -> ExGetEnd < '_ > {
            ExGetEnd::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for RegExMatch {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RegExMatch\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RegExMatch {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RegExMatch {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RegExMatch {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RegExMatch {
        
    }
    impl crate::obj::cap::GodotDefault for RegExMatch {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RegExMatch {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RegExMatch {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RegExMatch {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RegExMatch > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_string_ex`][super::RegExMatch::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'a > {
    surround_object: &'a re_export::RegExMatch, name: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetString < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        Self {
            surround_object, name: Variant::from(0),
        }
    }
    #[inline]
    pub fn name(self, value: Variant) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::RegExMatch::get_string_full(self.surround_object, self.name,)
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_start_ex`][super::RegExMatch::get_start_ex]."]
#[must_use]
pub struct ExGetStart < 'a > {
    surround_object: &'a re_export::RegExMatch, name: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStart < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        Self {
            surround_object, name: Variant::from(0),
        }
    }
    #[inline]
    pub fn name(self, value: Variant) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::RegExMatch::get_start_full(self.surround_object, self.name,)
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_end_ex`][super::RegExMatch::get_end_ex]."]
#[must_use]
pub struct ExGetEnd < 'a > {
    surround_object: &'a re_export::RegExMatch, name: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetEnd < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        Self {
            surround_object, name: Variant::from(0),
        }
    }
    #[inline]
    pub fn name(self, value: Variant) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::RegExMatch::get_end_full(self.surround_object, self.name,)
    }
}