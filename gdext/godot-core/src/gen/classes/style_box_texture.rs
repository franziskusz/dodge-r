#![doc = "Sidecar module for class [`StyleBoxTexture`][crate::engine::StyleBoxTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StyleBoxTexture` enums](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StyleBoxTexture.`\n\nInherits [`StyleBox`][crate::engine::StyleBox].\n\nRelated symbols:\n\n* [`style_box_texture`][crate::engine::style_box_texture]: sidecar module with related enum/flag types\n* [`IStyleBoxTexture`][crate::engine::IStyleBoxTexture]: virtual methods\n\n\nSee also [Godot docs for `StyleBoxTexture`](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StyleBoxTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StyleBoxTexture`][crate::engine::StyleBoxTexture].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StyleBoxTexture` methods](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStyleBoxTexture: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn draw(&self, to_canvas_item: Rid, rect: Rect2,) {
            unimplemented !()
        }
        fn get_draw_rect(&self, rect: Rect2,) -> Rect2 {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn test_mask(&self, point: Vector2, rect: Rect2,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl StyleBoxTexture {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_margin(&mut self, margin: crate::engine::global::Side, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, f32);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_margin_all(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_margin_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_margin(&self, margin: crate::engine::global::Side,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin(&mut self, margin: crate::engine::global::Side, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, f32);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin_all(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_expand_margin_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_expand_margin(&self, margin: crate::engine::global::Side,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_region_rect(&mut self, region: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_center(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_center_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_draw_center_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_axis_stretch_mode(&mut self, mode: crate::engine::style_box_texture::AxisStretchMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::style_box_texture::AxisStretchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_axis_stretch_mode(&self,) -> crate::engine::style_box_texture::AxisStretchMode {
            type RetMarshal = PtrcallReturnT < crate::engine::style_box_texture::AxisStretchMode >;
            type CallSig = (crate::engine::style_box_texture::AxisStretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_axis_stretch_mode(&mut self, mode: crate::engine::style_box_texture::AxisStretchMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::style_box_texture::AxisStretchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_axis_stretch_mode(&self,) -> crate::engine::style_box_texture::AxisStretchMode {
            type RetMarshal = PtrcallReturnT < crate::engine::style_box_texture::AxisStretchMode >;
            type CallSig = (crate::engine::style_box_texture::AxisStretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for StyleBoxTexture {
        type Base = crate::engine::StyleBox;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"StyleBoxTexture\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StyleBoxTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for StyleBoxTexture {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::StyleBox > for StyleBoxTexture {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for StyleBoxTexture {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for StyleBoxTexture {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for StyleBoxTexture {
        
    }
    impl crate::obj::ExportableObject for StyleBoxTexture {
        
    }
    impl crate::obj::cap::GodotDefault for StyleBoxTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StyleBoxTexture {
        type Target = crate::engine::StyleBox;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StyleBoxTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_StyleBoxTexture {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::StyleBoxTexture > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::StyleBox > for $Class {
                
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
pub struct AxisStretchMode {
    ord: i32
}
impl AxisStretchMode {
    pub const AXIS_STRETCH_MODE_STRETCH: Self = Self {
        ord: 0i32
    };
    pub const AXIS_STRETCH_MODE_TILE: Self = Self {
        ord: 1i32
    };
    pub const AXIS_STRETCH_MODE_TILE_FIT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AxisStretchMode {
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
impl crate::builtin::meta::GodotConvert for AxisStretchMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AxisStretchMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AxisStretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}