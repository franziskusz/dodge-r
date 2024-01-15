#![doc = "Sidecar module for class [`VisualShaderNodeParticleEmit`][crate::engine::VisualShaderNodeParticleEmit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeParticleEmit.`\n\nInherits [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_particle_emit`][crate::engine::visual_shader_node_particle_emit]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeParticleEmit`][crate::engine::IVisualShaderNodeParticleEmit]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeParticleEmit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeParticleEmit`][crate::engine::VisualShaderNodeParticleEmit].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeParticleEmit: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl VisualShaderNodeParticleEmit {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_flags(&mut self, flags: crate::engine::visual_shader_node_particle_emit::EmitFlags,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_particle_emit::EmitFlags);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flags(&self,) -> crate::engine::visual_shader_node_particle_emit::EmitFlags {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_particle_emit::EmitFlags >;
            type CallSig = (crate::engine::visual_shader_node_particle_emit::EmitFlags,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flags", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeParticleEmit {
        type Base = crate::engine::VisualShaderNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeParticleEmit\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeParticleEmit {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeParticleEmit {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeParticleEmit {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeParticleEmit {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeParticleEmit {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeParticleEmit {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeParticleEmit {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeParticleEmit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeParticleEmit {
        type Target = crate::engine::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeParticleEmit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeParticleEmit {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeParticleEmit > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNode > for $Class {
                
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
pub struct EmitFlags {
    ord: i32
}
impl EmitFlags {
    pub const EMIT_FLAG_POSITION: Self = Self {
        ord: 1i32
    };
    pub const EMIT_FLAG_ROT_SCALE: Self = Self {
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