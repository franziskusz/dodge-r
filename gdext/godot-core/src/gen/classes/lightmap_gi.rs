#![doc = "Sidecar module for class [`LightmapGi`][crate::engine::LightmapGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LightmapGI` enums](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LightmapGI.`\n\nInherits [`VisualInstance3D`][crate::engine::VisualInstance3D].\n\nRelated symbols:\n\n* [`lightmap_gi`][crate::engine::lightmap_gi]: sidecar module with related enum/flag types\n* [`ILightmapGi`][crate::engine::ILightmapGi]: virtual methods\n\n\nSee also [Godot docs for `LightmapGI`](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LightmapGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`LightmapGi`][crate::engine::LightmapGi].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `LightmapGI` methods](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILightmapGi: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LightmapGi {
        pub fn set_light_data(&mut self, data: Gd < crate::engine::LightmapGiData >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::LightmapGiData >);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_light_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_light_data(&self,) -> Option < Gd < crate::engine::LightmapGiData > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::LightmapGiData > >;
            type CallSig = (Option < Gd < crate::engine::LightmapGiData > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_light_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_quality(&mut self, bake_quality: crate::engine::lightmap_gi::BakeQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::lightmap_gi::BakeQuality);
            let args = (bake_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bake_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_quality(&self,) -> crate::engine::lightmap_gi::BakeQuality {
            type RetMarshal = PtrcallReturnT < crate::engine::lightmap_gi::BakeQuality >;
            type CallSig = (crate::engine::lightmap_gi::BakeQuality,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bounces(&mut self, bounces: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounces(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bounce_indirect_energy(&mut self, bounce_indirect_energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bounce_indirect_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bounce_indirect_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounce_indirect_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bounce_indirect_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_probes(&mut self, subdivision: crate::engine::lightmap_gi::GenerateProbes,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::lightmap_gi::GenerateProbes);
            let args = (subdivision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_generate_probes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_generate_probes(&self,) -> crate::engine::lightmap_gi::GenerateProbes {
            type RetMarshal = PtrcallReturnT < crate::engine::lightmap_gi::GenerateProbes >;
            type CallSig = (crate::engine::lightmap_gi::GenerateProbes,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_generate_probes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_mode(&mut self, mode: crate::engine::lightmap_gi::EnvironmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::lightmap_gi::EnvironmentMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_mode(&self,) -> crate::engine::lightmap_gi::EnvironmentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::lightmap_gi::EnvironmentMode >;
            type CallSig = (crate::engine::lightmap_gi::EnvironmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_sky(&mut self, sky: Gd < crate::engine::Sky >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Sky >);
            let args = (sky,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment_custom_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_sky(&self,) -> Option < Gd < crate::engine::Sky > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Sky > >;
            type CallSig = (Option < Gd < crate::engine::Sky > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment_custom_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_energy(&mut self, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment_custom_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment_custom_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_texture_size(&mut self, max_texture_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_texture_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_texture_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_denoiser(&mut self, use_denoiser: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_denoiser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_denoiser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_denoiser(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_denoiser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_denoiser_strength(&mut self, denoiser_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (denoiser_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_denoiser_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_denoiser_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_denoiser_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interior(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_interior(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_directional(&mut self, directional: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (directional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_directional", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_directional(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_directional", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_texture_for_bounces(&mut self, use_texture_for_bounces: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_texture_for_bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_texture_for_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_texture_for_bounces(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_texture_for_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: Gd < crate::engine::CameraAttributes >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CameraAttributes >);
            let args = (camera_attributes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::engine::CameraAttributes > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CameraAttributes > >;
            type CallSig = (Option < Gd < crate::engine::CameraAttributes > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_attributes", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for LightmapGi {
        type Base = crate::engine::VisualInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"LightmapGI\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LightmapGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for LightmapGi {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for LightmapGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for LightmapGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for LightmapGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for LightmapGi {
        
    }
    impl crate::obj::ExportableObject for LightmapGi {
        
    }
    impl crate::obj::cap::GodotDefault for LightmapGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LightmapGi {
        type Target = crate::engine::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LightmapGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_LightmapGi {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::LightmapGi > for $Class {
                
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
pub struct BakeQuality {
    ord: i32
}
impl BakeQuality {
    pub const BAKE_QUALITY_LOW: Self = Self {
        ord: 0i32
    };
    pub const BAKE_QUALITY_MEDIUM: Self = Self {
        ord: 1i32
    };
    pub const BAKE_QUALITY_HIGH: Self = Self {
        ord: 2i32
    };
    pub const BAKE_QUALITY_ULTRA: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for BakeQuality {
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
impl crate::builtin::meta::GodotConvert for BakeQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BakeQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BakeQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenerateProbes {
    ord: i32
}
impl GenerateProbes {
    pub const GENERATE_PROBES_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const GENERATE_PROBES_SUBDIV_4: Self = Self {
        ord: 1i32
    };
    pub const GENERATE_PROBES_SUBDIV_8: Self = Self {
        ord: 2i32
    };
    pub const GENERATE_PROBES_SUBDIV_16: Self = Self {
        ord: 3i32
    };
    pub const GENERATE_PROBES_SUBDIV_32: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for GenerateProbes {
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
impl crate::builtin::meta::GodotConvert for GenerateProbes {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GenerateProbes {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GenerateProbes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BakeError {
    ord: i32
}
impl BakeError {
    pub const BAKE_ERROR_OK: Self = Self {
        ord: 0i32
    };
    pub const BAKE_ERROR_NO_SCENE_ROOT: Self = Self {
        ord: 1i32
    };
    pub const BAKE_ERROR_FOREIGN_DATA: Self = Self {
        ord: 2i32
    };
    pub const BAKE_ERROR_NO_LIGHTMAPPER: Self = Self {
        ord: 3i32
    };
    pub const BAKE_ERROR_NO_SAVE_PATH: Self = Self {
        ord: 4i32
    };
    pub const BAKE_ERROR_NO_MESHES: Self = Self {
        ord: 5i32
    };
    pub const BAKE_ERROR_MESHES_INVALID: Self = Self {
        ord: 6i32
    };
    pub const BAKE_ERROR_CANT_CREATE_IMAGE: Self = Self {
        ord: 7i32
    };
    pub const BAKE_ERROR_USER_ABORTED: Self = Self {
        ord: 8i32
    };
    pub const BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for BakeError {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for BakeError {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BakeError {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BakeError {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentMode {
    ord: i32
}
impl EnvironmentMode {
    pub const ENVIRONMENT_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const ENVIRONMENT_MODE_SCENE: Self = Self {
        ord: 1i32
    };
    pub const ENVIRONMENT_MODE_CUSTOM_SKY: Self = Self {
        ord: 2i32
    };
    pub const ENVIRONMENT_MODE_CUSTOM_COLOR: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentMode {
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
impl crate::builtin::meta::GodotConvert for EnvironmentMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}