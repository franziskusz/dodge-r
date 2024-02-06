#![doc = "Sidecar module for class [`RegEx`][crate::engine::RegEx].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RegEx` enums](https://docs.godotengine.org/en/stable/classes/class_regex.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RegEx.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`reg_ex`][crate::engine::reg_ex]: sidecar module with related enum/flag types\n* [`IRegEx`][crate::engine::IRegEx]: virtual methods\n\n\nSee also [Godot docs for `RegEx`](https://docs.godotengine.org/en/stable/classes/class_regex.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RegEx {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RegEx`][crate::engine::RegEx].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RegEx` methods](https://docs.godotengine.org/en/stable/classes/class_regex.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRegEx: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RegEx {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn create_from_string(pattern: GString,) -> Option < Gd < crate::engine::RegEx > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RegEx > >;
            type CallSig = (Option < Gd < crate::engine::RegEx > >, GString);
            let args = (pattern,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_string", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compile(&mut self, pattern: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (pattern,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn search_full(&self, subject: GString, offset: i32, end: i32,) -> Option < Gd < crate::engine::RegExMatch > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RegExMatch > >;
            type CallSig = (Option < Gd < crate::engine::RegExMatch > >, GString, i32, i32);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn search(&self, subject: GString,) -> Option < Gd < crate::engine::RegExMatch > > {
            self.search_ex(subject,) . done()
        }
        #[inline]
        pub fn search_ex(&self, subject: GString,) -> ExSearch < '_ > {
            ExSearch::new(self, subject,)
        }
        pub(crate) fn search_all_full(&self, subject: GString, offset: i32, end: i32,) -> Array < Gd < crate::engine::RegExMatch > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::RegExMatch > > >;
            type CallSig = (Array < Gd < crate::engine::RegExMatch > >, GString, i32, i32);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "search_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn search_all(&self, subject: GString,) -> Array < Gd < crate::engine::RegExMatch > > {
            self.search_all_ex(subject,) . done()
        }
        #[inline]
        pub fn search_all_ex(&self, subject: GString,) -> ExSearchAll < '_ > {
            ExSearchAll::new(self, subject,)
        }
        pub(crate) fn sub_full(&self, subject: GString, replacement: GString, all: bool, offset: i32, end: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, GString, bool, i32, i32);
            let args = (subject, replacement, all, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sub", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn sub(&self, subject: GString, replacement: GString,) -> GString {
            self.sub_ex(subject, replacement,) . done()
        }
        #[inline]
        pub fn sub_ex(&self, subject: GString, replacement: GString,) -> ExSub < '_ > {
            ExSub::new(self, subject, replacement,)
        }
        pub fn is_valid(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pattern(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_names(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_names", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RegEx {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RegEx\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RegEx {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RegEx {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RegEx {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RegEx {
        
    }
    impl crate::obj::cap::GodotDefault for RegEx {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RegEx {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RegEx {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RegEx {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RegEx > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RegEx::search_ex`][super::RegEx::search_ex]."]
#[must_use]
pub struct ExSearch < 'a > {
    surround_object: &'a re_export::RegEx, subject: GString, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearch < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: GString,) -> Self {
        Self {
            surround_object, subject, offset: 0i32, end: - 1i32,
        }
    }
    #[inline]
    pub fn offset(self, value: i32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn end(self, value: i32) -> Self {
        Self {
            end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::RegExMatch > > {
        re_export::RegEx::search_full(self.surround_object, self.subject, self.offset, self.end,)
    }
}
#[doc = "Default-param extender for [`RegEx::search_all_ex`][super::RegEx::search_all_ex]."]
#[must_use]
pub struct ExSearchAll < 'a > {
    surround_object: &'a re_export::RegEx, subject: GString, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearchAll < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: GString,) -> Self {
        Self {
            surround_object, subject, offset: 0i32, end: - 1i32,
        }
    }
    #[inline]
    pub fn offset(self, value: i32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn end(self, value: i32) -> Self {
        Self {
            end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::engine::RegExMatch > > {
        re_export::RegEx::search_all_full(self.surround_object, self.subject, self.offset, self.end,)
    }
}
#[doc = "Default-param extender for [`RegEx::sub_ex`][super::RegEx::sub_ex]."]
#[must_use]
pub struct ExSub < 'a > {
    surround_object: &'a re_export::RegEx, subject: GString, replacement: GString, all: bool, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSub < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: GString, replacement: GString,) -> Self {
        Self {
            surround_object, subject, replacement, all: false, offset: 0i32, end: - 1i32,
        }
    }
    #[inline]
    pub fn all(self, value: bool) -> Self {
        Self {
            all: value, .. self
        }
    }
    #[inline]
    pub fn offset(self, value: i32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn end(self, value: i32) -> Self {
        Self {
            end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::RegEx::sub_full(self.surround_object, self.subject, self.replacement, self.all, self.offset, self.end,)
    }
}