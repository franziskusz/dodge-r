#![doc = "Sidecar module for class [`Camera3D`][crate::engine::Camera3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera3D` enums](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`camera_3d`][crate::engine::camera_3d]: sidecar module with related enum/flag types\n* [`ICamera3D`][crate::engine::ICamera3D]: virtual methods\n\n\nSee also [Godot docs for `Camera3D`](https://docs.godotengine.org/en/stable/classes/class_camera3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Camera3D`][crate::engine::Camera3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Camera3D` methods](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Camera3D {
        pub fn project_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector2);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "project_ray_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn project_local_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector2);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "project_local_ray_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn project_ray_origin(&self, screen_point: Vector2,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector2);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "project_ray_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unproject_position(&self, world_point: Vector3,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector3);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unproject_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_position_behind(&self, world_point: Vector3,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector3);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_position_behind", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn project_position(&self, screen_point: Vector2, z_depth: f32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector2, f32);
            let args = (screen_point, z_depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "project_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_perspective(&mut self, fov: f32, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, f32, f32);
            let args = (fov, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_perspective", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_orthogonal(&mut self, size: f32, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, f32, f32);
            let args = (size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_orthogonal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frustum(&mut self, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, Vector2, f32, f32);
            let args = (size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frustum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_current(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn clear_current_full(&mut self, enable_next: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable_next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn clear_current(&mut self,) {
            self.clear_current_ex() . done()
        }
        #[inline]
        pub fn clear_current_ex(&mut self,) -> ExClearCurrent < '_ > {
            ExClearCurrent::new(self,)
        }
        pub fn set_current(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_current(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_transform(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_projection(&self,) -> Projection {
            type RetMarshal = PtrcallReturnT < Projection >;
            type CallSig = (Projection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_projection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fov(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frustum_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frustum_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_far(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_far", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_near(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_near", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fov(&mut self, fov: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (fov,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frustum_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frustum_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_far(&mut self, far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_far", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_near(&mut self, near: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (near,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_near", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_projection(&self,) -> crate::engine::camera_3d::ProjectionType {
            type RetMarshal = PtrcallReturnT < crate::engine::camera_3d::ProjectionType >;
            type CallSig = (crate::engine::camera_3d::ProjectionType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_projection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_projection(&mut self, mode: crate::engine::camera_3d::ProjectionType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::camera_3d::ProjectionType);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_projection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_offset(&mut self, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_offset(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_offset(&mut self, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_offset(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment(&mut self, env: Gd < crate::engine::Environment >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Environment >);
            let args = (env,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment(&self,) -> Option < Gd < crate::engine::Environment > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Environment > >;
            type CallSig = (Option < Gd < crate::engine::Environment > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attributes(&mut self, env: Gd < crate::engine::CameraAttributes >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CameraAttributes >);
            let args = (env,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attributes(&self,) -> Option < Gd < crate::engine::CameraAttributes > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CameraAttributes > >;
            type CallSig = (Option < Gd < crate::engine::CameraAttributes > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keep_aspect_mode(&mut self, mode: crate::engine::camera_3d::KeepAspect,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::camera_3d::KeepAspect);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keep_aspect_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keep_aspect_mode(&self,) -> crate::engine::camera_3d::KeepAspect {
            type RetMarshal = PtrcallReturnT < crate::engine::camera_3d::KeepAspect >;
            type CallSig = (crate::engine::camera_3d::KeepAspect,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keep_aspect_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::engine::camera_3d::DopplerTracking,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::camera_3d::DopplerTracking);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::engine::camera_3d::DopplerTracking {
            type RetMarshal = PtrcallReturnT < crate::engine::camera_3d::DopplerTracking >;
            type CallSig = (crate::engine::camera_3d::DopplerTracking,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frustum(&self,) -> Array < Plane > {
            type RetMarshal = PtrcallReturnT < Array < Plane > >;
            type CallSig = (Array < Plane >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frustum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_position_in_frustum(&self, world_point: Vector3,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector3);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_position_in_frustum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_rid(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pyramid_shape_rid(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pyramid_shape_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mask_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cull_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mask_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cull_mask_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Camera3D {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Camera3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Camera3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for Camera3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Camera3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Camera3D {
        
    }
    impl crate::obj::ExportableObject for Camera3D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Camera3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Camera3D > for $Class {
                
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
#[doc = "Default-param extender for [`Camera3D::clear_current_ex`][super::Camera3D::clear_current_ex]."]
#[must_use]
pub struct ExClearCurrent < 'a > {
    surround_object: &'a mut re_export::Camera3D, enable_next: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearCurrent < 'a > {
    fn new(surround_object: &'a mut re_export::Camera3D,) -> Self {
        Self {
            surround_object, enable_next: true,
        }
    }
    #[inline]
    pub fn enable_next(self, value: bool) -> Self {
        Self {
            enable_next: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Camera3D::clear_current_full(self.surround_object, self.enable_next,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProjectionType {
    ord: i32
}
impl ProjectionType {
    pub const PROJECTION_PERSPECTIVE: Self = Self {
        ord: 0i32
    };
    pub const PROJECTION_ORTHOGONAL: Self = Self {
        ord: 1i32
    };
    pub const PROJECTION_FRUSTUM: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ProjectionType {
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
impl crate::builtin::meta::GodotConvert for ProjectionType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ProjectionType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ProjectionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct KeepAspect {
    ord: i32
}
impl KeepAspect {
    pub const KEEP_WIDTH: Self = Self {
        ord: 0i32
    };
    pub const KEEP_HEIGHT: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for KeepAspect {
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
impl crate::builtin::meta::GodotConvert for KeepAspect {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for KeepAspect {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for KeepAspect {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    pub const DOPPLER_TRACKING_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DOPPLER_TRACKING_IDLE_STEP: Self = Self {
        ord: 1i32
    };
    pub const DOPPLER_TRACKING_PHYSICS_STEP: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DopplerTracking {
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
impl crate::builtin::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DopplerTracking {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}