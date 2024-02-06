#![doc = "Sidecar module for class [`GradientTexture2D`][crate::engine::GradientTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GradientTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GradientTexture2D.`\n\nInherits [`Texture2D`][crate::engine::Texture2D].\n\nRelated symbols:\n\n* [`gradient_texture_2d`][crate::engine::gradient_texture_2d]: sidecar module with related enum/flag types\n* [`IGradientTexture2D`][crate::engine::IGradientTexture2D]: virtual methods\n\n\nSee also [Godot docs for `GradientTexture2D`](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GradientTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GradientTexture2D`][crate::engine::GradientTexture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GradientTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGradientTexture2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32 {
            unimplemented !()
        }
        fn get_height(&self,) -> i32 {
            unimplemented !()
        }
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl GradientTexture2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_gradient(&mut self, gradient: Gd < crate::engine::Gradient >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Gradient >);
            let args = (gradient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gradient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gradient(&self,) -> Option < Gd < crate::engine::Gradient > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Gradient > >;
            type CallSig = (Option < Gd < crate::engine::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gradient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hdr(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_hdr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_hdr(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_hdr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill(&mut self, fill: crate::engine::gradient_texture_2d::Fill,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gradient_texture_2d::Fill);
            let args = (fill,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill(&self,) -> crate::engine::gradient_texture_2d::Fill {
            type RetMarshal = PtrcallReturnT < crate::engine::gradient_texture_2d::Fill >;
            type CallSig = (crate::engine::gradient_texture_2d::Fill,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_from(&mut self, fill_from: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (fill_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fill_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_from(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fill_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_to(&mut self, fill_to: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (fill_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fill_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_to(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fill_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat(&mut self, repeat: crate::engine::gradient_texture_2d::Repeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gradient_texture_2d::Repeat);
            let args = (repeat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat(&self,) -> crate::engine::gradient_texture_2d::Repeat {
            type RetMarshal = PtrcallReturnT < crate::engine::gradient_texture_2d::Repeat >;
            type CallSig = (crate::engine::gradient_texture_2d::Repeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_repeat", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GradientTexture2D {
        type Base = crate::engine::Texture2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GradientTexture2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GradientTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GradientTexture2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Texture2D > for GradientTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Texture > for GradientTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GradientTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GradientTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GradientTexture2D {
        
    }
    impl crate::obj::ExportableObject for GradientTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for GradientTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GradientTexture2D {
        type Target = crate::engine::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GradientTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GradientTexture2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GradientTexture2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Texture2D > for $Class {
                
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
pub struct Fill {
    ord: i32
}
impl Fill {
    pub const FILL_LINEAR: Self = Self {
        ord: 0i32
    };
    pub const FILL_RADIAL: Self = Self {
        ord: 1i32
    };
    pub const FILL_SQUARE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Fill {
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
impl crate::builtin::meta::GodotConvert for Fill {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Fill {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Fill {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Repeat {
    ord: i32
}
impl Repeat {
    pub const REPEAT_NONE: Self = Self {
        ord: 0i32
    };
    pub const REPEAT: Self = Self {
        ord: 1i32
    };
    pub const REPEAT_MIRROR: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Repeat {
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
impl crate::builtin::meta::GodotConvert for Repeat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Repeat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Repeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}