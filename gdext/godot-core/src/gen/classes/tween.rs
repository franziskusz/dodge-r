#![doc = "Sidecar module for class [`Tween`][crate::engine::Tween].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tween` enums](https://docs.godotengine.org/en/stable/classes/class_tween.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tween.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`tween`][crate::engine::tween]: sidecar module with related enum/flag types\n* [`ITween`][crate::engine::ITween]: virtual methods\n\n\nSee also [Godot docs for `Tween`](https://docs.godotengine.org/en/stable/classes/class_tween.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tween {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Tween`][crate::engine::Tween].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Tween` methods](https://docs.godotengine.org/en/stable/classes/class_tween.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITween: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl Tween {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn tween_property(&mut self, object: Gd < crate::engine::Object >, property: NodePath, final_val: Variant, duration: f64,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >, Gd < crate::engine::Object >, NodePath, Variant, f64);
            let args = (object, property, final_val, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tween_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_interval(&mut self, time: f64,) -> Option < Gd < crate::engine::IntervalTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::IntervalTweener > >;
            type CallSig = (Option < Gd < crate::engine::IntervalTweener > >, f64);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tween_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_callback(&mut self, callback: Callable,) -> Option < Gd < crate::engine::CallbackTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CallbackTweener > >;
            type CallSig = (Option < Gd < crate::engine::CallbackTweener > >, Callable);
            let args = (callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tween_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_method(&mut self, method: Callable, from: Variant, to: Variant, duration: f64,) -> Option < Gd < crate::engine::MethodTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MethodTweener > >;
            type CallSig = (Option < Gd < crate::engine::MethodTweener > >, Callable, Variant, Variant, f64);
            let args = (method, from, to, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tween_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn custom_step(&mut self, delta: f64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, f64);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pause(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn kill(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "kill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_elapsed_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_elapsed_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bind_node(&mut self, node: Gd < crate::engine::Node >,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bind_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::engine::tween::TweenProcessMode,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, crate::engine::tween::TweenProcessMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pause_mode(&mut self, mode: crate::engine::tween::TweenPauseMode,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, crate::engine::tween::TweenPauseMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pause_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_parallel_full(&mut self, parallel: bool,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, bool);
            let args = (parallel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_parallel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_parallel(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            self.set_parallel_ex() . done()
        }
        #[inline]
        pub fn set_parallel_ex(&mut self,) -> ExSetParallel < '_ > {
            ExSetParallel::new(self,)
        }
        pub(crate) fn set_loops_full(&mut self, loops: i32,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, i32);
            let args = (loops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_loops", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_loops(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            self.set_loops_ex() . done()
        }
        #[inline]
        pub fn set_loops_ex(&mut self,) -> ExSetLoops < '_ > {
            ExSetLoops::new(self,)
        }
        pub fn get_loops_left(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loops_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trans(&mut self, trans: crate::engine::tween::TransitionType,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, crate::engine::tween::TransitionType);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_trans", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ease(&mut self, ease: crate::engine::tween::EaseType,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >, crate::engine::tween::EaseType);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ease", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parallel(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parallel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn chain(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "chain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn interpolate_value(initial_value: Variant, delta_value: Variant, elapsed_time: f64, duration: f64, trans_type: crate::engine::tween::TransitionType, ease_type: crate::engine::tween::EaseType,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Variant, Variant, f64, f64, crate::engine::tween::TransitionType, crate::engine::tween::EaseType);
            let args = (initial_value, delta_value, elapsed_time, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "interpolate_value", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for Tween {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Tween\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tween {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Tween {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Tween {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Tween {
        
    }
    impl crate::obj::cap::GodotDefault for Tween {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tween {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tween {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Tween {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Tween > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Tween::set_parallel_ex`][super::Tween::set_parallel_ex]."]
#[must_use]
pub struct ExSetParallel < 'a > {
    surround_object: &'a mut re_export::Tween, parallel: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetParallel < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        Self {
            surround_object, parallel: true,
        }
    }
    #[inline]
    pub fn parallel(self, value: bool) -> Self {
        Self {
            parallel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Tween > > {
        re_export::Tween::set_parallel_full(self.surround_object, self.parallel,)
    }
}
#[doc = "Default-param extender for [`Tween::set_loops_ex`][super::Tween::set_loops_ex]."]
#[must_use]
pub struct ExSetLoops < 'a > {
    surround_object: &'a mut re_export::Tween, loops: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLoops < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        Self {
            surround_object, loops: 0i32,
        }
    }
    #[inline]
    pub fn loops(self, value: i32) -> Self {
        Self {
            loops: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Tween > > {
        re_export::Tween::set_loops_full(self.surround_object, self.loops,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TweenProcessMode {
    ord: i32
}
impl TweenProcessMode {
    pub const TWEEN_PROCESS_PHYSICS: Self = Self {
        ord: 0i32
    };
    pub const TWEEN_PROCESS_IDLE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for TweenProcessMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TweenProcessMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TweenProcessMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TweenProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TweenPauseMode {
    ord: i32
}
impl TweenPauseMode {
    pub const TWEEN_PAUSE_BOUND: Self = Self {
        ord: 0i32
    };
    pub const TWEEN_PAUSE_STOP: Self = Self {
        ord: 1i32
    };
    pub const TWEEN_PAUSE_PROCESS: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TweenPauseMode {
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
impl crate::builtin::meta::GodotConvert for TweenPauseMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TweenPauseMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TweenPauseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TransitionType {
    ord: i32
}
impl TransitionType {
    pub const TRANS_LINEAR: Self = Self {
        ord: 0i32
    };
    pub const TRANS_SINE: Self = Self {
        ord: 1i32
    };
    pub const TRANS_QUINT: Self = Self {
        ord: 2i32
    };
    pub const TRANS_QUART: Self = Self {
        ord: 3i32
    };
    pub const TRANS_QUAD: Self = Self {
        ord: 4i32
    };
    pub const TRANS_EXPO: Self = Self {
        ord: 5i32
    };
    pub const TRANS_ELASTIC: Self = Self {
        ord: 6i32
    };
    pub const TRANS_CUBIC: Self = Self {
        ord: 7i32
    };
    pub const TRANS_CIRC: Self = Self {
        ord: 8i32
    };
    pub const TRANS_BOUNCE: Self = Self {
        ord: 9i32
    };
    pub const TRANS_BACK: Self = Self {
        ord: 10i32
    };
    pub const TRANS_SPRING: Self = Self {
        ord: 11i32
    };
    
}
impl crate::obj::EngineEnum for TransitionType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TransitionType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TransitionType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TransitionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EaseType {
    ord: i32
}
impl EaseType {
    pub const EASE_IN: Self = Self {
        ord: 0i32
    };
    pub const EASE_OUT: Self = Self {
        ord: 1i32
    };
    pub const EASE_IN_OUT: Self = Self {
        ord: 2i32
    };
    pub const EASE_OUT_IN: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EaseType {
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
impl crate::builtin::meta::GodotConvert for EaseType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EaseType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EaseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}