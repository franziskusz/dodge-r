#![doc = "Sidecar module for class [`Curve3D`][crate::engine::Curve3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve3D` enums](https://docs.godotengine.org/en/stable/classes/class_curve3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Curve3D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`curve_3d`][crate::engine::curve_3d]: sidecar module with related enum/flag types\n* [`ICurve3D`][crate::engine::ICurve3D]: virtual methods\n\n\nSee also [Godot docs for `Curve3D`](https://docs.godotengine.org/en/stable/classes/class_curve3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Curve3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Curve3D`][crate::engine::Curve3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Curve3D` methods](https://docs.godotengine.org/en/stable/classes/class_curve3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICurve3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Curve3D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_point_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, position: Vector3, in_: Vector3, out: Vector3, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, Vector3, Vector3, i32);
            let args = (position, in_, out, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_point(&mut self, position: Vector3,) {
            self.add_point_ex(position,) . done()
        }
        #[inline]
        pub fn add_point_ex(&mut self, position: Vector3,) -> ExAddPoint < '_ > {
            ExAddPoint::new(self, position,)
        }
        pub fn set_point_position(&mut self, idx: i32, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_position(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_tilt(&mut self, idx: i32, tilt: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (idx, tilt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_tilt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_tilt(&self, idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_tilt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_in(&mut self, idx: i32, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_in", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_in(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_in", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_out(&mut self, idx: i32, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_out", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_out(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_out", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_point(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_points(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample(&self, idx: i32, t: f32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32, f32);
            let args = (idx, t,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn samplef(&self, fofs: f32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, f32);
            let args = (fofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "samplef", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_interval(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bake_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_interval(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bake_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_up_vector_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_up_vector_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_up_vector_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_up_vector_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_baked_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_baked_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn sample_baked_full(&self, offset: f32, cubic: bool,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, f32, bool);
            let args = (offset, cubic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample_baked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn sample_baked(&self,) -> Vector3 {
            self.sample_baked_ex() . done()
        }
        #[inline]
        pub fn sample_baked_ex(&self,) -> ExSampleBaked < '_ > {
            ExSampleBaked::new(self,)
        }
        pub(crate) fn sample_baked_with_rotation_full(&self, offset: f32, cubic: bool, apply_tilt: bool,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, f32, bool, bool);
            let args = (offset, cubic, apply_tilt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample_baked_with_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn sample_baked_with_rotation(&self,) -> Transform3D {
            self.sample_baked_with_rotation_ex() . done()
        }
        #[inline]
        pub fn sample_baked_with_rotation_ex(&self,) -> ExSampleBakedWithRotation < '_ > {
            ExSampleBakedWithRotation::new(self,)
        }
        pub(crate) fn sample_baked_up_vector_full(&self, offset: f32, apply_tilt: bool,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, f32, bool);
            let args = (offset, apply_tilt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample_baked_up_vector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn sample_baked_up_vector(&self, offset: f32,) -> Vector3 {
            self.sample_baked_up_vector_ex(offset,) . done()
        }
        #[inline]
        pub fn sample_baked_up_vector_ex(&self, offset: f32,) -> ExSampleBakedUpVector < '_ > {
            ExSampleBakedUpVector::new(self, offset,)
        }
        pub fn get_baked_points(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_baked_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_baked_tilts(&self,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_baked_tilts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_baked_up_vectors(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_baked_up_vectors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point(&self, to_point: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3);
            let args = (to_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_offset(&self, to_point: Vector3,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector3);
            let args = (to_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn tessellate_full(&self, max_stages: i32, tolerance_degrees: f32,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, i32, f32);
            let args = (max_stages, tolerance_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tessellate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn tessellate(&self,) -> PackedVector3Array {
            self.tessellate_ex() . done()
        }
        #[inline]
        pub fn tessellate_ex(&self,) -> ExTessellate < '_ > {
            ExTessellate::new(self,)
        }
        pub(crate) fn tessellate_even_length_full(&self, max_stages: i32, tolerance_length: f32,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, i32, f32);
            let args = (max_stages, tolerance_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tessellate_even_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn tessellate_even_length(&self,) -> PackedVector3Array {
            self.tessellate_even_length_ex() . done()
        }
        #[inline]
        pub fn tessellate_even_length_ex(&self,) -> ExTessellateEvenLength < '_ > {
            ExTessellateEvenLength::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for Curve3D {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Curve3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Curve3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Curve3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Curve3D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Curve3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Curve3D {
        
    }
    impl crate::obj::ExportableObject for Curve3D {
        
    }
    impl crate::obj::cap::GodotDefault for Curve3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Curve3D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Curve3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Curve3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Curve3D > for $Class {
                
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
#[doc = "Default-param extender for [`Curve3D::add_point_ex`][super::Curve3D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    surround_object: &'a mut re_export::Curve3D, position: Vector3, in_: Vector3, out: Vector3, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::Curve3D, position: Vector3,) -> Self {
        Self {
            surround_object, position, in_: Vector3::new(0 as _, 0 as _, 0 as _), out: Vector3::new(0 as _, 0 as _, 0 as _), index: - 1i32,
        }
    }
    #[inline]
    pub fn in_(self, value: Vector3) -> Self {
        Self {
            in_: value, .. self
        }
    }
    #[inline]
    pub fn out(self, value: Vector3) -> Self {
        Self {
            out: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Curve3D::add_point_full(self.surround_object, self.position, self.in_, self.out, self.index,)
    }
}
#[doc = "Default-param extender for [`Curve3D::sample_baked_ex`][super::Curve3D::sample_baked_ex]."]
#[must_use]
pub struct ExSampleBaked < 'a > {
    surround_object: &'a re_export::Curve3D, offset: f32, cubic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSampleBaked < 'a > {
    fn new(surround_object: &'a re_export::Curve3D,) -> Self {
        Self {
            surround_object, offset: 0f32, cubic: false,
        }
    }
    #[inline]
    pub fn offset(self, value: f32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn cubic(self, value: bool) -> Self {
        Self {
            cubic: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        re_export::Curve3D::sample_baked_full(self.surround_object, self.offset, self.cubic,)
    }
}
#[doc = "Default-param extender for [`Curve3D::sample_baked_with_rotation_ex`][super::Curve3D::sample_baked_with_rotation_ex]."]
#[must_use]
pub struct ExSampleBakedWithRotation < 'a > {
    surround_object: &'a re_export::Curve3D, offset: f32, cubic: bool, apply_tilt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSampleBakedWithRotation < 'a > {
    fn new(surround_object: &'a re_export::Curve3D,) -> Self {
        Self {
            surround_object, offset: 0f32, cubic: false, apply_tilt: false,
        }
    }
    #[inline]
    pub fn offset(self, value: f32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn cubic(self, value: bool) -> Self {
        Self {
            cubic: value, .. self
        }
    }
    #[inline]
    pub fn apply_tilt(self, value: bool) -> Self {
        Self {
            apply_tilt: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Transform3D {
        re_export::Curve3D::sample_baked_with_rotation_full(self.surround_object, self.offset, self.cubic, self.apply_tilt,)
    }
}
#[doc = "Default-param extender for [`Curve3D::sample_baked_up_vector_ex`][super::Curve3D::sample_baked_up_vector_ex]."]
#[must_use]
pub struct ExSampleBakedUpVector < 'a > {
    surround_object: &'a re_export::Curve3D, offset: f32, apply_tilt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSampleBakedUpVector < 'a > {
    fn new(surround_object: &'a re_export::Curve3D, offset: f32,) -> Self {
        Self {
            surround_object, offset, apply_tilt: false,
        }
    }
    #[inline]
    pub fn apply_tilt(self, value: bool) -> Self {
        Self {
            apply_tilt: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        re_export::Curve3D::sample_baked_up_vector_full(self.surround_object, self.offset, self.apply_tilt,)
    }
}
#[doc = "Default-param extender for [`Curve3D::tessellate_ex`][super::Curve3D::tessellate_ex]."]
#[must_use]
pub struct ExTessellate < 'a > {
    surround_object: &'a re_export::Curve3D, max_stages: i32, tolerance_degrees: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTessellate < 'a > {
    fn new(surround_object: &'a re_export::Curve3D,) -> Self {
        Self {
            surround_object, max_stages: 5i32, tolerance_degrees: 4f32,
        }
    }
    #[inline]
    pub fn max_stages(self, value: i32) -> Self {
        Self {
            max_stages: value, .. self
        }
    }
    #[inline]
    pub fn tolerance_degrees(self, value: f32) -> Self {
        Self {
            tolerance_degrees: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector3Array {
        re_export::Curve3D::tessellate_full(self.surround_object, self.max_stages, self.tolerance_degrees,)
    }
}
#[doc = "Default-param extender for [`Curve3D::tessellate_even_length_ex`][super::Curve3D::tessellate_even_length_ex]."]
#[must_use]
pub struct ExTessellateEvenLength < 'a > {
    surround_object: &'a re_export::Curve3D, max_stages: i32, tolerance_length: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTessellateEvenLength < 'a > {
    fn new(surround_object: &'a re_export::Curve3D,) -> Self {
        Self {
            surround_object, max_stages: 5i32, tolerance_length: 0.2f32,
        }
    }
    #[inline]
    pub fn max_stages(self, value: i32) -> Self {
        Self {
            max_stages: value, .. self
        }
    }
    #[inline]
    pub fn tolerance_length(self, value: f32) -> Self {
        Self {
            tolerance_length: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector3Array {
        re_export::Curve3D::tessellate_even_length_full(self.surround_object, self.max_stages, self.tolerance_length,)
    }
}