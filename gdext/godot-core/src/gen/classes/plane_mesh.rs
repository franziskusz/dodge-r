#![doc = "Sidecar module for class [`PlaneMesh`][crate::engine::PlaneMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PlaneMesh` enums](https://docs.godotengine.org/en/stable/classes/class_planemesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PlaneMesh.`\n\nInherits [`PrimitiveMesh`][crate::engine::PrimitiveMesh].\n\nRelated symbols:\n\n* [`plane_mesh`][crate::engine::plane_mesh]: sidecar module with related enum/flag types\n* [`IPlaneMesh`][crate::engine::IPlaneMesh]: virtual methods\n\n\nSee also [Godot docs for `PlaneMesh`](https://docs.godotengine.org/en/stable/classes/class_planemesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PlaneMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PlaneMesh`][crate::engine::PlaneMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PlaneMesh` methods](https://docs.godotengine.org/en/stable/classes/class_planemesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPlaneMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_index_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_arrays(&self, index: i32,) -> VariantArray {
            unimplemented !()
        }
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray > {
            unimplemented !()
        }
        fn surface_get_lods(&self, index: i32,) -> Dictionary {
            unimplemented !()
        }
        fn surface_get_format(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_get_primitive_type(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_set_material(&mut self, index: i32, material: Gd < crate::engine::Material >,) {
            unimplemented !()
        }
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::engine::Material > > {
            unimplemented !()
        }
        fn get_blend_shape_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_blend_shape_name(&self, index: i32,) -> StringName {
            unimplemented !()
        }
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl PlaneMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_size(&mut self, size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subdivide_width(&mut self, subdivide: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (subdivide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subdivide_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subdivide_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subdivide_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subdivide_depth(&mut self, subdivide: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (subdivide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subdivide_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subdivide_depth(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subdivide_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_offset(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_offset(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::engine::plane_mesh::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::plane_mesh::Orientation);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::engine::plane_mesh::Orientation {
            type RetMarshal = PtrcallReturnT < crate::engine::plane_mesh::Orientation >;
            type CallSig = (crate::engine::plane_mesh::Orientation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_orientation", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PlaneMesh {
        type Base = crate::engine::PrimitiveMesh;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PlaneMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PlaneMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PlaneMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PrimitiveMesh > for PlaneMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Mesh > for PlaneMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for PlaneMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PlaneMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PlaneMesh {
        
    }
    impl crate::obj::ExportableObject for PlaneMesh {
        
    }
    impl crate::obj::cap::GodotDefault for PlaneMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PlaneMesh {
        type Target = crate::engine::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PlaneMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PlaneMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PlaneMesh > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PrimitiveMesh > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Mesh > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Orientation {
    ord: i32
}
impl Orientation {
    pub const FACE_X: Self = Self {
        ord: 0i32
    };
    pub const FACE_Y: Self = Self {
        ord: 1i32
    };
    pub const FACE_Z: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Orientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Orientation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Orientation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Orientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}