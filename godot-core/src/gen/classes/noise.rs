#![doc = "Sidecar module for class [`Noise`][crate::engine::Noise].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Noise` enums](https://docs.godotengine.org/en/stable/classes/class_noise.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Noise.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`noise`][crate::engine::noise]: sidecar module with related enum/flag types\n* [`INoise`][crate::engine::INoise]: virtual methods\n\n\nSee also [Godot docs for `Noise`](https://docs.godotengine.org/en/stable/classes/class_noise.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Noise {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Noise`][crate::engine::Noise].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Noise` methods](https://docs.godotengine.org/en/stable/classes/class_noise.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INoise: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Noise {
        pub fn get_noise_1d(&self, x: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32);
            let args = (x,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_1d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_2d(&self, x: f32, y: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32, f32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_2dv(&self, v: Vector2,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_2dv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_3d(&self, x: f32, y: f32, z: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, f32, f32, f32);
            let args = (x, y, z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_3dv(&self, v: Vector3,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector3);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_3dv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32, i32, bool, bool, bool);
            let args = (width, height, invert, in_3d_space, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_image(&self, width: i32, height: i32,) -> Option < Gd < crate::engine::Image > > {
            self.get_image_ex(width, height,) . done()
        }
        #[inline]
        pub fn get_image_ex(&self, width: i32, height: i32,) -> ExGetImage < '_ > {
            ExGetImage::new(self, width, height,)
        }
        pub(crate) fn get_seamless_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32, i32, bool, bool, f32, bool);
            let args = (width, height, invert, in_3d_space, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_seamless_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_seamless_image(&self, width: i32, height: i32,) -> Option < Gd < crate::engine::Image > > {
            self.get_seamless_image_ex(width, height,) . done()
        }
        #[inline]
        pub fn get_seamless_image_ex(&self, width: i32, height: i32,) -> ExGetSeamlessImage < '_ > {
            ExGetSeamlessImage::new(self, width, height,)
        }
        pub(crate) fn get_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,) -> Array < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Image > > >;
            type CallSig = (Array < Gd < crate::engine::Image > >, i32, i32, i32, bool, bool);
            let args = (width, height, depth, invert, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_image_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::engine::Image > > {
            self.get_image_3d_ex(width, height, depth,) . done()
        }
        #[inline]
        pub fn get_image_3d_ex(&self, width: i32, height: i32, depth: i32,) -> ExGetImage3d < '_ > {
            ExGetImage3d::new(self, width, height, depth,)
        }
        pub(crate) fn get_seamless_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,) -> Array < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Image > > >;
            type CallSig = (Array < Gd < crate::engine::Image > >, i32, i32, i32, bool, f32, bool);
            let args = (width, height, depth, invert, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_seamless_image_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_seamless_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::engine::Image > > {
            self.get_seamless_image_3d_ex(width, height, depth,) . done()
        }
        #[inline]
        pub fn get_seamless_image_3d_ex(&self, width: i32, height: i32, depth: i32,) -> ExGetSeamlessImage3d < '_ > {
            ExGetSeamlessImage3d::new(self, width, height, depth,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for Noise {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Noise\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Noise {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Noise {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Noise {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Noise {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Noise {
        
    }
    impl crate::obj::ExportableObject for Noise {
        
    }
    impl std::ops::Deref for Noise {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Noise {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Noise {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Noise > for $Class {
                
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
#[doc = "Default-param extender for [`Noise::get_image_ex`][super::Noise::get_image_ex]."]
#[must_use]
pub struct ExGetImage < 'a > {
    surround_object: &'a re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetImage < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32,) -> Self {
        Self {
            surround_object, width, height, invert: false, in_3d_space: false, normalize: true,
        }
    }
    #[inline]
    pub fn invert(self, value: bool) -> Self {
        Self {
            invert: value, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, value: bool) -> Self {
        Self {
            in_3d_space: value, .. self
        }
    }
    #[inline]
    pub fn normalize(self, value: bool) -> Self {
        Self {
            normalize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Image > > {
        re_export::Noise::get_image_full(self.surround_object, self.width, self.height, self.invert, self.in_3d_space, self.normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_ex`][super::Noise::get_seamless_image_ex]."]
#[must_use]
pub struct ExGetSeamlessImage < 'a > {
    surround_object: &'a re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSeamlessImage < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32,) -> Self {
        Self {
            surround_object, width, height, invert: false, in_3d_space: false, skirt: 0.1f32, normalize: true,
        }
    }
    #[inline]
    pub fn invert(self, value: bool) -> Self {
        Self {
            invert: value, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, value: bool) -> Self {
        Self {
            in_3d_space: value, .. self
        }
    }
    #[inline]
    pub fn skirt(self, value: f32) -> Self {
        Self {
            skirt: value, .. self
        }
    }
    #[inline]
    pub fn normalize(self, value: bool) -> Self {
        Self {
            normalize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Image > > {
        re_export::Noise::get_seamless_image_full(self.surround_object, self.width, self.height, self.invert, self.in_3d_space, self.skirt, self.normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_image_3d_ex`][super::Noise::get_image_3d_ex]."]
#[must_use]
pub struct ExGetImage3d < 'a > {
    surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetImage3d < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        Self {
            surround_object, width, height, depth, invert: false, normalize: true,
        }
    }
    #[inline]
    pub fn invert(self, value: bool) -> Self {
        Self {
            invert: value, .. self
        }
    }
    #[inline]
    pub fn normalize(self, value: bool) -> Self {
        Self {
            normalize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::engine::Image > > {
        re_export::Noise::get_image_3d_full(self.surround_object, self.width, self.height, self.depth, self.invert, self.normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_3d_ex`][super::Noise::get_seamless_image_3d_ex]."]
#[must_use]
pub struct ExGetSeamlessImage3d < 'a > {
    surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSeamlessImage3d < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        Self {
            surround_object, width, height, depth, invert: false, skirt: 0.1f32, normalize: true,
        }
    }
    #[inline]
    pub fn invert(self, value: bool) -> Self {
        Self {
            invert: value, .. self
        }
    }
    #[inline]
    pub fn skirt(self, value: f32) -> Self {
        Self {
            skirt: value, .. self
        }
    }
    #[inline]
    pub fn normalize(self, value: bool) -> Self {
        Self {
            normalize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::engine::Image > > {
        re_export::Noise::get_seamless_image_3d_full(self.surround_object, self.width, self.height, self.depth, self.invert, self.skirt, self.normalize,)
    }
}