#![doc = "Sidecar module for class [`BitMap`][crate::engine::BitMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BitMap` enums](https://docs.godotengine.org/en/stable/classes/class_bitmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BitMap.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`bit_map`][crate::engine::bit_map]: sidecar module with related enum/flag types\n* [`IBitMap`][crate::engine::IBitMap]: virtual methods\n\n\nSee also [Godot docs for `BitMap`](https://docs.godotengine.org/en/stable/classes/class_bitmap.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BitMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`BitMap`][crate::engine::BitMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BitMap` methods](https://docs.godotengine.org/en/stable/classes/class_bitmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBitMap: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl BitMap {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn create(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_from_image_alpha_full(&mut self, image: Gd < crate::engine::Image >, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, f32);
            let args = (image, threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_image_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_from_image_alpha(&mut self, image: Gd < crate::engine::Image >,) {
            self.create_from_image_alpha_ex(image,) . done()
        }
        #[inline]
        pub fn create_from_image_alpha_ex(&mut self, image: Gd < crate::engine::Image >,) -> ExCreateFromImageAlpha < '_ > {
            ExCreateFromImageAlpha::new(self, image,)
        }
        pub fn set_bitv(&mut self, position: Vector2i, bit: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, bool);
            let args = (position, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bitv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bit(&mut self, x: i32, y: i32, bit: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (x, y, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bitv(&self, position: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bitv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bit(&self, x: i32, y: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bit_rect(&mut self, rect: Rect2i, bit: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i, bool);
            let args = (rect, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bit_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_true_bit_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_true_bit_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn resize(&mut self, new_size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (new_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grow_mask(&mut self, pixels: i32, rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rect2i);
            let args = (pixels, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "grow_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_to_image(&self,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convert_to_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn opaque_to_polygons_full(&self, rect: Rect2i, epsilon: f32,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, Rect2i, f32);
            let args = (rect, epsilon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "opaque_to_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn opaque_to_polygons(&self, rect: Rect2i,) -> Array < PackedVector2Array > {
            self.opaque_to_polygons_ex(rect,) . done()
        }
        #[inline]
        pub fn opaque_to_polygons_ex(&self, rect: Rect2i,) -> ExOpaqueToPolygons < '_ > {
            ExOpaqueToPolygons::new(self, rect,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for BitMap {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"BitMap\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BitMap {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for BitMap {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for BitMap {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for BitMap {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for BitMap {
        
    }
    impl crate::obj::ExportableObject for BitMap {
        
    }
    impl crate::obj::cap::GodotDefault for BitMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for BitMap {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BitMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_BitMap {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::BitMap > for $Class {
                
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
#[doc = "Default-param extender for [`BitMap::create_from_image_alpha_ex`][super::BitMap::create_from_image_alpha_ex]."]
#[must_use]
pub struct ExCreateFromImageAlpha < 'a > {
    surround_object: &'a mut re_export::BitMap, image: Gd < crate::engine::Image >, threshold: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromImageAlpha < 'a > {
    fn new(surround_object: &'a mut re_export::BitMap, image: Gd < crate::engine::Image >,) -> Self {
        Self {
            surround_object, image, threshold: 0.1f32,
        }
    }
    #[inline]
    pub fn threshold(self, value: f32) -> Self {
        Self {
            threshold: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::BitMap::create_from_image_alpha_full(self.surround_object, self.image, self.threshold,)
    }
}
#[doc = "Default-param extender for [`BitMap::opaque_to_polygons_ex`][super::BitMap::opaque_to_polygons_ex]."]
#[must_use]
pub struct ExOpaqueToPolygons < 'a > {
    surround_object: &'a re_export::BitMap, rect: Rect2i, epsilon: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpaqueToPolygons < 'a > {
    fn new(surround_object: &'a re_export::BitMap, rect: Rect2i,) -> Self {
        Self {
            surround_object, rect, epsilon: 2f32,
        }
    }
    #[inline]
    pub fn epsilon(self, value: f32) -> Self {
        Self {
            epsilon: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        re_export::BitMap::opaque_to_polygons_full(self.surround_object, self.rect, self.epsilon,)
    }
}