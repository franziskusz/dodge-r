#![doc = "Sidecar module for class [`MeshLibrary`][crate::engine::MeshLibrary].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshLibrary` enums](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshLibrary.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IMeshLibrary`][crate::engine::IMeshLibrary]: virtual methods\n\n\nSee also [Godot docs for `MeshLibrary`](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshLibrary {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshLibrary`][crate::engine::MeshLibrary].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshLibrary` methods](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshLibrary: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshLibrary {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn create_item(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_name(&mut self, id: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (id, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_mesh(&mut self, id: i32, mesh: Gd < crate::engine::Mesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Mesh >);
            let args = (id, mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_mesh_transform(&mut self, id: i32, mesh_transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (id, mesh_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_mesh_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_navigation_mesh_transform(&mut self, id: i32, navigation_mesh: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (id, navigation_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_navigation_mesh_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_navigation_layers(&mut self, id: i32, navigation_layers: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, u32);
            let args = (id, navigation_layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_navigation_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_shapes(&mut self, id: i32, shapes: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, VariantArray);
            let args = (id, shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_preview(&mut self, id: i32, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (id, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_name(&self, id: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_mesh(&self, id: i32,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_mesh_transform(&self, id: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_mesh_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_navigation_mesh_transform(&self, id: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_navigation_mesh_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_navigation_layers(&self, id: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_navigation_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_shapes(&self, id: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_preview(&self, id: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_item_by_name(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_item_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_list(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_unused_item_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_unused_item_id", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MeshLibrary {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MeshLibrary\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshLibrary {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MeshLibrary {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for MeshLibrary {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MeshLibrary {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MeshLibrary {
        
    }
    impl crate::obj::ExportableObject for MeshLibrary {
        
    }
    impl crate::obj::cap::GodotDefault for MeshLibrary {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshLibrary {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshLibrary {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MeshLibrary {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MeshLibrary > for $Class {
                
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