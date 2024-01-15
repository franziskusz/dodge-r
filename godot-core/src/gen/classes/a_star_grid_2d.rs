#![doc = "Sidecar module for class [`AStarGrid2D`][crate::engine::AStarGrid2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AStarGrid2D` enums](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AStarGrid2D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`a_star_grid_2d`][crate::engine::a_star_grid_2d]: sidecar module with related enum/flag types\n* [`IAStarGrid2D`][crate::engine::IAStarGrid2D]: virtual methods\n\n\nSee also [Godot docs for `AStarGrid2D`](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AStarGrid2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AStarGrid2D`][crate::engine::AStarGrid2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AStarGrid2D` methods](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAStarGrid2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn estimate_cost(&self, from_id: Vector2i, to_id: Vector2i,) -> f32 {
            unimplemented !()
        }
        fn compute_cost(&self, from_id: Vector2i, to_id: Vector2i,) -> f32 {
            unimplemented !()
        }
    }
    impl AStarGrid2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_region(&mut self, region: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(50usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region(&self,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(51usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(52usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(53usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(54usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(55usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size(&mut self, cell_size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (cell_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(56usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(57usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_bounds(&self, x: i32, y: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(58usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_boundsv(&self, id: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2i);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(59usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_boundsv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dirty(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(60usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_dirty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(61usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_jumping_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(62usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_jumping_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_jumping_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(63usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_jumping_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_diagonal_mode(&mut self, mode: crate::engine::a_star_grid_2d::DiagonalMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::a_star_grid_2d::DiagonalMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(64usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_diagonal_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_diagonal_mode(&self,) -> crate::engine::a_star_grid_2d::DiagonalMode {
            type RetMarshal = PtrcallReturnT < crate::engine::a_star_grid_2d::DiagonalMode >;
            type CallSig = (crate::engine::a_star_grid_2d::DiagonalMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(65usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_diagonal_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_compute_heuristic(&mut self, heuristic: crate::engine::a_star_grid_2d::Heuristic,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::a_star_grid_2d::Heuristic);
            let args = (heuristic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(66usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_compute_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_compute_heuristic(&self,) -> crate::engine::a_star_grid_2d::Heuristic {
            type RetMarshal = PtrcallReturnT < crate::engine::a_star_grid_2d::Heuristic >;
            type CallSig = (crate::engine::a_star_grid_2d::Heuristic,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(67usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_compute_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_estimate_heuristic(&mut self, heuristic: crate::engine::a_star_grid_2d::Heuristic,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::a_star_grid_2d::Heuristic);
            let args = (heuristic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(68usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_estimate_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_estimate_heuristic(&self,) -> crate::engine::a_star_grid_2d::Heuristic {
            type RetMarshal = PtrcallReturnT < crate::engine::a_star_grid_2d::Heuristic >;
            type CallSig = (crate::engine::a_star_grid_2d::Heuristic,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(69usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_estimate_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_point_solid_full(&mut self, id: Vector2i, solid: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, bool);
            let args = (id, solid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(70usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_solid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_point_solid(&mut self, id: Vector2i,) {
            self.set_point_solid_ex(id,) . done()
        }
        #[inline]
        pub fn set_point_solid_ex(&mut self, id: Vector2i,) -> ExSetPointSolid < '_ > {
            ExSetPointSolid::new(self, id,)
        }
        pub fn is_point_solid(&self, id: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2i);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(71usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_point_solid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_weight_scale(&mut self, id: Vector2i, weight_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, f32);
            let args = (id, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(72usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_weight_scale(&self, id: Vector2i,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2i);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(73usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn fill_solid_region_full(&mut self, region: Rect2i, solid: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i, bool);
            let args = (region, solid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(74usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fill_solid_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn fill_solid_region(&mut self, region: Rect2i,) {
            self.fill_solid_region_ex(region,) . done()
        }
        #[inline]
        pub fn fill_solid_region_ex(&mut self, region: Rect2i,) -> ExFillSolidRegion < '_ > {
            ExFillSolidRegion::new(self, region,)
        }
        pub fn fill_weight_scale_region(&mut self, region: Rect2i, weight_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i, f32);
            let args = (region, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(75usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fill_weight_scale_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(76usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_position(&self, id: Vector2i,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2i);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(77usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_path(&mut self, from_id: Vector2i, to_id: Vector2i,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Vector2i, Vector2i);
            let args = (from_id, to_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(78usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_id_path(&mut self, from_id: Vector2i, to_id: Vector2i,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, Vector2i, Vector2i);
            let args = (from_id, to_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(79usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_id_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AStarGrid2D {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AStarGrid2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AStarGrid2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AStarGrid2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AStarGrid2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AStarGrid2D {
        
    }
    impl crate::obj::cap::GodotDefault for AStarGrid2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AStarGrid2D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AStarGrid2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AStarGrid2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AStarGrid2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::set_point_solid_ex`][super::AStarGrid2D::set_point_solid_ex]."]
#[must_use]
pub struct ExSetPointSolid < 'a > {
    surround_object: &'a mut re_export::AStarGrid2D, id: Vector2i, solid: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointSolid < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, id: Vector2i,) -> Self {
        Self {
            surround_object, id, solid: true,
        }
    }
    #[inline]
    pub fn solid(self, value: bool) -> Self {
        Self {
            solid: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStarGrid2D::set_point_solid_full(self.surround_object, self.id, self.solid,)
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::fill_solid_region_ex`][super::AStarGrid2D::fill_solid_region_ex]."]
#[must_use]
pub struct ExFillSolidRegion < 'a > {
    surround_object: &'a mut re_export::AStarGrid2D, region: Rect2i, solid: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFillSolidRegion < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, region: Rect2i,) -> Self {
        Self {
            surround_object, region, solid: true,
        }
    }
    #[inline]
    pub fn solid(self, value: bool) -> Self {
        Self {
            solid: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStarGrid2D::fill_solid_region_full(self.surround_object, self.region, self.solid,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Heuristic {
    ord: i32
}
impl Heuristic {
    pub const HEURISTIC_EUCLIDEAN: Self = Self {
        ord: 0i32
    };
    pub const HEURISTIC_MANHATTAN: Self = Self {
        ord: 1i32
    };
    pub const HEURISTIC_OCTILE: Self = Self {
        ord: 2i32
    };
    pub const HEURISTIC_CHEBYSHEV: Self = Self {
        ord: 3i32
    };
    pub const HEURISTIC_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Heuristic {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Heuristic {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for Heuristic {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Heuristic {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Heuristic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DiagonalMode {
    ord: i32
}
impl DiagonalMode {
    pub const DIAGONAL_MODE_ALWAYS: Self = Self {
        ord: 0i32
    };
    pub const DIAGONAL_MODE_NEVER: Self = Self {
        ord: 1i32
    };
    pub const DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE: Self = Self {
        ord: 2i32
    };
    pub const DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES: Self = Self {
        ord: 3i32
    };
    pub const DIAGONAL_MODE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for DiagonalMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for DiagonalMode {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for DiagonalMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DiagonalMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DiagonalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}