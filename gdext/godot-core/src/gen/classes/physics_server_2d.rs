#![doc = "Sidecar module for class [`PhysicsServer2D`][crate::engine::PhysicsServer2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`physics_server_2d`][crate::engine::physics_server_2d]: sidecar module with related enum/flag types\n* [`IPhysicsServer2D`][crate::engine::IPhysicsServer2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer2D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsServer2D`][crate::engine::PhysicsServer2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsServer2D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"PhysicsServer2D\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "world_boundary_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "separation_ray_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn circle_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "circle_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rectangle_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rectangle_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "capsule_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convex_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "concave_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_set_data(&mut self, shape: Rid, data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Variant);
            let args = (shape, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_type(&self, shape: Rid,) -> crate::engine::physics_server_2d::ShapeType {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_2d::ShapeType >;
            type CallSig = (crate::engine::physics_server_2d::ShapeType, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_param(&mut self, space: Rid, param: crate::engine::physics_server_2d::SpaceParameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::SpaceParameter, f32);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_param(&self, space: Rid, param: crate::engine::physics_server_2d::SpaceParameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_2d::SpaceParameter);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::engine::PhysicsDirectSpaceState2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsDirectSpaceState2D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsDirectSpaceState2D > >, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform2D, bool);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn area_add_shape(&mut self, area: Rid, shape: Rid,) {
            self.area_add_shape_ex(area, shape,) . done()
        }
        #[inline]
        pub fn area_add_shape_ex(&mut self, area: Rid, shape: Rid,) -> ExAreaAddShape < '_ > {
            ExAreaAddShape::new(self, area, shape,)
        }
        pub fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Rid);
            let args = (area, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_param(&mut self, area: Rid, param: crate::engine::physics_server_2d::AreaParameter, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::AreaParameter, Variant);
            let args = (area, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_param(&self, area: Rid, param: crate::engine::physics_server_2d::AreaParameter,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_2d::AreaParameter);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_transform(&self, area: Rid,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_attach_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_canvas_instance_id(&self, area: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable);
            let args = (area, callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable);
            let args = (area, callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_area_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::engine::physics_server_2d::BodyMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::BodyMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_mode(&self, body: Rid,) -> crate::engine::physics_server_2d::BodyMode {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_2d::BodyMode >;
            type CallSig = (crate::engine::physics_server_2d::BodyMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform2D, bool);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_add_shape(&mut self, body: Rid, shape: Rid,) {
            self.body_add_shape_ex(body, shape,) . done()
        }
        #[inline]
        pub fn body_add_shape_ex(&mut self, body: Rid, shape: Rid,) -> ExBodyAddShape < '_ > {
            ExBodyAddShape::new(self, body, shape,)
        }
        pub fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Rid);
            let args = (body, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool, f32);
            let args = (body, shape_idx, enable, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape_as_one_way_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_attach_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_canvas_instance_id(&self, body: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::engine::physics_server_2d::CCDMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::CCDMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::engine::physics_server_2d::CCDMode {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_2d::CCDMode >;
            type CallSig = (crate::engine::physics_server_2d::CCDMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_param(&mut self, body: Rid, param: crate::engine::physics_server_2d::BodyParameter, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::BodyParameter, Variant);
            let args = (body, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_param(&self, body: Rid, param: crate::engine::physics_server_2d::BodyParameter,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_2d::BodyParameter);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_reset_mass_properties", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state(&mut self, body: Rid, state: crate::engine::physics_server_2d::BodyState, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::BodyState, Variant);
            let args = (body, state, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_state(&self, body: Rid, state: crate::engine::physics_server_2d::BodyState,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_2d::BodyState);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[inline]
        pub fn body_apply_impulse_ex(&mut self, body: Rid, impulse: Vector2,) -> ExBodyApplyImpulse < '_ > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector2,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_apply_force_ex(&mut self, body: Rid, force: Vector2,) -> ExBodyApplyForce < '_ > {
            ExBodyApplyForce::new(self, body, force,)
        }
        pub fn body_apply_torque(&mut self, body: Rid, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector2,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_add_constant_force_ex(&mut self, body: Rid, force: Vector2,) -> ExBodyAddConstantForce < '_ > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_torque(&self, body: Rid,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_omit_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_is_omitting_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: Callable, userdata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable, Variant);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_force_integration_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_set_force_integration_callback(&mut self, body: Rid, callable: Callable,) {
            self.body_set_force_integration_callback_ex(body, callable,) . done()
        }
        #[inline]
        pub fn body_set_force_integration_callback_ex(&mut self, body: Rid, callable: Callable,) -> ExBodySetForceIntegrationCallback < '_ > {
            ExBodySetForceIntegrationCallback::new(self, body, callable,)
        }
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters2D >, result: Gd < crate::engine::PhysicsTestMotionResult2D >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, Gd < crate::engine::PhysicsTestMotionParameters2D >, Gd < crate::engine::PhysicsTestMotionResult2D >);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_test_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters2D >,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[inline]
        pub fn body_test_motion_ex(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters2D >,) -> ExBodyTestMotion < '_ > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::engine::PhysicsDirectBodyState2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsDirectBodyState2D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsDirectBodyState2D > >, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_clear(&mut self, joint: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::JointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::JointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::JointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_2d::JointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_disable_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_is_disabled_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn joint_make_pin_full(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Rid, Rid);
            let args = (joint, anchor, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_pin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid,) {
            self.joint_make_pin_ex(joint, anchor, body_a,) . done()
        }
        #[inline]
        pub fn joint_make_pin_ex(&mut self, joint: Rid, anchor: Vector2, body_a: Rid,) -> ExJointMakePin < '_ > {
            ExJointMakePin::new(self, joint, anchor, body_a,)
        }
        pub(crate) fn joint_make_groove_full(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2, Vector2, Rid, Rid);
            let args = (joint, groove1_a, groove2_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_groove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn joint_make_groove(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) {
            self.joint_make_groove_ex(joint, groove1_a, groove2_a, anchor_b,) . done()
        }
        #[inline]
        pub fn joint_make_groove_ex(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> ExJointMakeGroove < '_ > {
            ExJointMakeGroove::new(self, joint, groove1_a, groove2_a, anchor_b,)
        }
        pub(crate) fn joint_make_damped_spring_full(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2, Rid, Rid);
            let args = (joint, anchor_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_damped_spring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) {
            self.joint_make_damped_spring_ex(joint, anchor_a, anchor_b, body_a,) . done()
        }
        #[inline]
        pub fn joint_make_damped_spring_ex(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> ExJointMakeDampedSpring < '_ > {
            ExJointMakeDampedSpring::new(self, joint, anchor_a, anchor_b, body_a,)
        }
        pub fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::engine::physics_server_2d::PinJointFlag, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::PinJointFlag, bool);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_flag(&self, joint: Rid, flag: crate::engine::physics_server_2d::PinJointFlag,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, crate::engine::physics_server_2d::PinJointFlag);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::PinJointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::PinJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::PinJointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_2d::PinJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::DampedSpringParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_2d::DampedSpringParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "damped_spring_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::DampedSpringParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_2d::DampedSpringParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "damped_spring_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_type(&self, joint: Rid,) -> crate::engine::physics_server_2d::JointType {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_2d::JointType >;
            type CallSig = (crate::engine::physics_server_2d::JointType, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_info(&mut self, process_info: crate::engine::physics_server_2d::ProcessInfo,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::physics_server_2d::ProcessInfo);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_info", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsServer2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsServer2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsServer2D {
        
    }
    impl std::ops::Deref for PhysicsServer2D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsServer2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsServer2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::area_add_shape_ex`][super::PhysicsServer2D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAreaAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, area: Rid, shape: Rid,) -> Self {
        Self {
            surround_object, area, shape, transform: Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _), disabled: false,
        }
    }
    #[inline]
    pub fn transform(self, value: Transform2D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn disabled(self, value: bool) -> Self {
        Self {
            disabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::area_add_shape_full(self.surround_object, self.area, self.shape, self.transform, self.disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_shape_ex`][super::PhysicsServer2D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, shape: Rid,) -> Self {
        Self {
            surround_object, body, shape, transform: Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _), disabled: false,
        }
    }
    #[inline]
    pub fn transform(self, value: Transform2D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn disabled(self, value: bool) -> Self {
        Self {
            disabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::body_add_shape_full(self.surround_object, self.body, self.shape, self.transform, self.disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_impulse_ex`][super::PhysicsServer2D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2,) -> Self {
        Self {
            surround_object, body, impulse, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::body_apply_impulse_full(self.surround_object, self.body, self.impulse, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_force_ex`][super::PhysicsServer2D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        Self {
            surround_object, body, force, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::body_apply_force_full(self.surround_object, self.body, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_constant_force_ex`][super::PhysicsServer2D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        Self {
            surround_object, body, force, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::body_add_constant_force_full(self.surround_object, self.body, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_set_force_integration_callback_ex`][super::PhysicsServer2D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, callable: Callable, userdata: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodySetForceIntegrationCallback < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, callable: Callable,) -> Self {
        Self {
            surround_object, body, callable, userdata: Variant::nil(),
        }
    }
    #[inline]
    pub fn userdata(self, value: Variant) -> Self {
        Self {
            userdata: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::body_set_force_integration_callback_full(self.surround_object, self.body, self.callable, self.userdata,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_test_motion_ex`][super::PhysicsServer2D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters2D >, result: Gd < crate::engine::PhysicsTestMotionResult2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyTestMotion < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters2D >,) -> Self {
        Self {
            surround_object, body, parameters, result: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn result(self, value: Gd < crate::engine::PhysicsTestMotionResult2D >) -> Self {
        Self {
            result: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::PhysicsServer2D::body_test_motion_full(self.surround_object, self.body, self.parameters, self.result,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_pin_ex`][super::PhysicsServer2D::joint_make_pin_ex]."]
#[must_use]
pub struct ExJointMakePin < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakePin < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid,) -> Self {
        Self {
            surround_object, joint, anchor, body_a, body_b: Rid::Invalid,
        }
    }
    #[inline]
    pub fn body_b(self, value: Rid) -> Self {
        Self {
            body_b: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::joint_make_pin_full(self.surround_object, self.joint, self.anchor, self.body_a, self.body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_groove_ex`][super::PhysicsServer2D::joint_make_groove_ex]."]
#[must_use]
pub struct ExJointMakeGroove < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakeGroove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> Self {
        Self {
            surround_object, joint, groove1_a, groove2_a, anchor_b, body_a: Rid::Invalid, body_b: Rid::Invalid,
        }
    }
    #[inline]
    pub fn body_a(self, value: Rid) -> Self {
        Self {
            body_a: value, .. self
        }
    }
    #[inline]
    pub fn body_b(self, value: Rid) -> Self {
        Self {
            body_b: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::joint_make_groove_full(self.surround_object, self.joint, self.groove1_a, self.groove2_a, self.anchor_b, self.body_a, self.body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_damped_spring_ex`][super::PhysicsServer2D::joint_make_damped_spring_ex]."]
#[must_use]
pub struct ExJointMakeDampedSpring < 'a > {
    surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakeDampedSpring < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> Self {
        Self {
            surround_object, joint, anchor_a, anchor_b, body_a, body_b: Rid::Invalid,
        }
    }
    #[inline]
    pub fn body_b(self, value: Rid) -> Self {
        Self {
            body_b: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer2D::joint_make_damped_spring_full(self.surround_object, self.joint, self.anchor_a, self.anchor_b, self.body_a, self.body_b,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpaceParameter {
    ord: i32
}
impl SpaceParameter {
    pub const SPACE_PARAM_CONTACT_RECYCLE_RADIUS: Self = Self {
        ord: 0i32
    };
    pub const SPACE_PARAM_CONTACT_MAX_SEPARATION: Self = Self {
        ord: 1i32
    };
    pub const SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION: Self = Self {
        ord: 2i32
    };
    pub const SPACE_PARAM_CONTACT_DEFAULT_BIAS: Self = Self {
        ord: 3i32
    };
    pub const SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD: Self = Self {
        ord: 4i32
    };
    pub const SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD: Self = Self {
        ord: 5i32
    };
    pub const SPACE_PARAM_BODY_TIME_TO_SLEEP: Self = Self {
        ord: 6i32
    };
    pub const SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS: Self = Self {
        ord: 7i32
    };
    pub const SPACE_PARAM_SOLVER_ITERATIONS: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for SpaceParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SpaceParameter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpaceParameter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpaceParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShapeType {
    ord: i32
}
impl ShapeType {
    pub const SHAPE_WORLD_BOUNDARY: Self = Self {
        ord: 0i32
    };
    pub const SHAPE_SEPARATION_RAY: Self = Self {
        ord: 1i32
    };
    pub const SHAPE_SEGMENT: Self = Self {
        ord: 2i32
    };
    pub const SHAPE_CIRCLE: Self = Self {
        ord: 3i32
    };
    pub const SHAPE_RECTANGLE: Self = Self {
        ord: 4i32
    };
    pub const SHAPE_CAPSULE: Self = Self {
        ord: 5i32
    };
    pub const SHAPE_CONVEX_POLYGON: Self = Self {
        ord: 6i32
    };
    pub const SHAPE_CONCAVE_POLYGON: Self = Self {
        ord: 7i32
    };
    pub const SHAPE_CUSTOM: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for ShapeType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ShapeType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShapeType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AreaParameter {
    ord: i32
}
impl AreaParameter {
    pub const AREA_PARAM_GRAVITY_OVERRIDE_MODE: Self = Self {
        ord: 0i32
    };
    pub const AREA_PARAM_GRAVITY: Self = Self {
        ord: 1i32
    };
    pub const AREA_PARAM_GRAVITY_VECTOR: Self = Self {
        ord: 2i32
    };
    pub const AREA_PARAM_GRAVITY_IS_POINT: Self = Self {
        ord: 3i32
    };
    pub const AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE: Self = Self {
        ord: 4i32
    };
    pub const AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE: Self = Self {
        ord: 5i32
    };
    pub const AREA_PARAM_LINEAR_DAMP: Self = Self {
        ord: 6i32
    };
    pub const AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE: Self = Self {
        ord: 7i32
    };
    pub const AREA_PARAM_ANGULAR_DAMP: Self = Self {
        ord: 8i32
    };
    pub const AREA_PARAM_PRIORITY: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for AreaParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for AreaParameter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AreaParameter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AreaParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AreaSpaceOverrideMode {
    ord: i32
}
impl AreaSpaceOverrideMode {
    pub const AREA_SPACE_OVERRIDE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const AREA_SPACE_OVERRIDE_COMBINE: Self = Self {
        ord: 1i32
    };
    pub const AREA_SPACE_OVERRIDE_COMBINE_REPLACE: Self = Self {
        ord: 2i32
    };
    pub const AREA_SPACE_OVERRIDE_REPLACE: Self = Self {
        ord: 3i32
    };
    pub const AREA_SPACE_OVERRIDE_REPLACE_COMBINE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for AreaSpaceOverrideMode {
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
impl crate::builtin::meta::GodotConvert for AreaSpaceOverrideMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AreaSpaceOverrideMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AreaSpaceOverrideMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BodyMode {
    ord: i32
}
impl BodyMode {
    pub const BODY_MODE_STATIC: Self = Self {
        ord: 0i32
    };
    pub const BODY_MODE_KINEMATIC: Self = Self {
        ord: 1i32
    };
    pub const BODY_MODE_RIGID: Self = Self {
        ord: 2i32
    };
    pub const BODY_MODE_RIGID_LINEAR: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for BodyMode {
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
impl crate::builtin::meta::GodotConvert for BodyMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BodyMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BodyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BodyParameter {
    ord: i32
}
impl BodyParameter {
    pub const BODY_PARAM_BOUNCE: Self = Self {
        ord: 0i32
    };
    pub const BODY_PARAM_FRICTION: Self = Self {
        ord: 1i32
    };
    pub const BODY_PARAM_MASS: Self = Self {
        ord: 2i32
    };
    pub const BODY_PARAM_INERTIA: Self = Self {
        ord: 3i32
    };
    pub const BODY_PARAM_CENTER_OF_MASS: Self = Self {
        ord: 4i32
    };
    pub const BODY_PARAM_GRAVITY_SCALE: Self = Self {
        ord: 5i32
    };
    pub const BODY_PARAM_LINEAR_DAMP_MODE: Self = Self {
        ord: 6i32
    };
    pub const BODY_PARAM_ANGULAR_DAMP_MODE: Self = Self {
        ord: 7i32
    };
    pub const BODY_PARAM_LINEAR_DAMP: Self = Self {
        ord: 8i32
    };
    pub const BODY_PARAM_ANGULAR_DAMP: Self = Self {
        ord: 9i32
    };
    pub const BODY_PARAM_MAX: Self = Self {
        ord: 10i32
    };
    
}
impl crate::obj::EngineEnum for BodyParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for BodyParameter {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::builtin::meta::GodotConvert for BodyParameter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BodyParameter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BodyParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BodyDampMode {
    ord: i32
}
impl BodyDampMode {
    pub const BODY_DAMP_MODE_COMBINE: Self = Self {
        ord: 0i32
    };
    pub const BODY_DAMP_MODE_REPLACE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for BodyDampMode {
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
impl crate::builtin::meta::GodotConvert for BodyDampMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BodyDampMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BodyDampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BodyState {
    ord: i32
}
impl BodyState {
    pub const BODY_STATE_TRANSFORM: Self = Self {
        ord: 0i32
    };
    pub const BODY_STATE_LINEAR_VELOCITY: Self = Self {
        ord: 1i32
    };
    pub const BODY_STATE_ANGULAR_VELOCITY: Self = Self {
        ord: 2i32
    };
    pub const BODY_STATE_SLEEPING: Self = Self {
        ord: 3i32
    };
    pub const BODY_STATE_CAN_SLEEP: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for BodyState {
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
impl crate::builtin::meta::GodotConvert for BodyState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BodyState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BodyState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct JointType {
    ord: i32
}
impl JointType {
    pub const JOINT_TYPE_PIN: Self = Self {
        ord: 0i32
    };
    pub const JOINT_TYPE_GROOVE: Self = Self {
        ord: 1i32
    };
    pub const JOINT_TYPE_DAMPED_SPRING: Self = Self {
        ord: 2i32
    };
    pub const JOINT_TYPE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for JointType {
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
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for JointType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for JointType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct JointParam {
    ord: i32
}
impl JointParam {
    pub const JOINT_PARAM_BIAS: Self = Self {
        ord: 0i32
    };
    pub const JOINT_PARAM_MAX_BIAS: Self = Self {
        ord: 1i32
    };
    pub const JOINT_PARAM_MAX_FORCE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for JointParam {
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
impl crate::builtin::meta::GodotConvert for JointParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for JointParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for JointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PinJointParam {
    ord: i32
}
impl PinJointParam {
    pub const PIN_JOINT_SOFTNESS: Self = Self {
        ord: 0i32
    };
    pub const PIN_JOINT_LIMIT_UPPER: Self = Self {
        ord: 1i32
    };
    pub const PIN_JOINT_LIMIT_LOWER: Self = Self {
        ord: 2i32
    };
    pub const PIN_JOINT_MOTOR_TARGET_VELOCITY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for PinJointParam {
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
impl crate::builtin::meta::GodotConvert for PinJointParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PinJointParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PinJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PinJointFlag {
    ord: i32
}
impl PinJointFlag {
    pub const PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED: Self = Self {
        ord: 0i32
    };
    pub const PIN_JOINT_FLAG_MOTOR_ENABLED: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for PinJointFlag {
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
impl crate::builtin::meta::GodotConvert for PinJointFlag {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PinJointFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PinJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DampedSpringParam {
    ord: i32
}
impl DampedSpringParam {
    pub const DAMPED_SPRING_REST_LENGTH: Self = Self {
        ord: 0i32
    };
    pub const DAMPED_SPRING_STIFFNESS: Self = Self {
        ord: 1i32
    };
    pub const DAMPED_SPRING_DAMPING: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DampedSpringParam {
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
impl crate::builtin::meta::GodotConvert for DampedSpringParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DampedSpringParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DampedSpringParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CCDMode {
    ord: i32
}
impl CCDMode {
    pub const CCD_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CCD_MODE_CAST_RAY: Self = Self {
        ord: 1i32
    };
    pub const CCD_MODE_CAST_SHAPE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CCDMode {
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
impl crate::builtin::meta::GodotConvert for CCDMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CCDMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CCDMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AreaBodyStatus {
    ord: i32
}
impl AreaBodyStatus {
    pub const AREA_BODY_ADDED: Self = Self {
        ord: 0i32
    };
    pub const AREA_BODY_REMOVED: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for AreaBodyStatus {
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
impl crate::builtin::meta::GodotConvert for AreaBodyStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AreaBodyStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AreaBodyStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProcessInfo {
    ord: i32
}
impl ProcessInfo {
    pub const INFO_ACTIVE_OBJECTS: Self = Self {
        ord: 0i32
    };
    pub const INFO_COLLISION_PAIRS: Self = Self {
        ord: 1i32
    };
    pub const INFO_ISLAND_COUNT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ProcessInfo {
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
impl crate::builtin::meta::GodotConvert for ProcessInfo {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ProcessInfo {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ProcessInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}