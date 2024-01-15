#![doc = "Sidecar module for class [`PhysicsServer2DExtension`][crate::engine::PhysicsServer2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2DExtension.`\n\nInherits [`PhysicsServer2D`][crate::engine::PhysicsServer2D].\n\nRelated symbols:\n\n* [`IPhysicsServer2DExtension`][crate::engine::IPhysicsServer2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsServer2DExtension`][crate::engine::PhysicsServer2DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer2DExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn world_boundary_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn separation_ray_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn segment_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn circle_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn rectangle_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn capsule_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn convex_polygon_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn concave_polygon_shape_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn shape_set_data(&mut self, shape: Rid, data: Variant,) {
            unimplemented !()
        }
        fn shape_set_custom_solver_bias(&mut self, shape: Rid, bias: f32,) {
            unimplemented !()
        }
        fn shape_get_type(&self, shape: Rid,) -> crate::engine::physics_server_2d::ShapeType {
            unimplemented !()
        }
        fn shape_get_data(&self, shape: Rid,) -> Variant {
            unimplemented !()
        }
        fn shape_get_custom_solver_bias(&self, shape: Rid,) -> f32 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn shape_collide(&mut self, shape_A: Rid, xform_A: Transform2D, motion_A: Vector2, shape_B: Rid, xform_B: Transform2D, motion_B: Vector2, results: * mut c_void, result_max: i32, result_count: * mut i32,) -> bool {
            unimplemented !()
        }
        fn space_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn space_set_active(&mut self, space: Rid, active: bool,) {
            unimplemented !()
        }
        fn space_is_active(&self, space: Rid,) -> bool {
            unimplemented !()
        }
        fn space_set_param(&mut self, space: Rid, param: crate::engine::physics_server_2d::SpaceParameter, value: f32,) {
            unimplemented !()
        }
        fn space_get_param(&self, space: Rid, param: crate::engine::physics_server_2d::SpaceParameter,) -> f32 {
            unimplemented !()
        }
        fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::engine::PhysicsDirectSpaceState2D > > {
            unimplemented !()
        }
        fn space_set_debug_contacts(&mut self, space: Rid, max_contacts: i32,) {
            unimplemented !()
        }
        fn space_get_contacts(&self, space: Rid,) -> PackedVector2Array {
            unimplemented !()
        }
        fn space_get_contact_count(&self, space: Rid,) -> i32 {
            unimplemented !()
        }
        fn area_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn area_set_space(&mut self, area: Rid, space: Rid,) {
            unimplemented !()
        }
        fn area_get_space(&self, area: Rid,) -> Rid {
            unimplemented !()
        }
        fn area_add_shape(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            unimplemented !()
        }
        fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,) {
            unimplemented !()
        }
        fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,) {
            unimplemented !()
        }
        fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            unimplemented !()
        }
        fn area_get_shape_count(&self, area: Rid,) -> i32 {
            unimplemented !()
        }
        fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            unimplemented !()
        }
        fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D {
            unimplemented !()
        }
        fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            unimplemented !()
        }
        fn area_clear_shapes(&mut self, area: Rid,) {
            unimplemented !()
        }
        fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            unimplemented !()
        }
        fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            unimplemented !()
        }
        fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,) {
            unimplemented !()
        }
        fn area_get_canvas_instance_id(&self, area: Rid,) -> u64 {
            unimplemented !()
        }
        fn area_set_param(&mut self, area: Rid, param: crate::engine::physics_server_2d::AreaParameter, value: Variant,) {
            unimplemented !()
        }
        fn area_set_transform(&mut self, area: Rid, transform: Transform2D,) {
            unimplemented !()
        }
        fn area_get_param(&self, area: Rid, param: crate::engine::physics_server_2d::AreaParameter,) -> Variant {
            unimplemented !()
        }
        fn area_get_transform(&self, area: Rid,) -> Transform2D {
            unimplemented !()
        }
        fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            unimplemented !()
        }
        fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            unimplemented !()
        }
        fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            unimplemented !()
        }
        fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            unimplemented !()
        }
        fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            unimplemented !()
        }
        fn area_set_pickable(&mut self, area: Rid, pickable: bool,) {
            unimplemented !()
        }
        fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            unimplemented !()
        }
        fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable,) {
            unimplemented !()
        }
        fn body_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn body_set_space(&mut self, body: Rid, space: Rid,) {
            unimplemented !()
        }
        fn body_get_space(&self, body: Rid,) -> Rid {
            unimplemented !()
        }
        fn body_set_mode(&mut self, body: Rid, mode: crate::engine::physics_server_2d::BodyMode,) {
            unimplemented !()
        }
        fn body_get_mode(&self, body: Rid,) -> crate::engine::physics_server_2d::BodyMode {
            unimplemented !()
        }
        fn body_add_shape(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            unimplemented !()
        }
        fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,) {
            unimplemented !()
        }
        fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,) {
            unimplemented !()
        }
        fn body_get_shape_count(&self, body: Rid,) -> i32 {
            unimplemented !()
        }
        fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            unimplemented !()
        }
        fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D {
            unimplemented !()
        }
        fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            unimplemented !()
        }
        fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,) {
            unimplemented !()
        }
        fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            unimplemented !()
        }
        fn body_clear_shapes(&mut self, body: Rid,) {
            unimplemented !()
        }
        fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            unimplemented !()
        }
        fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            unimplemented !()
        }
        fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,) {
            unimplemented !()
        }
        fn body_get_canvas_instance_id(&self, body: Rid,) -> u64 {
            unimplemented !()
        }
        fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::engine::physics_server_2d::CCDMode,) {
            unimplemented !()
        }
        fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::engine::physics_server_2d::CCDMode {
            unimplemented !()
        }
        fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            unimplemented !()
        }
        fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            unimplemented !()
        }
        fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            unimplemented !()
        }
        fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            unimplemented !()
        }
        fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            unimplemented !()
        }
        fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            unimplemented !()
        }
        fn body_set_param(&mut self, body: Rid, param: crate::engine::physics_server_2d::BodyParameter, value: Variant,) {
            unimplemented !()
        }
        fn body_get_param(&self, body: Rid, param: crate::engine::physics_server_2d::BodyParameter,) -> Variant {
            unimplemented !()
        }
        fn body_reset_mass_properties(&mut self, body: Rid,) {
            unimplemented !()
        }
        fn body_set_state(&mut self, body: Rid, state: crate::engine::physics_server_2d::BodyState, value: Variant,) {
            unimplemented !()
        }
        fn body_get_state(&self, body: Rid, state: crate::engine::physics_server_2d::BodyState,) -> Variant {
            unimplemented !()
        }
        fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,) {
            unimplemented !()
        }
        fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,) {
            unimplemented !()
        }
        fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn body_apply_central_force(&mut self, body: Rid, force: Vector2,) {
            unimplemented !()
        }
        fn body_apply_force(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn body_apply_torque(&mut self, body: Rid, torque: f32,) {
            unimplemented !()
        }
        fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,) {
            unimplemented !()
        }
        fn body_add_constant_force(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn body_add_constant_torque(&mut self, body: Rid, torque: f32,) {
            unimplemented !()
        }
        fn body_set_constant_force(&mut self, body: Rid, force: Vector2,) {
            unimplemented !()
        }
        fn body_get_constant_force(&self, body: Rid,) -> Vector2 {
            unimplemented !()
        }
        fn body_set_constant_torque(&mut self, body: Rid, torque: f32,) {
            unimplemented !()
        }
        fn body_get_constant_torque(&self, body: Rid,) -> f32 {
            unimplemented !()
        }
        fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,) {
            unimplemented !()
        }
        fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            unimplemented !()
        }
        fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            unimplemented !()
        }
        fn body_get_collision_exceptions(&self, body: Rid,) -> Array < Rid > {
            unimplemented !()
        }
        fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            unimplemented !()
        }
        fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            unimplemented !()
        }
        fn body_set_contacts_reported_depth_threshold(&mut self, body: Rid, threshold: f32,) {
            unimplemented !()
        }
        fn body_get_contacts_reported_depth_threshold(&self, body: Rid,) -> f32 {
            unimplemented !()
        }
        fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            unimplemented !()
        }
        fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            unimplemented !()
        }
        fn body_set_state_sync_callback(&mut self, body: Rid, callable: Callable,) {
            unimplemented !()
        }
        fn body_set_force_integration_callback(&mut self, body: Rid, callable: Callable, userdata: Variant,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn body_collide_shape(&mut self, body: Rid, body_shape: i32, shape: Rid, shape_xform: Transform2D, motion: Vector2, results: * mut c_void, result_max: i32, result_count: * mut i32,) -> bool {
            unimplemented !()
        }
        fn body_set_pickable(&mut self, body: Rid, pickable: bool,) {
            unimplemented !()
        }
        fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::engine::PhysicsDirectBodyState2D > > {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn body_test_motion(&self, body: Rid, from: Transform2D, motion: Vector2, margin: f32, collide_separation_ray: bool, recovery_as_collision: bool, result: * mut PhysicsServer2DExtensionMotionResult,) -> bool {
            unimplemented !()
        }
        fn joint_create(&mut self,) -> Rid {
            unimplemented !()
        }
        fn joint_clear(&mut self, joint: Rid,) {
            unimplemented !()
        }
        fn joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::JointParam, value: f32,) {
            unimplemented !()
        }
        fn joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::JointParam,) -> f32 {
            unimplemented !()
        }
        fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            unimplemented !()
        }
        fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            unimplemented !()
        }
        fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,) {
            unimplemented !()
        }
        fn joint_make_groove(&mut self, joint: Rid, a_groove1: Vector2, a_groove2: Vector2, b_anchor: Vector2, body_a: Rid, body_b: Rid,) {
            unimplemented !()
        }
        fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            unimplemented !()
        }
        fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::engine::physics_server_2d::PinJointFlag, enabled: bool,) {
            unimplemented !()
        }
        fn pin_joint_get_flag(&self, joint: Rid, flag: crate::engine::physics_server_2d::PinJointFlag,) -> bool {
            unimplemented !()
        }
        fn pin_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::PinJointParam, value: f32,) {
            unimplemented !()
        }
        fn pin_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::PinJointParam,) -> f32 {
            unimplemented !()
        }
        fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::engine::physics_server_2d::DampedSpringParam, value: f32,) {
            unimplemented !()
        }
        fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::engine::physics_server_2d::DampedSpringParam,) -> f32 {
            unimplemented !()
        }
        fn joint_get_type(&self, joint: Rid,) -> crate::engine::physics_server_2d::JointType {
            unimplemented !()
        }
        fn free_rid(&mut self, rid: Rid,) {
            unimplemented !()
        }
        fn set_active(&mut self, active: bool,) {
            unimplemented !()
        }
        fn init_ext(&mut self,) {
            unimplemented !()
        }
        fn step(&mut self, step: f32,) {
            unimplemented !()
        }
        fn sync(&mut self,) {
            unimplemented !()
        }
        fn flush_queries(&mut self,) {
            unimplemented !()
        }
        fn end_sync(&mut self,) {
            unimplemented !()
        }
        fn finish(&mut self,) {
            unimplemented !()
        }
        fn is_flushing_queries(&self,) -> bool {
            unimplemented !()
        }
        fn get_process_info(&mut self, process_info: crate::engine::physics_server_2d::ProcessInfo,) -> i32 {
            unimplemented !()
        }
    }
    impl PhysicsServer2DExtension {
        pub fn body_test_motion_is_excluding_body(&self, body: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_test_motion_is_excluding_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_test_motion_is_excluding_object(&self, object: u64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, u64);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "body_test_motion_is_excluding_object", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2DExtension {
        type Base = crate::engine::PhysicsServer2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsServer2DExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsServer2DExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PhysicsServer2D > for PhysicsServer2DExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsServer2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsServer2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsServer2DExtension {
        type Target = crate::engine::PhysicsServer2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsServer2DExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsServer2DExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsServer2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}