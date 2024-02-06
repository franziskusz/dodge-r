#![doc = "Sidecar module for class [`ParticleProcessMaterial`][crate::engine::ParticleProcessMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ParticleProcessMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ParticleProcessMaterial.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`particle_process_material`][crate::engine::particle_process_material]: sidecar module with related enum/flag types\n* [`IParticleProcessMaterial`][crate::engine::IParticleProcessMaterial]: virtual methods\n\n\nSee also [Godot docs for `ParticleProcessMaterial`](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ParticleProcessMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ParticleProcessMaterial`][crate::engine::ParticleProcessMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ParticleProcessMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IParticleProcessMaterial: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_shader_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn get_shader_mode(&self,) -> crate::engine::shader::Mode {
            unimplemented !()
        }
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl ParticleProcessMaterial {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_direction(&mut self, degrees: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inherit_velocity_ratio(&mut self, ratio: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inherit_velocity_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inherit_velocity_ratio(&mut self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inherit_velocity_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spread(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flatness(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flatness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::engine::particle_process_material::Parameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::engine::particle_process_material::Parameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::engine::particle_process_material::Parameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::engine::particle_process_material::Parameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_texture(&mut self, param: crate::engine::particle_process_material::Parameter, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::Parameter, Gd < crate::engine::Texture2D >);
            let args = (param, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_texture(&self, param: crate::engine::particle_process_material::Parameter,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, crate::engine::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (ramp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_curve(&mut self, curve: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_curve(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_curve(&mut self, curve: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_curve(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (ramp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity_limit_curve(&mut self, curve: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_velocity_limit_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_limit_curve(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity_limit_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::engine::particle_process_material::ParticleFlags, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::ParticleFlags, bool);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::engine::particle_process_material::ParticleFlags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::particle_process_material::ParticleFlags);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity_pivot(&mut self, pivot: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (pivot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_velocity_pivot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_pivot(&mut self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity_pivot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::engine::particle_process_material::EmissionShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::EmissionShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::engine::particle_process_material::EmissionShape {
            type RetMarshal = PtrcallReturnT < crate::engine::particle_process_material::EmissionShape >;
            type CallSig = (crate::engine::particle_process_material::EmissionShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_box_extents(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_point_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_point_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_point_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_point_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_normal_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_normal_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_normal_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_normal_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_color_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_color_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_point_count(&mut self, point_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (point_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_point_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_axis(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_height(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape_offset(&mut self, emission_shape_offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (emission_shape_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_shape_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape_offset(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_shape_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape_scale(&mut self, emission_shape_scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (emission_shape_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_shape_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape_scale(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_shape_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_turbulence_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_enabled(&mut self, turbulence_enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (turbulence_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_turbulence_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_turbulence_noise_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_strength(&mut self, turbulence_noise_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (turbulence_noise_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_turbulence_noise_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_turbulence_noise_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_scale(&mut self, turbulence_noise_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (turbulence_noise_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_turbulence_noise_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_speed_random(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_turbulence_noise_speed_random", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_speed_random(&mut self, turbulence_noise_speed_random: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (turbulence_noise_speed_random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_turbulence_noise_speed_random", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_speed(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_turbulence_noise_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_speed(&mut self, turbulence_noise_speed: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (turbulence_noise_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_turbulence_noise_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, randomness: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (randomness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_mode(&self,) -> crate::engine::particle_process_material::SubEmitterMode {
            type RetMarshal = PtrcallReturnT < crate::engine::particle_process_material::SubEmitterMode >;
            type CallSig = (crate::engine::particle_process_material::SubEmitterMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_mode(&mut self, mode: crate::engine::particle_process_material::SubEmitterMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::SubEmitterMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_frequency(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_frequency(&mut self, hz: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_end(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter_amount_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_end(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter_amount_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_collision(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter_amount_at_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_collision(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter_amount_at_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_keep_velocity(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sub_emitter_keep_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_keep_velocity(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sub_emitter_keep_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attractor_interaction_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attractor_interaction_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_attractor_interaction_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_attractor_interaction_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mode(&mut self, mode: crate::engine::particle_process_material::CollisionMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::particle_process_material::CollisionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mode(&self,) -> crate::engine::particle_process_material::CollisionMode {
            type RetMarshal = PtrcallReturnT < crate::engine::particle_process_material::CollisionMode >;
            type CallSig = (crate::engine::particle_process_material::CollisionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_use_scale(&mut self, radius: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_use_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collision_using_scale(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collision_using_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_friction(&mut self, friction: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_friction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_bounce(&mut self, bounce: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_bounce", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_bounce(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_bounce", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ParticleProcessMaterial {
        type Base = crate::engine::Material;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ParticleProcessMaterial\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ParticleProcessMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ParticleProcessMaterial {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Material > for ParticleProcessMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ParticleProcessMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ParticleProcessMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ParticleProcessMaterial {
        
    }
    impl crate::obj::ExportableObject for ParticleProcessMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for ParticleProcessMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ParticleProcessMaterial {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ParticleProcessMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ParticleProcessMaterial {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ParticleProcessMaterial > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Material > for $Class {
                
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
    pub const PARAM_RADIAL_VELOCITY: Self = Self {
        ord: 15i32
    };
    pub const PARAM_DIRECTIONAL_VELOCITY: Self = Self {
        ord: 16i32
    };
    pub const PARAM_SCALE_OVER_VELOCITY: Self = Self {
        ord: 17i32
    };
    pub const PARAM_MAX: Self = Self {
        ord: 18i32
    };
    pub const PARAM_TURB_VEL_INFLUENCE: Self = Self {
        ord: 13i32
    };
    pub const PARAM_TURB_INIT_DISPLACEMENT: Self = Self {
        ord: 14i32
    };
    pub const PARAM_TURB_INFLUENCE_OVER_LIFE: Self = Self {
        ord: 12i32
    };
    
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 18usize;
    
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
    pub const PARTICLE_FLAG_DAMPING_AS_FRICTION: Self = Self {
        ord: 3i32
    };
    pub const PARTICLE_FLAG_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ParticleFlags {
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
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SubEmitterMode {
    ord: i32
}
impl SubEmitterMode {
    pub const SUB_EMITTER_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SUB_EMITTER_CONSTANT: Self = Self {
        ord: 1i32
    };
    pub const SUB_EMITTER_AT_END: Self = Self {
        ord: 2i32
    };
    pub const SUB_EMITTER_AT_COLLISION: Self = Self {
        ord: 3i32
    };
    pub const SUB_EMITTER_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SubEmitterMode {
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
impl crate::obj::IndexEnum for SubEmitterMode {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for SubEmitterMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SubEmitterMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SubEmitterMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CollisionMode {
    ord: i32
}
impl CollisionMode {
    pub const COLLISION_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const COLLISION_RIGID: Self = Self {
        ord: 1i32
    };
    pub const COLLISION_HIDE_ON_CONTACT: Self = Self {
        ord: 2i32
    };
    pub const COLLISION_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for CollisionMode {
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
impl crate::obj::IndexEnum for CollisionMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for CollisionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CollisionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CollisionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}