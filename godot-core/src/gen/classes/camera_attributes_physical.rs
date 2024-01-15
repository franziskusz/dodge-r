#![doc = "Sidecar module for class [`CameraAttributesPhysical`][crate::engine::CameraAttributesPhysical].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CameraAttributesPhysical` enums](https://docs.godotengine.org/en/stable/classes/class_cameraattributesphysical.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CameraAttributesPhysical.`\n\nInherits [`CameraAttributes`][crate::engine::CameraAttributes].\n\nRelated symbols:\n\n* [`ICameraAttributesPhysical`][crate::engine::ICameraAttributesPhysical]: virtual methods\n\n\nSee also [Godot docs for `CameraAttributesPhysical`](https://docs.godotengine.org/en/stable/classes/class_cameraattributesphysical.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CameraAttributesPhysical {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CameraAttributesPhysical`][crate::engine::CameraAttributesPhysical].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CameraAttributesPhysical` methods](https://docs.godotengine.org/en/stable/classes/class_cameraattributesphysical.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICameraAttributesPhysical: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CameraAttributesPhysical {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_aperture(&mut self, aperture: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (aperture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_aperture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aperture(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_aperture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shutter_speed(&mut self, shutter_speed: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (shutter_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shutter_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shutter_speed(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shutter_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focal_length(&mut self, focal_length: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (focal_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focal_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focal_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focal_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_distance(&mut self, focus_distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (focus_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focus_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focus_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_near(&mut self, near: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (near,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_near", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_near(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_near", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_far(&mut self, far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_far", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_far(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_far", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fov(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_exposure_max_exposure_value(&mut self, exposure_value_max: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (exposure_value_max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_exposure_max_exposure_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_exposure_max_exposure_value(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_exposure_max_exposure_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_exposure_min_exposure_value(&mut self, exposure_value_min: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (exposure_value_min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_exposure_min_exposure_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_exposure_min_exposure_value(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_exposure_min_exposure_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CameraAttributesPhysical {
        type Base = crate::engine::CameraAttributes;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CameraAttributesPhysical\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CameraAttributesPhysical {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CameraAttributesPhysical {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CameraAttributes > for CameraAttributesPhysical {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for CameraAttributesPhysical {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for CameraAttributesPhysical {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CameraAttributesPhysical {
        
    }
    impl crate::obj::ExportableObject for CameraAttributesPhysical {
        
    }
    impl crate::obj::cap::GodotDefault for CameraAttributesPhysical {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CameraAttributesPhysical {
        type Target = crate::engine::CameraAttributes;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CameraAttributesPhysical {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CameraAttributesPhysical {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CameraAttributesPhysical > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CameraAttributes > for $Class {
                
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