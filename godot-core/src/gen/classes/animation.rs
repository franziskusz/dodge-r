#![doc = "Sidecar module for class [`Animation`][crate::engine::Animation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Animation` enums](https://docs.godotengine.org/en/stable/classes/class_animation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Animation.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation`][crate::engine::animation]: sidecar module with related enum/flag types\n* [`IAnimation`][crate::engine::IAnimation]: virtual methods\n\n\nSee also [Godot docs for `Animation`](https://docs.godotengine.org/en/stable/classes/class_animation.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Animation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Animation`][crate::engine::Animation].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Animation` methods](https://docs.godotengine.org/en/stable/classes/class_animation.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimation: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Animation {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_track_full(&mut self, type_: crate::engine::animation::TrackType, at_position: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::animation::TrackType, i32);
            let args = (type_, at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_track(&mut self, type_: crate::engine::animation::TrackType,) -> i32 {
            self.add_track_ex(type_,) . done()
        }
        #[inline]
        pub fn add_track_ex(&mut self, type_: crate::engine::animation::TrackType,) -> ExAddTrack < '_ > {
            ExAddTrack::new(self, type_,)
        }
        pub fn remove_track(&mut self, track_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_track_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_track_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_type(&self, track_idx: i32,) -> crate::engine::animation::TrackType {
            type RetMarshal = PtrcallReturnT < crate::engine::animation::TrackType >;
            type CallSig = (crate::engine::animation::TrackType, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_path(&self, track_idx: i32,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_path(&mut self, track_idx: i32, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, NodePath);
            let args = (track_idx, path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_track(&self, path: NodePath, type_: crate::engine::animation::TrackType,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, NodePath, crate::engine::animation::TrackType);
            let args = (path, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_move_up(&mut self, track_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_move_up", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_move_down(&mut self, track_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_move_down", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_move_to(&mut self, track_idx: i32, to_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (track_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_move_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_swap(&mut self, track_idx: i32, with_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (track_idx, with_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_swap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_imported(&mut self, track_idx: i32, imported: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (track_idx, imported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_imported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_is_imported(&self, track_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_is_imported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_enabled(&mut self, track_idx: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (track_idx, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_is_enabled(&self, track_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn position_track_insert_key(&mut self, track_idx: i32, time: f64, position: Vector3,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, Vector3);
            let args = (track_idx, time, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "position_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotation_track_insert_key(&mut self, track_idx: i32, time: f64, rotation: Quaternion,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, Quaternion);
            let args = (track_idx, time, rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotation_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scale_track_insert_key(&mut self, track_idx: i32, time: f64, scale: Vector3,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, Vector3);
            let args = (track_idx, time, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scale_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blend_shape_track_insert_key(&mut self, track_idx: i32, time: f64, amount: f32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, f32);
            let args = (track_idx, time, amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_shape_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn position_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32, f64);
            let args = (track_idx, time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "position_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotation_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion, i32, f64);
            let args = (track_idx, time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotation_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scale_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32, f64);
            let args = (track_idx, time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scale_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blend_shape_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, f64);
            let args = (track_idx, time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_shape_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn track_insert_key_full(&mut self, track_idx: i32, time: f64, key: Variant, transition: f32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, Variant, f32);
            let args = (track_idx, time, key, transition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn track_insert_key(&mut self, track_idx: i32, time: f64, key: Variant,) -> i32 {
            self.track_insert_key_ex(track_idx, time, key,) . done()
        }
        #[inline]
        pub fn track_insert_key_ex(&mut self, track_idx: i32, time: f64, key: Variant,) -> ExTrackInsertKey < '_ > {
            ExTrackInsertKey::new(self, track_idx, time, key,)
        }
        pub fn track_remove_key(&mut self, track_idx: i32, key_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_remove_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_remove_key_at_time(&mut self, track_idx: i32, time: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f64);
            let args = (track_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_remove_key_at_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_key_value(&mut self, track_idx: i32, key: i32, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Variant);
            let args = (track_idx, key, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_key_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_key_transition(&mut self, track_idx: i32, key_idx: i32, transition: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (track_idx, key_idx, transition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_key_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_key_time(&mut self, track_idx: i32, key_idx: i32, time: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f64);
            let args = (track_idx, key_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_key_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_key_transition(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_key_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_key_count(&self, track_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_key_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_key_value(&self, track_idx: i32, key_idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_key_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_key_time(&self, track_idx: i32, key_idx: i32,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_key_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn track_find_key_full(&self, track_idx: i32, time: f64, find_mode: crate::engine::animation::FindMode,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, crate::engine::animation::FindMode);
            let args = (track_idx, time, find_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_find_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn track_find_key(&self, track_idx: i32, time: f64,) -> i32 {
            self.track_find_key_ex(track_idx, time,) . done()
        }
        #[inline]
        pub fn track_find_key_ex(&self, track_idx: i32, time: f64,) -> ExTrackFindKey < '_ > {
            ExTrackFindKey::new(self, track_idx, time,)
        }
        pub fn track_set_interpolation_type(&mut self, track_idx: i32, interpolation: crate::engine::animation::InterpolationType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::animation::InterpolationType);
            let args = (track_idx, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_interpolation_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_interpolation_type(&self, track_idx: i32,) -> crate::engine::animation::InterpolationType {
            type RetMarshal = PtrcallReturnT < crate::engine::animation::InterpolationType >;
            type CallSig = (crate::engine::animation::InterpolationType, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_interpolation_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_set_interpolation_loop_wrap(&mut self, track_idx: i32, interpolation: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (track_idx, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_set_interpolation_loop_wrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_get_interpolation_loop_wrap(&self, track_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_get_interpolation_loop_wrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn track_is_compressed(&self, track_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "track_is_compressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn value_track_set_update_mode(&mut self, track_idx: i32, mode: crate::engine::animation::UpdateMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::animation::UpdateMode);
            let args = (track_idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "value_track_set_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn value_track_get_update_mode(&self, track_idx: i32,) -> crate::engine::animation::UpdateMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation::UpdateMode >;
            type CallSig = (crate::engine::animation::UpdateMode, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "value_track_get_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn value_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32, f64);
            let args = (track_idx, time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "value_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn method_track_get_name(&self, track_idx: i32, key_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "method_track_get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn method_track_get_params(&self, track_idx: i32, key_idx: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "method_track_get_params", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bezier_track_insert_key_full(&mut self, track_idx: i32, time: f64, value: f32, in_handle: Vector2, out_handle: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, f32, Vector2, Vector2);
            let args = (track_idx, time, value, in_handle, out_handle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bezier_track_insert_key(&mut self, track_idx: i32, time: f64, value: f32,) -> i32 {
            self.bezier_track_insert_key_ex(track_idx, time, value,) . done()
        }
        #[inline]
        pub fn bezier_track_insert_key_ex(&mut self, track_idx: i32, time: f64, value: f32,) -> ExBezierTrackInsertKey < '_ > {
            ExBezierTrackInsertKey::new(self, track_idx, time, value,)
        }
        pub fn bezier_track_set_key_value(&mut self, track_idx: i32, key_idx: i32, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (track_idx, key_idx, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_set_key_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bezier_track_set_key_in_handle_full(&mut self, track_idx: i32, key_idx: i32, in_handle: Vector2, balanced_value_time_ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Vector2, f32);
            let args = (track_idx, key_idx, in_handle, balanced_value_time_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_set_key_in_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bezier_track_set_key_in_handle(&mut self, track_idx: i32, key_idx: i32, in_handle: Vector2,) {
            self.bezier_track_set_key_in_handle_ex(track_idx, key_idx, in_handle,) . done()
        }
        #[inline]
        pub fn bezier_track_set_key_in_handle_ex(&mut self, track_idx: i32, key_idx: i32, in_handle: Vector2,) -> ExBezierTrackSetKeyInHandle < '_ > {
            ExBezierTrackSetKeyInHandle::new(self, track_idx, key_idx, in_handle,)
        }
        pub(crate) fn bezier_track_set_key_out_handle_full(&mut self, track_idx: i32, key_idx: i32, out_handle: Vector2, balanced_value_time_ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Vector2, f32);
            let args = (track_idx, key_idx, out_handle, balanced_value_time_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_set_key_out_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bezier_track_set_key_out_handle(&mut self, track_idx: i32, key_idx: i32, out_handle: Vector2,) {
            self.bezier_track_set_key_out_handle_ex(track_idx, key_idx, out_handle,) . done()
        }
        #[inline]
        pub fn bezier_track_set_key_out_handle_ex(&mut self, track_idx: i32, key_idx: i32, out_handle: Vector2,) -> ExBezierTrackSetKeyOutHandle < '_ > {
            ExBezierTrackSetKeyOutHandle::new(self, track_idx, key_idx, out_handle,)
        }
        pub fn bezier_track_get_key_value(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_get_key_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bezier_track_get_key_in_handle(&self, track_idx: i32, key_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_get_key_in_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bezier_track_get_key_out_handle(&self, track_idx: i32, key_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_get_key_out_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bezier_track_interpolate(&self, track_idx: i32, time: f64,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, f64);
            let args = (track_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bezier_track_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn audio_track_insert_key_full(&mut self, track_idx: i32, time: f64, stream: Gd < crate::engine::Resource >, start_offset: f32, end_offset: f32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, Gd < crate::engine::Resource >, f32, f32);
            let args = (track_idx, time, stream, start_offset, end_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn audio_track_insert_key(&mut self, track_idx: i32, time: f64, stream: Gd < crate::engine::Resource >,) -> i32 {
            self.audio_track_insert_key_ex(track_idx, time, stream,) . done()
        }
        #[inline]
        pub fn audio_track_insert_key_ex(&mut self, track_idx: i32, time: f64, stream: Gd < crate::engine::Resource >,) -> ExAudioTrackInsertKey < '_ > {
            ExAudioTrackInsertKey::new(self, track_idx, time, stream,)
        }
        pub fn audio_track_set_key_stream(&mut self, track_idx: i32, key_idx: i32, stream: Gd < crate::engine::Resource >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Gd < crate::engine::Resource >);
            let args = (track_idx, key_idx, stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_set_key_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_set_key_start_offset(&mut self, track_idx: i32, key_idx: i32, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (track_idx, key_idx, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_set_key_start_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_set_key_end_offset(&mut self, track_idx: i32, key_idx: i32, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (track_idx, key_idx, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_set_key_end_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_get_key_stream(&self, track_idx: i32, key_idx: i32,) -> Option < Gd < crate::engine::Resource > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Resource > >;
            type CallSig = (Option < Gd < crate::engine::Resource > >, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_get_key_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_get_key_start_offset(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_get_key_start_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_get_key_end_offset(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_get_key_end_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_set_use_blend(&mut self, track_idx: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (track_idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_set_use_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn audio_track_is_use_blend(&self, track_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "audio_track_is_use_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn animation_track_insert_key(&mut self, track_idx: i32, time: f64, animation: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f64, StringName);
            let args = (track_idx, time, animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "animation_track_insert_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn animation_track_set_key_animation(&mut self, track_idx: i32, key_idx: i32, animation: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, StringName);
            let args = (track_idx, key_idx, animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "animation_track_set_key_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn animation_track_get_key_animation(&self, track_idx: i32, key_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32, i32);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "animation_track_get_key_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_length(&mut self, time_sec: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: crate::engine::animation::LoopMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation::LoopMode);
            let args = (loop_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_loop_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loop_mode(&self,) -> crate::engine::animation::LoopMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation::LoopMode >;
            type CallSig = (crate::engine::animation::LoopMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loop_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, size_sec: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn copy_track(&mut self, track_idx: i32, to_animation: Gd < crate::engine::Animation >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Animation >);
            let args = (track_idx, to_animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "copy_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn compress_full(&mut self, page_size: u32, fps: u32, split_tolerance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, u32, f32);
            let args = (page_size, fps, split_tolerance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compress(&mut self,) {
            self.compress_ex() . done()
        }
        #[inline]
        pub fn compress_ex(&mut self,) -> ExCompress < '_ > {
            ExCompress::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for Animation {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Animation\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Animation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Animation {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Animation {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Animation {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Animation {
        
    }
    impl crate::obj::ExportableObject for Animation {
        
    }
    impl crate::obj::cap::GodotDefault for Animation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Animation {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Animation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Animation {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Animation > for $Class {
                
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
#[doc = "Default-param extender for [`Animation::add_track_ex`][super::Animation::add_track_ex]."]
#[must_use]
pub struct ExAddTrack < 'a > {
    surround_object: &'a mut re_export::Animation, type_: crate::engine::animation::TrackType, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTrack < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, type_: crate::engine::animation::TrackType,) -> Self {
        Self {
            surround_object, type_, at_position: - 1i32,
        }
    }
    #[inline]
    pub fn at_position(self, value: i32) -> Self {
        Self {
            at_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Animation::add_track_full(self.surround_object, self.type_, self.at_position,)
    }
}
#[doc = "Default-param extender for [`Animation::track_insert_key_ex`][super::Animation::track_insert_key_ex]."]
#[must_use]
pub struct ExTrackInsertKey < 'a > {
    surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, key: Variant, transition: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, key: Variant,) -> Self {
        Self {
            surround_object, track_idx, time, key, transition: 1f32,
        }
    }
    #[inline]
    pub fn transition(self, value: f32) -> Self {
        Self {
            transition: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Animation::track_insert_key_full(self.surround_object, self.track_idx, self.time, self.key, self.transition,)
    }
}
#[doc = "Default-param extender for [`Animation::track_find_key_ex`][super::Animation::track_find_key_ex]."]
#[must_use]
pub struct ExTrackFindKey < 'a > {
    surround_object: &'a re_export::Animation, track_idx: i32, time: f64, find_mode: crate::engine::animation::FindMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrackFindKey < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time: f64,) -> Self {
        Self {
            surround_object, track_idx, time, find_mode: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn find_mode(self, value: crate::engine::animation::FindMode) -> Self {
        Self {
            find_mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Animation::track_find_key_full(self.surround_object, self.track_idx, self.time, self.find_mode,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_insert_key_ex`][super::Animation::bezier_track_insert_key_ex]."]
#[must_use]
pub struct ExBezierTrackInsertKey < 'a > {
    surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, value: f32, in_handle: Vector2, out_handle: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, value: f32,) -> Self {
        Self {
            surround_object, track_idx, time, value, in_handle: Vector2::new(0 as _, 0 as _), out_handle: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn in_handle(self, value: Vector2) -> Self {
        Self {
            in_handle: value, .. self
        }
    }
    #[inline]
    pub fn out_handle(self, value: Vector2) -> Self {
        Self {
            out_handle: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Animation::bezier_track_insert_key_full(self.surround_object, self.track_idx, self.time, self.value, self.in_handle, self.out_handle,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_set_key_in_handle_ex`][super::Animation::bezier_track_set_key_in_handle_ex]."]
#[must_use]
pub struct ExBezierTrackSetKeyInHandle < 'a > {
    surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, in_handle: Vector2, balanced_value_time_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackSetKeyInHandle < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, in_handle: Vector2,) -> Self {
        Self {
            surround_object, track_idx, key_idx, in_handle, balanced_value_time_ratio: 1f32,
        }
    }
    #[inline]
    pub fn balanced_value_time_ratio(self, value: f32) -> Self {
        Self {
            balanced_value_time_ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Animation::bezier_track_set_key_in_handle_full(self.surround_object, self.track_idx, self.key_idx, self.in_handle, self.balanced_value_time_ratio,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_set_key_out_handle_ex`][super::Animation::bezier_track_set_key_out_handle_ex]."]
#[must_use]
pub struct ExBezierTrackSetKeyOutHandle < 'a > {
    surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, out_handle: Vector2, balanced_value_time_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackSetKeyOutHandle < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, out_handle: Vector2,) -> Self {
        Self {
            surround_object, track_idx, key_idx, out_handle, balanced_value_time_ratio: 1f32,
        }
    }
    #[inline]
    pub fn balanced_value_time_ratio(self, value: f32) -> Self {
        Self {
            balanced_value_time_ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Animation::bezier_track_set_key_out_handle_full(self.surround_object, self.track_idx, self.key_idx, self.out_handle, self.balanced_value_time_ratio,)
    }
}
#[doc = "Default-param extender for [`Animation::audio_track_insert_key_ex`][super::Animation::audio_track_insert_key_ex]."]
#[must_use]
pub struct ExAudioTrackInsertKey < 'a > {
    surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, stream: Gd < crate::engine::Resource >, start_offset: f32, end_offset: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAudioTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, stream: Gd < crate::engine::Resource >,) -> Self {
        Self {
            surround_object, track_idx, time, stream, start_offset: 0f32, end_offset: 0f32,
        }
    }
    #[inline]
    pub fn start_offset(self, value: f32) -> Self {
        Self {
            start_offset: value, .. self
        }
    }
    #[inline]
    pub fn end_offset(self, value: f32) -> Self {
        Self {
            end_offset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Animation::audio_track_insert_key_full(self.surround_object, self.track_idx, self.time, self.stream, self.start_offset, self.end_offset,)
    }
}
#[doc = "Default-param extender for [`Animation::compress_ex`][super::Animation::compress_ex]."]
#[must_use]
pub struct ExCompress < 'a > {
    surround_object: &'a mut re_export::Animation, page_size: u32, fps: u32, split_tolerance: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompress < 'a > {
    fn new(surround_object: &'a mut re_export::Animation,) -> Self {
        Self {
            surround_object, page_size: 8192u32, fps: 120u32, split_tolerance: 4f32,
        }
    }
    #[inline]
    pub fn page_size(self, value: u32) -> Self {
        Self {
            page_size: value, .. self
        }
    }
    #[inline]
    pub fn fps(self, value: u32) -> Self {
        Self {
            fps: value, .. self
        }
    }
    #[inline]
    pub fn split_tolerance(self, value: f32) -> Self {
        Self {
            split_tolerance: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Animation::compress_full(self.surround_object, self.page_size, self.fps, self.split_tolerance,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackType {
    ord: i32
}
impl TrackType {
    pub const TYPE_VALUE: Self = Self {
        ord: 0i32
    };
    pub const TYPE_POSITION_3D: Self = Self {
        ord: 1i32
    };
    pub const TYPE_ROTATION_3D: Self = Self {
        ord: 2i32
    };
    pub const TYPE_SCALE_3D: Self = Self {
        ord: 3i32
    };
    pub const TYPE_BLEND_SHAPE: Self = Self {
        ord: 4i32
    };
    pub const TYPE_METHOD: Self = Self {
        ord: 5i32
    };
    pub const TYPE_BEZIER: Self = Self {
        ord: 6i32
    };
    pub const TYPE_AUDIO: Self = Self {
        ord: 7i32
    };
    pub const TYPE_ANIMATION: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for TrackType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TrackType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TrackType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TrackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InterpolationType {
    ord: i32
}
impl InterpolationType {
    pub const INTERPOLATION_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const INTERPOLATION_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const INTERPOLATION_CUBIC: Self = Self {
        ord: 2i32
    };
    pub const INTERPOLATION_LINEAR_ANGLE: Self = Self {
        ord: 3i32
    };
    pub const INTERPOLATION_CUBIC_ANGLE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for InterpolationType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for InterpolationType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InterpolationType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InterpolationType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    pub const UPDATE_CONTINUOUS: Self = Self {
        ord: 0i32
    };
    pub const UPDATE_DISCRETE: Self = Self {
        ord: 1i32
    };
    pub const UPDATE_CAPTURE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for UpdateMode {
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
impl crate::builtin::meta::GodotConvert for UpdateMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for UpdateMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LoopMode {
    ord: i32
}
impl LoopMode {
    pub const LOOP_NONE: Self = Self {
        ord: 0i32
    };
    pub const LOOP_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const LOOP_PINGPONG: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LoopMode {
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
impl crate::builtin::meta::GodotConvert for LoopMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LoopMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LoopMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LoopedFlag {
    ord: i32
}
impl LoopedFlag {
    pub const LOOPED_FLAG_NONE: Self = Self {
        ord: 0i32
    };
    pub const LOOPED_FLAG_END: Self = Self {
        ord: 1i32
    };
    pub const LOOPED_FLAG_START: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LoopedFlag {
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
impl crate::builtin::meta::GodotConvert for LoopedFlag {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LoopedFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LoopedFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FindMode {
    ord: i32
}
impl FindMode {
    pub const FIND_MODE_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const FIND_MODE_APPROX: Self = Self {
        ord: 1i32
    };
    pub const FIND_MODE_EXACT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for FindMode {
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
impl crate::builtin::meta::GodotConvert for FindMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FindMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FindMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}