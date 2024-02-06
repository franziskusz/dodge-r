#![doc = "Sidecar module for class [`CharacterBody3D`][crate::engine::CharacterBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharacterBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharacterBody3D.`\n\nInherits [`PhysicsBody3D`][crate::engine::PhysicsBody3D].\n\nRelated symbols:\n\n* [`character_body_3d`][crate::engine::character_body_3d]: sidecar module with related enum/flag types\n* [`ICharacterBody3D`][crate::engine::ICharacterBody3D]: virtual methods\n\n\nSee also [Godot docs for `CharacterBody3D`](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharacterBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CharacterBody3D`][crate::engine::CharacterBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CharacterBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharacterBody3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CharacterBody3D {
        pub fn move_and_slide(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_and_slide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_floor_snap(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_floor_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity(&mut self, velocity: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_safe_margin(&mut self, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_safe_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_safe_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_safe_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_stop_on_slope_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_floor_stop_on_slope_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_stop_on_slope_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_floor_stop_on_slope_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_constant_speed_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_floor_constant_speed_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_constant_speed_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_floor_constant_speed_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_block_on_wall_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_floor_block_on_wall_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_block_on_wall_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_floor_block_on_wall_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_slide_on_ceiling_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_slide_on_ceiling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_slide_on_ceiling_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_slide_on_ceiling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_floor_layers(&mut self, exclude_layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_platform_floor_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_floor_layers(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_platform_floor_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_wall_layers(&mut self, exclude_layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_platform_wall_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_wall_layers(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_platform_wall_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_slides(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_slides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_slides(&mut self, max_slides: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_slides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_slides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_max_angle(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_floor_max_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_max_angle(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_floor_max_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_snap_length(&mut self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_floor_snap_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_snap_length(&mut self, floor_snap_length: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (floor_snap_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_floor_snap_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wall_min_slide_angle(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_wall_min_slide_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wall_min_slide_angle(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_wall_min_slide_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_up_direction(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_up_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_up_direction(&mut self, up_direction: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_up_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_mode(&mut self, mode: crate::engine::character_body_3d::MotionMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::character_body_3d::MotionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_mode(&self,) -> crate::engine::character_body_3d::MotionMode {
            type RetMarshal = PtrcallReturnT < crate::engine::character_body_3d::MotionMode >;
            type CallSig = (crate::engine::character_body_3d::MotionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_on_leave(&mut self, on_leave_apply_velocity: crate::engine::character_body_3d::PlatformOnLeave,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::character_body_3d::PlatformOnLeave);
            let args = (on_leave_apply_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_platform_on_leave", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_on_leave(&self,) -> crate::engine::character_body_3d::PlatformOnLeave {
            type RetMarshal = PtrcallReturnT < crate::engine::character_body_3d::PlatformOnLeave >;
            type CallSig = (crate::engine::character_body_3d::PlatformOnLeave,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_platform_on_leave", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_floor(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_floor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_floor_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_floor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_ceiling(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_ceiling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_ceiling_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_ceiling_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_wall(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_wall", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_wall_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_on_wall_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_normal(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_floor_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wall_normal(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_wall_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_motion(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position_delta(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_real_velocity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_real_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_floor_angle_full(&self, up_direction: Vector3,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector3);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_floor_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_floor_angle(&self,) -> f32 {
            self.get_floor_angle_ex() . done()
        }
        #[inline]
        pub fn get_floor_angle_ex(&self,) -> ExGetFloorAngle < '_ > {
            ExGetFloorAngle::new(self,)
        }
        pub fn get_platform_velocity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_platform_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_angular_velocity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_platform_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_slide_collision_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_slide_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_slide_collision(&mut self, slide_idx: i32,) -> Option < Gd < crate::engine::KinematicCollision3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::KinematicCollision3D > >;
            type CallSig = (Option < Gd < crate::engine::KinematicCollision3D > >, i32);
            let args = (slide_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_slide_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_slide_collision(&mut self,) -> Option < Gd < crate::engine::KinematicCollision3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::KinematicCollision3D > >;
            type CallSig = (Option < Gd < crate::engine::KinematicCollision3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_slide_collision", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CharacterBody3D {
        type Base = crate::engine::PhysicsBody3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CharacterBody3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharacterBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CharacterBody3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PhysicsBody3D > for CharacterBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::CollisionObject3D > for CharacterBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for CharacterBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for CharacterBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CharacterBody3D {
        
    }
    impl crate::obj::ExportableObject for CharacterBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for CharacterBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharacterBody3D {
        type Target = crate::engine::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharacterBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CharacterBody3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CharacterBody3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsBody3D > for $Class {
                
            }
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
#[doc = "Default-param extender for [`CharacterBody3D::get_floor_angle_ex`][super::CharacterBody3D::get_floor_angle_ex]."]
#[must_use]
pub struct ExGetFloorAngle < 'a > {
    surround_object: &'a re_export::CharacterBody3D, up_direction: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetFloorAngle < 'a > {
    fn new(surround_object: &'a re_export::CharacterBody3D,) -> Self {
        Self {
            surround_object, up_direction: Vector3::new(0 as _, 1 as _, 0 as _),
        }
    }
    #[inline]
    pub fn up_direction(self, value: Vector3) -> Self {
        Self {
            up_direction: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::CharacterBody3D::get_floor_angle_full(self.surround_object, self.up_direction,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MotionMode {
    ord: i32
}
impl MotionMode {
    pub const MOTION_MODE_GROUNDED: Self = Self {
        ord: 0i32
    };
    pub const MOTION_MODE_FLOATING: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for MotionMode {
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
impl crate::builtin::meta::GodotConvert for MotionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MotionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MotionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PlatformOnLeave {
    ord: i32
}
impl PlatformOnLeave {
    pub const PLATFORM_ON_LEAVE_ADD_VELOCITY: Self = Self {
        ord: 0i32
    };
    pub const PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY: Self = Self {
        ord: 1i32
    };
    pub const PLATFORM_ON_LEAVE_DO_NOTHING: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PlatformOnLeave {
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
impl crate::builtin::meta::GodotConvert for PlatformOnLeave {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PlatformOnLeave {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PlatformOnLeave {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}