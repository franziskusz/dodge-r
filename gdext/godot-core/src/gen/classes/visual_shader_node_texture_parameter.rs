#![doc = "Sidecar module for class [`VisualShaderNodeTextureParameter`][crate::engine::VisualShaderNodeTextureParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeTextureParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::engine::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_texture_parameter`][crate::engine::visual_shader_node_texture_parameter]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeTextureParameter`][crate::engine::IVisualShaderNodeTextureParameter]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeTextureParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeTextureParameter`][crate::engine::VisualShaderNodeTextureParameter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeTextureParameter: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeTextureParameter {
        pub fn set_texture_type(&mut self, type_: crate::engine::visual_shader_node_texture_parameter::TextureType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_texture_parameter::TextureType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::engine::visual_shader_node_texture_parameter::TextureType {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_texture_parameter::TextureType >;
            type CallSig = (crate::engine::visual_shader_node_texture_parameter::TextureType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_default(&mut self, color: crate::engine::visual_shader_node_texture_parameter::ColorDefault,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_texture_parameter::ColorDefault);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_default(&self,) -> crate::engine::visual_shader_node_texture_parameter::ColorDefault {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_texture_parameter::ColorDefault >;
            type CallSig = (crate::engine::visual_shader_node_texture_parameter::ColorDefault,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, filter: crate::engine::visual_shader_node_texture_parameter::TextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_texture_parameter::TextureFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::engine::visual_shader_node_texture_parameter::TextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_texture_parameter::TextureFilter >;
            type CallSig = (crate::engine::visual_shader_node_texture_parameter::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, repeat: crate::engine::visual_shader_node_texture_parameter::TextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_texture_parameter::TextureRepeat);
            let args = (repeat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::engine::visual_shader_node_texture_parameter::TextureRepeat {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_texture_parameter::TextureRepeat >;
            type CallSig = (crate::engine::visual_shader_node_texture_parameter::TextureRepeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_source(&mut self, source: crate::engine::visual_shader_node_texture_parameter::TextureSource,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_texture_parameter::TextureSource);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_source(&self,) -> crate::engine::visual_shader_node_texture_parameter::TextureSource {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_texture_parameter::TextureSource >;
            type CallSig = (crate::engine::visual_shader_node_texture_parameter::TextureSource,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_source", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeTextureParameter {
        type Base = crate::engine::VisualShaderNodeParameter;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeTextureParameter\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeTextureParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeTextureParameter {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNodeParameter > for VisualShaderNodeTextureParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeTextureParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeTextureParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeTextureParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeTextureParameter {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeTextureParameter {
        
    }
    impl std::ops::Deref for VisualShaderNodeTextureParameter {
        type Target = crate::engine::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTextureParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeTextureParameter {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeTextureParameter > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeParameter > for $Class {
                
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
pub struct TextureType {
    ord: i32
}
impl TextureType {
    pub const TYPE_DATA: Self = Self {
        ord: 0i32
    };
    pub const TYPE_COLOR: Self = Self {
        ord: 1i32
    };
    pub const TYPE_NORMAL_MAP: Self = Self {
        ord: 2i32
    };
    pub const TYPE_ANISOTROPY: Self = Self {
        ord: 3i32
    };
    pub const TYPE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TextureType {
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
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColorDefault {
    ord: i32
}
impl ColorDefault {
    pub const COLOR_DEFAULT_WHITE: Self = Self {
        ord: 0i32
    };
    pub const COLOR_DEFAULT_BLACK: Self = Self {
        ord: 1i32
    };
    pub const COLOR_DEFAULT_TRANSPARENT: Self = Self {
        ord: 2i32
    };
    pub const COLOR_DEFAULT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ColorDefault {
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
impl crate::obj::IndexEnum for ColorDefault {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ColorDefault {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ColorDefault {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ColorDefault {
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
    pub const FILTER_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const FILTER_NEAREST: Self = Self {
        ord: 1i32
    };
    pub const FILTER_LINEAR: Self = Self {
        ord: 2i32
    };
    pub const FILTER_NEAREST_MIPMAP: Self = Self {
        ord: 3i32
    };
    pub const FILTER_LINEAR_MIPMAP: Self = Self {
        ord: 4i32
    };
    pub const FILTER_NEAREST_MIPMAP_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    pub const FILTER_LINEAR_MIPMAP_ANISOTROPIC: Self = Self {
        ord: 6i32
    };
    pub const FILTER_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for TextureFilter {
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
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
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
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    pub const REPEAT_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const REPEAT_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const REPEAT_DISABLED: Self = Self {
        ord: 2i32
    };
    pub const REPEAT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for TextureRepeat {
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
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureRepeat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureSource {
    ord: i32
}
impl TextureSource {
    pub const SOURCE_NONE: Self = Self {
        ord: 0i32
    };
    pub const SOURCE_SCREEN: Self = Self {
        ord: 1i32
    };
    pub const SOURCE_DEPTH: Self = Self {
        ord: 2i32
    };
    pub const SOURCE_NORMAL_ROUGHNESS: Self = Self {
        ord: 3i32
    };
    pub const SOURCE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TextureSource {
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
impl crate::obj::IndexEnum for TextureSource {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}