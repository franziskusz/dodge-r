#![doc = "Sidecar module for class [`RdSamplerState`][crate::engine::RdSamplerState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDSamplerState` enums](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDSamplerState.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IRdSamplerState`][crate::engine::IRdSamplerState]: virtual methods\n\n\nSee also [Godot docs for `RDSamplerState`](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdSamplerState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdSamplerState`][crate::engine::RdSamplerState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDSamplerState` methods](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdSamplerState: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdSamplerState {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_mag_filter(&mut self, p_member: crate::engine::rendering_device::SamplerFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mag_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mag_filter(&self,) -> crate::engine::rendering_device::SamplerFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerFilter >;
            type CallSig = (crate::engine::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mag_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_filter(&mut self, p_member: crate::engine::rendering_device::SamplerFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_filter(&self,) -> crate::engine::rendering_device::SamplerFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerFilter >;
            type CallSig = (crate::engine::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mip_filter(&mut self, p_member: crate::engine::rendering_device::SamplerFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mip_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mip_filter(&self,) -> crate::engine::rendering_device::SamplerFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerFilter >;
            type CallSig = (crate::engine::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mip_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_u(&mut self, p_member: crate::engine::rendering_device::SamplerRepeatMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_repeat_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_u(&self,) -> crate::engine::rendering_device::SamplerRepeatMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerRepeatMode >;
            type CallSig = (crate::engine::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_repeat_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_v(&mut self, p_member: crate::engine::rendering_device::SamplerRepeatMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_repeat_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_v(&self,) -> crate::engine::rendering_device::SamplerRepeatMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerRepeatMode >;
            type CallSig = (crate::engine::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_repeat_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_w(&mut self, p_member: crate::engine::rendering_device::SamplerRepeatMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_repeat_w", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_w(&self,) -> crate::engine::rendering_device::SamplerRepeatMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerRepeatMode >;
            type CallSig = (crate::engine::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_repeat_w", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lod_bias(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_anisotropy(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_anisotropy(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anisotropy_max(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anisotropy_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anisotropy_max(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_anisotropy_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_compare(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_compare(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_compare_op(&mut self, p_member: crate::engine::rendering_device::CompareOperator,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::CompareOperator);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_compare_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_compare_op(&self,) -> crate::engine::rendering_device::CompareOperator {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::CompareOperator >;
            type CallSig = (crate::engine::rendering_device::CompareOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_compare_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_lod(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_lod(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_lod(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_lod(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_color(&mut self, p_member: crate::engine::rendering_device::SamplerBorderColor,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::SamplerBorderColor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_color(&self,) -> crate::engine::rendering_device::SamplerBorderColor {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::SamplerBorderColor >;
            type CallSig = (crate::engine::rendering_device::SamplerBorderColor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unnormalized_uvw(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unnormalized_uvw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unnormalized_uvw(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unnormalized_uvw", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdSamplerState {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RDSamplerState\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdSamplerState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RdSamplerState {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RdSamplerState {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RdSamplerState {
        
    }
    impl crate::obj::cap::GodotDefault for RdSamplerState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdSamplerState {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdSamplerState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RdSamplerState {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RdSamplerState > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}