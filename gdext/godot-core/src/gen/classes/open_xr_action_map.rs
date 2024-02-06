#![doc = "Sidecar module for class [`OpenXrActionMap`][crate::engine::OpenXrActionMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRActionMap` enums](https://docs.godotengine.org/en/stable/classes/class_openxractionmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRActionMap.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IOpenXrActionMap`][crate::engine::IOpenXrActionMap]: virtual methods\n\n\nSee also [Godot docs for `OpenXRActionMap`](https://docs.godotengine.org/en/stable/classes/class_openxractionmap.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrActionMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrActionMap`][crate::engine::OpenXrActionMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRActionMap` methods](https://docs.godotengine.org/en/stable/classes/class_openxractionmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrActionMap: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl OpenXrActionMap {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_action_sets(&mut self, action_sets: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (action_sets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_action_sets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_sets(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_sets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_set_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_set_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_action_set(&self, name: GString,) -> Option < Gd < crate::engine::OpenXrActionSet > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OpenXrActionSet > >;
            type CallSig = (Option < Gd < crate::engine::OpenXrActionSet > >, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_action_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_set(&self, idx: i32,) -> Option < Gd < crate::engine::OpenXrActionSet > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OpenXrActionSet > >;
            type CallSig = (Option < Gd < crate::engine::OpenXrActionSet > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_action_set(&mut self, action_set: Gd < crate::engine::OpenXrActionSet >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::OpenXrActionSet >);
            let args = (action_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_action_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_action_set(&mut self, action_set: Gd < crate::engine::OpenXrActionSet >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::OpenXrActionSet >);
            let args = (action_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_action_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interaction_profiles(&mut self, interaction_profiles: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (interaction_profiles,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interaction_profiles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interaction_profiles(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interaction_profiles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interaction_profile_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interaction_profile_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_interaction_profile(&self, name: GString,) -> Option < Gd < crate::engine::OpenXrInteractionProfile > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OpenXrInteractionProfile > >;
            type CallSig = (Option < Gd < crate::engine::OpenXrInteractionProfile > >, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_interaction_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interaction_profile(&self, idx: i32,) -> Option < Gd < crate::engine::OpenXrInteractionProfile > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OpenXrInteractionProfile > >;
            type CallSig = (Option < Gd < crate::engine::OpenXrInteractionProfile > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interaction_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_interaction_profile(&mut self, interaction_profile: Gd < crate::engine::OpenXrInteractionProfile >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::OpenXrInteractionProfile >);
            let args = (interaction_profile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_interaction_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_interaction_profile(&mut self, interaction_profile: Gd < crate::engine::OpenXrInteractionProfile >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::OpenXrInteractionProfile >);
            let args = (interaction_profile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_interaction_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_default_action_sets(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_default_action_sets", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrActionMap {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OpenXRActionMap\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrActionMap {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OpenXrActionMap {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for OpenXrActionMap {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for OpenXrActionMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for OpenXrActionMap {
        
    }
    impl crate::obj::ExportableObject for OpenXrActionMap {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrActionMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrActionMap {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrActionMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OpenXrActionMap {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OpenXrActionMap > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}