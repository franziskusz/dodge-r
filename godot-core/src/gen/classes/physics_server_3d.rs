#![doc = "Sidecar module for class [`PhysicsServer3D`][crate::engine::PhysicsServer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer3D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`physics_server_3d`][crate::engine::physics_server_3d]: sidecar module with related enum/flag types\n* [`IPhysicsServer3D`][crate::engine::IPhysicsServer3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsServer3D`][crate::engine::PhysicsServer3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsServer3D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"PhysicsServer3D\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "world_boundary_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "separation_ray_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sphere_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sphere_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn box_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "box_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "capsule_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cylinder_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cylinder_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convex_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "concave_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn heightmap_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "heightmap_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn custom_shape_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "custom_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_set_data(&mut self, shape: Rid, data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Variant);
            let args = (shape, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_type(&self, shape: Rid,) -> crate::engine::physics_server_3d::ShapeType {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_3d::ShapeType >;
            type CallSig = (crate::engine::physics_server_3d::ShapeType, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shape_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_param(&mut self, space: Rid, param: crate::engine::physics_server_3d::SpaceParameter, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::SpaceParameter, f32);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_param(&self, space: Rid, param: crate::engine::physics_server_3d::SpaceParameter,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_3d::SpaceParameter);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::engine::PhysicsDirectSpaceState3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsDirectSpaceState3D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsDirectSpaceState3D > >, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "space_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, bool);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5991usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(5992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_param(&mut self, area: Rid, param: crate::engine::physics_server_3d::AreaParameter, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::AreaParameter, Variant);
            let args = (area, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_param(&self, area: Rid, param: crate::engine::physics_server_3d::AreaParameter,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_3d::AreaParameter);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_transform(&self, area: Rid,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable);
            let args = (area, callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable);
            let args = (area, callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_area_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_ray_pickable(&mut self, area: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (area, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "area_set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::engine::physics_server_3d::BodyMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::BodyMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_mode(&self, body: Rid,) -> crate::engine::physics_server_3d::BodyMode {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_3d::BodyMode >;
            type CallSig = (crate::engine::physics_server_3d::BodyMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, bool);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6025usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(6026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_enable_continuous_collision_detection(&mut self, body: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_enable_continuous_collision_detection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_continuous_collision_detection_enabled(&self, body: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_is_continuous_collision_detection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_param(&mut self, body: Rid, param: crate::engine::physics_server_3d::BodyParameter, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::BodyParameter, Variant);
            let args = (body, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_param(&self, body: Rid, param: crate::engine::physics_server_3d::BodyParameter,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_3d::BodyParameter);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_reset_mass_properties", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state(&mut self, body: Rid, state: crate::engine::physics_server_3d::BodyState, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::BodyState, Variant);
            let args = (body, state, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_state(&self, body: Rid, state: crate::engine::physics_server_3d::BodyState,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, crate::engine::physics_server_3d::BodyState);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector3, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector3,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[inline]
        pub fn body_apply_impulse_ex(&mut self, body: Rid, impulse: Vector3,) -> ExBodyApplyImpulse < '_ > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector3,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_apply_force_ex(&mut self, body: Rid, force: Vector3,) -> ExBodyApplyForce < '_ > {
            ExBodyApplyForce::new(self, body, force,)
        }
        pub fn body_apply_torque(&mut self, body: Rid, torque: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector3,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_add_constant_force_ex(&mut self, body: Rid, force: Vector3,) -> ExBodyAddConstantForce < '_ > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_torque(&self, body: Rid,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_lock(&mut self, body: Rid, axis: crate::engine::physics_server_3d::BodyAxis, lock: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::BodyAxis, bool);
            let args = (body, axis, lock,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_axis_locked(&self, body: Rid, axis: crate::engine::physics_server_3d::BodyAxis,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, crate::engine::physics_server_3d::BodyAxis);
            let args = (body, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_is_axis_locked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_omit_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_is_omitting_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: Callable, userdata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable, Variant);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6065usize);
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
        pub fn body_set_ray_pickable(&mut self, body: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters3D >, result: Gd < crate::engine::PhysicsTestMotionResult3D >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, Gd < crate::engine::PhysicsTestMotionParameters3D >, Gd < crate::engine::PhysicsTestMotionResult3D >);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_test_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters3D >,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[inline]
        pub fn body_test_motion_ex(&mut self, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters3D >,) -> ExBodyTestMotion < '_ > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::engine::PhysicsDirectBodyState3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsDirectBodyState3D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsDirectBodyState3D > >, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_bounds(&self, body: Rid,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "soft_body_get_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_clear(&mut self, joint: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_pin(&mut self, joint: Rid, body_A: Rid, local_A: Vector3, body_B: Rid, local_B: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Vector3, Rid, Vector3);
            let args = (joint, body_A, local_A, body_B, local_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_pin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_3d::PinJointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::PinJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_3d::PinJointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_3d::PinJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_local_a(&mut self, joint: Rid, local_A: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (joint, local_A,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_set_local_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_local_a(&self, joint: Rid,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_get_local_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_local_b(&mut self, joint: Rid, local_B: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (joint, local_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_set_local_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_local_b(&self, joint: Rid,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pin_joint_get_local_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_hinge(&mut self, joint: Rid, body_A: Rid, hinge_A: Transform3D, body_B: Rid, hinge_B: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, hinge_A, body_B, hinge_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_hinge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_3d::HingeJointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::HingeJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hinge_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_3d::HingeJointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_3d::HingeJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hinge_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_set_flag(&mut self, joint: Rid, flag: crate::engine::physics_server_3d::HingeJointFlag, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::HingeJointFlag, bool);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hinge_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_get_flag(&self, joint: Rid, flag: crate::engine::physics_server_3d::HingeJointFlag,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, crate::engine::physics_server_3d::HingeJointFlag);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hinge_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_slider(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_slider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn slider_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_3d::SliderJointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::SliderJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "slider_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn slider_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_3d::SliderJointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_3d::SliderJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "slider_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_cone_twist(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_cone_twist", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cone_twist_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_3d::ConeTwistJointParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::physics_server_3d::ConeTwistJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cone_twist_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cone_twist_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_3d::ConeTwistJointParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, crate::engine::physics_server_3d::ConeTwistJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cone_twist_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_type(&self, joint: Rid,) -> crate::engine::physics_server_3d::JointType {
            type RetMarshal = PtrcallReturnT < crate::engine::physics_server_3d::JointType >;
            type CallSig = (crate::engine::physics_server_3d::JointType, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_set_solver_priority(&mut self, joint: Rid, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (joint, priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_set_solver_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_solver_priority(&self, joint: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_get_solver_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_disable_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_is_disabled_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_generic_6dof(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "joint_make_generic_6dof", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_set_param(&mut self, joint: Rid, axis: Vector3Axis, param: crate::engine::physics_server_3d::G6DOFJointAxisParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3Axis, crate::engine::physics_server_3d::G6DOFJointAxisParam, f32);
            let args = (joint, axis, param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generic_6dof_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_get_param(&self, joint: Rid, axis: Vector3Axis, param: crate::engine::physics_server_3d::G6DOFJointAxisParam,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, Vector3Axis, crate::engine::physics_server_3d::G6DOFJointAxisParam);
            let args = (joint, axis, param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generic_6dof_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_set_flag(&mut self, joint: Rid, axis: Vector3Axis, flag: crate::engine::physics_server_3d::G6DOFJointAxisFlag, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3Axis, crate::engine::physics_server_3d::G6DOFJointAxisFlag, bool);
            let args = (joint, axis, flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generic_6dof_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_get_flag(&self, joint: Rid, axis: Vector3Axis, flag: crate::engine::physics_server_3d::G6DOFJointAxisFlag,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, Vector3Axis, crate::engine::physics_server_3d::G6DOFJointAxisFlag);
            let args = (joint, axis, flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generic_6dof_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_info(&mut self, process_info: crate::engine::physics_server_3d::ProcessInfo,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::physics_server_3d::ProcessInfo);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6102usize);
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
    impl crate::obj::GodotClass for PhysicsServer3D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsServer3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsServer3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsServer3D {
        
    }
    impl std::ops::Deref for PhysicsServer3D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsServer3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsServer3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::area_add_shape_ex`][super::PhysicsServer3D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAreaAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, area: Rid, shape: Rid,) -> Self {
        Self {
            surround_object, area, shape, transform: Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _), disabled: false,
        }
    }
    #[inline]
    pub fn transform(self, value: Transform3D) -> Self {
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
        re_export::PhysicsServer3D::area_add_shape_full(self.surround_object, self.area, self.shape, self.transform, self.disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_shape_ex`][super::PhysicsServer3D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, shape: Rid,) -> Self {
        Self {
            surround_object, body, shape, transform: Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _), disabled: false,
        }
    }
    #[inline]
    pub fn transform(self, value: Transform3D) -> Self {
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
        re_export::PhysicsServer3D::body_add_shape_full(self.surround_object, self.body, self.shape, self.transform, self.disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_impulse_ex`][super::PhysicsServer3D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3,) -> Self {
        Self {
            surround_object, body, impulse, position: Vector3::new(0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector3) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer3D::body_apply_impulse_full(self.surround_object, self.body, self.impulse, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_force_ex`][super::PhysicsServer3D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        Self {
            surround_object, body, force, position: Vector3::new(0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector3) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer3D::body_apply_force_full(self.surround_object, self.body, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_constant_force_ex`][super::PhysicsServer3D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        Self {
            surround_object, body, force, position: Vector3::new(0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector3) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsServer3D::body_add_constant_force_full(self.surround_object, self.body, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_set_force_integration_callback_ex`][super::PhysicsServer3D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, callable: Callable, userdata: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodySetForceIntegrationCallback < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, callable: Callable,) -> Self {
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
        re_export::PhysicsServer3D::body_set_force_integration_callback_full(self.surround_object, self.body, self.callable, self.userdata,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_test_motion_ex`][super::PhysicsServer3D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'a > {
    surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters3D >, result: Gd < crate::engine::PhysicsTestMotionResult3D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyTestMotion < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, parameters: Gd < crate::engine::PhysicsTestMotionParameters3D >,) -> Self {
        Self {
            surround_object, body, parameters, result: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn result(self, value: Gd < crate::engine::PhysicsTestMotionResult3D >) -> Self {
        Self {
            result: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::PhysicsServer3D::body_test_motion_full(self.surround_object, self.body, self.parameters, self.result,)
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
    pub const JOINT_TYPE_HINGE: Self = Self {
        ord: 1i32
    };
    pub const JOINT_TYPE_SLIDER: Self = Self {
        ord: 2i32
    };
    pub const JOINT_TYPE_CONE_TWIST: Self = Self {
        ord: 3i32
    };
    pub const JOINT_TYPE_6DOF: Self = Self {
        ord: 4i32
    };
    pub const JOINT_TYPE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for JointType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
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
pub struct PinJointParam {
    ord: i32
}
impl PinJointParam {
    pub const PIN_JOINT_BIAS: Self = Self {
        ord: 0i32
    };
    pub const PIN_JOINT_DAMPING: Self = Self {
        ord: 1i32
    };
    pub const PIN_JOINT_IMPULSE_CLAMP: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PinJointParam {
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
pub struct HingeJointParam {
    ord: i32
}
impl HingeJointParam {
    pub const HINGE_JOINT_BIAS: Self = Self {
        ord: 0i32
    };
    pub const HINGE_JOINT_LIMIT_UPPER: Self = Self {
        ord: 1i32
    };
    pub const HINGE_JOINT_LIMIT_LOWER: Self = Self {
        ord: 2i32
    };
    pub const HINGE_JOINT_LIMIT_BIAS: Self = Self {
        ord: 3i32
    };
    pub const HINGE_JOINT_LIMIT_SOFTNESS: Self = Self {
        ord: 4i32
    };
    pub const HINGE_JOINT_LIMIT_RELAXATION: Self = Self {
        ord: 5i32
    };
    pub const HINGE_JOINT_MOTOR_TARGET_VELOCITY: Self = Self {
        ord: 6i32
    };
    pub const HINGE_JOINT_MOTOR_MAX_IMPULSE: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for HingeJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for HingeJointParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HingeJointParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HingeJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HingeJointFlag {
    ord: i32
}
impl HingeJointFlag {
    pub const HINGE_JOINT_FLAG_USE_LIMIT: Self = Self {
        ord: 0i32
    };
    pub const HINGE_JOINT_FLAG_ENABLE_MOTOR: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for HingeJointFlag {
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
impl crate::builtin::meta::GodotConvert for HingeJointFlag {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HingeJointFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HingeJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SliderJointParam {
    ord: i32
}
impl SliderJointParam {
    pub const SLIDER_JOINT_LINEAR_LIMIT_UPPER: Self = Self {
        ord: 0i32
    };
    pub const SLIDER_JOINT_LINEAR_LIMIT_LOWER: Self = Self {
        ord: 1i32
    };
    pub const SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS: Self = Self {
        ord: 2i32
    };
    pub const SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION: Self = Self {
        ord: 3i32
    };
    pub const SLIDER_JOINT_LINEAR_LIMIT_DAMPING: Self = Self {
        ord: 4i32
    };
    pub const SLIDER_JOINT_LINEAR_MOTION_SOFTNESS: Self = Self {
        ord: 5i32
    };
    pub const SLIDER_JOINT_LINEAR_MOTION_RESTITUTION: Self = Self {
        ord: 6i32
    };
    pub const SLIDER_JOINT_LINEAR_MOTION_DAMPING: Self = Self {
        ord: 7i32
    };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS: Self = Self {
        ord: 8i32
    };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION: Self = Self {
        ord: 9i32
    };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING: Self = Self {
        ord: 10i32
    };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_UPPER: Self = Self {
        ord: 11i32
    };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_LOWER: Self = Self {
        ord: 12i32
    };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS: Self = Self {
        ord: 13i32
    };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION: Self = Self {
        ord: 14i32
    };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_DAMPING: Self = Self {
        ord: 15i32
    };
    pub const SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS: Self = Self {
        ord: 16i32
    };
    pub const SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION: Self = Self {
        ord: 17i32
    };
    pub const SLIDER_JOINT_ANGULAR_MOTION_DAMPING: Self = Self {
        ord: 18i32
    };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS: Self = Self {
        ord: 19i32
    };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION: Self = Self {
        ord: 20i32
    };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING: Self = Self {
        ord: 21i32
    };
    pub const SLIDER_JOINT_MAX: Self = Self {
        ord: 22i32
    };
    
}
impl crate::obj::EngineEnum for SliderJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for SliderJointParam {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::builtin::meta::GodotConvert for SliderJointParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SliderJointParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SliderJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ConeTwistJointParam {
    ord: i32
}
impl ConeTwistJointParam {
    pub const CONE_TWIST_JOINT_SWING_SPAN: Self = Self {
        ord: 0i32
    };
    pub const CONE_TWIST_JOINT_TWIST_SPAN: Self = Self {
        ord: 1i32
    };
    pub const CONE_TWIST_JOINT_BIAS: Self = Self {
        ord: 2i32
    };
    pub const CONE_TWIST_JOINT_SOFTNESS: Self = Self {
        ord: 3i32
    };
    pub const CONE_TWIST_JOINT_RELAXATION: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ConeTwistJointParam {
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
impl crate::builtin::meta::GodotConvert for ConeTwistJointParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ConeTwistJointParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ConeTwistJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct G6DOFJointAxisParam {
    ord: i32
}
impl G6DOFJointAxisParam {
    pub const G6DOF_JOINT_LINEAR_LOWER_LIMIT: Self = Self {
        ord: 0i32
    };
    pub const G6DOF_JOINT_LINEAR_UPPER_LIMIT: Self = Self {
        ord: 1i32
    };
    pub const G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS: Self = Self {
        ord: 2i32
    };
    pub const G6DOF_JOINT_LINEAR_RESTITUTION: Self = Self {
        ord: 3i32
    };
    pub const G6DOF_JOINT_LINEAR_DAMPING: Self = Self {
        ord: 4i32
    };
    pub const G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY: Self = Self {
        ord: 5i32
    };
    pub const G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT: Self = Self {
        ord: 6i32
    };
    pub const G6DOF_JOINT_ANGULAR_LOWER_LIMIT: Self = Self {
        ord: 10i32
    };
    pub const G6DOF_JOINT_ANGULAR_UPPER_LIMIT: Self = Self {
        ord: 11i32
    };
    pub const G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS: Self = Self {
        ord: 12i32
    };
    pub const G6DOF_JOINT_ANGULAR_DAMPING: Self = Self {
        ord: 13i32
    };
    pub const G6DOF_JOINT_ANGULAR_RESTITUTION: Self = Self {
        ord: 14i32
    };
    pub const G6DOF_JOINT_ANGULAR_FORCE_LIMIT: Self = Self {
        ord: 15i32
    };
    pub const G6DOF_JOINT_ANGULAR_ERP: Self = Self {
        ord: 16i32
    };
    pub const G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY: Self = Self {
        ord: 17i32
    };
    pub const G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT: Self = Self {
        ord: 18i32
    };
    
}
impl crate::obj::EngineEnum for G6DOFJointAxisParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for G6DOFJointAxisParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for G6DOFJointAxisParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for G6DOFJointAxisParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct G6DOFJointAxisFlag {
    ord: i32
}
impl G6DOFJointAxisFlag {
    pub const G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT: Self = Self {
        ord: 0i32
    };
    pub const G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT: Self = Self {
        ord: 1i32
    };
    pub const G6DOF_JOINT_FLAG_ENABLE_MOTOR: Self = Self {
        ord: 4i32
    };
    pub const G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for G6DOFJointAxisFlag {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for G6DOFJointAxisFlag {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for G6DOFJointAxisFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for G6DOFJointAxisFlag {
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
    pub const SHAPE_SPHERE: Self = Self {
        ord: 2i32
    };
    pub const SHAPE_BOX: Self = Self {
        ord: 3i32
    };
    pub const SHAPE_CAPSULE: Self = Self {
        ord: 4i32
    };
    pub const SHAPE_CYLINDER: Self = Self {
        ord: 5i32
    };
    pub const SHAPE_CONVEX_POLYGON: Self = Self {
        ord: 6i32
    };
    pub const SHAPE_CONCAVE_POLYGON: Self = Self {
        ord: 7i32
    };
    pub const SHAPE_HEIGHTMAP: Self = Self {
        ord: 8i32
    };
    pub const SHAPE_SOFT_BODY: Self = Self {
        ord: 9i32
    };
    pub const SHAPE_CUSTOM: Self = Self {
        ord: 10i32
    };
    
}
impl crate::obj::EngineEnum for ShapeType {
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
    pub const AREA_PARAM_WIND_FORCE_MAGNITUDE: Self = Self {
        ord: 10i32
    };
    pub const AREA_PARAM_WIND_SOURCE: Self = Self {
        ord: 11i32
    };
    pub const AREA_PARAM_WIND_DIRECTION: Self = Self {
        ord: 12i32
    };
    pub const AREA_PARAM_WIND_ATTENUATION_FACTOR: Self = Self {
        ord: 13i32
    };
    
}
impl crate::obj::EngineEnum for AreaParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
    pub const SPACE_PARAM_SOLVER_ITERATIONS: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for SpaceParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
pub struct BodyAxis {
    ord: i32
}
impl BodyAxis {
    pub const BODY_AXIS_LINEAR_X: Self = Self {
        ord: 1i32
    };
    pub const BODY_AXIS_LINEAR_Y: Self = Self {
        ord: 2i32
    };
    pub const BODY_AXIS_LINEAR_Z: Self = Self {
        ord: 4i32
    };
    pub const BODY_AXIS_ANGULAR_X: Self = Self {
        ord: 8i32
    };
    pub const BODY_AXIS_ANGULAR_Y: Self = Self {
        ord: 16i32
    };
    pub const BODY_AXIS_ANGULAR_Z: Self = Self {
        ord: 32i32
    };
    
}
impl crate::obj::EngineEnum for BodyAxis {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for BodyAxis {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BodyAxis {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BodyAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}