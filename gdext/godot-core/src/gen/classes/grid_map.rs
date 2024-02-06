#![doc = "Sidecar module for class [`GridMap`][crate::engine::GridMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GridMap` enums](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GridMap.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`grid_map`][crate::engine::grid_map]: sidecar module with related enum/flag types\n* [`IGridMap`][crate::engine::IGridMap]: virtual methods\n\n\nSee also [Godot docs for `GridMap`](https://docs.godotengine.org/en/stable/classes/class_gridmap.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GridMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GridMap`][crate::engine::GridMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GridMap` methods](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGridMap: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GridMap {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_material(&mut self, material: Gd < crate::engine::PhysicsMaterial >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::PhysicsMaterial >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_material(&self,) -> Option < Gd < crate::engine::PhysicsMaterial > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsMaterial > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsMaterial > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_navigation(&mut self, bake_navigation: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (bake_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bake_navigation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_baking_navigation(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_baking_navigation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_map(&mut self, navigation_map: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (navigation_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_map(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_library(&mut self, mesh_library: Gd < crate::engine::MeshLibrary >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::MeshLibrary >);
            let args = (mesh_library,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_library(&self,) -> Option < Gd < crate::engine::MeshLibrary > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MeshLibrary > >;
            type CallSig = (Option < Gd < crate::engine::MeshLibrary > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size(&mut self, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_scale(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_octant_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_octant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_octant_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_octant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_cell_item_full(&mut self, position: Vector3i, item: i32, orientation: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3i, i32, i32);
            let args = (position, item, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_cell_item(&mut self, position: Vector3i, item: i32,) {
            self.set_cell_item_ex(position, item,) . done()
        }
        #[inline]
        pub fn set_cell_item_ex(&mut self, position: Vector3i, item: i32,) -> ExSetCellItem < '_ > {
            ExSetCellItem::new(self, position, item,)
        }
        pub fn get_cell_item(&self, position: Vector3i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_item_orientation(&self, position: Vector3i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_item_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_item_basis(&self, position: Vector3i,) -> Basis {
            type RetMarshal = PtrcallReturnT < Basis >;
            type CallSig = (Basis, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_item_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_basis_with_orthogonal_index(&self, index: i32,) -> Basis {
            type RetMarshal = PtrcallReturnT < Basis >;
            type CallSig = (Basis, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_basis_with_orthogonal_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_orthogonal_index_from_basis(&self, basis: Basis,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_orthogonal_index_from_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector3,) -> Vector3i {
            type RetMarshal = PtrcallReturnT < Vector3i >;
            type CallSig = (Vector3i, Vector3);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "local_to_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector3i,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3i);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "map_to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn resource_changed(&mut self, resource: Gd < crate::engine::Resource >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Resource >);
            let args = (resource,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resource_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_x(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_x(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_y(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_y(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_z(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_z(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector3i > {
            type RetMarshal = PtrcallReturnT < Array < Vector3i > >;
            type CallSig = (Array < Vector3i >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells_by_item(&self, item: i32,) -> Array < Vector3i > {
            type RetMarshal = PtrcallReturnT < Array < Vector3i > >;
            type CallSig = (Array < Vector3i >, i32);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_cells_by_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meshes(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_meshes(&mut self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mesh_instance(&mut self, idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_mesh_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_baked_meshes(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_baked_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn make_baked_meshes_full(&mut self, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, f32);
            let args = (gen_lightmap_uv, lightmap_uv_texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_baked_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn make_baked_meshes(&mut self,) {
            self.make_baked_meshes_ex() . done()
        }
        #[inline]
        pub fn make_baked_meshes_ex(&mut self,) -> ExMakeBakedMeshes < '_ > {
            ExMakeBakedMeshes::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const INVALID_CELL_ITEM: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for GridMap {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GridMap\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GridMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GridMap {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for GridMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for GridMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GridMap {
        
    }
    impl crate::obj::ExportableObject for GridMap {
        
    }
    impl crate::obj::cap::GodotDefault for GridMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GridMap {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GridMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GridMap {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GridMap > for $Class {
                
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
#[doc = "Default-param extender for [`GridMap::set_cell_item_ex`][super::GridMap::set_cell_item_ex]."]
#[must_use]
pub struct ExSetCellItem < 'a > {
    surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32, orientation: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellItem < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32,) -> Self {
        Self {
            surround_object, position, item, orientation: 0i32,
        }
    }
    #[inline]
    pub fn orientation(self, value: i32) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::GridMap::set_cell_item_full(self.surround_object, self.position, self.item, self.orientation,)
    }
}
#[doc = "Default-param extender for [`GridMap::make_baked_meshes_ex`][super::GridMap::make_baked_meshes_ex]."]
#[must_use]
pub struct ExMakeBakedMeshes < 'a > {
    surround_object: &'a mut re_export::GridMap, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMakeBakedMeshes < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap,) -> Self {
        Self {
            surround_object, gen_lightmap_uv: false, lightmap_uv_texel_size: 0.1f32,
        }
    }
    #[inline]
    pub fn gen_lightmap_uv(self, value: bool) -> Self {
        Self {
            gen_lightmap_uv: value, .. self
        }
    }
    #[inline]
    pub fn lightmap_uv_texel_size(self, value: f32) -> Self {
        Self {
            lightmap_uv_texel_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::GridMap::make_baked_meshes_full(self.surround_object, self.gen_lightmap_uv, self.lightmap_uv_texel_size,)
    }
}