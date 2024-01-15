#![doc = "Sidecar module for class [`RandomNumberGenerator`][crate::engine::RandomNumberGenerator].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RandomNumberGenerator` enums](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RandomNumberGenerator.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`random_number_generator`][crate::engine::random_number_generator]: sidecar module with related enum/flag types\n* [`IRandomNumberGenerator`][crate::engine::IRandomNumberGenerator]: virtual methods\n\n\nSee also [Godot docs for `RandomNumberGenerator`](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RandomNumberGenerator {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RandomNumberGenerator`][crate::engine::RandomNumberGenerator].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RandomNumberGenerator` methods](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRandomNumberGenerator: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RandomNumberGenerator {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_seed(&mut self, seed: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u64);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seed(&mut self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_state(&mut self, state: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u64);
            let args = (state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randi(&mut self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randi", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randf(&mut self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randf", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn randfn_full(&mut self, mean: f32, deviation: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32, f32);
            let args = (mean, deviation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randfn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn randfn(&mut self,) -> f32 {
            self.randfn_ex() . done()
        }
        #[inline]
        pub fn randfn_ex(&mut self,) -> ExRandfn < '_ > {
            ExRandfn::new(self,)
        }
        pub fn randf_range(&mut self, from: f32, to: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32, f32);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randf_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randi_range(&mut self, from: i32, to: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randi_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randomize(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "randomize", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RandomNumberGenerator {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RandomNumberGenerator\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RandomNumberGenerator {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RandomNumberGenerator {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RandomNumberGenerator {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RandomNumberGenerator {
        
    }
    impl crate::obj::cap::GodotDefault for RandomNumberGenerator {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RandomNumberGenerator {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RandomNumberGenerator {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RandomNumberGenerator {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RandomNumberGenerator > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RandomNumberGenerator::randfn_ex`][super::RandomNumberGenerator::randfn_ex]."]
#[must_use]
pub struct ExRandfn < 'a > {
    surround_object: &'a mut re_export::RandomNumberGenerator, mean: f32, deviation: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRandfn < 'a > {
    fn new(surround_object: &'a mut re_export::RandomNumberGenerator,) -> Self {
        Self {
            surround_object, mean: 0f32, deviation: 1f32,
        }
    }
    #[inline]
    pub fn mean(self, value: f32) -> Self {
        Self {
            mean: value, .. self
        }
    }
    #[inline]
    pub fn deviation(self, value: f32) -> Self {
        Self {
            deviation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::RandomNumberGenerator::randfn_full(self.surround_object, self.mean, self.deviation,)
    }
}