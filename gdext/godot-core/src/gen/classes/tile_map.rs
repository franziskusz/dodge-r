#![doc = "Sidecar module for class [`TileMap`][crate::engine::TileMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMap` enums](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMap.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`tile_map`][crate::engine::tile_map]: sidecar module with related enum/flag types\n* [`ITileMap`][crate::engine::ITileMap]: virtual methods\n\n\nSee also [Godot docs for `TileMap`](https://docs.godotengine.org/en/stable/classes/class_tilemap.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileMap`][crate::engine::TileMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileMap` methods](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMap: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented !()
        }
        fn use_tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i,) -> bool {
            unimplemented !()
        }
        fn tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i, tile_data: Gd < crate::engine::TileData >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl TileMap {
        pub fn set_navigation_map(&mut self, layer: i32, map: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rid);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_map(&self, layer: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn force_update_full(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn force_update(&mut self,) {
            self.force_update_ex() . done()
        }
        #[inline]
        pub fn force_update_ex(&mut self,) -> ExForceUpdate < '_ > {
            ExForceUpdate::new(self,)
        }
        pub fn set_tileset(&mut self, tileset: Gd < crate::engine::TileSet >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TileSet >);
            let args = (tileset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tileset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tileset(&self,) -> Option < Gd < crate::engine::TileSet > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileSet > >;
            type CallSig = (Option < Gd < crate::engine::TileSet > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tileset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rendering_quadrant_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rendering_quadrant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rendering_quadrant_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rendering_quadrant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layers_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_layer(&mut self, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_layer(&mut self, layer: i32, to_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_layer(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_name(&mut self, layer: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (layer, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_name(&self, layer: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_enabled(&mut self, layer: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layer_enabled(&self, layer: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_layer_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_modulate(&mut self, layer: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (layer, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_modulate(&self, layer: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_y_sort_enabled(&mut self, layer: i32, y_sort_enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer, y_sort_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_y_sort_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layer_y_sort_enabled(&self, layer: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_layer_y_sort_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_y_sort_origin(&mut self, layer: i32, y_sort_origin: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer, y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_y_sort_origin(&self, layer: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_z_index(&mut self, layer: i32, z_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (layer, z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_z_index(&self, layer: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_navigation_enabled(&mut self, layer: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_navigation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layer_navigation_enabled(&self, layer: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_layer_navigation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_navigation_map(&mut self, layer: i32, map: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rid);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layer_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_navigation_map(&self, layer: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_animatable(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_animatable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collision_animatable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collision_animatable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_visibility_mode(&mut self, collision_visibility_mode: crate::engine::tile_map::VisibilityMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_map::VisibilityMode);
            let args = (collision_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_visibility_mode(&mut self,) -> crate::engine::tile_map::VisibilityMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_map::VisibilityMode >;
            type CallSig = (crate::engine::tile_map::VisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_visibility_mode(&mut self, navigation_visibility_mode: crate::engine::tile_map::VisibilityMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tile_map::VisibilityMode);
            let args = (navigation_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_navigation_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_visibility_mode(&mut self,) -> crate::engine::tile_map::VisibilityMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_map::VisibilityMode >;
            type CallSig = (crate::engine::tile_map::VisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_navigation_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_cell_full(&mut self, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Vector2i, i32);
            let args = (layer, coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_cell(&mut self, layer: i32, coords: Vector2i,) {
            self.set_cell_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn set_cell_ex(&mut self, layer: i32, coords: Vector2i,) -> ExSetCell < '_ > {
            ExSetCell::new(self, layer, coords,)
        }
        pub fn erase_cell(&mut self, layer: i32, coords: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i);
            let args = (layer, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_cell_source_id_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, Vector2i, bool);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_cell_source_id(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_source_id_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_source_id_ex(&self, layer: i32, coords: Vector2i,) -> ExGetCellSourceId < '_ > {
            ExGetCellSourceId::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_atlas_coords_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32, Vector2i, bool);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_atlas_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_cell_atlas_coords(&self, layer: i32, coords: Vector2i,) -> Vector2i {
            self.get_cell_atlas_coords_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_atlas_coords_ex(&self, layer: i32, coords: Vector2i,) -> ExGetCellAtlasCoords < '_ > {
            ExGetCellAtlasCoords::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_alternative_tile_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, Vector2i, bool);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_cell_alternative_tile(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_alternative_tile_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_alternative_tile_ex(&self, layer: i32, coords: Vector2i,) -> ExGetCellAlternativeTile < '_ > {
            ExGetCellAlternativeTile::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_tile_data_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Option < Gd < crate::engine::TileData > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileData > >;
            type CallSig = (Option < Gd < crate::engine::TileData > >, i32, Vector2i, bool);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_tile_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_cell_tile_data(&self, layer: i32, coords: Vector2i,) -> Option < Gd < crate::engine::TileData > > {
            self.get_cell_tile_data_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_tile_data_ex(&self, layer: i32, coords: Vector2i,) -> ExGetCellTileData < '_ > {
            ExGetCellTileData::new(self, layer, coords,)
        }
        pub fn get_coords_for_body_rid(&mut self, body: Rid,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_coords_for_body_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_for_body_rid(&mut self, body: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_for_body_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pattern(&mut self, layer: i32, coords_array: Array < Vector2i >,) -> Option < Gd < crate::engine::TileMapPattern > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileMapPattern > >;
            type CallSig = (Option < Gd < crate::engine::TileMapPattern > >, i32, Array < Vector2i >);
            let args = (layer, coords_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_pattern(&mut self, position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: Gd < crate::engine::TileMapPattern >,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i, Vector2i, Gd < crate::engine::TileMapPattern >);
            let args = (position_in_tilemap, coords_in_pattern, pattern,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "map_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pattern(&mut self, layer: i32, position: Vector2i, pattern: Gd < crate::engine::TileMapPattern >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, Gd < crate::engine::TileMapPattern >);
            let args = (layer, position, pattern,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_cells_terrain_connect_full(&mut self, layer: i32, cells: Array < Vector2i >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Array < Vector2i >, i32, i32, bool);
            let args = (layer, cells, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cells_terrain_connect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_cells_terrain_connect(&mut self, layer: i32, cells: Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_connect_ex(layer, cells, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_connect_ex(&mut self, layer: i32, cells: Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainConnect < '_ > {
            ExSetCellsTerrainConnect::new(self, layer, cells, terrain_set, terrain,)
        }
        pub(crate) fn set_cells_terrain_path_full(&mut self, layer: i32, path: Array < Vector2i >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Array < Vector2i >, i32, i32, bool);
            let args = (layer, path, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cells_terrain_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_cells_terrain_path(&mut self, layer: i32, path: Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_path_ex(layer, path, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_path_ex(&mut self, layer: i32, path: Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainPath < '_ > {
            ExSetCellsTerrainPath::new(self, layer, path, terrain_set, terrain,)
        }
        pub fn fix_invalid_tiles(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fix_invalid_tiles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_layer(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_internals(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_internals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn notify_runtime_tile_data_update_full(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_runtime_tile_data_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn notify_runtime_tile_data_update(&mut self,) {
            self.notify_runtime_tile_data_update_ex() . done()
        }
        #[inline]
        pub fn notify_runtime_tile_data_update_ex(&mut self,) -> ExNotifyRuntimeTileDataUpdate < '_ > {
            ExNotifyRuntimeTileDataUpdate::new(self,)
        }
        pub fn get_surrounding_cells(&mut self, coords: Vector2i,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surrounding_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells(&self, layer: i32,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_used_cells_by_id_full(&self, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, i32, i32, Vector2i, i32);
            let args = (layer, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_cells_by_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_used_cells_by_id(&self, layer: i32,) -> Array < Vector2i > {
            self.get_used_cells_by_id_ex(layer,) . done()
        }
        #[inline]
        pub fn get_used_cells_by_id_ex(&self, layer: i32,) -> ExGetUsedCellsById < '_ > {
            ExGetUsedCellsById::new(self, layer,)
        }
        pub fn get_used_rect(&self,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector2i,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2i);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "map_to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector2,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "local_to_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_neighbor_cell(&self, coords: Vector2i, neighbor: crate::engine::tile_set::CellNeighbor,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i, crate::engine::tile_set::CellNeighbor);
            let args = (coords, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_neighbor_cell", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileMap {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileMap\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileMap {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for TileMap {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for TileMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for TileMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileMap {
        
    }
    impl crate::obj::ExportableObject for TileMap {
        
    }
    impl crate::obj::cap::GodotDefault for TileMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMap {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileMap {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileMap > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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
#[doc = "Default-param extender for [`TileMap::force_update_ex`][super::TileMap::force_update_ex]."]
#[must_use]
pub struct ExForceUpdate < 'a > {
    surround_object: &'a mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExForceUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap,) -> Self {
        Self {
            surround_object, layer: - 1i32,
        }
    }
    #[inline]
    pub fn layer(self, value: i32) -> Self {
        Self {
            layer: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMap::force_update_full(self.surround_object, self.layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cell_ex`][super::TileMap::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'a > {
    surround_object: &'a mut re_export::TileMap, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCell < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        Self {
            surround_object, layer, coords, source_id: - 1i32, atlas_coords: Vector2i::new(- 1 as _, - 1 as _), alternative_tile: 0i32,
        }
    }
    #[inline]
    pub fn source_id(self, value: i32) -> Self {
        Self {
            source_id: value, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, value: Vector2i) -> Self {
        Self {
            atlas_coords: value, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, value: i32) -> Self {
        Self {
            alternative_tile: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMap::set_cell_full(self.surround_object, self.layer, self.coords, self.source_id, self.atlas_coords, self.alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_source_id_ex`][super::TileMap::get_cell_source_id_ex]."]
#[must_use]
pub struct ExGetCellSourceId < 'a > {
    surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellSourceId < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        Self {
            surround_object, layer, coords, use_proxies: false,
        }
    }
    #[inline]
    pub fn use_proxies(self, value: bool) -> Self {
        Self {
            use_proxies: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileMap::get_cell_source_id_full(self.surround_object, self.layer, self.coords, self.use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_atlas_coords_ex`][super::TileMap::get_cell_atlas_coords_ex]."]
#[must_use]
pub struct ExGetCellAtlasCoords < 'a > {
    surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellAtlasCoords < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        Self {
            surround_object, layer, coords, use_proxies: false,
        }
    }
    #[inline]
    pub fn use_proxies(self, value: bool) -> Self {
        Self {
            use_proxies: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::TileMap::get_cell_atlas_coords_full(self.surround_object, self.layer, self.coords, self.use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_alternative_tile_ex`][super::TileMap::get_cell_alternative_tile_ex]."]
#[must_use]
pub struct ExGetCellAlternativeTile < 'a > {
    surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellAlternativeTile < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        Self {
            surround_object, layer, coords, use_proxies: false,
        }
    }
    #[inline]
    pub fn use_proxies(self, value: bool) -> Self {
        Self {
            use_proxies: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileMap::get_cell_alternative_tile_full(self.surround_object, self.layer, self.coords, self.use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_tile_data_ex`][super::TileMap::get_cell_tile_data_ex]."]
#[must_use]
pub struct ExGetCellTileData < 'a > {
    surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellTileData < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        Self {
            surround_object, layer, coords, use_proxies: false,
        }
    }
    #[inline]
    pub fn use_proxies(self, value: bool) -> Self {
        Self {
            use_proxies: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TileData > > {
        re_export::TileMap::get_cell_tile_data_full(self.surround_object, self.layer, self.coords, self.use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_connect_ex`][super::TileMap::set_cells_terrain_connect_ex]."]
#[must_use]
pub struct ExSetCellsTerrainConnect < 'a > {
    surround_object: &'a mut re_export::TileMap, layer: i32, cells: Array < Vector2i >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainConnect < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, cells: Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        Self {
            surround_object, layer, cells, terrain_set, terrain, ignore_empty_terrains: true,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, value: bool) -> Self {
        Self {
            ignore_empty_terrains: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMap::set_cells_terrain_connect_full(self.surround_object, self.layer, self.cells, self.terrain_set, self.terrain, self.ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_path_ex`][super::TileMap::set_cells_terrain_path_ex]."]
#[must_use]
pub struct ExSetCellsTerrainPath < 'a > {
    surround_object: &'a mut re_export::TileMap, layer: i32, path: Array < Vector2i >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainPath < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, path: Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        Self {
            surround_object, layer, path, terrain_set, terrain, ignore_empty_terrains: true,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, value: bool) -> Self {
        Self {
            ignore_empty_terrains: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMap::set_cells_terrain_path_full(self.surround_object, self.layer, self.path, self.terrain_set, self.terrain, self.ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::notify_runtime_tile_data_update_ex`][super::TileMap::notify_runtime_tile_data_update_ex]."]
#[must_use]
pub struct ExNotifyRuntimeTileDataUpdate < 'a > {
    surround_object: &'a mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExNotifyRuntimeTileDataUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap,) -> Self {
        Self {
            surround_object, layer: - 1i32,
        }
    }
    #[inline]
    pub fn layer(self, value: i32) -> Self {
        Self {
            layer: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMap::notify_runtime_tile_data_update_full(self.surround_object, self.layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_used_cells_by_id_ex`][super::TileMap::get_used_cells_by_id_ex]."]
#[must_use]
pub struct ExGetUsedCellsById < 'a > {
    surround_object: &'a re_export::TileMap, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUsedCellsById < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32,) -> Self {
        Self {
            surround_object, layer, source_id: - 1i32, atlas_coords: Vector2i::new(- 1 as _, - 1 as _), alternative_tile: - 1i32,
        }
    }
    #[inline]
    pub fn source_id(self, value: i32) -> Self {
        Self {
            source_id: value, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, value: Vector2i) -> Self {
        Self {
            atlas_coords: value, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, value: i32) -> Self {
        Self {
            alternative_tile: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        re_export::TileMap::get_used_cells_by_id_full(self.surround_object, self.layer, self.source_id, self.atlas_coords, self.alternative_tile,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    pub const VISIBILITY_MODE_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const VISIBILITY_MODE_FORCE_HIDE: Self = Self {
        ord: 2i32
    };
    pub const VISIBILITY_MODE_FORCE_SHOW: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for VisibilityMode {
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
impl crate::builtin::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibilityMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}