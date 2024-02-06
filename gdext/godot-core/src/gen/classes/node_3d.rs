#![doc = "Sidecar module for class [`Node3D`][crate::engine::Node3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node3D` enums](https://docs.godotengine.org/en/stable/classes/class_node3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node3D.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`node_3d`][crate::engine::node_3d]: sidecar module with related enum/flag types\n* [`INode3D`][crate::engine::INode3D]: virtual methods\n* [`Node3DNotification`][crate::engine::notify::Node3DNotification]: notification type\n\n\nSee also [Godot docs for `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node3D`][crate::engine::Node3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node3D` methods](https://docs.godotengine.org/en/stable/classes/class_node3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Node3D`][crate::engine::Node3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum Node3DNotification {
        TransformChanged = 2000i32, EnterWorld = 41i32, ExitWorld = 42i32, VisibilityChangedOrNodeRecacheRequested = 43i32, LocalTransformChanged = 44i32, EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for Node3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TransformChanged, 41i32 => Self::EnterWorld, 42i32 => Self::ExitWorld, 43i32 => Self::VisibilityChangedOrNodeRecacheRequested, 44i32 => Self::LocalTransformChanged, 10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < Node3DNotification > for i32 {
        fn from(notification: Node3DNotification) -> i32 {
            match notification {
                Node3DNotification::TransformChanged => 2000i32, Node3DNotification::EnterWorld => 41i32, Node3DNotification::ExitWorld => 42i32, Node3DNotification::VisibilityChangedOrNodeRecacheRequested => 43i32, Node3DNotification::LocalTransformChanged => 44i32, Node3DNotification::EnterTree => 10i32, Node3DNotification::ExitTree => 11i32, Node3DNotification::MovedInParent => 12i32, Node3DNotification::Ready => 13i32, Node3DNotification::Paused => 14i32, Node3DNotification::Unpaused => 15i32, Node3DNotification::PhysicsProcess => 16i32, Node3DNotification::Process => 17i32, Node3DNotification::Parented => 18i32, Node3DNotification::Unparented => 19i32, Node3DNotification::SceneInstantiated => 20i32, Node3DNotification::DragBegin => 21i32, Node3DNotification::DragEnd => 22i32, Node3DNotification::PathRenamed => 23i32, Node3DNotification::ChildOrderChanged => 24i32, Node3DNotification::InternalProcess => 25i32, Node3DNotification::InternalPhysicsProcess => 26i32, Node3DNotification::PostEnterTree => 27i32, Node3DNotification::Disabled => 28i32, Node3DNotification::Enabled => 29i32, Node3DNotification::EditorPreSave => 9001i32, Node3DNotification::EditorPostSave => 9002i32, Node3DNotification::WmMouseEnter => 1002i32, Node3DNotification::WmMouseExit => 1003i32, Node3DNotification::WmWindowFocusIn => 1004i32, Node3DNotification::WmWindowFocusOut => 1005i32, Node3DNotification::WmCloseRequest => 1006i32, Node3DNotification::WmGoBackRequest => 1007i32, Node3DNotification::WmSizeChanged => 1008i32, Node3DNotification::WmDpiChange => 1009i32, Node3DNotification::VpMouseEnter => 1010i32, Node3DNotification::VpMouseExit => 1011i32, Node3DNotification::OsMemoryWarning => 2009i32, Node3DNotification::TranslationChanged => 2010i32, Node3DNotification::WmAbout => 2011i32, Node3DNotification::Crash => 2012i32, Node3DNotification::OsImeUpdate => 2013i32, Node3DNotification::ApplicationResumed => 2014i32, Node3DNotification::ApplicationPaused => 2015i32, Node3DNotification::ApplicationFocusIn => 2016i32, Node3DNotification::ApplicationFocusOut => 2017i32, Node3DNotification::TextServerChanged => 2018i32, Node3DNotification::Postinitialize => 0i32, Node3DNotification::Predelete => 1i32, Node3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Node3D {
        pub fn set_transform(&mut self, local: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform3D);
            let args = (local,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation(&mut self, euler_radians: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_order(&mut self, order: crate::engine::global::EulerOrder,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::EulerOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_order(&self,) -> crate::engine::global::EulerOrder {
            type RetMarshal = PtrcallReturnT < crate::engine::global::EulerOrder >;
            type CallSig = (crate::engine::global::EulerOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_edit_mode(&mut self, edit_mode: crate::engine::node_3d::RotationEditMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::node_3d::RotationEditMode);
            let args = (edit_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation_edit_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_edit_mode(&self,) -> crate::engine::node_3d::RotationEditMode {
            type RetMarshal = PtrcallReturnT < crate::engine::node_3d::RotationEditMode >;
            type CallSig = (crate::engine::node_3d::RotationEditMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation_edit_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_quaternion(&mut self, quaternion: Quaternion,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Quaternion);
            let args = (quaternion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_quaternion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_quaternion(&self,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_quaternion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_basis(&mut self, basis: Basis,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_basis(&self,) -> Basis {
            type RetMarshal = PtrcallReturnT < Basis >;
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_transform(&mut self, global: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform3D);
            let args = (global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_transform(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_basis(&mut self, basis: Basis,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_basis(&self,) -> Basis {
            type RetMarshal = PtrcallReturnT < Basis >;
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation(&mut self, euler_radians: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_node_3d(&self,) -> Option < Gd < crate::engine::Node3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node3D > >;
            type CallSig = (Option < Gd < crate::engine::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent_node_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_transform_notification(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ignore_transform_notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_scale(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scale_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_scale_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::engine::World3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World3D > >;
            type CallSig = (Option < Gd < crate::engine::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_transform(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_parent(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_parent(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_gizmos(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_gizmo(&mut self, gizmo: Gd < crate::engine::Node3DGizmo >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node3DGizmo >);
            let args = (gizmo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_gizmo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gizmos(&self,) -> Array < Gd < crate::engine::Node3DGizmo > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node3DGizmo > > >;
            type CallSig = (Array < Gd < crate::engine::Node3DGizmo > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_gizmos(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subgizmo_selection(&mut self, gizmo: Gd < crate::engine::Node3DGizmo >, id: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node3DGizmo >, i32, Transform3D);
            let args = (gizmo, id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subgizmo_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_subgizmo_selection(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_subgizmo_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_notify_local_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_local_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_notify_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate(&mut self, axis: Vector3, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_rotate(&mut self, axis: Vector3, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_scale(&mut self, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_translate(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_object_local(&mut self, axis: Vector3, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scale_object_local(&mut self, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scale_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn translate_object_local(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "translate_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_x(&mut self, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_y(&mut self, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_z(&mut self, angle: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn translate(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn orthonormalize(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "orthonormalize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_identity(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_identity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn look_at_full(&mut self, target: Vector3, up: Vector3, use_model_front: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, Vector3, bool);
            let args = (target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "look_at", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn look_at(&mut self, target: Vector3,) {
            self.look_at_ex(target,) . done()
        }
        #[inline]
        pub fn look_at_ex(&mut self, target: Vector3,) -> ExLookAt < '_ > {
            ExLookAt::new(self, target,)
        }
        pub(crate) fn look_at_from_position_full(&mut self, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3, Vector3, Vector3, bool);
            let args = (position, target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "look_at_from_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn look_at_from_position(&mut self, position: Vector3, target: Vector3,) {
            self.look_at_from_position_ex(position, target,) . done()
        }
        #[inline]
        pub fn look_at_from_position_ex(&mut self, position: Vector3, target: Vector3,) -> ExLookAtFromPosition < '_ > {
            ExLookAtFromPosition::new(self, position, target,)
        }
        pub fn to_local(&self, global_point: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_global(&self, local_point: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_global", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_ENTER_WORLD: i32 = 41i32;
        pub(crate) const NOTIFICATION_EXIT_WORLD: i32 = 42i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 43i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 44i32;
        
    }
    impl crate::obj::GodotClass for Node3D {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Node3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Node3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for Node3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Node3D {
        
    }
    impl crate::obj::ExportableObject for Node3D {
        
    }
    impl crate::obj::cap::GodotDefault for Node3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node3D {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Node3D {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`Node3D::look_at_ex`][super::Node3D::look_at_ex]."]
#[must_use]
pub struct ExLookAt < 'a > {
    surround_object: &'a mut re_export::Node3D, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAt < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, target: Vector3,) -> Self {
        Self {
            surround_object, target, up: Vector3::new(0 as _, 1 as _, 0 as _), use_model_front: false,
        }
    }
    #[inline]
    pub fn up(self, value: Vector3) -> Self {
        Self {
            up: value, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, value: bool) -> Self {
        Self {
            use_model_front: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node3D::look_at_full(self.surround_object, self.target, self.up, self.use_model_front,)
    }
}
#[doc = "Default-param extender for [`Node3D::look_at_from_position_ex`][super::Node3D::look_at_from_position_ex]."]
#[must_use]
pub struct ExLookAtFromPosition < 'a > {
    surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAtFromPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3,) -> Self {
        Self {
            surround_object, position, target, up: Vector3::new(0 as _, 1 as _, 0 as _), use_model_front: false,
        }
    }
    #[inline]
    pub fn up(self, value: Vector3) -> Self {
        Self {
            up: value, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, value: bool) -> Self {
        Self {
            use_model_front: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node3D::look_at_from_position_full(self.surround_object, self.position, self.target, self.up, self.use_model_front,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RotationEditMode {
    ord: i32
}
impl RotationEditMode {
    pub const ROTATION_EDIT_MODE_EULER: Self = Self {
        ord: 0i32
    };
    pub const ROTATION_EDIT_MODE_QUATERNION: Self = Self {
        ord: 1i32
    };
    pub const ROTATION_EDIT_MODE_BASIS: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for RotationEditMode {
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
impl crate::builtin::meta::GodotConvert for RotationEditMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RotationEditMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RotationEditMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}