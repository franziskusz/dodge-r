#![doc = "Sidecar module for class [`TileSetAtlasSource`][crate::engine::TileSetAtlasSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetAtlasSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetAtlasSource.`\n\nInherits [`TileSetSource`][crate::engine::TileSetSource].\n\nRelated symbols:\n\n* [`tile_set_atlas_source`][crate::engine::tile_set_atlas_source]: sidecar module with related enum/flag types\n* [`ITileSetAtlasSource`][crate::engine::ITileSetAtlasSource]: virtual methods\n\n\nSee also [Godot docs for `TileSetAtlasSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetAtlasSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileSetAtlasSource`][crate::engine::TileSetAtlasSource].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSetAtlasSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSetAtlasSource: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSetAtlasSource {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margins(&mut self, margins: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (margins,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_margins(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_separation(&mut self, separation: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_separation(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_region_size(&mut self, texture_region_size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_region_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_region_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_region_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_texture_padding(&mut self, use_texture_padding: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_texture_padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_texture_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_texture_padding(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_texture_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_tile_full(&mut self, atlas_coords: Vector2i, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, Vector2i);
            let args = (atlas_coords, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_tile(&mut self, atlas_coords: Vector2i,) {
            self.create_tile_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn create_tile_ex(&mut self, atlas_coords: Vector2i,) -> ExCreateTile < '_ > {
            ExCreateTile::new(self, atlas_coords,)
        }
        pub fn remove_tile(&mut self, atlas_coords: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn move_tile_in_atlas_full(&mut self, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, Vector2i, Vector2i);
            let args = (atlas_coords, new_atlas_coords, new_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_tile_in_atlas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn move_tile_in_atlas(&mut self, atlas_coords: Vector2i,) {
            self.move_tile_in_atlas_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn move_tile_in_atlas_ex(&mut self, atlas_coords: Vector2i,) -> ExMoveTileInAtlas < '_ > {
            ExMoveTileInAtlas::new(self, atlas_coords,)
        }
        pub fn get_tile_size_in_atlas(&self, atlas_coords: Vector2i,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_size_in_atlas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn has_room_for_tile_full(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2i, Vector2i, i32, Vector2i, i32, Vector2i);
            let args = (atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_room_for_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_room_for_tile(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> bool {
            self.has_room_for_tile_ex(atlas_coords, size, animation_columns, animation_separation, frames_count,) . done()
        }
        #[inline]
        pub fn has_room_for_tile_ex(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> ExHasRoomForTile < '_ > {
            ExHasRoomForTile::new(self, atlas_coords, size, animation_columns, animation_separation, frames_count,)
        }
        pub fn get_tiles_to_be_removed_on_change(&mut self, texture: Gd < crate::engine::Texture2D >, margins: Vector2i, separation: Vector2i, texture_region_size: Vector2i,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Gd < crate::engine::Texture2D >, Vector2i, Vector2i, Vector2i);
            let args = (texture, margins, separation, texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tiles_to_be_removed_on_change", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_at_coords(&self, atlas_coords: Vector2i,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_at_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_tiles_outside_texture(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_tiles_outside_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tiles_outside_texture(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_tiles_outside_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_columns(&mut self, atlas_coords: Vector2i, frame_columns: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, frame_columns,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_columns(&self, atlas_coords: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_separation(&mut self, atlas_coords: Vector2i, separation: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, Vector2i);
            let args = (atlas_coords, separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_separation(&self, atlas_coords: Vector2i,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_speed(&mut self, atlas_coords: Vector2i, speed: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, f32);
            let args = (atlas_coords, speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_speed(&self, atlas_coords: Vector2i,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_mode(&mut self, atlas_coords: Vector2i, mode: crate::engine::tile_set_atlas_source::TileAnimationMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, crate::engine::tile_set_atlas_source::TileAnimationMode);
            let args = (atlas_coords, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_mode(&self, atlas_coords: Vector2i,) -> crate::engine::tile_set_atlas_source::TileAnimationMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tile_set_atlas_source::TileAnimationMode >;
            type CallSig = (crate::engine::tile_set_atlas_source::TileAnimationMode, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_frames_count(&mut self, atlas_coords: Vector2i, frames_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, frames_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_frames_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_frames_count(&self, atlas_coords: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_frames_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_frame_duration(&mut self, atlas_coords: Vector2i, frame_index: i32, duration: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32, f32);
            let args = (atlas_coords, frame_index, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tile_animation_frame_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_frame_duration(&self, atlas_coords: Vector2i, frame_index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2i, i32);
            let args = (atlas_coords, frame_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_frame_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_total_duration(&self, atlas_coords: Vector2i,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_animation_total_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_alternative_tile_full(&mut self, atlas_coords: Vector2i, alternative_id_override: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i, i32);
            let args = (atlas_coords, alternative_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_alternative_tile(&mut self, atlas_coords: Vector2i,) -> i32 {
            self.create_alternative_tile_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn create_alternative_tile_ex(&mut self, atlas_coords: Vector2i,) -> ExCreateAlternativeTile < '_ > {
            ExCreateAlternativeTile::new(self, atlas_coords,)
        }
        pub fn remove_alternative_tile(&mut self, atlas_coords: Vector2i, alternative_tile: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alternative_tile_id(&mut self, atlas_coords: Vector2i, alternative_tile: i32, new_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32, i32);
            let args = (atlas_coords, alternative_tile, new_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alternative_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_alternative_tile_id(&self, atlas_coords: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_alternative_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_data(&self, atlas_coords: Vector2i, alternative_tile: i32,) -> Option < Gd < crate::engine::TileData > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TileData > >;
            type CallSig = (Option < Gd < crate::engine::TileData > >, Vector2i, i32);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_atlas_grid_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_atlas_grid_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_tile_texture_region_full(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i, Vector2i, i32);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tile_texture_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_tile_texture_region(&self, atlas_coords: Vector2i,) -> Rect2i {
            self.get_tile_texture_region_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn get_tile_texture_region_ex(&self, atlas_coords: Vector2i,) -> ExGetTileTextureRegion < '_ > {
            ExGetTileTextureRegion::new(self, atlas_coords,)
        }
        pub fn get_runtime_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_runtime_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_runtime_tile_texture_region(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i, Vector2i, i32);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_runtime_tile_texture_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const TRANSFORM_FLIP_H: i32 = 4096i32;
        pub const TRANSFORM_FLIP_V: i32 = 8192i32;
        pub const TRANSFORM_TRANSPOSE: i32 = 16384i32;
        
    }
    impl crate::obj::GodotClass for TileSetAtlasSource {
        type Base = crate::engine::TileSetSource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileSetAtlasSource\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetAtlasSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileSetAtlasSource {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::TileSetSource > for TileSetAtlasSource {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TileSetAtlasSource {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TileSetAtlasSource {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileSetAtlasSource {
        
    }
    impl crate::obj::ExportableObject for TileSetAtlasSource {
        
    }
    impl crate::obj::cap::GodotDefault for TileSetAtlasSource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSetAtlasSource {
        type Target = crate::engine::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetAtlasSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileSetAtlasSource {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileSetAtlasSource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::TileSetSource > for $Class {
                
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
#[doc = "Default-param extender for [`TileSetAtlasSource::create_tile_ex`][super::TileSetAtlasSource::create_tile_ex]."]
#[must_use]
pub struct ExCreateTile < 'a > {
    surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        Self {
            surround_object, atlas_coords, size: Vector2i::new(1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn size(self, value: Vector2i) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSetAtlasSource::create_tile_full(self.surround_object, self.atlas_coords, self.size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::move_tile_in_atlas_ex`][super::TileSetAtlasSource::move_tile_in_atlas_ex]."]
#[must_use]
pub struct ExMoveTileInAtlas < 'a > {
    surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveTileInAtlas < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        Self {
            surround_object, atlas_coords, new_atlas_coords: Vector2i::new(- 1 as _, - 1 as _), new_size: Vector2i::new(- 1 as _, - 1 as _),
        }
    }
    #[inline]
    pub fn new_atlas_coords(self, value: Vector2i) -> Self {
        Self {
            new_atlas_coords: value, .. self
        }
    }
    #[inline]
    pub fn new_size(self, value: Vector2i) -> Self {
        Self {
            new_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileSetAtlasSource::move_tile_in_atlas_full(self.surround_object, self.atlas_coords, self.new_atlas_coords, self.new_size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::has_room_for_tile_ex`][super::TileSetAtlasSource::has_room_for_tile_ex]."]
#[must_use]
pub struct ExHasRoomForTile < 'a > {
    surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasRoomForTile < 'a > {
    fn new(surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> Self {
        Self {
            surround_object, atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile: Vector2i::new(- 1 as _, - 1 as _),
        }
    }
    #[inline]
    pub fn ignored_tile(self, value: Vector2i) -> Self {
        Self {
            ignored_tile: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TileSetAtlasSource::has_room_for_tile_full(self.surround_object, self.atlas_coords, self.size, self.animation_columns, self.animation_separation, self.frames_count, self.ignored_tile,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::create_alternative_tile_ex`][super::TileSetAtlasSource::create_alternative_tile_ex]."]
#[must_use]
pub struct ExCreateAlternativeTile < 'a > {
    surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, alternative_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAlternativeTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        Self {
            surround_object, atlas_coords, alternative_id_override: - 1i32,
        }
    }
    #[inline]
    pub fn alternative_id_override(self, value: i32) -> Self {
        Self {
            alternative_id_override: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileSetAtlasSource::create_alternative_tile_full(self.surround_object, self.atlas_coords, self.alternative_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::get_tile_texture_region_ex`][super::TileSetAtlasSource::get_tile_texture_region_ex]."]
#[must_use]
pub struct ExGetTileTextureRegion < 'a > {
    surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, frame: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTileTextureRegion < 'a > {
    fn new(surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        Self {
            surround_object, atlas_coords, frame: 0i32,
        }
    }
    #[inline]
    pub fn frame(self, value: i32) -> Self {
        Self {
            frame: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        re_export::TileSetAtlasSource::get_tile_texture_region_full(self.surround_object, self.atlas_coords, self.frame,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TileAnimationMode {
    ord: i32
}
impl TileAnimationMode {
    pub const TILE_ANIMATION_MODE_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const TILE_ANIMATION_MODE_RANDOM_START_TIMES: Self = Self {
        ord: 1i32
    };
    pub const TILE_ANIMATION_MODE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TileAnimationMode {
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
impl crate::obj::IndexEnum for TileAnimationMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for TileAnimationMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TileAnimationMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TileAnimationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}