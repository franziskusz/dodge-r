#![doc = "Sidecar module for class [`BaseMaterial3D`][crate::engine::BaseMaterial3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseMaterial3D` enums](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BaseMaterial3D.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`base_material_3d`][crate::engine::base_material_3d]: sidecar module with related enum/flag types\n* [`IBaseMaterial3D`][crate::engine::IBaseMaterial3D]: virtual methods\n\n\nSee also [Godot docs for `BaseMaterial3D`](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BaseMaterial3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`BaseMaterial3D`][crate::engine::BaseMaterial3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BaseMaterial3D` methods](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBaseMaterial3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl BaseMaterial3D {
        pub fn set_albedo(&mut self, albedo: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: crate::engine::base_material_3d::Transparency,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::Transparency);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transparency(&self,) -> crate::engine::base_material_3d::Transparency {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::Transparency >;
            type CallSig = (crate::engine::base_material_3d::Transparency,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::engine::base_material_3d::AlphaAntiAliasing,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::AlphaAntiAliasing);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::engine::base_material_3d::AlphaAntiAliasing {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::AlphaAntiAliasing >;
            type CallSig = (crate::engine::base_material_3d::AlphaAntiAliasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shading_mode(&mut self, shading_mode: crate::engine::base_material_3d::ShadingMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::ShadingMode);
            let args = (shading_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shading_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shading_mode(&self,) -> crate::engine::base_material_3d::ShadingMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::ShadingMode >;
            type CallSig = (crate::engine::base_material_3d::ShadingMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shading_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_specular(&mut self, specular: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (specular,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_specular", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_specular(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_specular", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metallic(&mut self, metallic: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (metallic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_metallic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metallic(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_metallic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roughness(&mut self, roughness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roughness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_energy_multiplier(&mut self, emission_energy_multiplier: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_energy_multiplier(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_intensity(&mut self, emission_energy_multiplier: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_intensity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normal_scale(&mut self, normal_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (normal_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_normal_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normal_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_normal_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rim(&mut self, rim: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (rim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rim", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rim(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rim", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rim_tint(&mut self, rim_tint: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (rim_tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rim_tint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rim_tint(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rim_tint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clearcoat(&mut self, clearcoat: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (clearcoat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clearcoat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clearcoat(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_clearcoat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clearcoat_roughness(&mut self, clearcoat_roughness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (clearcoat_roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clearcoat_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clearcoat_roughness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_clearcoat_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anisotropy(&mut self, anisotropy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (anisotropy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anisotropy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_scale(&mut self, heightmap_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (heightmap_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_heightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subsurface_scattering_strength(&mut self, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subsurface_scattering_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subsurface_scattering_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subsurface_scattering_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transmittance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transmittance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_depth(&mut self, depth: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transmittance_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_depth(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transmittance_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_boost(&mut self, boost: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (boost,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transmittance_boost", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_boost(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transmittance_boost", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_backlight(&mut self, backlight: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (backlight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_backlight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_backlight(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_backlight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refraction(&mut self, refraction: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (refraction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_refraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_refraction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_refraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_size(&mut self, point_size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (point_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_detail_uv(&mut self, detail_uv: crate::engine::base_material_3d::DetailUV,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::DetailUV);
            let args = (detail_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_detail_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_detail_uv(&self,) -> crate::engine::base_material_3d::DetailUV {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::DetailUV >;
            type CallSig = (crate::engine::base_material_3d::DetailUV,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_detail_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_mode(&mut self, blend_mode: crate::engine::base_material_3d::BlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::BlendMode);
            let args = (blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::engine::base_material_3d::BlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::BlendMode >;
            type CallSig = (crate::engine::base_material_3d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_draw_mode(&mut self, depth_draw_mode: crate::engine::base_material_3d::DepthDrawMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::DepthDrawMode);
            let args = (depth_draw_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth_draw_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_draw_mode(&self,) -> crate::engine::base_material_3d::DepthDrawMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::DepthDrawMode >;
            type CallSig = (crate::engine::base_material_3d::DepthDrawMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_draw_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mode(&mut self, cull_mode: crate::engine::base_material_3d::CullMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::CullMode);
            let args = (cull_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mode(&self,) -> crate::engine::base_material_3d::CullMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::CullMode >;
            type CallSig = (crate::engine::base_material_3d::CullMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_diffuse_mode(&mut self, diffuse_mode: crate::engine::base_material_3d::DiffuseMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::DiffuseMode);
            let args = (diffuse_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_diffuse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_diffuse_mode(&self,) -> crate::engine::base_material_3d::DiffuseMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::DiffuseMode >;
            type CallSig = (crate::engine::base_material_3d::DiffuseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_diffuse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_specular_mode(&mut self, specular_mode: crate::engine::base_material_3d::SpecularMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::SpecularMode);
            let args = (specular_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_specular_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_specular_mode(&self,) -> crate::engine::base_material_3d::SpecularMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::SpecularMode >;
            type CallSig = (crate::engine::base_material_3d::SpecularMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_specular_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::engine::base_material_3d::Flags, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::Flags, bool);
            let args = (flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::engine::base_material_3d::Flags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::base_material_3d::Flags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::engine::base_material_3d::TextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::engine::base_material_3d::TextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureFilter >;
            type CallSig = (crate::engine::base_material_3d::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_feature(&mut self, feature: crate::engine::base_material_3d::Feature, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::Feature, bool);
            let args = (feature, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_feature(&self, feature: crate::engine::base_material_3d::Feature,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::base_material_3d::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, param: crate::engine::base_material_3d::TextureParam, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureParam, Gd < crate::engine::Texture2D >);
            let args = (param, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, param: crate::engine::base_material_3d::TextureParam,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, crate::engine::base_material_3d::TextureParam);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_detail_blend_mode(&mut self, detail_blend_mode: crate::engine::base_material_3d::BlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::BlendMode);
            let args = (detail_blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_detail_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_detail_blend_mode(&self,) -> crate::engine::base_material_3d::BlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::BlendMode >;
            type CallSig = (crate::engine::base_material_3d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_detail_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_scale(&mut self, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv1_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_scale(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv1_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_offset(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv1_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_offset(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv1_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv1_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_triplanar_blend_sharpness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv1_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_scale(&mut self, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv2_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_scale(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv2_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_offset(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv2_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_offset(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv2_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv2_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_triplanar_blend_sharpness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uv2_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::engine::base_material_3d::BillboardMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::BillboardMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::engine::base_material_3d::BillboardMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::BillboardMode >;
            type CallSig = (crate::engine::base_material_3d::BillboardMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_h_frames(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_v_frames(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_loop(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_deep_parallax", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_heightmap_deep_parallax_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_heightmap_deep_parallax_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_min_layers(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_deep_parallax_min_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_min_layers(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_heightmap_deep_parallax_min_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_max_layers(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_deep_parallax_max_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_max_layers(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_heightmap_deep_parallax_max_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_tangent(&mut self, flip: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_deep_parallax_flip_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_tangent(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_heightmap_deep_parallax_flip_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_binormal(&mut self, flip: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_heightmap_deep_parallax_flip_binormal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_binormal(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_heightmap_deep_parallax_flip_binormal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_grow(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_grow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_grow(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_grow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_operator(&mut self, operator: crate::engine::base_material_3d::EmissionOperator,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::EmissionOperator);
            let args = (operator,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_operator(&self,) -> crate::engine::base_material_3d::EmissionOperator {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::EmissionOperator >;
            type CallSig = (crate::engine::base_material_3d::EmissionOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ao_light_affect(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ao_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ao_light_affect(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ao_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_grow_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_grow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_grow_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_grow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metallic_texture_channel(&mut self, channel: crate::engine::base_material_3d::TextureChannel,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_metallic_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metallic_texture_channel(&self,) -> crate::engine::base_material_3d::TextureChannel {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureChannel >;
            type CallSig = (crate::engine::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_metallic_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roughness_texture_channel(&mut self, channel: crate::engine::base_material_3d::TextureChannel,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_roughness_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roughness_texture_channel(&self,) -> crate::engine::base_material_3d::TextureChannel {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureChannel >;
            type CallSig = (crate::engine::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_roughness_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ao_texture_channel(&mut self, channel: crate::engine::base_material_3d::TextureChannel,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ao_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ao_texture_channel(&self,) -> crate::engine::base_material_3d::TextureChannel {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureChannel >;
            type CallSig = (crate::engine::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ao_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refraction_texture_channel(&mut self, channel: crate::engine::base_material_3d::TextureChannel,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_refraction_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_refraction_texture_channel(&self,) -> crate::engine::base_material_3d::TextureChannel {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureChannel >;
            type CallSig = (crate::engine::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_refraction_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_proximity_fade_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_proximity_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_proximity_fade_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_proximity_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_proximity_fade_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_proximity_fade_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_proximity_fade_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_proximity_fade_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, range: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_outline_size(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msdf_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_outline_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msdf_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade(&mut self, mode: crate::engine::base_material_3d::DistanceFadeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::DistanceFadeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade(&self,) -> crate::engine::base_material_3d::DistanceFadeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::DistanceFadeMode >;
            type CallSig = (crate::engine::base_material_3d::DistanceFadeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_max_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_max_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_min_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_min_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade_min_distance", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for BaseMaterial3D {
        type Base = crate::engine::Material;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"BaseMaterial3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BaseMaterial3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for BaseMaterial3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Material > for BaseMaterial3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for BaseMaterial3D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for BaseMaterial3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for BaseMaterial3D {
        
    }
    impl crate::obj::ExportableObject for BaseMaterial3D {
        
    }
    impl std::ops::Deref for BaseMaterial3D {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BaseMaterial3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_BaseMaterial3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::BaseMaterial3D > for $Class {
                
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
pub struct TextureParam {
    ord: i32
}
impl TextureParam {
    pub const TEXTURE_ALBEDO: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_METALLIC: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_ROUGHNESS: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_EMISSION: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_NORMAL: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_RIM: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_CLEARCOAT: Self = Self {
        ord: 6i32
    };
    pub const TEXTURE_FLOWMAP: Self = Self {
        ord: 7i32
    };
    pub const TEXTURE_AMBIENT_OCCLUSION: Self = Self {
        ord: 8i32
    };
    pub const TEXTURE_HEIGHTMAP: Self = Self {
        ord: 9i32
    };
    pub const TEXTURE_SUBSURFACE_SCATTERING: Self = Self {
        ord: 10i32
    };
    pub const TEXTURE_SUBSURFACE_TRANSMITTANCE: Self = Self {
        ord: 11i32
    };
    pub const TEXTURE_BACKLIGHT: Self = Self {
        ord: 12i32
    };
    pub const TEXTURE_REFRACTION: Self = Self {
        ord: 13i32
    };
    pub const TEXTURE_DETAIL_MASK: Self = Self {
        ord: 14i32
    };
    pub const TEXTURE_DETAIL_ALBEDO: Self = Self {
        ord: 15i32
    };
    pub const TEXTURE_DETAIL_NORMAL: Self = Self {
        ord: 16i32
    };
    pub const TEXTURE_ORM: Self = Self {
        ord: 17i32
    };
    pub const TEXTURE_MAX: Self = Self {
        ord: 18i32
    };
    
}
impl crate::obj::EngineEnum for TextureParam {
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
impl crate::obj::IndexEnum for TextureParam {
    const ENUMERATOR_COUNT: usize = 18usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    pub const TEXTURE_FILTER_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_FILTER_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_FILTER_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for TextureFilter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DetailUV {
    ord: i32
}
impl DetailUV {
    pub const DETAIL_UV_1: Self = Self {
        ord: 0i32
    };
    pub const DETAIL_UV_2: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for DetailUV {
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
impl crate::builtin::meta::GodotConvert for DetailUV {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DetailUV {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DetailUV {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Transparency {
    ord: i32
}
impl Transparency {
    pub const TRANSPARENCY_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const TRANSPARENCY_ALPHA: Self = Self {
        ord: 1i32
    };
    pub const TRANSPARENCY_ALPHA_SCISSOR: Self = Self {
        ord: 2i32
    };
    pub const TRANSPARENCY_ALPHA_HASH: Self = Self {
        ord: 3i32
    };
    pub const TRANSPARENCY_ALPHA_DEPTH_PRE_PASS: Self = Self {
        ord: 4i32
    };
    pub const TRANSPARENCY_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for Transparency {
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
impl crate::obj::IndexEnum for Transparency {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for Transparency {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Transparency {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Transparency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShadingMode {
    ord: i32
}
impl ShadingMode {
    pub const SHADING_MODE_UNSHADED: Self = Self {
        ord: 0i32
    };
    pub const SHADING_MODE_PER_PIXEL: Self = Self {
        ord: 1i32
    };
    pub const SHADING_MODE_PER_VERTEX: Self = Self {
        ord: 2i32
    };
    pub const SHADING_MODE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ShadingMode {
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
impl crate::obj::IndexEnum for ShadingMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ShadingMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShadingMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShadingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Feature {
    ord: i32
}
impl Feature {
    pub const FEATURE_EMISSION: Self = Self {
        ord: 0i32
    };
    pub const FEATURE_NORMAL_MAPPING: Self = Self {
        ord: 1i32
    };
    pub const FEATURE_RIM: Self = Self {
        ord: 2i32
    };
    pub const FEATURE_CLEARCOAT: Self = Self {
        ord: 3i32
    };
    pub const FEATURE_ANISOTROPY: Self = Self {
        ord: 4i32
    };
    pub const FEATURE_AMBIENT_OCCLUSION: Self = Self {
        ord: 5i32
    };
    pub const FEATURE_HEIGHT_MAPPING: Self = Self {
        ord: 6i32
    };
    pub const FEATURE_SUBSURFACE_SCATTERING: Self = Self {
        ord: 7i32
    };
    pub const FEATURE_SUBSURFACE_TRANSMITTANCE: Self = Self {
        ord: 8i32
    };
    pub const FEATURE_BACKLIGHT: Self = Self {
        ord: 9i32
    };
    pub const FEATURE_REFRACTION: Self = Self {
        ord: 10i32
    };
    pub const FEATURE_DETAIL: Self = Self {
        ord: 11i32
    };
    pub const FEATURE_MAX: Self = Self {
        ord: 12i32
    };
    
}
impl crate::obj::EngineEnum for Feature {
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
impl crate::obj::IndexEnum for Feature {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::builtin::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Feature {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    pub const BLEND_MODE_MIX: Self = Self {
        ord: 0i32
    };
    pub const BLEND_MODE_ADD: Self = Self {
        ord: 1i32
    };
    pub const BLEND_MODE_SUB: Self = Self {
        ord: 2i32
    };
    pub const BLEND_MODE_MUL: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for BlendMode {
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
impl crate::builtin::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlphaAntiAliasing {
    ord: i32
}
impl AlphaAntiAliasing {
    pub const ALPHA_ANTIALIASING_OFF: Self = Self {
        ord: 0i32
    };
    pub const ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE: Self = Self {
        ord: 1i32
    };
    pub const ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AlphaAntiAliasing {
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
impl crate::builtin::meta::GodotConvert for AlphaAntiAliasing {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AlphaAntiAliasing {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AlphaAntiAliasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DepthDrawMode {
    ord: i32
}
impl DepthDrawMode {
    pub const DEPTH_DRAW_OPAQUE_ONLY: Self = Self {
        ord: 0i32
    };
    pub const DEPTH_DRAW_ALWAYS: Self = Self {
        ord: 1i32
    };
    pub const DEPTH_DRAW_DISABLED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DepthDrawMode {
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
impl crate::builtin::meta::GodotConvert for DepthDrawMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DepthDrawMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DepthDrawMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CullMode {
    ord: i32
}
impl CullMode {
    pub const CULL_BACK: Self = Self {
        ord: 0i32
    };
    pub const CULL_FRONT: Self = Self {
        ord: 1i32
    };
    pub const CULL_DISABLED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CullMode {
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
impl crate::builtin::meta::GodotConvert for CullMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CullMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Flags {
    ord: i32
}
impl Flags {
    pub const FLAG_DISABLE_DEPTH_TEST: Self = Self {
        ord: 0i32
    };
    pub const FLAG_ALBEDO_FROM_VERTEX_COLOR: Self = Self {
        ord: 1i32
    };
    pub const FLAG_SRGB_VERTEX_COLOR: Self = Self {
        ord: 2i32
    };
    pub const FLAG_USE_POINT_SIZE: Self = Self {
        ord: 3i32
    };
    pub const FLAG_FIXED_SIZE: Self = Self {
        ord: 4i32
    };
    pub const FLAG_BILLBOARD_KEEP_SCALE: Self = Self {
        ord: 5i32
    };
    pub const FLAG_UV1_USE_TRIPLANAR: Self = Self {
        ord: 6i32
    };
    pub const FLAG_UV2_USE_TRIPLANAR: Self = Self {
        ord: 7i32
    };
    pub const FLAG_UV1_USE_WORLD_TRIPLANAR: Self = Self {
        ord: 8i32
    };
    pub const FLAG_UV2_USE_WORLD_TRIPLANAR: Self = Self {
        ord: 9i32
    };
    pub const FLAG_AO_ON_UV2: Self = Self {
        ord: 10i32
    };
    pub const FLAG_EMISSION_ON_UV2: Self = Self {
        ord: 11i32
    };
    pub const FLAG_ALBEDO_TEXTURE_FORCE_SRGB: Self = Self {
        ord: 12i32
    };
    pub const FLAG_DONT_RECEIVE_SHADOWS: Self = Self {
        ord: 13i32
    };
    pub const FLAG_DISABLE_AMBIENT_LIGHT: Self = Self {
        ord: 14i32
    };
    pub const FLAG_USE_SHADOW_TO_OPACITY: Self = Self {
        ord: 15i32
    };
    pub const FLAG_USE_TEXTURE_REPEAT: Self = Self {
        ord: 16i32
    };
    pub const FLAG_INVERT_HEIGHTMAP: Self = Self {
        ord: 17i32
    };
    pub const FLAG_SUBSURFACE_MODE_SKIN: Self = Self {
        ord: 18i32
    };
    pub const FLAG_PARTICLE_TRAILS_MODE: Self = Self {
        ord: 19i32
    };
    pub const FLAG_ALBEDO_TEXTURE_MSDF: Self = Self {
        ord: 20i32
    };
    pub const FLAG_DISABLE_FOG: Self = Self {
        ord: 21i32
    };
    pub const FLAG_MAX: Self = Self {
        ord: 22i32
    };
    
}
impl crate::obj::EngineEnum for Flags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Flags {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::builtin::meta::GodotConvert for Flags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Flags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DiffuseMode {
    ord: i32
}
impl DiffuseMode {
    pub const DIFFUSE_BURLEY: Self = Self {
        ord: 0i32
    };
    pub const DIFFUSE_LAMBERT: Self = Self {
        ord: 1i32
    };
    pub const DIFFUSE_LAMBERT_WRAP: Self = Self {
        ord: 2i32
    };
    pub const DIFFUSE_TOON: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for DiffuseMode {
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
impl crate::builtin::meta::GodotConvert for DiffuseMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DiffuseMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DiffuseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpecularMode {
    ord: i32
}
impl SpecularMode {
    pub const SPECULAR_SCHLICK_GGX: Self = Self {
        ord: 0i32
    };
    pub const SPECULAR_TOON: Self = Self {
        ord: 1i32
    };
    pub const SPECULAR_DISABLED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for SpecularMode {
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
impl crate::builtin::meta::GodotConvert for SpecularMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpecularMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpecularMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BillboardMode {
    ord: i32
}
impl BillboardMode {
    pub const BILLBOARD_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const BILLBOARD_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const BILLBOARD_FIXED_Y: Self = Self {
        ord: 2i32
    };
    pub const BILLBOARD_PARTICLES: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for BillboardMode {
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
impl crate::builtin::meta::GodotConvert for BillboardMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BillboardMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BillboardMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureChannel {
    ord: i32
}
impl TextureChannel {
    pub const TEXTURE_CHANNEL_RED: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_CHANNEL_GREEN: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_CHANNEL_BLUE: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_CHANNEL_ALPHA: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_CHANNEL_GRAYSCALE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TextureChannel {
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
impl crate::builtin::meta::GodotConvert for TextureChannel {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureChannel {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EmissionOperator {
    ord: i32
}
impl EmissionOperator {
    pub const EMISSION_OP_ADD: Self = Self {
        ord: 0i32
    };
    pub const EMISSION_OP_MULTIPLY: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for EmissionOperator {
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
impl crate::builtin::meta::GodotConvert for EmissionOperator {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EmissionOperator {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EmissionOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DistanceFadeMode {
    ord: i32
}
impl DistanceFadeMode {
    pub const DISTANCE_FADE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DISTANCE_FADE_PIXEL_ALPHA: Self = Self {
        ord: 1i32
    };
    pub const DISTANCE_FADE_PIXEL_DITHER: Self = Self {
        ord: 2i32
    };
    pub const DISTANCE_FADE_OBJECT_DITHER: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for DistanceFadeMode {
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
impl crate::builtin::meta::GodotConvert for DistanceFadeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DistanceFadeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DistanceFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}