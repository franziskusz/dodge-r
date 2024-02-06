#![doc = "Sidecar module for class [`TextureLayered`][crate::engine::TextureLayered].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureLayered` enums](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureLayered.`\n\nInherits [`Texture`][crate::engine::Texture].\n\nRelated symbols:\n\n* [`texture_layered`][crate::engine::texture_layered]: sidecar module with related enum/flag types\n* [`ITextureLayered`][crate::engine::ITextureLayered]: virtual methods\n\n\nSee also [Godot docs for `TextureLayered`](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureLayered {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextureLayered`][crate::engine::TextureLayered].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextureLayered` methods](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextureLayered: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_format(&self,) -> crate::engine::image::Format {
            unimplemented !()
        }
        fn get_layered_type(&self,) -> u32 {
            unimplemented !()
        }
        fn get_width(&self,) -> i32 {
            unimplemented !()
        }
        fn get_height(&self,) -> i32 {
            unimplemented !()
        }
        fn get_layers(&self,) -> i32 {
            unimplemented !()
        }
        fn has_mipmaps(&self,) -> bool {
            unimplemented !()
        }
        fn get_layer_data(&self, layer_index: i32,) -> Option < Gd < crate::engine::Image > > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl TextureLayered {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_format(&self,) -> crate::engine::image::Format {
            type RetMarshal = PtrcallReturnT < crate::engine::image::Format >;
            type CallSig = (crate::engine::image::Format,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layered_type(&self,) -> crate::engine::texture_layered::LayeredType {
            type RetMarshal = PtrcallReturnT < crate::engine::texture_layered::LayeredType >;
            type CallSig = (crate::engine::texture_layered::LayeredType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layered_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layers(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_mipmaps(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_data(&self, layer: i32,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layer_data", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextureLayered {
        type Base = crate::engine::Texture;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TextureLayered\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureLayered {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TextureLayered {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Texture > for TextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TextureLayered {
        
    }
    impl crate::obj::ExportableObject for TextureLayered {
        
    }
    impl crate::obj::cap::GodotDefault for TextureLayered {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextureLayered {
        type Target = crate::engine::Texture;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureLayered {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TextureLayered {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TextureLayered > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Texture > for $Class {
                
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
pub struct LayeredType {
    ord: i32
}
impl LayeredType {
    pub const LAYERED_TYPE_2D_ARRAY: Self = Self {
        ord: 0i32
    };
    pub const LAYERED_TYPE_CUBEMAP: Self = Self {
        ord: 1i32
    };
    pub const LAYERED_TYPE_CUBEMAP_ARRAY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LayeredType {
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
impl crate::builtin::meta::GodotConvert for LayeredType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LayeredType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LayeredType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}