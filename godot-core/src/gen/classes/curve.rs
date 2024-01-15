#![doc = "Sidecar module for class [`Curve`][crate::engine::Curve].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve` enums](https://docs.godotengine.org/en/stable/classes/class_curve.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Curve.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`curve`][crate::engine::curve]: sidecar module with related enum/flag types\n* [`ICurve`][crate::engine::ICurve]: virtual methods\n\n\nSee also [Godot docs for `Curve`](https://docs.godotengine.org/en/stable/classes/class_curve.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Curve {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Curve`][crate::engine::Curve].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Curve` methods](https://docs.godotengine.org/en/stable/classes/class_curve.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICurve: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Curve {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_point_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::engine::curve::TangentMode, right_mode: crate::engine::curve::TangentMode,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2, f32, f32, crate::engine::curve::TangentMode, crate::engine::curve::TangentMode);
            let args = (position, left_tangent, right_tangent, left_mode, right_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_point(&mut self, position: Vector2,) -> i32 {
            self.add_point_ex(position,) . done()
        }
        #[inline]
        pub fn add_point_ex(&mut self, position: Vector2,) -> ExAddPoint < '_ > {
            ExAddPoint::new(self, position,)
        }
        pub fn remove_point(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_points(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_position(&self, index: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_value(&mut self, index: i32, y: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (index, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_offset(&mut self, index: i32, offset: f32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, f32);
            let args = (index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample(&self, offset: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample_baked(&self, offset: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample_baked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_left_tangent(&self, index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_left_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_right_tangent(&self, index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_right_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_left_mode(&self, index: i32,) -> crate::engine::curve::TangentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::curve::TangentMode >;
            type CallSig = (crate::engine::curve::TangentMode, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_left_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_right_mode(&self, index: i32,) -> crate::engine::curve::TangentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::curve::TangentMode >;
            type CallSig = (crate::engine::curve::TangentMode, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_right_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_left_tangent(&mut self, index: i32, tangent: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_left_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_right_tangent(&mut self, index: i32, tangent: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_right_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_left_mode(&mut self, index: i32, mode: crate::engine::curve::TangentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::curve::TangentMode);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_left_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_right_mode(&mut self, index: i32, mode: crate::engine::curve::TangentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::curve::TangentMode);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_right_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_value(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_value(&mut self, min: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_value(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_value(&mut self, max: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clean_dupes(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clean_dupes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bake(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bake", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_resolution(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_resolution(&mut self, resolution: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (resolution,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bake_resolution", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Curve {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Curve\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Curve {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Curve {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Curve {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Curve {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Curve {
        
    }
    impl crate::obj::ExportableObject for Curve {
        
    }
    impl crate::obj::cap::GodotDefault for Curve {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Curve {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Curve {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Curve {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Curve > for $Class {
                
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
#[doc = "Default-param extender for [`Curve::add_point_ex`][super::Curve::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    surround_object: &'a mut re_export::Curve, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::engine::curve::TangentMode, right_mode: crate::engine::curve::TangentMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::Curve, position: Vector2,) -> Self {
        Self {
            surround_object, position, left_tangent: 0f32, right_tangent: 0f32, left_mode: crate::obj::EngineEnum::from_ord(0), right_mode: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn left_tangent(self, value: f32) -> Self {
        Self {
            left_tangent: value, .. self
        }
    }
    #[inline]
    pub fn right_tangent(self, value: f32) -> Self {
        Self {
            right_tangent: value, .. self
        }
    }
    #[inline]
    pub fn left_mode(self, value: crate::engine::curve::TangentMode) -> Self {
        Self {
            left_mode: value, .. self
        }
    }
    #[inline]
    pub fn right_mode(self, value: crate::engine::curve::TangentMode) -> Self {
        Self {
            right_mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Curve::add_point_full(self.surround_object, self.position, self.left_tangent, self.right_tangent, self.left_mode, self.right_mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TangentMode {
    ord: i32
}
impl TangentMode {
    pub const TANGENT_FREE: Self = Self {
        ord: 0i32
    };
    pub const TANGENT_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const TANGENT_MODE_COUNT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TangentMode {
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
impl crate::builtin::meta::GodotConvert for TangentMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TangentMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TangentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}