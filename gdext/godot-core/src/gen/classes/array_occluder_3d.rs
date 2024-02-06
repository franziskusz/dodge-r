#![doc = "Sidecar module for class [`ArrayOccluder3D`][crate::engine::ArrayOccluder3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayOccluder3D` enums](https://docs.godotengine.org/en/stable/classes/class_arrayoccluder3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ArrayOccluder3D.`\n\nInherits [`Occluder3D`][crate::engine::Occluder3D].\n\nRelated symbols:\n\n* [`IArrayOccluder3D`][crate::engine::IArrayOccluder3D]: virtual methods\n\n\nSee also [Godot docs for `ArrayOccluder3D`](https://docs.godotengine.org/en/stable/classes/class_arrayoccluder3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ArrayOccluder3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ArrayOccluder3D`][crate::engine::ArrayOccluder3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ArrayOccluder3D` methods](https://docs.godotengine.org/en/stable/classes/class_arrayoccluder3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArrayOccluder3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ArrayOccluder3D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_arrays(&mut self, vertices: PackedVector3Array, indices: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array, PackedInt32Array);
            let args = (vertices, indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertices(&mut self, vertices: PackedVector3Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array);
            let args = (vertices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indices(&mut self, indices: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedInt32Array);
            let args = (indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_indices", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ArrayOccluder3D {
        type Base = crate::engine::Occluder3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ArrayOccluder3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ArrayOccluder3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ArrayOccluder3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Occluder3D > for ArrayOccluder3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ArrayOccluder3D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ArrayOccluder3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ArrayOccluder3D {
        
    }
    impl crate::obj::ExportableObject for ArrayOccluder3D {
        
    }
    impl crate::obj::cap::GodotDefault for ArrayOccluder3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ArrayOccluder3D {
        type Target = crate::engine::Occluder3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ArrayOccluder3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ArrayOccluder3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ArrayOccluder3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Occluder3D > for $Class {
                
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