#![doc = "Sidecar module for class [`SpriteFrames`][crate::engine::SpriteFrames].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpriteFrames` enums](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpriteFrames.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`sprite_frames`][crate::engine::sprite_frames]: sidecar module with related enum/flag types\n* [`ISpriteFrames`][crate::engine::ISpriteFrames]: virtual methods\n\n\nSee also [Godot docs for `SpriteFrames`](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpriteFrames {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SpriteFrames`][crate::engine::SpriteFrames].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SpriteFrames` methods](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISpriteFrames: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SpriteFrames {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_animation(&mut self, anim: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_animation(&self, anim: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_animation(&mut self, anim: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_animation(&mut self, anim: StringName, newname: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (anim, newname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_names(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animation_speed(&mut self, anim: StringName, fps: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f64);
            let args = (anim, fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_speed(&self, anim: StringName,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animation_loop(&mut self, anim: StringName, loop_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (anim, loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_animation_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_loop(&self, anim: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_frame_full(&mut self, anim: StringName, texture: Gd < crate::engine::Texture2D >, duration: f32, at_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Texture2D >, f32, i32);
            let args = (anim, texture, duration, at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_frame(&mut self, anim: StringName, texture: Gd < crate::engine::Texture2D >,) {
            self.add_frame_ex(anim, texture,) . done()
        }
        #[inline]
        pub fn add_frame_ex(&mut self, anim: StringName, texture: Gd < crate::engine::Texture2D >,) -> ExAddFrame < '_ > {
            ExAddFrame::new(self, anim, texture,)
        }
        pub(crate) fn set_frame_full(&mut self, anim: StringName, idx: i32, texture: Gd < crate::engine::Texture2D >, duration: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32, Gd < crate::engine::Texture2D >, f32);
            let args = (anim, idx, texture, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_frame(&mut self, anim: StringName, idx: i32, texture: Gd < crate::engine::Texture2D >,) {
            self.set_frame_ex(anim, idx, texture,) . done()
        }
        #[inline]
        pub fn set_frame_ex(&mut self, anim: StringName, idx: i32, texture: Gd < crate::engine::Texture2D >,) -> ExSetFrame < '_ > {
            ExSetFrame::new(self, anim, idx, texture,)
        }
        pub fn remove_frame(&mut self, anim: StringName, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (anim, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_count(&self, anim: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_texture(&self, anim: StringName, idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, StringName, i32);
            let args = (anim, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_duration(&self, anim: StringName, idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName, i32);
            let args = (anim, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self, anim: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_all", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SpriteFrames {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SpriteFrames\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpriteFrames {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SpriteFrames {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for SpriteFrames {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for SpriteFrames {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SpriteFrames {
        
    }
    impl crate::obj::ExportableObject for SpriteFrames {
        
    }
    impl crate::obj::cap::GodotDefault for SpriteFrames {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SpriteFrames {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpriteFrames {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SpriteFrames {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SpriteFrames > for $Class {
                
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
#[doc = "Default-param extender for [`SpriteFrames::add_frame_ex`][super::SpriteFrames::add_frame_ex]."]
#[must_use]
pub struct ExAddFrame < 'a > {
    surround_object: &'a mut re_export::SpriteFrames, anim: StringName, texture: Gd < crate::engine::Texture2D >, duration: f32, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFrame < 'a > {
    fn new(surround_object: &'a mut re_export::SpriteFrames, anim: StringName, texture: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, anim, texture, duration: 1f32, at_position: - 1i32,
        }
    }
    #[inline]
    pub fn duration(self, value: f32) -> Self {
        Self {
            duration: value, .. self
        }
    }
    #[inline]
    pub fn at_position(self, value: i32) -> Self {
        Self {
            at_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SpriteFrames::add_frame_full(self.surround_object, self.anim, self.texture, self.duration, self.at_position,)
    }
}
#[doc = "Default-param extender for [`SpriteFrames::set_frame_ex`][super::SpriteFrames::set_frame_ex]."]
#[must_use]
pub struct ExSetFrame < 'a > {
    surround_object: &'a mut re_export::SpriteFrames, anim: StringName, idx: i32, texture: Gd < crate::engine::Texture2D >, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetFrame < 'a > {
    fn new(surround_object: &'a mut re_export::SpriteFrames, anim: StringName, idx: i32, texture: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, anim, idx, texture, duration: 1f32,
        }
    }
    #[inline]
    pub fn duration(self, value: f32) -> Self {
        Self {
            duration: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SpriteFrames::set_frame_full(self.surround_object, self.anim, self.idx, self.texture, self.duration,)
    }
}