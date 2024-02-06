#![doc = "Sidecar module for class [`SpriteBase3D`][crate::engine::SpriteBase3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpriteBase3D` enums](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpriteBase3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`sprite_base_3d`][crate::engine::sprite_base_3d]: sidecar module with related enum/flag types\n* [`ISpriteBase3D`][crate::engine::ISpriteBase3D]: virtual methods\n\n\nSee also [Godot docs for `SpriteBase3D`](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpriteBase3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SpriteBase3D`][crate::engine::SpriteBase3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SpriteBase3D` methods](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISpriteBase3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SpriteBase3D {
        pub fn set_centered(&mut self, centered: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (centered,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_centered(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_flipped_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_flipped_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_axis(&mut self, axis: Vector3Axis,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3Axis);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_axis(&self,) -> Vector3Axis {
            type RetMarshal = PtrcallReturnT < Vector3Axis >;
            type CallSig = (Vector3Axis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_flag(&mut self, flag: crate::engine::sprite_base_3d::DrawFlags, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::sprite_base_3d::DrawFlags, bool);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_flag(&self, flag: crate::engine::sprite_base_3d::DrawFlags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::sprite_base_3d::DrawFlags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: crate::engine::sprite_base_3d::AlphaCutMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::sprite_base_3d::AlphaCutMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_cut_mode(&self,) -> crate::engine::sprite_base_3d::AlphaCutMode {
            type RetMarshal = PtrcallReturnT < crate::engine::sprite_base_3d::AlphaCutMode >;
            type CallSig = (crate::engine::sprite_base_3d::AlphaCutMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::engine::base_material_3d::AlphaAntiAliasing,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::AlphaAntiAliasing);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::engine::base_material_3d::AlphaAntiAliasing {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::AlphaAntiAliasing >;
            type CallSig = (crate::engine::base_material_3d::AlphaAntiAliasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::engine::base_material_3d::BillboardMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::BillboardMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::engine::base_material_3d::BillboardMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::BillboardMode >;
            type CallSig = (crate::engine::base_material_3d::BillboardMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::engine::base_material_3d::TextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::engine::base_material_3d::TextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureFilter >;
            type CallSig = (crate::engine::base_material_3d::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::engine::TriangleMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TriangleMesh > >;
            type CallSig = (Option < Gd < crate::engine::TriangleMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_triangle_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SpriteBase3D {
        type Base = crate::engine::GeometryInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SpriteBase3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpriteBase3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SpriteBase3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for SpriteBase3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for SpriteBase3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for SpriteBase3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for SpriteBase3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SpriteBase3D {
        
    }
    impl crate::obj::ExportableObject for SpriteBase3D {
        
    }
    impl std::ops::Deref for SpriteBase3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpriteBase3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SpriteBase3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SpriteBase3D > for $Class {
                
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
pub struct DrawFlags {
    ord: i32
}
impl DrawFlags {
    pub const FLAG_TRANSPARENT: Self = Self {
        ord: 0i32
    };
    pub const FLAG_SHADED: Self = Self {
        ord: 1i32
    };
    pub const FLAG_DOUBLE_SIDED: Self = Self {
        ord: 2i32
    };
    pub const FLAG_DISABLE_DEPTH_TEST: Self = Self {
        ord: 3i32
    };
    pub const FLAG_FIXED_SIZE: Self = Self {
        ord: 4i32
    };
    pub const FLAG_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for DrawFlags {
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
impl crate::obj::IndexEnum for DrawFlags {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for DrawFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DrawFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlphaCutMode {
    ord: i32
}
impl AlphaCutMode {
    pub const ALPHA_CUT_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const ALPHA_CUT_DISCARD: Self = Self {
        ord: 1i32
    };
    pub const ALPHA_CUT_OPAQUE_PREPASS: Self = Self {
        ord: 2i32
    };
    pub const ALPHA_CUT_HASH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AlphaCutMode {
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
impl crate::builtin::meta::GodotConvert for AlphaCutMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AlphaCutMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AlphaCutMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}