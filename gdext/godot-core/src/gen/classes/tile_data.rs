#![doc = "Sidecar module for class [`TileData`][crate::engine::TileData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileData` enums](https://docs.godotengine.org/en/stable/classes/class_tiledata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileData.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`ITileData`][crate::engine::ITileData]: virtual methods\n\n\nSee also [Godot docs for `TileData`](https://docs.godotengine.org/en/stable/classes/class_tiledata.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileData`][crate::engine::TileData].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileData` methods](https://docs.godotengine.org/en/stable/classes/class_tiledata.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileData: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileData {
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flip_h(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flip_v(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transpose(&mut self, transpose: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transpose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transpose(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transpose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_origin(&mut self, texture_origin: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (texture_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_origin(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_z_index(&mut self, z_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_z_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_y_sort_origin(&mut self, y_sort_origin: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_y_sort_origin(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_occluder(&mut self, layer_id: i32, occluder_polygon: Gd < crate::engine::OccluderPolygon2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::OccluderPolygon2D >);
            let args = (layer_id, occluder_polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_occluder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occluder(&self, layer_id: i32,) -> Option < Gd < crate::engine::OccluderPolygon2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OccluderPolygon2D > >;
            type CallSig = (Option < Gd < crate::engine::OccluderPolygon2D > >, i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_occluder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_linear_velocity(&mut self, layer_id: i32, velocity: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2);
            let args = (layer_id, velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_linear_velocity(&self, layer_id: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_angular_velocity(&mut self, layer_id: i32, velocity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (layer_id, velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_angular_velocity(&self, layer_id: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_polygons_count(&mut self, layer_id: i32, polygons_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_id, polygons_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_polygons_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_polygons_count(&self, layer_id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_polygons_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_polygon(&mut self, layer_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_collision_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_collision_polygon(&mut self, layer_id: i32, polygon_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_collision_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_polygon_points(&mut self, layer_id: i32, polygon_index: i32, polygon: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, PackedVector2Array);
            let args = (layer_id, polygon_index, polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_polygon_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_polygon_points(&self, layer_id: i32, polygon_index: i32,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, i32, i32);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_polygon_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_polygon_one_way(&mut self, layer_id: i32, polygon_index: i32, one_way: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (layer_id, polygon_index, one_way,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_polygon_one_way", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collision_polygon_one_way(&self, layer_id: i32, polygon_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collision_polygon_one_way", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_polygon_one_way_margin(&mut self, layer_id: i32, polygon_index: i32, one_way_margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (layer_id, polygon_index, one_way_margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_polygon_one_way_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_polygon_one_way_margin(&self, layer_id: i32, polygon_index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_polygon_one_way_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_set(&mut self, terrain_set: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_set(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain(&mut self, terrain: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (terrain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_peering_bit(&mut self, peering_bit: crate::engine::tile_set::CellNeighbor, terrain: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_set::CellNeighbor, i32);
            let args = (peering_bit, terrain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain_peering_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_peering_bit(&self, peering_bit: crate::engine::tile_set::CellNeighbor,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::tile_set::CellNeighbor);
            let args = (peering_bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_peering_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_probability(&mut self, probability: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (probability,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_probability", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_probability(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_probability", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data(&mut self, layer_name: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Variant);
            let args = (layer_name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data(&self, layer_name: GString,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString);
            let args = (layer_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data_by_layer_id(&mut self, layer_id: i32, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (layer_id, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_data_by_layer_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_by_layer_id(&self, layer_id: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data_by_layer_id", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileData {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileData\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileData {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileData {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileData {
        
    }
    impl crate::obj::cap::GodotDefault for TileData {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileData {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileData {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileData > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}