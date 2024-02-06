#![doc = "Sidecar module for class [`CpuParticles3D`][crate::engine::CpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`cpu_particles_3d`][crate::engine::cpu_particles_3d]: sidecar module with related enum/flag types\n* [`ICpuParticles3D`][crate::engine::ICpuParticles3D]: virtual methods\n\n\nSee also [Godot docs for `CPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CpuParticles3D`][crate::engine::CpuParticles3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICpuParticles3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl CpuParticles3D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, random: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::engine::cpu_particles_3d::DrawOrder,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::DrawOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::engine::cpu_particles_3d::DrawOrder {
            type RetMarshal = PtrcallReturnT < crate::engine::cpu_particles_3d::DrawOrder >;
            type CallSig = (crate::engine::cpu_particles_3d::DrawOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh(&mut self, mesh: Gd < crate::engine::Mesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn restart(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_direction(&mut self, direction: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spread(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flatness(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flatness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::engine::cpu_particles_3d::Parameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::engine::cpu_particles_3d::Parameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::cpu_particles_3d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::engine::cpu_particles_3d::Parameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::engine::cpu_particles_3d::Parameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::cpu_particles_3d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_curve(&mut self, param: crate::engine::cpu_particles_3d::Parameter, curve: Gd < crate::engine::Curve >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::Parameter, Gd < crate::engine::Curve >);
            let args = (param, curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_curve(&self, param: crate::engine::cpu_particles_3d::Parameter,) -> Option < Gd < crate::engine::Curve > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Curve > >;
            type CallSig = (Option < Gd < crate::engine::Curve > >, crate::engine::cpu_particles_3d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: Gd < crate::engine::Gradient >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Gradient >);
            let args = (ramp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::engine::Gradient > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Gradient > >;
            type CallSig = (Option < Gd < crate::engine::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: Gd < crate::engine::Gradient >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Gradient >);
            let args = (ramp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::engine::Gradient > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Gradient > >;
            type CallSig = (Option < Gd < crate::engine::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::engine::cpu_particles_3d::ParticleFlags, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::ParticleFlags, bool);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::engine::cpu_particles_3d::ParticleFlags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::cpu_particles_3d::ParticleFlags);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::engine::cpu_particles_3d::EmissionShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::cpu_particles_3d::EmissionShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::engine::cpu_particles_3d::EmissionShape {
            type RetMarshal = PtrcallReturnT < crate::engine::cpu_particles_3d::EmissionShape >;
            type CallSig = (crate::engine::cpu_particles_3d::EmissionShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_box_extents(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_points(&mut self, array: PackedVector3Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array);
            let args = (array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_points(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_normals(&mut self, array: PackedVector3Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array);
            let args = (array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_normals(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_colors(&mut self, array: PackedColorArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedColorArray);
            let args = (array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_colors(&self,) -> PackedColorArray {
            type RetMarshal = PtrcallReturnT < PackedColorArray >;
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_axis(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_height(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_split_scale(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_split_scale(&mut self, split_scale: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (split_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_x(&self,) -> Option < Gd < crate::engine::Curve > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Curve > >;
            type CallSig = (Option < Gd < crate::engine::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_x(&mut self, scale_curve: Gd < crate::engine::Curve >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Curve >);
            let args = (scale_curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_y(&self,) -> Option < Gd < crate::engine::Curve > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Curve > >;
            type CallSig = (Option < Gd < crate::engine::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_y(&mut self, scale_curve: Gd < crate::engine::Curve >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Curve >);
            let args = (scale_curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_z(&self,) -> Option < Gd < crate::engine::Curve > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Curve > >;
            type CallSig = (Option < Gd < crate::engine::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale_curve_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_z(&mut self, scale_curve: Gd < crate::engine::Curve >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Curve >);
            let args = (scale_curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale_curve_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CpuParticles3D {
        type Base = crate::engine::GeometryInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CPUParticles3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CpuParticles3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CpuParticles3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for CpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for CpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for CpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for CpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CpuParticles3D {
        
    }
    impl crate::obj::ExportableObject for CpuParticles3D {
        
    }
    impl crate::obj::cap::GodotDefault for CpuParticles3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CpuParticles3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CpuParticles3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CpuParticles3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::GeometryInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualInstance3D > for $Class {
                
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
pub struct DrawOrder {
    ord: i32
}
impl DrawOrder {
    pub const DRAW_ORDER_INDEX: Self = Self {
        ord: 0i32
    };
    pub const DRAW_ORDER_LIFETIME: Self = Self {
        ord: 1i32
    };
    pub const DRAW_ORDER_VIEW_DEPTH: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DrawOrder {
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
impl crate::builtin::meta::GodotConvert for DrawOrder {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DrawOrder {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Parameter {
    ord: i32
}
impl Parameter {
    pub const PARAM_INITIAL_LINEAR_VELOCITY: Self = Self {
        ord: 0i32
    };
    pub const PARAM_ANGULAR_VELOCITY: Self = Self {
        ord: 1i32
    };
    pub const PARAM_ORBIT_VELOCITY: Self = Self {
        ord: 2i32
    };
    pub const PARAM_LINEAR_ACCEL: Self = Self {
        ord: 3i32
    };
    pub const PARAM_RADIAL_ACCEL: Self = Self {
        ord: 4i32
    };
    pub const PARAM_TANGENTIAL_ACCEL: Self = Self {
        ord: 5i32
    };
    pub const PARAM_DAMPING: Self = Self {
        ord: 6i32
    };
    pub const PARAM_ANGLE: Self = Self {
        ord: 7i32
    };
    pub const PARAM_SCALE: Self = Self {
        ord: 8i32
    };
    pub const PARAM_HUE_VARIATION: Self = Self {
        ord: 9i32
    };
    pub const PARAM_ANIM_SPEED: Self = Self {
        ord: 10i32
    };
    pub const PARAM_ANIM_OFFSET: Self = Self {
        ord: 11i32
    };
    pub const PARAM_MAX: Self = Self {
        ord: 12i32
    };
    
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::builtin::meta::GodotConvert for Parameter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Parameter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Parameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticleFlags {
    ord: i32
}
impl ParticleFlags {
    pub const PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY: Self = Self {
        ord: 0i32
    };
    pub const PARTICLE_FLAG_ROTATE_Y: Self = Self {
        ord: 1i32
    };
    pub const PARTICLE_FLAG_DISABLE_Z: Self = Self {
        ord: 2i32
    };
    pub const PARTICLE_FLAG_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ParticleFlags {
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
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ParticleFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticleFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticleFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EmissionShape {
    ord: i32
}
impl EmissionShape {
    pub const EMISSION_SHAPE_POINT: Self = Self {
        ord: 0i32
    };
    pub const EMISSION_SHAPE_SPHERE: Self = Self {
        ord: 1i32
    };
    pub const EMISSION_SHAPE_SPHERE_SURFACE: Self = Self {
        ord: 2i32
    };
    pub const EMISSION_SHAPE_BOX: Self = Self {
        ord: 3i32
    };
    pub const EMISSION_SHAPE_POINTS: Self = Self {
        ord: 4i32
    };
    pub const EMISSION_SHAPE_DIRECTED_POINTS: Self = Self {
        ord: 5i32
    };
    pub const EMISSION_SHAPE_RING: Self = Self {
        ord: 6i32
    };
    pub const EMISSION_SHAPE_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for EmissionShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for EmissionShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EmissionShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EmissionShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}