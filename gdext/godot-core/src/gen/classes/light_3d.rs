#![doc = "Sidecar module for class [`Light3D`][crate::engine::Light3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Light3D` enums](https://docs.godotengine.org/en/stable/classes/class_light3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Light3D.`\n\nInherits [`VisualInstance3D`][crate::engine::VisualInstance3D].\n\nRelated symbols:\n\n* [`light_3d`][crate::engine::light_3d]: sidecar module with related enum/flag types\n* [`ILight3D`][crate::engine::ILight3D]: virtual methods\n\n\nSee also [Godot docs for `Light3D`](https://docs.godotengine.org/en/stable/classes/class_light3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Light3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Light3D`][crate::engine::Light3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Light3D` methods](https://docs.godotengine.org/en/stable/classes/class_light3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILight3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Light3D {
        pub fn set_editor_only(&mut self, editor_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (editor_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editor_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param(&mut self, param: crate::engine::light_3d::Param, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::light_3d::Param, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param(&self, param: crate::engine::light_3d::Param,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::light_3d::Param);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_shadow(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_negative(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_negative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_negative(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_negative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mask(&mut self, cull_mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_distance_fade(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_distance_fade_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_distance_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_begin(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_begin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_shadow(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_shadow(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_length(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distance_fade_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distance_fade_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_reverse_cull_face(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shadow_reverse_cull_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_reverse_cull_face(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shadow_reverse_cull_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_mode(&mut self, bake_mode: crate::engine::light_3d::BakeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::light_3d::BakeMode);
            let args = (bake_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bake_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mode(&self,) -> crate::engine::light_3d::BakeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::light_3d::BakeMode >;
            type CallSig = (crate::engine::light_3d::BakeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_projector(&mut self, projector: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (projector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_projector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_projector(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_projector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_temperature(&mut self, temperature: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (temperature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_temperature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_temperature(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_temperature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_correlated_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_correlated_color", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Light3D {
        type Base = crate::engine::VisualInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Light3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Light3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Light3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for Light3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for Light3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Light3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Light3D {
        
    }
    impl crate::obj::ExportableObject for Light3D {
        
    }
    impl std::ops::Deref for Light3D {
        type Target = crate::engine::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Light3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Light3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Light3D > for $Class {
                
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
pub struct Param {
    ord: i32
}
impl Param {
    pub const PARAM_ENERGY: Self = Self {
        ord: 0i32
    };
    pub const PARAM_INDIRECT_ENERGY: Self = Self {
        ord: 1i32
    };
    pub const PARAM_VOLUMETRIC_FOG_ENERGY: Self = Self {
        ord: 2i32
    };
    pub const PARAM_SPECULAR: Self = Self {
        ord: 3i32
    };
    pub const PARAM_RANGE: Self = Self {
        ord: 4i32
    };
    pub const PARAM_SIZE: Self = Self {
        ord: 5i32
    };
    pub const PARAM_ATTENUATION: Self = Self {
        ord: 6i32
    };
    pub const PARAM_SPOT_ANGLE: Self = Self {
        ord: 7i32
    };
    pub const PARAM_SPOT_ATTENUATION: Self = Self {
        ord: 8i32
    };
    pub const PARAM_SHADOW_MAX_DISTANCE: Self = Self {
        ord: 9i32
    };
    pub const PARAM_SHADOW_SPLIT_1_OFFSET: Self = Self {
        ord: 10i32
    };
    pub const PARAM_SHADOW_SPLIT_2_OFFSET: Self = Self {
        ord: 11i32
    };
    pub const PARAM_SHADOW_SPLIT_3_OFFSET: Self = Self {
        ord: 12i32
    };
    pub const PARAM_SHADOW_FADE_START: Self = Self {
        ord: 13i32
    };
    pub const PARAM_SHADOW_NORMAL_BIAS: Self = Self {
        ord: 14i32
    };
    pub const PARAM_SHADOW_BIAS: Self = Self {
        ord: 15i32
    };
    pub const PARAM_SHADOW_PANCAKE_SIZE: Self = Self {
        ord: 16i32
    };
    pub const PARAM_SHADOW_OPACITY: Self = Self {
        ord: 17i32
    };
    pub const PARAM_SHADOW_BLUR: Self = Self {
        ord: 18i32
    };
    pub const PARAM_TRANSMITTANCE_BIAS: Self = Self {
        ord: 19i32
    };
    pub const PARAM_INTENSITY: Self = Self {
        ord: 20i32
    };
    pub const PARAM_MAX: Self = Self {
        ord: 21i32
    };
    
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::builtin::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Param {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BakeMode {
    ord: i32
}
impl BakeMode {
    pub const BAKE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const BAKE_STATIC: Self = Self {
        ord: 1i32
    };
    pub const BAKE_DYNAMIC: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for BakeMode {
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
impl crate::builtin::meta::GodotConvert for BakeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BakeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}