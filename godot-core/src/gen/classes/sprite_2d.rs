#![doc = "Sidecar module for class [`Sprite2D`][crate::engine::Sprite2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Sprite2D` enums](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Sprite2D.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`ISprite2D`][crate::engine::ISprite2D]: virtual methods\n\n\nSee also [Godot docs for `Sprite2D`](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Sprite2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Sprite2D`][crate::engine::Sprite2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Sprite2D` methods](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISprite2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl Sprite2D {
        pub fn set_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_centered(&mut self, centered: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (centered,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_centered(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_flipped_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_flipped_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_region_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_region_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_region_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_region_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pixel_opaque(&self, pos: Vector2,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2);
            let args = (pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_pixel_opaque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_region_rect(&mut self, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_region_filter_clip_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_region_filter_clip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_region_filter_clip_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_region_filter_clip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame_coords(&mut self, coords: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_coords(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vframes(&mut self, vframes: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (vframes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vframes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vframes(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vframes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hframes(&mut self, hframes: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (hframes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hframes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hframes(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hframes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rect", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Sprite2D {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Sprite2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Sprite2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Sprite2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for Sprite2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Sprite2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Sprite2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Sprite2D {
        
    }
    impl crate::obj::ExportableObject for Sprite2D {
        
    }
    impl crate::obj::cap::GodotDefault for Sprite2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Sprite2D {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Sprite2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Sprite2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Sprite2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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