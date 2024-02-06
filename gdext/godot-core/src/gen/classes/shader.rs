#![doc = "Sidecar module for class [`Shader`][crate::engine::Shader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Shader` enums](https://docs.godotengine.org/en/stable/classes/class_shader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Shader.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`shader`][crate::engine::shader]: sidecar module with related enum/flag types\n* [`IShader`][crate::engine::IShader]: virtual methods\n\n\nSee also [Godot docs for `Shader`](https://docs.godotengine.org/en/stable/classes/class_shader.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Shader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Shader`][crate::engine::Shader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Shader` methods](https://docs.godotengine.org/en/stable/classes/class_shader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IShader: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Shader {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_mode(&self,) -> crate::engine::shader::Mode {
            type RetMarshal = PtrcallReturnT < crate::engine::shader::Mode >;
            type CallSig = (crate::engine::shader::Mode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code(&mut self, code: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_default_texture_parameter_full(&mut self, name: StringName, texture: Gd < crate::engine::Texture2D >, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Texture2D >, i32);
            let args = (name, texture, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_texture_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_default_texture_parameter(&mut self, name: StringName, texture: Gd < crate::engine::Texture2D >,) {
            self.set_default_texture_parameter_ex(name, texture,) . done()
        }
        #[inline]
        pub fn set_default_texture_parameter_ex(&mut self, name: StringName, texture: Gd < crate::engine::Texture2D >,) -> ExSetDefaultTextureParameter < '_ > {
            ExSetDefaultTextureParameter::new(self, name, texture,)
        }
        pub(crate) fn get_default_texture_parameter_full(&self, name: StringName, index: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, StringName, i32);
            let args = (name, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_texture_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_default_texture_parameter(&self, name: StringName,) -> Option < Gd < crate::engine::Texture2D > > {
            self.get_default_texture_parameter_ex(name,) . done()
        }
        #[inline]
        pub fn get_default_texture_parameter_ex(&self, name: StringName,) -> ExGetDefaultTextureParameter < '_ > {
            ExGetDefaultTextureParameter::new(self, name,)
        }
        pub(crate) fn get_shader_uniform_list_full(&mut self, get_groups: bool,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, bool);
            let args = (get_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shader_uniform_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_shader_uniform_list(&mut self,) -> VariantArray {
            self.get_shader_uniform_list_ex() . done()
        }
        #[inline]
        pub fn get_shader_uniform_list_ex(&mut self,) -> ExGetShaderUniformList < '_ > {
            ExGetShaderUniformList::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for Shader {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Shader\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Shader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Shader {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Shader {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Shader {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Shader {
        
    }
    impl crate::obj::ExportableObject for Shader {
        
    }
    impl crate::obj::cap::GodotDefault for Shader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Shader {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Shader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Shader {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Shader > for $Class {
                
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
#[doc = "Default-param extender for [`Shader::set_default_texture_parameter_ex`][super::Shader::set_default_texture_parameter_ex]."]
#[must_use]
pub struct ExSetDefaultTextureParameter < 'a > {
    surround_object: &'a mut re_export::Shader, name: StringName, texture: Gd < crate::engine::Texture2D >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a mut re_export::Shader, name: StringName, texture: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, name, texture, index: 0i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Shader::set_default_texture_parameter_full(self.surround_object, self.name, self.texture, self.index,)
    }
}
#[doc = "Default-param extender for [`Shader::get_default_texture_parameter_ex`][super::Shader::get_default_texture_parameter_ex]."]
#[must_use]
pub struct ExGetDefaultTextureParameter < 'a > {
    surround_object: &'a re_export::Shader, name: StringName, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a re_export::Shader, name: StringName,) -> Self {
        Self {
            surround_object, name, index: 0i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Texture2D > > {
        re_export::Shader::get_default_texture_parameter_full(self.surround_object, self.name, self.index,)
    }
}
#[doc = "Default-param extender for [`Shader::get_shader_uniform_list_ex`][super::Shader::get_shader_uniform_list_ex]."]
#[must_use]
pub struct ExGetShaderUniformList < 'a > {
    surround_object: &'a mut re_export::Shader, get_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetShaderUniformList < 'a > {
    fn new(surround_object: &'a mut re_export::Shader,) -> Self {
        Self {
            surround_object, get_groups: false,
        }
    }
    #[inline]
    pub fn get_groups(self, value: bool) -> Self {
        Self {
            get_groups: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VariantArray {
        re_export::Shader::get_shader_uniform_list_full(self.surround_object, self.get_groups,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mode {
    ord: i32
}
impl Mode {
    pub const MODE_SPATIAL: Self = Self {
        ord: 0i32
    };
    pub const MODE_CANVAS_ITEM: Self = Self {
        ord: 1i32
    };
    pub const MODE_PARTICLES: Self = Self {
        ord: 2i32
    };
    pub const MODE_SKY: Self = Self {
        ord: 3i32
    };
    pub const MODE_FOG: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Mode {
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
impl crate::builtin::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Mode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}