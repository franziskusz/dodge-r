#![doc = "Sidecar module for class [`CollisionObject3D`][crate::engine::CollisionObject3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CollisionObject3D` enums](https://docs.godotengine.org/en/stable/classes/class_collisionobject3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CollisionObject3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`collision_object_3d`][crate::engine::collision_object_3d]: sidecar module with related enum/flag types\n* [`ICollisionObject3D`][crate::engine::ICollisionObject3D]: virtual methods\n\n\nSee also [Godot docs for `CollisionObject3D`](https://docs.godotengine.org/en/stable/classes/class_collisionobject3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CollisionObject3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CollisionObject3D`][crate::engine::CollisionObject3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CollisionObject3D` methods](https://docs.godotengine.org/en/stable/classes/class_collisionobject3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICollisionObject3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn input_event(&mut self, camera: Gd < crate::engine::Camera3D >, event: Gd < crate::engine::InputEvent >, position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl CollisionObject3D {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_mode(&mut self, mode: crate::engine::collision_object_3d::DisableMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::collision_object_3d::DisableMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_disable_mode(&self,) -> crate::engine::collision_object_3d::DisableMode {
            type RetMarshal = PtrcallReturnT < crate::engine::collision_object_3d::DisableMode >;
            type CallSig = (crate::engine::collision_object_3d::DisableMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ray_pickable(&mut self, ray_pickable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (ray_pickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ray_pickable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_capture_input_on_drag(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_capture_input_on_drag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_capture_input_on_drag(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_capture_input_on_drag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_shape_owner(&mut self, owner: Gd < crate::engine::Object >,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Gd < crate::engine::Object >);
            let args = (owner,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_shape_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_shape_owner(&mut self, owner_id: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_shape_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape_owners(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shape_owners", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_transform(&mut self, owner_id: u32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, Transform3D);
            let args = (owner_id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_transform(&self, owner_id: u32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_owner(&self, owner_id: u32,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_get_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_disabled(&mut self, owner_id: u32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, bool);
            let args = (owner_id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_set_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_owner_disabled(&self, owner_id: u32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shape_owner_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_add_shape(&mut self, owner_id: u32, shape: Gd < crate::engine::Shape3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, Gd < crate::engine::Shape3D >);
            let args = (owner_id, shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape_count(&self, owner_id: u32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape(&self, owner_id: u32, shape_id: i32,) -> Option < Gd < crate::engine::Shape3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Shape3D > >;
            type CallSig = (Option < Gd < crate::engine::Shape3D > >, u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape_index(&self, owner_id: u32, shape_id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_get_shape_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_remove_shape(&mut self, owner_id: u32, shape_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_clear_shapes(&mut self, owner_id: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_owner_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_find_owner(&self, shape_index: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i32);
            let args = (shape_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_find_owner", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CollisionObject3D {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CollisionObject3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CollisionObject3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CollisionObject3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for CollisionObject3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for CollisionObject3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CollisionObject3D {
        
    }
    impl crate::obj::ExportableObject for CollisionObject3D {
        
    }
    impl std::ops::Deref for CollisionObject3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CollisionObject3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CollisionObject3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CollisionObject3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
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
pub struct DisableMode {
    ord: i32
}
impl DisableMode {
    pub const DISABLE_MODE_REMOVE: Self = Self {
        ord: 0i32
    };
    pub const DISABLE_MODE_MAKE_STATIC: Self = Self {
        ord: 1i32
    };
    pub const DISABLE_MODE_KEEP_ACTIVE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DisableMode {
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
impl crate::builtin::meta::GodotConvert for DisableMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DisableMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DisableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}