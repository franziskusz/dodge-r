#![doc = "Sidecar module for class [`AnimatedSprite3D`][crate::engine::AnimatedSprite3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimatedSprite3D` enums](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimatedSprite3D.`\n\nInherits [`SpriteBase3D`][crate::engine::SpriteBase3D].\n\nRelated symbols:\n\n* [`animated_sprite_3d`][crate::engine::animated_sprite_3d]: sidecar module with related enum/flag types\n* [`IAnimatedSprite3D`][crate::engine::IAnimatedSprite3D]: virtual methods\n\n\nSee also [Godot docs for `AnimatedSprite3D`](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimatedSprite3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimatedSprite3D`][crate::engine::AnimatedSprite3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimatedSprite3D` methods](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimatedSprite3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimatedSprite3D {
        pub fn set_sprite_frames(&mut self, sprite_frames: Gd < crate::engine::SpriteFrames >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::SpriteFrames >);
            let args = (sprite_frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sprite_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sprite_frames(&self,) -> Option < Gd < crate::engine::SpriteFrames > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SpriteFrames > >;
            type CallSig = (Option < Gd < crate::engine::SpriteFrames > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sprite_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animation(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autoplay(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, name: StringName, custom_speed: f32, from_end: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f32, bool);
            let args = (name, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex(&mut self,) -> ExPlay < '_ > {
            ExPlay::new(self,)
        }
        pub(crate) fn play_backwards_full(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_backwards", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[inline]
        pub fn play_backwards_ex(&mut self,) -> ExPlayBackwards < '_ > {
            ExPlayBackwards::new(self,)
        }
        pub fn pause(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame_progress(&mut self, progress: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_progress(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame_and_progress(&mut self, frame: i32, progress: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (frame, progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame_and_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (speed_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_speed(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playing_speed", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimatedSprite3D {
        type Base = crate::engine::SpriteBase3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimatedSprite3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimatedSprite3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimatedSprite3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::SpriteBase3D > for AnimatedSprite3D {
        
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for AnimatedSprite3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for AnimatedSprite3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for AnimatedSprite3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for AnimatedSprite3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimatedSprite3D {
        
    }
    impl crate::obj::ExportableObject for AnimatedSprite3D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimatedSprite3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimatedSprite3D {
        type Target = crate::engine::SpriteBase3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimatedSprite3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimatedSprite3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimatedSprite3D > for $Class {
                
            }
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
#[doc = "Default-param extender for [`AnimatedSprite3D::play_ex`][super::AnimatedSprite3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    surround_object: &'a mut re_export::AnimatedSprite3D, name: StringName, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        Self {
            surround_object, name: StringName::from(""), custom_speed: 1f32, from_end: false,
        }
    }
    #[inline]
    pub fn name(self, value: StringName) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, value: f32) -> Self {
        Self {
            custom_speed: value, .. self
        }
    }
    #[inline]
    pub fn from_end(self, value: bool) -> Self {
        Self {
            from_end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimatedSprite3D::play_full(self.surround_object, self.name, self.custom_speed, self.from_end,)
    }
}
#[doc = "Default-param extender for [`AnimatedSprite3D::play_backwards_ex`][super::AnimatedSprite3D::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'a > {
    surround_object: &'a mut re_export::AnimatedSprite3D, name: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayBackwards < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        Self {
            surround_object, name: StringName::from(""),
        }
    }
    #[inline]
    pub fn name(self, value: StringName) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimatedSprite3D::play_backwards_full(self.surround_object, self.name,)
    }
}