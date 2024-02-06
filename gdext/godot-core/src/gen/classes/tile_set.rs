#![doc = "Sidecar module for class [`TileSet`][crate::engine::TileSet].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSet` enums](https://docs.godotengine.org/en/stable/classes/class_tileset.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSet.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`tile_set`][crate::engine::tile_set]: sidecar module with related enum/flag types\n* [`ITileSet`][crate::engine::ITileSet]: virtual methods\n\n\nSee also [Godot docs for `TileSet`](https://docs.godotengine.org/en/stable/classes/class_tileset.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSet {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileSet`][crate::engine::TileSet].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSet` methods](https://docs.godotengine.org/en/stable/classes/class_tileset.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSet: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSet {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_next_source_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_source_full(&mut self, source: Gd < crate::engine::TileSetSource >, atlas_source_id_override: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::TileSetSource >, i32);
            let args = (source, atlas_source_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_source(&mut self, source: Gd < crate::engine::TileSetSource >,) -> i32 {
            self.add_source_ex(source,) . done()
        }
        #[inline]
        pub fn add_source_ex(&mut self, source: Gd < crate::engine::TileSetSource >,) -> ExAddSource < '_ > {
            ExAddSource::new(self, source,)
        }
        pub fn remove_source(&mut self, source_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_source_id(&mut self, source_id: i32, new_source_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (source_id, new_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_source_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_id(&self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_source(&self, source_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source(&self, source_id: i32,) -> Option < Gd < crate::engine::TileSetSource > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileSetSource > >;
            type CallSig = (Option < Gd < crate::engine::TileSetSource > >, i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_shape(&mut self, shape: crate::engine::tile_set::TileShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_set::TileShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_shape(&self,) -> crate::engine::tile_set::TileShape {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_set::TileShape >;
            type CallSig = (crate::engine::tile_set::TileShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_layout(&mut self, layout: crate::engine::tile_set::TileLayout,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_set::TileLayout);
            let args = (layout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_layout(&self,) -> crate::engine::tile_set::TileLayout {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_set::TileLayout >;
            type CallSig = (crate::engine::tile_set::TileLayout,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_offset_axis(&mut self, alignment: crate::engine::tile_set::TileOffsetAxis,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_set::TileOffsetAxis);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_offset_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_offset_axis(&self,) -> crate::engine::tile_set::TileOffsetAxis {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_set::TileOffsetAxis >;
            type CallSig = (crate::engine::tile_set::TileOffsetAxis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_offset_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv_clipping(&mut self, uv_clipping: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (uv_clipping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv_clipping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_uv_clipping(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_uv_clipping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layers_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_occlusion_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_occlusion_layer_full(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_occlusion_layer(&mut self,) {
            self.add_occlusion_layer_ex() . done()
        }
        #[inline]
        pub fn add_occlusion_layer_ex(&mut self,) -> ExAddOcclusionLayer < '_ > {
            ExAddOcclusionLayer::new(self,)
        }
        pub fn move_occlusion_layer(&mut self, layer_index: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_occlusion_layer(&mut self, layer_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_occlusion_layer_light_mask(&mut self, layer_index: i32, light_mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_index, light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_occlusion_layer_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layer_light_mask(&self, layer_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_occlusion_layer_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_occlusion_layer_sdf_collision(&mut self, layer_index: i32, sdf_collision: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_index, sdf_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_occlusion_layer_sdf_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layer_sdf_collision(&self, layer_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_occlusion_layer_sdf_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layers_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_physics_layer_full(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_physics_layer(&mut self,) {
            self.add_physics_layer_ex() . done()
        }
        #[inline]
        pub fn add_physics_layer_ex(&mut self,) -> ExAddPhysicsLayer < '_ > {
            ExAddPhysicsLayer::new(self,)
        }
        pub fn move_physics_layer(&mut self, layer_index: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_physics_layer(&mut self, layer_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_collision_layer(&mut self, layer_index: i32, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, u32);
            let args = (layer_index, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_layer_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_collision_layer(&self, layer_index: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_layer_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_collision_mask(&mut self, layer_index: i32, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, u32);
            let args = (layer_index, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_layer_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_collision_mask(&self, layer_index: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_layer_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_physics_material(&mut self, layer_index: i32, physics_material: Gd < crate::engine::PhysicsMaterial >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::PhysicsMaterial >);
            let args = (layer_index, physics_material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_layer_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_physics_material(&self, layer_index: i32,) -> Option < Gd < crate::engine::PhysicsMaterial > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsMaterial > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsMaterial > >, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_layer_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_sets_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_sets_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_terrain_set_full(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_terrain_set(&mut self,) {
            self.add_terrain_set_ex() . done()
        }
        #[inline]
        pub fn add_terrain_set_ex(&mut self,) -> ExAddTerrainSet < '_ > {
            ExAddTerrainSet::new(self,)
        }
        pub fn move_terrain_set(&mut self, terrain_set: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_terrain_set(&mut self, terrain_set: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_set_mode(&mut self, terrain_set: i32, mode: crate::engine::tile_set::TerrainMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::tile_set::TerrainMode);
            let args = (terrain_set, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_set_mode(&self, terrain_set: i32,) -> crate::engine::tile_set::TerrainMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_set::TerrainMode >;
            type CallSig = (crate::engine::tile_set::TerrainMode, i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrains_count(&self, terrain_set: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrains_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_terrain_full(&mut self, terrain_set: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_terrain(&mut self, terrain_set: i32,) {
            self.add_terrain_ex(terrain_set,) . done()
        }
        #[inline]
        pub fn add_terrain_ex(&mut self, terrain_set: i32,) -> ExAddTerrain < '_ > {
            ExAddTerrain::new(self, terrain_set,)
        }
        pub fn move_terrain(&mut self, terrain_set: i32, terrain_index: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32);
            let args = (terrain_set, terrain_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_terrain(&mut self, terrain_set: i32, terrain_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_name(&mut self, terrain_set: i32, terrain_index: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, GString);
            let args = (terrain_set, terrain_index, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_name(&self, terrain_set: i32, terrain_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_color(&mut self, terrain_set: i32, terrain_index: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Color);
            let args = (terrain_set, terrain_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_terrain_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_color(&self, terrain_set: i32, terrain_index: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_terrain_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layers_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_navigation_layer_full(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_navigation_layer(&mut self,) {
            self.add_navigation_layer_ex() . done()
        }
        #[inline]
        pub fn add_navigation_layer_ex(&mut self,) -> ExAddNavigationLayer < '_ > {
            ExAddNavigationLayer::new(self,)
        }
        pub fn move_navigation_layer(&mut self, layer_index: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_navigation_layer(&mut self, layer_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_layer_layers(&mut self, layer_index: i32, layers: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, u32);
            let args = (layer_index, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_navigation_layer_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layer_layers(&self, layer_index: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_layer_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_layer_layer_value(&mut self, layer_index: i32, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (layer_index, layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_navigation_layer_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layer_layer_value(&self, layer_index: i32, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (layer_index, layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_layer_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layers_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_custom_data_layer_full(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_custom_data_layer(&mut self,) {
            self.add_custom_data_layer_ex() . done()
        }
        #[inline]
        pub fn add_custom_data_layer_ex(&mut self,) -> ExAddCustomDataLayer < '_ > {
            ExAddCustomDataLayer::new(self,)
        }
        pub fn move_custom_data_layer(&mut self, layer_index: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_custom_data_layer(&mut self, layer_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_by_name(&self, layer_name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (layer_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data_layer_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data_layer_name(&mut self, layer_index: i32, layer_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (layer_index, layer_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_data_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_name(&self, layer_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data_layer_type(&mut self, layer_index: i32, layer_type: VariantType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, VariantType);
            let args = (layer_index, layer_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_data_layer_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_type(&self, layer_index: i32,) -> VariantType {
            type RetMarshal = PtrcallReturnT < VariantType >;
            type CallSig = (VariantType, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_data_layer_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_source_level_tile_proxy(&mut self, source_from: i32, source_to: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (source_from, source_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_level_tile_proxy(&mut self, source_from: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_source_level_tile_proxy(&mut self, source_from: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_source_level_tile_proxy(&mut self, source_from: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_coords_level_tile_proxy(&mut self, p_source_from: i32, coords_from: Vector2i, source_to: i32, coords_to: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Vector2i);
            let args = (p_source_from, coords_from, source_to, coords_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32, source_to: i32, coords_to: Vector2i, alternative_to: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from, source_to, coords_to, alternative_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "map_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cleanup_invalid_tile_proxies(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cleanup_invalid_tile_proxies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tile_proxies(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_tile_proxies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_pattern_full(&mut self, pattern: Gd < crate::engine::TileMapPattern >, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::TileMapPattern >, i32);
            let args = (pattern, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_pattern(&mut self, pattern: Gd < crate::engine::TileMapPattern >,) -> i32 {
            self.add_pattern_ex(pattern,) . done()
        }
        #[inline]
        pub fn add_pattern_ex(&mut self, pattern: Gd < crate::engine::TileMapPattern >,) -> ExAddPattern < '_ > {
            ExAddPattern::new(self, pattern,)
        }
        pub(crate) fn get_pattern_full(&mut self, index: i32,) -> Option < Gd < crate::engine::TileMapPattern > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileMapPattern > >;
            type CallSig = (Option < Gd < crate::engine::TileMapPattern > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_pattern(&mut self,) -> Option < Gd < crate::engine::TileMapPattern > > {
            self.get_pattern_ex() . done()
        }
        #[inline]
        pub fn get_pattern_ex(&mut self,) -> ExGetPattern < '_ > {
            ExGetPattern::new(self,)
        }
        pub fn remove_pattern(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_patterns_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_patterns_count", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileSet {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileSet\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSet {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileSet {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TileSet {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TileSet {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileSet {
        
    }
    impl crate::obj::ExportableObject for TileSet {
        
    }
    impl crate::obj::cap::GodotDefault for TileSet {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSet {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileSet {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileSet > for $Class {
                
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
#[doc = "Default-param extender for [`TileSet::add_source_ex`][super::TileSet::add_source_ex]."]
#[must_use]
pub struct ExAddSource < 'a > {
    surround_object: &'a mut re_export::TileSet, source: Gd < crate::engine::TileSetSource >, atlas_source_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSource < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, source: Gd < crate::engine::TileSetSource >,) -> Self {
        Self {
            surround_object, source, atlas_source_id_override: - 1i32,
        }
    }
    #[inline]
    pub fn atlas_source_id_override(self, value: i32) -> Self {
        Self {
            atlas_source_id_override: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileSet::add_source_full(self.surround_object, self.source, self.atlas_source_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_occlusion_layer_ex`][super::TileSet::add_occlusion_layer_ex]."]
#[must_use]
pub struct ExAddOcclusionLayer < 'a > {
    surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddOcclusionLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_occlusion_layer_full(self.surround_object, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_physics_layer_ex`][super::TileSet::add_physics_layer_ex]."]
#[must_use]
pub struct ExAddPhysicsLayer < 'a > {
    surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPhysicsLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_physics_layer_full(self.surround_object, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_set_ex`][super::TileSet::add_terrain_set_ex]."]
#[must_use]
pub struct ExAddTerrainSet < 'a > {
    surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrainSet < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_terrain_set_full(self.surround_object, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_ex`][super::TileSet::add_terrain_ex]."]
#[must_use]
pub struct ExAddTerrain < 'a > {
    surround_object: &'a mut re_export::TileSet, terrain_set: i32, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrain < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, terrain_set: i32,) -> Self {
        Self {
            surround_object, terrain_set, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_terrain_full(self.surround_object, self.terrain_set, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_navigation_layer_ex`][super::TileSet::add_navigation_layer_ex]."]
#[must_use]
pub struct ExAddNavigationLayer < 'a > {
    surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNavigationLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_navigation_layer_full(self.surround_object, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_custom_data_layer_ex`][super::TileSet::add_custom_data_layer_ex]."]
#[must_use]
pub struct ExAddCustomDataLayer < 'a > {
    surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomDataLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, to_position: - 1i32,
        }
    }
    #[inline]
    pub fn to_position(self, value: i32) -> Self {
        Self {
            to_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSet::add_custom_data_layer_full(self.surround_object, self.to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_pattern_ex`][super::TileSet::add_pattern_ex]."]
#[must_use]
pub struct ExAddPattern < 'a > {
    surround_object: &'a mut re_export::TileSet, pattern: Gd < crate::engine::TileMapPattern >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, pattern: Gd < crate::engine::TileMapPattern >,) -> Self {
        Self {
            surround_object, pattern, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileSet::add_pattern_full(self.surround_object, self.pattern, self.index,)
    }
}
#[doc = "Default-param extender for [`TileSet::get_pattern_ex`][super::TileSet::get_pattern_ex]."]
#[must_use]
pub struct ExGetPattern < 'a > {
    surround_object: &'a mut re_export::TileSet, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        Self {
            surround_object, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TileMapPattern > > {
        re_export::TileSet::get_pattern_full(self.surround_object, self.index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TileShape {
    ord: i32
}
impl TileShape {
    pub const TILE_SHAPE_SQUARE: Self = Self {
        ord: 0i32
    };
    pub const TILE_SHAPE_ISOMETRIC: Self = Self {
        ord: 1i32
    };
    pub const TILE_SHAPE_HALF_OFFSET_SQUARE: Self = Self {
        ord: 2i32
    };
    pub const TILE_SHAPE_HEXAGON: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for TileShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TileShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TileShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TileShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TileLayout {
    ord: i32
}
impl TileLayout {
    pub const TILE_LAYOUT_STACKED: Self = Self {
        ord: 0i32
    };
    pub const TILE_LAYOUT_STACKED_OFFSET: Self = Self {
        ord: 1i32
    };
    pub const TILE_LAYOUT_STAIRS_RIGHT: Self = Self {
        ord: 2i32
    };
    pub const TILE_LAYOUT_STAIRS_DOWN: Self = Self {
        ord: 3i32
    };
    pub const TILE_LAYOUT_DIAMOND_RIGHT: Self = Self {
        ord: 4i32
    };
    pub const TILE_LAYOUT_DIAMOND_DOWN: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for TileLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TileLayout {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TileLayout {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TileLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TileOffsetAxis {
    ord: i32
}
impl TileOffsetAxis {
    pub const TILE_OFFSET_AXIS_HORIZONTAL: Self = Self {
        ord: 0i32
    };
    pub const TILE_OFFSET_AXIS_VERTICAL: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for TileOffsetAxis {
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
impl crate::builtin::meta::GodotConvert for TileOffsetAxis {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TileOffsetAxis {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TileOffsetAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CellNeighbor {
    ord: i32
}
impl CellNeighbor {
    pub const CELL_NEIGHBOR_RIGHT_SIDE: Self = Self {
        ord: 0i32
    };
    pub const CELL_NEIGHBOR_RIGHT_CORNER: Self = Self {
        ord: 1i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE: Self = Self {
        ord: 2i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER: Self = Self {
        ord: 3i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_SIDE: Self = Self {
        ord: 4i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_CORNER: Self = Self {
        ord: 5i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_LEFT_SIDE: Self = Self {
        ord: 6i32
    };
    pub const CELL_NEIGHBOR_BOTTOM_LEFT_CORNER: Self = Self {
        ord: 7i32
    };
    pub const CELL_NEIGHBOR_LEFT_SIDE: Self = Self {
        ord: 8i32
    };
    pub const CELL_NEIGHBOR_LEFT_CORNER: Self = Self {
        ord: 9i32
    };
    pub const CELL_NEIGHBOR_TOP_LEFT_SIDE: Self = Self {
        ord: 10i32
    };
    pub const CELL_NEIGHBOR_TOP_LEFT_CORNER: Self = Self {
        ord: 11i32
    };
    pub const CELL_NEIGHBOR_TOP_SIDE: Self = Self {
        ord: 12i32
    };
    pub const CELL_NEIGHBOR_TOP_CORNER: Self = Self {
        ord: 13i32
    };
    pub const CELL_NEIGHBOR_TOP_RIGHT_SIDE: Self = Self {
        ord: 14i32
    };
    pub const CELL_NEIGHBOR_TOP_RIGHT_CORNER: Self = Self {
        ord: 15i32
    };
    
}
impl crate::obj::EngineEnum for CellNeighbor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CellNeighbor {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CellNeighbor {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CellNeighbor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TerrainMode {
    ord: i32
}
impl TerrainMode {
    pub const TERRAIN_MODE_MATCH_CORNERS_AND_SIDES: Self = Self {
        ord: 0i32
    };
    pub const TERRAIN_MODE_MATCH_CORNERS: Self = Self {
        ord: 1i32
    };
    pub const TERRAIN_MODE_MATCH_SIDES: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TerrainMode {
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
impl crate::builtin::meta::GodotConvert for TerrainMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TerrainMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TerrainMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}