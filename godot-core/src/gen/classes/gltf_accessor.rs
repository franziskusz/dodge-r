#![doc = "Sidecar module for class [`GltfAccessor`][crate::engine::GltfAccessor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFAccessor` enums](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFAccessor.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IGltfAccessor`][crate::engine::IGltfAccessor]: virtual methods\n\n\nSee also [Godot docs for `GLTFAccessor`](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfAccessor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfAccessor`][crate::engine::GltfAccessor].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFAccessor` methods](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfAccessor: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfAccessor {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_buffer_view(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer_view(&mut self, buffer_view: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_byte_offset(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_byte_offset(&mut self, byte_offset: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_component_type(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_component_type(&mut self, component_type: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normalized(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normalized(&mut self, normalized: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (normalized,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_type(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_type(&mut self, type_: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&mut self,) -> PackedFloat64Array {
            type RetMarshal = PtrcallReturnT < PackedFloat64Array >;
            type CallSig = (PackedFloat64Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, min: PackedFloat64Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat64Array);
            let args = (min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&mut self,) -> PackedFloat64Array {
            type RetMarshal = PtrcallReturnT < PackedFloat64Array >;
            type CallSig = (PackedFloat64Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, max: PackedFloat64Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat64Array);
            let args = (max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_count(&mut self, sparse_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_buffer_view(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_indices_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_buffer_view(&mut self, sparse_indices_buffer_view: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_indices_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_indices_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_byte_offset(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_indices_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_byte_offset(&mut self, sparse_indices_byte_offset: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_indices_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_indices_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_component_type(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_indices_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_component_type(&mut self, sparse_indices_component_type: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_indices_component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_indices_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_values_buffer_view(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_values_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_values_buffer_view(&mut self, sparse_values_buffer_view: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_values_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_values_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_values_byte_offset(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sparse_values_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_values_byte_offset(&mut self, sparse_values_byte_offset: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (sparse_values_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sparse_values_byte_offset", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfAccessor {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFAccessor\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfAccessor {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfAccessor {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfAccessor {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfAccessor {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfAccessor {
        
    }
    impl crate::obj::ExportableObject for GltfAccessor {
        
    }
    impl crate::obj::cap::GodotDefault for GltfAccessor {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfAccessor {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfAccessor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfAccessor {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfAccessor > for $Class {
                
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