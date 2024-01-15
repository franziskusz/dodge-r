#![doc = "Sidecar module for class [`CanvasItemMaterial`][crate::engine::CanvasItemMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItemMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CanvasItemMaterial.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`canvas_item_material`][crate::engine::canvas_item_material]: sidecar module with related enum/flag types\n* [`ICanvasItemMaterial`][crate::engine::ICanvasItemMaterial]: virtual methods\n\n\nSee also [Godot docs for `CanvasItemMaterial`](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CanvasItemMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CanvasItemMaterial`][crate::engine::CanvasItemMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CanvasItemMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICanvasItemMaterial: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CanvasItemMaterial {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_blend_mode(&mut self, blend_mode: crate::engine::canvas_item_material::BlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::canvas_item_material::BlendMode);
            let args = (blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::engine::canvas_item_material::BlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::canvas_item_material::BlendMode >;
            type CallSig = (crate::engine::canvas_item_material::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_light_mode(&mut self, light_mode: crate::engine::canvas_item_material::LightMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::canvas_item_material::LightMode);
            let args = (light_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_light_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_light_mode(&self,) -> crate::engine::canvas_item_material::LightMode {
            type RetMarshal = PtrcallReturnT < crate::engine::canvas_item_material::LightMode >;
            type CallSig = (crate::engine::canvas_item_material::LightMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_light_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_animation(&mut self, particles_anim: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (particles_anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_animation(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_h_frames(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_v_frames(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_loop(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CanvasItemMaterial {
        type Base = crate::engine::Material;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CanvasItemMaterial\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CanvasItemMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CanvasItemMaterial {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Material > for CanvasItemMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for CanvasItemMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for CanvasItemMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CanvasItemMaterial {
        
    }
    impl crate::obj::ExportableObject for CanvasItemMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for CanvasItemMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CanvasItemMaterial {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CanvasItemMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CanvasItemMaterial {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CanvasItemMaterial > for $Class {
                
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
    pub const BLEND_MODE_PREMULT_ALPHA: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for BlendMode {
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
pub struct LightMode {
    ord: i32
}
impl LightMode {
    pub const LIGHT_MODE_NORMAL: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_MODE_UNSHADED: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_MODE_LIGHT_ONLY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LightMode {
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
impl crate::builtin::meta::GodotConvert for LightMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}