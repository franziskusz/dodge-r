#![doc = "Sidecar module for class [`Skeleton3D`][crate::engine::Skeleton3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Skeleton3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Skeleton3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`skeleton_3d`][crate::engine::skeleton_3d]: sidecar module with related enum/flag types\n* [`ISkeleton3D`][crate::engine::ISkeleton3D]: virtual methods\n* [`Skeleton3DNotification`][crate::engine::notify::Skeleton3DNotification]: notification type\n\n\nSee also [Godot docs for `Skeleton3D`](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Skeleton3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Skeleton3D`][crate::engine::Skeleton3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Skeleton3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeleton3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Skeleton3DNotification) {
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
    #[doc = "Notification type for class [`Skeleton3D`][crate::engine::Skeleton3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum Skeleton3DNotification {
        UpdateSkeleton = 50i32, TransformChanged = 2000i32, EnterWorld = 41i32, ExitWorld = 42i32, VisibilityChangedOrNodeRecacheRequested = 43i32, LocalTransformChanged = 44i32, EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for Skeleton3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                50i32 => Self::UpdateSkeleton, 2000i32 => Self::TransformChanged, 41i32 => Self::EnterWorld, 42i32 => Self::ExitWorld, 43i32 => Self::VisibilityChangedOrNodeRecacheRequested, 44i32 => Self::LocalTransformChanged, 10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < Skeleton3DNotification > for i32 {
        fn from(notification: Skeleton3DNotification) -> i32 {
            match notification {
                Skeleton3DNotification::UpdateSkeleton => 50i32, Skeleton3DNotification::TransformChanged => 2000i32, Skeleton3DNotification::EnterWorld => 41i32, Skeleton3DNotification::ExitWorld => 42i32, Skeleton3DNotification::VisibilityChangedOrNodeRecacheRequested => 43i32, Skeleton3DNotification::LocalTransformChanged => 44i32, Skeleton3DNotification::EnterTree => 10i32, Skeleton3DNotification::ExitTree => 11i32, Skeleton3DNotification::MovedInParent => 12i32, Skeleton3DNotification::Ready => 13i32, Skeleton3DNotification::Paused => 14i32, Skeleton3DNotification::Unpaused => 15i32, Skeleton3DNotification::PhysicsProcess => 16i32, Skeleton3DNotification::Process => 17i32, Skeleton3DNotification::Parented => 18i32, Skeleton3DNotification::Unparented => 19i32, Skeleton3DNotification::SceneInstantiated => 20i32, Skeleton3DNotification::DragBegin => 21i32, Skeleton3DNotification::DragEnd => 22i32, Skeleton3DNotification::PathRenamed => 23i32, Skeleton3DNotification::ChildOrderChanged => 24i32, Skeleton3DNotification::InternalProcess => 25i32, Skeleton3DNotification::InternalPhysicsProcess => 26i32, Skeleton3DNotification::PostEnterTree => 27i32, Skeleton3DNotification::Disabled => 28i32, Skeleton3DNotification::Enabled => 29i32, Skeleton3DNotification::EditorPreSave => 9001i32, Skeleton3DNotification::EditorPostSave => 9002i32, Skeleton3DNotification::WmMouseEnter => 1002i32, Skeleton3DNotification::WmMouseExit => 1003i32, Skeleton3DNotification::WmWindowFocusIn => 1004i32, Skeleton3DNotification::WmWindowFocusOut => 1005i32, Skeleton3DNotification::WmCloseRequest => 1006i32, Skeleton3DNotification::WmGoBackRequest => 1007i32, Skeleton3DNotification::WmSizeChanged => 1008i32, Skeleton3DNotification::WmDpiChange => 1009i32, Skeleton3DNotification::VpMouseEnter => 1010i32, Skeleton3DNotification::VpMouseExit => 1011i32, Skeleton3DNotification::OsMemoryWarning => 2009i32, Skeleton3DNotification::TranslationChanged => 2010i32, Skeleton3DNotification::WmAbout => 2011i32, Skeleton3DNotification::Crash => 2012i32, Skeleton3DNotification::OsImeUpdate => 2013i32, Skeleton3DNotification::ApplicationResumed => 2014i32, Skeleton3DNotification::ApplicationPaused => 2015i32, Skeleton3DNotification::ApplicationFocusIn => 2016i32, Skeleton3DNotification::ApplicationFocusOut => 2017i32, Skeleton3DNotification::TextServerChanged => 2018i32, Skeleton3DNotification::Postinitialize => 0i32, Skeleton3DNotification::Predelete => 1i32, Skeleton3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Skeleton3D {
        pub fn add_bone(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_bone(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_name(&self, bone_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (bone_idx, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i32, parent_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (bone_idx, parent_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unparent_bone_and_rest(&mut self, bone_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unparent_bone_and_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_children(&self, bone_idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parentless_bones(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parentless_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_rest(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_rest(&mut self, bone_idx: i32, rest: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, rest,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_global_rest(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_global_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_skin_from_rest_transforms(&mut self,) -> Option < Gd < crate::engine::Skin > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Skin > >;
            type CallSig = (Option < Gd < crate::engine::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_skin_from_rest_transforms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_skin(&mut self, skin: Gd < crate::engine::Skin >,) -> Option < Gd < crate::engine::SkinReference > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SkinReference > >;
            type CallSig = (Option < Gd < crate::engine::SkinReference > >, Gd < crate::engine::Skin >);
            let args = (skin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn localize_rests(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "localize_rests", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_bones(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_position(&mut self, bone_idx: i32, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (bone_idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_pose_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_rotation(&mut self, bone_idx: i32, rotation: Quaternion,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Quaternion);
            let args = (bone_idx, rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_pose_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_scale(&mut self, bone_idx: i32, scale: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (bone_idx, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_pose_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_position(&self, bone_idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_pose_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_rotation(&self, bone_idx: i32,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_pose_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_scale(&self, bone_idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_pose_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_bone_pose(&mut self, bone_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reset_bone_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_bone_poses(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reset_bone_poses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bone_enabled(&self, bone_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bone_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_bone_enabled_full(&mut self, bone_idx: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (bone_idx, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_bone_enabled(&mut self, bone_idx: i32,) {
            self.set_bone_enabled_ex(bone_idx,) . done()
        }
        #[inline]
        pub fn set_bone_enabled_ex(&mut self, bone_idx: i32,) -> ExSetBoneEnabled < '_ > {
            ExSetBoneEnabled::new(self, bone_idx,)
        }
        pub fn clear_bones_global_pose_override(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_bones_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_bone_global_pose_override_full(&mut self, bone_idx: i32, pose: Transform3D, amount: f32, persistent: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D, f32, bool);
            let args = (bone_idx, pose, amount, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_bone_global_pose_override(&mut self, bone_idx: i32, pose: Transform3D, amount: f32,) {
            self.set_bone_global_pose_override_ex(bone_idx, pose, amount,) . done()
        }
        #[inline]
        pub fn set_bone_global_pose_override_ex(&mut self, bone_idx: i32, pose: Transform3D, amount: f32,) -> ExSetBoneGlobalPoseOverride < '_ > {
            ExSetBoneGlobalPoseOverride::new(self, bone_idx, pose, amount,)
        }
        pub fn get_bone_global_pose_override(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_global_pose(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_global_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_global_pose_no_override(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_global_pose_no_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_all_bone_transforms(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update_all_bone_transforms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_bone_child_transform(&mut self, bone_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update_bone_child_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_scale(&mut self, motion_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (motion_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_show_rest_only(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_show_rest_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_show_rest_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_show_rest_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animate_physical_bones(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_animate_physical_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animate_physical_bones(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animate_physical_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn physical_bones_stop_simulation(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "physical_bones_stop_simulation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn physical_bones_start_simulation_full(&mut self, bones: Array < StringName >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < StringName >);
            let args = (bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "physical_bones_start_simulation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn physical_bones_start_simulation(&mut self,) {
            self.physical_bones_start_simulation_ex() . done()
        }
        #[inline]
        pub fn physical_bones_start_simulation_ex(&mut self,) -> ExPhysicalBonesStartSimulation < '_ > {
            ExPhysicalBonesStartSimulation::new(self,)
        }
        pub fn physical_bones_add_collision_exception(&mut self, exception: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (exception,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "physical_bones_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn physical_bones_remove_collision_exception(&mut self, exception: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (exception,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "physical_bones_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
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
        pub fn notify(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_UPDATE_SKELETON: i32 = 50i32;
        
    }
    impl crate::obj::GodotClass for Skeleton3D {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Skeleton3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Skeleton3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Skeleton3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for Skeleton3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Skeleton3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Skeleton3D {
        
    }
    impl crate::obj::ExportableObject for Skeleton3D {
        
    }
    impl crate::obj::cap::GodotDefault for Skeleton3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Skeleton3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Skeleton3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Skeleton3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Skeleton3D > for $Class {
                
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
#[doc = "Default-param extender for [`Skeleton3D::set_bone_enabled_ex`][super::Skeleton3D::set_bone_enabled_ex]."]
#[must_use]
pub struct ExSetBoneEnabled < 'a > {
    surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, enabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBoneEnabled < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32,) -> Self {
        Self {
            surround_object, bone_idx, enabled: true,
        }
    }
    #[inline]
    pub fn enabled(self, value: bool) -> Self {
        Self {
            enabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Skeleton3D::set_bone_enabled_full(self.surround_object, self.bone_idx, self.enabled,)
    }
}
#[doc = "Default-param extender for [`Skeleton3D::set_bone_global_pose_override_ex`][super::Skeleton3D::set_bone_global_pose_override_ex]."]
#[must_use]
pub struct ExSetBoneGlobalPoseOverride < 'a > {
    surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, pose: Transform3D, amount: f32, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBoneGlobalPoseOverride < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, pose: Transform3D, amount: f32,) -> Self {
        Self {
            surround_object, bone_idx, pose, amount, persistent: false,
        }
    }
    #[inline]
    pub fn persistent(self, value: bool) -> Self {
        Self {
            persistent: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Skeleton3D::set_bone_global_pose_override_full(self.surround_object, self.bone_idx, self.pose, self.amount, self.persistent,)
    }
}
#[doc = "Default-param extender for [`Skeleton3D::physical_bones_start_simulation_ex`][super::Skeleton3D::physical_bones_start_simulation_ex]."]
#[must_use]
pub struct ExPhysicalBonesStartSimulation < 'a > {
    surround_object: &'a mut re_export::Skeleton3D, bones: Array < StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPhysicalBonesStartSimulation < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D,) -> Self {
        Self {
            surround_object, bones: Array::new(),
        }
    }
    #[inline]
    pub fn bones(self, value: Array < StringName >) -> Self {
        Self {
            bones: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Skeleton3D::physical_bones_start_simulation_full(self.surround_object, self.bones,)
    }
}