#![doc = "Sidecar module for class [`NavigationMeshSourceGeometryData2D`][crate::engine::NavigationMeshSourceGeometryData2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NavigationMeshSourceGeometryData2D` enums](https://docs.godotengine.org/en/stable/classes/class_navigationmeshsourcegeometrydata2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NavigationMeshSourceGeometryData2D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`INavigationMeshSourceGeometryData2D`][crate::engine::INavigationMeshSourceGeometryData2D]: virtual methods\n\n\nSee also [Godot docs for `NavigationMeshSourceGeometryData2D`](https://docs.godotengine.org/en/stable/classes/class_navigationmeshsourcegeometrydata2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NavigationMeshSourceGeometryData2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`NavigationMeshSourceGeometryData2D`][crate::engine::NavigationMeshSourceGeometryData2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `NavigationMeshSourceGeometryData2D` methods](https://docs.godotengine.org/en/stable/classes/class_navigationmeshsourcegeometrydata2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INavigationMeshSourceGeometryData2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl NavigationMeshSourceGeometryData2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_data(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_traversable_outlines(&mut self, traversable_outlines: Array < PackedVector2Array >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < PackedVector2Array >);
            let args = (traversable_outlines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_traversable_outlines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_traversable_outlines(&self,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_traversable_outlines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_obstruction_outlines(&mut self, obstruction_outlines: Array < PackedVector2Array >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < PackedVector2Array >);
            let args = (obstruction_outlines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_obstruction_outlines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_obstruction_outlines(&self,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_obstruction_outlines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_traversable_outline(&mut self, shape_outline: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array);
            let args = (shape_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_traversable_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_obstruction_outline(&mut self, shape_outline: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array);
            let args = (shape_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_obstruction_outline", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for NavigationMeshSourceGeometryData2D {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"NavigationMeshSourceGeometryData2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NavigationMeshSourceGeometryData2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for NavigationMeshSourceGeometryData2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for NavigationMeshSourceGeometryData2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for NavigationMeshSourceGeometryData2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for NavigationMeshSourceGeometryData2D {
        
    }
    impl crate::obj::ExportableObject for NavigationMeshSourceGeometryData2D {
        
    }
    impl crate::obj::cap::GodotDefault for NavigationMeshSourceGeometryData2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for NavigationMeshSourceGeometryData2D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NavigationMeshSourceGeometryData2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_NavigationMeshSourceGeometryData2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::NavigationMeshSourceGeometryData2D > for $Class {
                
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