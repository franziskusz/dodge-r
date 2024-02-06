#![doc = "Sidecar module for class [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GeometryInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GeometryInstance3D.`\n\nInherits [`VisualInstance3D`][crate::engine::VisualInstance3D].\n\nRelated symbols:\n\n* [`geometry_instance_3d`][crate::engine::geometry_instance_3d]: sidecar module with related enum/flag types\n* [`IGeometryInstance3D`][crate::engine::IGeometryInstance3D]: virtual methods\n\n\nSee also [Godot docs for `GeometryInstance3D`](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GeometryInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GeometryInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometryInstance3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GeometryInstance3D {
        pub fn set_material_override(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material_override(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material_overlay(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material_overlay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material_overlay(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material_overlay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cast_shadows_setting(&mut self, shadow_casting_setting: crate::engine::geometry_instance_3d::ShadowCastingSetting,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::geometry_instance_3d::ShadowCastingSetting);
            let args = (shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cast_shadows_setting(&self,) -> crate::engine::geometry_instance_3d::ShadowCastingSetting {
            type RetMarshal = PtrcallReturnT < crate::engine::geometry_instance_3d::ShadowCastingSetting >;
            type CallSig = (crate::engine::geometry_instance_3d::ShadowCastingSetting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lod_bias(&mut self, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transparency(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end_margin(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin_margin(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_fade_mode(&mut self, mode: crate::engine::geometry_instance_3d::VisibilityRangeFadeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::geometry_instance_3d::VisibilityRangeFadeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_fade_mode(&self,) -> crate::engine::geometry_instance_3d::VisibilityRangeFadeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::geometry_instance_3d::VisibilityRangeFadeMode >;
            type CallSig = (crate::engine::geometry_instance_3d::VisibilityRangeFadeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_shader_parameter(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_instance_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_shader_parameter(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_extra_cull_margin(&mut self, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_extra_cull_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extra_cull_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_extra_cull_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lightmap_scale(&mut self, scale: crate::engine::geometry_instance_3d::LightmapScale,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::geometry_instance_3d::LightmapScale);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lightmap_scale(&self,) -> crate::engine::geometry_instance_3d::LightmapScale {
            type RetMarshal = PtrcallReturnT < crate::engine::geometry_instance_3d::LightmapScale >;
            type CallSig = (crate::engine::geometry_instance_3d::LightmapScale,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gi_mode(&mut self, mode: crate::engine::geometry_instance_3d::GIMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::geometry_instance_3d::GIMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gi_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gi_mode(&self,) -> crate::engine::geometry_instance_3d::GIMode {
            type RetMarshal = PtrcallReturnT < crate::engine::geometry_instance_3d::GIMode >;
            type CallSig = (crate::engine::geometry_instance_3d::GIMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gi_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_occlusion_culling(&mut self, ignore_culling: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (ignore_culling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ignore_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ignoring_occlusion_culling(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ignoring_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GeometryInstance3D {
        type Base = crate::engine::VisualInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GeometryInstance3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GeometryInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GeometryInstance3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for GeometryInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for GeometryInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for GeometryInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GeometryInstance3D {
        
    }
    impl crate::obj::ExportableObject for GeometryInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for GeometryInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GeometryInstance3D {
        type Target = crate::engine::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GeometryInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GeometryInstance3D {
        ($Class: ident) => {
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
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    pub const SHADOW_CASTING_SETTING_OFF: Self = Self {
        ord: 0i32
    };
    pub const SHADOW_CASTING_SETTING_ON: Self = Self {
        ord: 1i32
    };
    pub const SHADOW_CASTING_SETTING_DOUBLE_SIDED: Self = Self {
        ord: 2i32
    };
    pub const SHADOW_CASTING_SETTING_SHADOWS_ONLY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
impl crate::builtin::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShadowCastingSetting {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GIMode {
    ord: i32
}
impl GIMode {
    pub const GI_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const GI_MODE_STATIC: Self = Self {
        ord: 1i32
    };
    pub const GI_MODE_DYNAMIC: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for GIMode {
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
impl crate::builtin::meta::GodotConvert for GIMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GIMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GIMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightmapScale {
    ord: i32
}
impl LightmapScale {
    pub const LIGHTMAP_SCALE_1X: Self = Self {
        ord: 0i32
    };
    pub const LIGHTMAP_SCALE_2X: Self = Self {
        ord: 1i32
    };
    pub const LIGHTMAP_SCALE_4X: Self = Self {
        ord: 2i32
    };
    pub const LIGHTMAP_SCALE_8X: Self = Self {
        ord: 3i32
    };
    pub const LIGHTMAP_SCALE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for LightmapScale {
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
impl crate::obj::IndexEnum for LightmapScale {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for LightmapScale {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightmapScale {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightmapScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    pub const VISIBILITY_RANGE_FADE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VISIBILITY_RANGE_FADE_SELF: Self = Self {
        ord: 1i32
    };
    pub const VISIBILITY_RANGE_FADE_DEPENDENCIES: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
impl crate::builtin::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibilityRangeFadeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}