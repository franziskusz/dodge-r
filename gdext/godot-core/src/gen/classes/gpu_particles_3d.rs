#![doc = "Sidecar module for class [`GpuParticles3D`][crate::engine::GpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`gpu_particles_3d`][crate::engine::gpu_particles_3d]: sidecar module with related enum/flag types\n* [`IGpuParticles3D`][crate::engine::IGpuParticles3D]: virtual methods\n\n\nSee also [Godot docs for `GPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GpuParticles3D`][crate::engine::GpuParticles3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticles3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GpuParticles3D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_aabb(&mut self, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolate(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_base_size(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interp_to_end(&mut self, interp: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (interp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolate(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_material(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_base_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interp_to_end(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::engine::gpu_particles_3d::DrawOrder,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gpu_particles_3d::DrawOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::engine::gpu_particles_3d::DrawOrder {
            type RetMarshal = PtrcallReturnT < crate::engine::gpu_particles_3d::DrawOrder >;
            type CallSig = (crate::engine::gpu_particles_3d::DrawOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_passes(&mut self, passes: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (passes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_passes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_pass_mesh(&mut self, pass: i32, mesh: Gd < crate::engine::Mesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Mesh >);
            let args = (pass, mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_pass_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_passes(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_passes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_pass_mesh(&self, pass: i32,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >, i32);
            let args = (pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_pass_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skin(&mut self, skin: Gd < crate::engine::Skin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Skin >);
            let args = (skin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::engine::Skin > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Skin > >;
            type CallSig = (Option < Gd < crate::engine::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn restart(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "capture_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn emit_particle(&mut self, xform: Transform3D, velocity: Vector3, color: Color, custom: Color, flags: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform3D, Vector3, Color, Color, u32);
            let args = (xform, velocity, color, custom, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "emit_particle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_lifetime(&mut self, secs: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_trail_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trail_lifetime(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform_align(&mut self, align: crate::engine::gpu_particles_3d::TransformAlign,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gpu_particles_3d::TransformAlign);
            let args = (align,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_align(&self,) -> crate::engine::gpu_particles_3d::TransformAlign {
            type RetMarshal = PtrcallReturnT < crate::engine::gpu_particles_3d::TransformAlign >;
            type CallSig = (crate::engine::gpu_particles_3d::TransformAlign,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_amount_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_amount_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const MAX_DRAW_PASSES: i32 = 4i32;
        
    }
    impl crate::obj::GodotClass for GpuParticles3D {
        type Base = crate::engine::GeometryInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GPUParticles3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticles3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GpuParticles3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for GpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for GpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for GpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for GpuParticles3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GpuParticles3D {
        
    }
    impl crate::obj::ExportableObject for GpuParticles3D {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticles3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticles3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GpuParticles3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GpuParticles3D > for $Class {
                
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
    pub const DRAW_ORDER_REVERSE_LIFETIME: Self = Self {
        ord: 2i32
    };
    pub const DRAW_ORDER_VIEW_DEPTH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for DrawOrder {
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
pub struct EmitFlags {
    ord: i32
}
impl EmitFlags {
    pub const EMIT_FLAG_POSITION: Self = Self {
        ord: 1i32
    };
    pub const EMIT_FLAG_ROTATION_SCALE: Self = Self {
        ord: 2i32
    };
    pub const EMIT_FLAG_VELOCITY: Self = Self {
        ord: 4i32
    };
    pub const EMIT_FLAG_COLOR: Self = Self {
        ord: 8i32
    };
    pub const EMIT_FLAG_CUSTOM: Self = Self {
        ord: 16i32
    };
    
}
impl crate::obj::EngineEnum for EmitFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for EmitFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EmitFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EmitFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TransformAlign {
    ord: i32
}
impl TransformAlign {
    pub const TRANSFORM_ALIGN_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const TRANSFORM_ALIGN_Z_BILLBOARD: Self = Self {
        ord: 1i32
    };
    pub const TRANSFORM_ALIGN_Y_TO_VELOCITY: Self = Self {
        ord: 2i32
    };
    pub const TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for TransformAlign {
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
impl crate::builtin::meta::GodotConvert for TransformAlign {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TransformAlign {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}