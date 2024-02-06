#![doc = "Sidecar module for class [`SkeletonIk3d`][crate::engine::SkeletonIk3d].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonIK3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonIK3D.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`skeleton_ik_3d`][crate::engine::skeleton_ik_3d]: sidecar module with related enum/flag types\n* [`ISkeletonIk3d`][crate::engine::ISkeletonIk3d]: virtual methods\n\n\nSee also [Godot docs for `SkeletonIK3D`](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonIk3d {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SkeletonIk3d`][crate::engine::SkeletonIk3d].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SkeletonIK3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonIk3d: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl SkeletonIk3d {
        pub fn set_root_bone(&mut self, root_bone: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (root_bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_bone(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tip_bone(&mut self, tip_bone: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (tip_bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tip_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tip_bone(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tip_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolation(&mut self, interpolation: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interpolation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interpolation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_transform(&mut self, target: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform3D);
            let args = (target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_target_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_transform(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_target_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_node(&mut self, node: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_target_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_node(&mut self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_target_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_override_tip_basis(&mut self, override_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (override_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_override_tip_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_override_tip_basis(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_override_tip_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_magnet(&mut self, use_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_magnet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_magnet(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_magnet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_magnet_position(&mut self, local_position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_magnet_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_magnet_position(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_magnet_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_skeleton(&self,) -> Option < Gd < crate::engine::Skeleton3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Skeleton3D > >;
            type CallSig = (Option < Gd < crate::engine::Skeleton3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_distance(&mut self, min_distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (min_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_iterations(&mut self, iterations: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (iterations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_iterations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_iterations(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_iterations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn start_full(&mut self, one_time: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (one_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn start(&mut self,) {
            self.start_ex() . done()
        }
        #[inline]
        pub fn start_ex(&mut self,) -> ExStart < '_ > {
            ExStart::new(self,)
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SkeletonIk3d {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SkeletonIK3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonIk3d {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SkeletonIk3d {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for SkeletonIk3d {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SkeletonIk3d {
        
    }
    impl crate::obj::ExportableObject for SkeletonIk3d {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonIk3d {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonIk3d {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonIk3d {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SkeletonIk3d {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SkeletonIk3d > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SkeletonIk3d::start_ex`][super::SkeletonIk3d::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    surround_object: &'a mut re_export::SkeletonIk3d, one_time: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::SkeletonIk3d,) -> Self {
        Self {
            surround_object, one_time: false,
        }
    }
    #[inline]
    pub fn one_time(self, value: bool) -> Self {
        Self {
            one_time: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SkeletonIk3d::start_full(self.surround_object, self.one_time,)
    }
}