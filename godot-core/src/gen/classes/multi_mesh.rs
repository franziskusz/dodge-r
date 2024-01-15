#![doc = "Sidecar module for class [`MultiMesh`][crate::engine::MultiMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiMesh` enums](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiMesh.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`multi_mesh`][crate::engine::multi_mesh]: sidecar module with related enum/flag types\n* [`IMultiMesh`][crate::engine::IMultiMesh]: virtual methods\n\n\nSee also [Godot docs for `MultiMesh`](https://docs.godotengine.org/en/stable/classes/class_multimesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiMesh`][crate::engine::MultiMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiMesh` methods](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_mesh(&mut self, mesh: Gd < crate::engine::Mesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_colors(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_colors(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_custom_data(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_custom_data(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform_format(&mut self, format: crate::engine::multi_mesh::TransformFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::multi_mesh::TransformFormat);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_format(&self,) -> crate::engine::multi_mesh::TransformFormat {
            type RetMarshal = PtrcallReturnT < crate::engine::multi_mesh::TransformFormat >;
            type CallSig = (crate::engine::multi_mesh::TransformFormat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_instance_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_instance_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_transform(&mut self, instance: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_transform_2d(&mut self, instance: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform2D);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_transform(&self, instance: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_transform_2d(&self, instance: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_color(&mut self, instance: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (instance, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_color(&self, instance: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_custom_data(&mut self, instance: i32, custom_data: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (instance, custom_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_custom_data(&self, instance: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer(&self,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer(&mut self, buffer: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat32Array);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_buffer", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MultiMesh {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MultiMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MultiMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for MultiMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MultiMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MultiMesh {
        
    }
    impl crate::obj::ExportableObject for MultiMesh {
        
    }
    impl crate::obj::cap::GodotDefault for MultiMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiMesh {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MultiMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MultiMesh > for $Class {
                
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
pub struct TransformFormat {
    ord: i32
}
impl TransformFormat {
    pub const TRANSFORM_2D: Self = Self {
        ord: 0i32
    };
    pub const TRANSFORM_3D: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for TransformFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TransformFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TransformFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}