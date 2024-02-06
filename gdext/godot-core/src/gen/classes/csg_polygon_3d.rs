#![doc = "Sidecar module for class [`CsgPolygon3D`][crate::engine::CsgPolygon3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGPolygon3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGPolygon3D.`\n\nInherits [`CsgPrimitive3D`][crate::engine::CsgPrimitive3D].\n\nRelated symbols:\n\n* [`csg_polygon_3d`][crate::engine::csg_polygon_3d]: sidecar module with related enum/flag types\n* [`ICsgPolygon3D`][crate::engine::ICsgPolygon3D]: virtual methods\n\n\nSee also [Godot docs for `CSGPolygon3D`](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgPolygon3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CsgPolygon3D`][crate::engine::CsgPolygon3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CSGPolygon3D` methods](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICsgPolygon3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
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
    impl CsgPolygon3D {
        pub fn set_polygon(&mut self, polygon: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_polygon(&self,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::engine::csg_polygon_3d::Mode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::csg_polygon_3d::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::engine::csg_polygon_3d::Mode {
            type RetMarshal = PtrcallReturnT < crate::engine::csg_polygon_3d::Mode >;
            type CallSig = (crate::engine::csg_polygon_3d::Mode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spin_degrees(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_spin_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spin_degrees(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spin_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spin_sides(&mut self, spin_sides: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (spin_sides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_spin_sides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spin_sides(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spin_sides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_node(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_node(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_interval_type(&mut self, interval_type: crate::engine::csg_polygon_3d::PathIntervalType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::csg_polygon_3d::PathIntervalType);
            let args = (interval_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_interval_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_interval_type(&self,) -> crate::engine::csg_polygon_3d::PathIntervalType {
            type RetMarshal = PtrcallReturnT < crate::engine::csg_polygon_3d::PathIntervalType >;
            type CallSig = (crate::engine::csg_polygon_3d::PathIntervalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_interval_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_interval(&mut self, interval: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_interval(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_simplify_angle(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_simplify_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_simplify_angle(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_simplify_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_rotation(&mut self, path_rotation: crate::engine::csg_polygon_3d::PathRotation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::csg_polygon_3d::PathRotation);
            let args = (path_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_rotation(&self,) -> crate::engine::csg_polygon_3d::PathRotation {
            type RetMarshal = PtrcallReturnT < crate::engine::csg_polygon_3d::PathRotation >;
            type CallSig = (crate::engine::csg_polygon_3d::PathRotation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_local(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_local(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_path_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_continuous_u(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_continuous_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_continuous_u(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_path_continuous_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_u_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_u_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_u_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_u_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_joined(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_path_joined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_joined(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_path_joined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_smooth_faces(&mut self, smooth_faces: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (smooth_faces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_smooth_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_smooth_faces(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_smooth_faces", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CsgPolygon3D {
        type Base = crate::engine::CsgPrimitive3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CSGPolygon3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgPolygon3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CsgPolygon3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CsgPrimitive3D > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::CsgShape3D > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for CsgPolygon3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CsgPolygon3D {
        
    }
    impl crate::obj::ExportableObject for CsgPolygon3D {
        
    }
    impl crate::obj::cap::GodotDefault for CsgPolygon3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CsgPolygon3D {
        type Target = crate::engine::CsgPrimitive3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgPolygon3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CsgPolygon3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CsgPolygon3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CsgPrimitive3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CsgShape3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::GeometryInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mode {
    ord: i32
}
impl Mode {
    pub const MODE_DEPTH: Self = Self {
        ord: 0i32
    };
    pub const MODE_SPIN: Self = Self {
        ord: 1i32
    };
    pub const MODE_PATH: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Mode {
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
impl crate::builtin::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Mode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PathRotation {
    ord: i32
}
impl PathRotation {
    pub const PATH_ROTATION_POLYGON: Self = Self {
        ord: 0i32
    };
    pub const PATH_ROTATION_PATH: Self = Self {
        ord: 1i32
    };
    pub const PATH_ROTATION_PATH_FOLLOW: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PathRotation {
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
impl crate::builtin::meta::GodotConvert for PathRotation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PathRotation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PathRotation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PathIntervalType {
    ord: i32
}
impl PathIntervalType {
    pub const PATH_INTERVAL_DISTANCE: Self = Self {
        ord: 0i32
    };
    pub const PATH_INTERVAL_SUBDIVIDE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for PathIntervalType {
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
impl crate::builtin::meta::GodotConvert for PathIntervalType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PathIntervalType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PathIntervalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}