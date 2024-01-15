#![doc = "Sidecar module for class [`Node`][crate::engine::Node].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node` enums](https://docs.godotengine.org/en/stable/classes/class_node.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`node`][crate::engine::node]: sidecar module with related enum/flag types\n* [`INode`][crate::engine::INode]: virtual methods\n* [`NodeNotification`][crate::engine::notify::NodeNotification]: notification type\n\n\nSee also [Godot docs for `Node`](https://docs.godotengine.org/en/stable/classes/class_node.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node`][crate::engine::Node].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node` methods](https://docs.godotengine.org/en/stable/classes/class_node.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Node`][crate::engine::Node]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum NodeNotification {
        EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for NodeNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < NodeNotification > for i32 {
        fn from(notification: NodeNotification) -> i32 {
            match notification {
                NodeNotification::EnterTree => 10i32, NodeNotification::ExitTree => 11i32, NodeNotification::MovedInParent => 12i32, NodeNotification::Ready => 13i32, NodeNotification::Paused => 14i32, NodeNotification::Unpaused => 15i32, NodeNotification::PhysicsProcess => 16i32, NodeNotification::Process => 17i32, NodeNotification::Parented => 18i32, NodeNotification::Unparented => 19i32, NodeNotification::SceneInstantiated => 20i32, NodeNotification::DragBegin => 21i32, NodeNotification::DragEnd => 22i32, NodeNotification::PathRenamed => 23i32, NodeNotification::ChildOrderChanged => 24i32, NodeNotification::InternalProcess => 25i32, NodeNotification::InternalPhysicsProcess => 26i32, NodeNotification::PostEnterTree => 27i32, NodeNotification::Disabled => 28i32, NodeNotification::Enabled => 29i32, NodeNotification::EditorPreSave => 9001i32, NodeNotification::EditorPostSave => 9002i32, NodeNotification::WmMouseEnter => 1002i32, NodeNotification::WmMouseExit => 1003i32, NodeNotification::WmWindowFocusIn => 1004i32, NodeNotification::WmWindowFocusOut => 1005i32, NodeNotification::WmCloseRequest => 1006i32, NodeNotification::WmGoBackRequest => 1007i32, NodeNotification::WmSizeChanged => 1008i32, NodeNotification::WmDpiChange => 1009i32, NodeNotification::VpMouseEnter => 1010i32, NodeNotification::VpMouseExit => 1011i32, NodeNotification::OsMemoryWarning => 2009i32, NodeNotification::TranslationChanged => 2010i32, NodeNotification::WmAbout => 2011i32, NodeNotification::Crash => 2012i32, NodeNotification::OsImeUpdate => 2013i32, NodeNotification::ApplicationResumed => 2014i32, NodeNotification::ApplicationPaused => 2015i32, NodeNotification::ApplicationFocusIn => 2016i32, NodeNotification::ApplicationFocusOut => 2017i32, NodeNotification::TextServerChanged => 2018i32, NodeNotification::Postinitialize => 0i32, NodeNotification::Predelete => 1i32, NodeNotification::Unknown(int) => int,
            }
        }
    }
    impl Node {
        pub fn print_orphan_nodes() {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "print_orphan_nodes", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn add_sibling_full(&mut self, sibling: Gd < crate::engine::Node >, force_readable_name: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool);
            let args = (sibling, force_readable_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_sibling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_sibling(&mut self, sibling: Gd < crate::engine::Node >,) {
            self.add_sibling_ex(sibling,) . done()
        }
        #[inline]
        pub fn add_sibling_ex(&mut self, sibling: Gd < crate::engine::Node >,) -> ExAddSibling < '_ > {
            ExAddSibling::new(self, sibling,)
        }
        pub fn set_name(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_child_full(&mut self, node: Gd < crate::engine::Node >, force_readable_name: bool, internal: crate::engine::node::InternalMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool, crate::engine::node::InternalMode);
            let args = (node, force_readable_name, internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_child(&mut self, node: Gd < crate::engine::Node >,) {
            self.add_child_ex(node,) . done()
        }
        #[inline]
        pub fn add_child_ex(&mut self, node: Gd < crate::engine::Node >,) -> ExAddChild < '_ > {
            ExAddChild::new(self, node,)
        }
        pub fn remove_child(&mut self, node: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn reparent_full(&mut self, new_parent: Gd < crate::engine::Node >, keep_global_transform: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool);
            let args = (new_parent, keep_global_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reparent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn reparent(&mut self, new_parent: Gd < crate::engine::Node >,) {
            self.reparent_ex(new_parent,) . done()
        }
        #[inline]
        pub fn reparent_ex(&mut self, new_parent: Gd < crate::engine::Node >,) -> ExReparent < '_ > {
            ExReparent::new(self, new_parent,)
        }
        pub(crate) fn get_child_count_full(&self, include_internal: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_child_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_child_count(&self,) -> i32 {
            self.get_child_count_ex() . done()
        }
        #[inline]
        pub fn get_child_count_ex(&self,) -> ExGetChildCount < '_ > {
            ExGetChildCount::new(self,)
        }
        pub(crate) fn get_children_full(&self, include_internal: bool,) -> Array < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node > > >;
            type CallSig = (Array < Gd < crate::engine::Node > >, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_children(&self,) -> Array < Gd < crate::engine::Node > > {
            self.get_children_ex() . done()
        }
        #[inline]
        pub fn get_children_ex(&self,) -> ExGetChildren < '_ > {
            ExGetChildren::new(self,)
        }
        pub(crate) fn get_child_full(&self, idx: i32, include_internal: bool,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, i32, bool);
            let args = (idx, include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_child(&self, idx: i32,) -> Option < Gd < crate::engine::Node > > {
            self.get_child_ex(idx,) . done()
        }
        #[inline]
        pub fn get_child_ex(&self, idx: i32,) -> ExGetChild < '_ > {
            ExGetChild::new(self, idx,)
        }
        pub fn has_node(&self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node(&self, path: NodePath,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_or_null(&self, path: NodePath,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_or_null", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn find_child_full(&self, pattern: GString, recursive: bool, owned: bool,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, GString, bool, bool);
            let args = (pattern, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn find_child(&self, pattern: GString,) -> Option < Gd < crate::engine::Node > > {
            self.find_child_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_child_ex(&self, pattern: GString,) -> ExFindChild < '_ > {
            ExFindChild::new(self, pattern,)
        }
        pub(crate) fn find_children_full(&self, pattern: GString, type_: GString, recursive: bool, owned: bool,) -> Array < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node > > >;
            type CallSig = (Array < Gd < crate::engine::Node > >, GString, GString, bool, bool);
            let args = (pattern, type_, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn find_children(&self, pattern: GString,) -> Array < Gd < crate::engine::Node > > {
            self.find_children_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_children_ex(&self, pattern: GString,) -> ExFindChildren < '_ > {
            ExFindChildren::new(self, pattern,)
        }
        pub fn find_parent(&self, pattern: GString,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, GString);
            let args = (pattern,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_node_and_resource(&self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_node_and_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_and_resource(&mut self, path: NodePath,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_and_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_inside_tree(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_inside_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ancestor_of(&self, node: Gd < crate::engine::Node >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ancestor_of", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_greater_than(&self, node: Gd < crate::engine::Node >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_greater_than", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_path_to_full(&self, node: Gd < crate::engine::Node >, use_unique_path: bool,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath, Gd < crate::engine::Node >, bool);
            let args = (node, use_unique_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_path_to(&self, node: Gd < crate::engine::Node >,) -> NodePath {
            self.get_path_to_ex(node,) . done()
        }
        #[inline]
        pub fn get_path_to_ex(&self, node: Gd < crate::engine::Node >,) -> ExGetPathTo < '_ > {
            ExGetPathTo::new(self, node,)
        }
        pub(crate) fn add_to_group_full(&mut self, group: StringName, persistent: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (group, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_to_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_to_group(&mut self, group: StringName,) {
            self.add_to_group_ex(group,) . done()
        }
        #[inline]
        pub fn add_to_group_ex(&mut self, group: StringName,) -> ExAddToGroup < '_ > {
            ExAddToGroup::new(self, group,)
        }
        pub fn remove_from_group(&mut self, group: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (group,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_from_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_group(&self, group: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (group,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_child(&mut self, child_node: Gd < crate::engine::Node >, to_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, i32);
            let args = (child_node, to_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_groups(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_groups", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_owner(&mut self, owner: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (owner,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_owner(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_index_full(&self, include_internal: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_index(&self,) -> i32 {
            self.get_index_ex() . done()
        }
        #[inline]
        pub fn get_index_ex(&self,) -> ExGetIndex < '_ > {
            ExGetIndex::new(self,)
        }
        pub fn print_tree(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "print_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn print_tree_pretty(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "print_tree_pretty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree_string(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tree_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree_string_pretty(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tree_string_pretty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_file_path(&mut self, scene_file_path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (scene_file_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_file_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_file_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_file_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn propagate_notification(&mut self, what: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "propagate_notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn propagate_call_full(&mut self, method: StringName, args: VariantArray, parent_first: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, VariantArray, bool);
            let args = (method, args, parent_first,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "propagate_call", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn propagate_call(&mut self, method: StringName,) {
            self.propagate_call_ex(method,) . done()
        }
        #[inline]
        pub fn propagate_call_ex(&mut self, method: StringName,) -> ExPropagateCall < '_ > {
            ExPropagateCall::new(self, method,)
        }
        pub fn set_physics_process(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_process_delta_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_process_delta_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_processing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_physics_processing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_delta_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_delta_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_process_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_process_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_input(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_input(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_shortcut_input(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_shortcut_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_shortcut_input(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing_shortcut_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_unhandled_input(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_unhandled_input(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_unhandled_key_input(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_unhandled_key_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_unhandled_key_input(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing_unhandled_key_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::engine::node::ProcessMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::node::ProcessMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_mode(&self,) -> crate::engine::node::ProcessMode {
            type RetMarshal = PtrcallReturnT < crate::engine::node::ProcessMode >;
            type CallSig = (crate::engine::node::ProcessMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_process(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_group(&mut self, mode: crate::engine::node::ProcessThreadGroup,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::node::ProcessThreadGroup);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_group(&self,) -> crate::engine::node::ProcessThreadGroup {
            type RetMarshal = PtrcallReturnT < crate::engine::node::ProcessThreadGroup >;
            type CallSig = (crate::engine::node::ProcessThreadGroup,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_messages(&mut self, flags: crate::engine::node::ProcessThreadMessages,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::node::ProcessThreadMessages);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_thread_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_messages(&self,) -> crate::engine::node::ProcessThreadMessages {
            type RetMarshal = PtrcallReturnT < crate::engine::node::ProcessThreadMessages >;
            type CallSig = (crate::engine::node::ProcessThreadMessages,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_thread_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_group_order(&mut self, order: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_thread_group_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_group_order(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_thread_group_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_folded(&mut self, fold: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (fold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_display_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_displayed_folded(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_displayed_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_internal(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_internal(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_processing_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_process_internal(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_process_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_processing_internal(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_physics_processing_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window(&self,) -> Option < Gd < crate::engine::Window > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Window > >;
            type CallSig = (Option < Gd < crate::engine::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_window", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_exclusive_window(&self,) -> Option < Gd < crate::engine::Window > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Window > >;
            type CallSig = (Option < Gd < crate::engine::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_exclusive_window", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::engine::SceneTree > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SceneTree > >;
            type CallSig = (Option < Gd < crate::engine::SceneTree > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_tween(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_tween", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn duplicate_full(&self, flags: i32,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, i32);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "duplicate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::engine::Node > > {
            self.duplicate_ex() . done()
        }
        #[inline]
        pub fn duplicate_ex(&self,) -> ExDuplicate < '_ > {
            ExDuplicate::new(self,)
        }
        pub(crate) fn replace_by_full(&mut self, node: Gd < crate::engine::Node >, keep_groups: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool);
            let args = (node, keep_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "replace_by", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn replace_by(&mut self, node: Gd < crate::engine::Node >,) {
            self.replace_by_ex(node,) . done()
        }
        #[inline]
        pub fn replace_by_ex(&mut self, node: Gd < crate::engine::Node >,) -> ExReplaceBy < '_ > {
            ExReplaceBy::new(self, node,)
        }
        pub fn set_scene_instance_load_placeholder(&mut self, load_placeholder: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (load_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_instance_load_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_instance_load_placeholder(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_instance_load_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable_instance(&mut self, node: Gd < crate::engine::Node >, is_editable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool);
            let args = (node, is_editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_editable_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable_instance(&self, node: Gd < crate::engine::Node >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_editable_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport(&self,) -> Option < Gd < crate::engine::Viewport > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Viewport > >;
            type CallSig = (Option < Gd < crate::engine::Viewport > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue_free(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "queue_free", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_ready(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_node_ready(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_node_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_multiplayer_authority_full(&mut self, id: i32, recursive: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (id, recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_multiplayer_authority(&mut self, id: i32,) {
            self.set_multiplayer_authority_ex(id,) . done()
        }
        #[inline]
        pub fn set_multiplayer_authority_ex(&mut self, id: i32,) -> ExSetMultiplayerAuthority < '_ > {
            ExSetMultiplayerAuthority::new(self, id,)
        }
        pub fn get_multiplayer_authority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multiplayer_authority(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::engine::MultiplayerApi > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MultiplayerApi > >;
            type CallSig = (Option < Gd < crate::engine::MultiplayerApi > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_multiplayer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rpc_config(&mut self, method: StringName, config: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (method, config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rpc_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editor_description(&mut self, editor_description: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (editor_description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_editor_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_description(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_name_in_owner(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unique_name_in_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_unique_name_in_owner(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_unique_name_in_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rpc(&mut self, method: StringName, varargs: &[Variant]) -> crate::engine::global::Error {
            type CallSig = (crate::engine::global::Error, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4977usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "rpc", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn rpc_id(&mut self, peer_id: i64, method: StringName, varargs: &[Variant]) -> crate::engine::global::Error {
            type CallSig = (crate::engine::global::Error, i64, StringName);
            let args = (peer_id, method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4978usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "rpc_id", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn update_configuration_warnings(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_configuration_warnings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_deferred_thread_group(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            type CallSig = (Variant, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4980usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_deferred_thread_group", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_deferred_thread_group(&mut self, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deferred_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_deferred_thread_group(&mut self, what: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_deferred_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_thread_safe(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            type CallSig = (Variant, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4983usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_thread_safe", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_thread_safe(&mut self, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_thread_safe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_thread_safe(&mut self, what: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_thread_safe", self.object_ptr, self.__checked_id(), args,)
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
        pub fn notify(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_ENTER_TREE: i32 = 10i32;
        pub(crate) const NOTIFICATION_EXIT_TREE: i32 = 11i32;
        pub(crate) const NOTIFICATION_MOVED_IN_PARENT: i32 = 12i32;
        pub(crate) const NOTIFICATION_READY: i32 = 13i32;
        pub(crate) const NOTIFICATION_PAUSED: i32 = 14i32;
        pub(crate) const NOTIFICATION_UNPAUSED: i32 = 15i32;
        pub(crate) const NOTIFICATION_PHYSICS_PROCESS: i32 = 16i32;
        pub(crate) const NOTIFICATION_PROCESS: i32 = 17i32;
        pub(crate) const NOTIFICATION_PARENTED: i32 = 18i32;
        pub(crate) const NOTIFICATION_UNPARENTED: i32 = 19i32;
        pub(crate) const NOTIFICATION_SCENE_INSTANTIATED: i32 = 20i32;
        pub(crate) const NOTIFICATION_DRAG_BEGIN: i32 = 21i32;
        pub(crate) const NOTIFICATION_DRAG_END: i32 = 22i32;
        pub(crate) const NOTIFICATION_PATH_RENAMED: i32 = 23i32;
        pub(crate) const NOTIFICATION_CHILD_ORDER_CHANGED: i32 = 24i32;
        pub(crate) const NOTIFICATION_INTERNAL_PROCESS: i32 = 25i32;
        pub(crate) const NOTIFICATION_INTERNAL_PHYSICS_PROCESS: i32 = 26i32;
        pub(crate) const NOTIFICATION_POST_ENTER_TREE: i32 = 27i32;
        pub(crate) const NOTIFICATION_DISABLED: i32 = 28i32;
        pub(crate) const NOTIFICATION_ENABLED: i32 = 29i32;
        pub(crate) const NOTIFICATION_EDITOR_PRE_SAVE: i32 = 9001i32;
        pub(crate) const NOTIFICATION_EDITOR_POST_SAVE: i32 = 9002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_ENTER: i32 = 1002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_EXIT: i32 = 1003i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_IN: i32 = 1004i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_OUT: i32 = 1005i32;
        pub(crate) const NOTIFICATION_WM_CLOSE_REQUEST: i32 = 1006i32;
        pub(crate) const NOTIFICATION_WM_GO_BACK_REQUEST: i32 = 1007i32;
        pub(crate) const NOTIFICATION_WM_SIZE_CHANGED: i32 = 1008i32;
        pub(crate) const NOTIFICATION_WM_DPI_CHANGE: i32 = 1009i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_ENTER: i32 = 1010i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_EXIT: i32 = 1011i32;
        pub(crate) const NOTIFICATION_OS_MEMORY_WARNING: i32 = 2009i32;
        pub(crate) const NOTIFICATION_TRANSLATION_CHANGED: i32 = 2010i32;
        pub(crate) const NOTIFICATION_WM_ABOUT: i32 = 2011i32;
        pub(crate) const NOTIFICATION_CRASH: i32 = 2012i32;
        pub(crate) const NOTIFICATION_OS_IME_UPDATE: i32 = 2013i32;
        pub(crate) const NOTIFICATION_APPLICATION_RESUMED: i32 = 2014i32;
        pub(crate) const NOTIFICATION_APPLICATION_PAUSED: i32 = 2015i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_IN: i32 = 2016i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_OUT: i32 = 2017i32;
        pub(crate) const NOTIFICATION_TEXT_SERVER_CHANGED: i32 = 2018i32;
        
    }
    impl crate::obj::GodotClass for Node {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Node\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Node {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Node {
        
    }
    impl crate::obj::ExportableObject for Node {
        
    }
    impl crate::obj::cap::GodotDefault for Node {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Node {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node::add_sibling_ex`][super::Node::add_sibling_ex]."]
#[must_use]
pub struct ExAddSibling < 'a > {
    surround_object: &'a mut re_export::Node, sibling: Gd < crate::engine::Node >, force_readable_name: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSibling < 'a > {
    fn new(surround_object: &'a mut re_export::Node, sibling: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, sibling, force_readable_name: false,
        }
    }
    #[inline]
    pub fn force_readable_name(self, value: bool) -> Self {
        Self {
            force_readable_name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::add_sibling_full(self.surround_object, self.sibling, self.force_readable_name,)
    }
}
#[doc = "Default-param extender for [`Node::add_child_ex`][super::Node::add_child_ex]."]
#[must_use]
pub struct ExAddChild < 'a > {
    surround_object: &'a mut re_export::Node, node: Gd < crate::engine::Node >, force_readable_name: bool, internal: crate::engine::node::InternalMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddChild < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, node, force_readable_name: false, internal: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn force_readable_name(self, value: bool) -> Self {
        Self {
            force_readable_name: value, .. self
        }
    }
    #[inline]
    pub fn internal(self, value: crate::engine::node::InternalMode) -> Self {
        Self {
            internal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::add_child_full(self.surround_object, self.node, self.force_readable_name, self.internal,)
    }
}
#[doc = "Default-param extender for [`Node::reparent_ex`][super::Node::reparent_ex]."]
#[must_use]
pub struct ExReparent < 'a > {
    surround_object: &'a mut re_export::Node, new_parent: Gd < crate::engine::Node >, keep_global_transform: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReparent < 'a > {
    fn new(surround_object: &'a mut re_export::Node, new_parent: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, new_parent, keep_global_transform: true,
        }
    }
    #[inline]
    pub fn keep_global_transform(self, value: bool) -> Self {
        Self {
            keep_global_transform: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::reparent_full(self.surround_object, self.new_parent, self.keep_global_transform,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_count_ex`][super::Node::get_child_count_ex]."]
#[must_use]
pub struct ExGetChildCount < 'a > {
    surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildCount < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        Self {
            surround_object, include_internal: false,
        }
    }
    #[inline]
    pub fn include_internal(self, value: bool) -> Self {
        Self {
            include_internal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Node::get_child_count_full(self.surround_object, self.include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_children_ex`][super::Node::get_children_ex]."]
#[must_use]
pub struct ExGetChildren < 'a > {
    surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildren < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        Self {
            surround_object, include_internal: false,
        }
    }
    #[inline]
    pub fn include_internal(self, value: bool) -> Self {
        Self {
            include_internal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::engine::Node > > {
        re_export::Node::get_children_full(self.surround_object, self.include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_ex`][super::Node::get_child_ex]."]
#[must_use]
pub struct ExGetChild < 'a > {
    surround_object: &'a re_export::Node, idx: i32, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChild < 'a > {
    fn new(surround_object: &'a re_export::Node, idx: i32,) -> Self {
        Self {
            surround_object, idx, include_internal: false,
        }
    }
    #[inline]
    pub fn include_internal(self, value: bool) -> Self {
        Self {
            include_internal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::Node::get_child_full(self.surround_object, self.idx, self.include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::find_child_ex`][super::Node::find_child_ex]."]
#[must_use]
pub struct ExFindChild < 'a > {
    surround_object: &'a re_export::Node, pattern: GString, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChild < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: GString,) -> Self {
        Self {
            surround_object, pattern, recursive: true, owned: true,
        }
    }
    #[inline]
    pub fn recursive(self, value: bool) -> Self {
        Self {
            recursive: value, .. self
        }
    }
    #[inline]
    pub fn owned(self, value: bool) -> Self {
        Self {
            owned: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::Node::find_child_full(self.surround_object, self.pattern, self.recursive, self.owned,)
    }
}
#[doc = "Default-param extender for [`Node::find_children_ex`][super::Node::find_children_ex]."]
#[must_use]
pub struct ExFindChildren < 'a > {
    surround_object: &'a re_export::Node, pattern: GString, type_: GString, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChildren < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: GString,) -> Self {
        Self {
            surround_object, pattern, type_: GString::from(""), recursive: true, owned: true,
        }
    }
    #[inline]
    pub fn type_(self, value: GString) -> Self {
        Self {
            type_: value, .. self
        }
    }
    #[inline]
    pub fn recursive(self, value: bool) -> Self {
        Self {
            recursive: value, .. self
        }
    }
    #[inline]
    pub fn owned(self, value: bool) -> Self {
        Self {
            owned: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::engine::Node > > {
        re_export::Node::find_children_full(self.surround_object, self.pattern, self.type_, self.recursive, self.owned,)
    }
}
#[doc = "Default-param extender for [`Node::get_path_to_ex`][super::Node::get_path_to_ex]."]
#[must_use]
pub struct ExGetPathTo < 'a > {
    surround_object: &'a re_export::Node, node: Gd < crate::engine::Node >, use_unique_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPathTo < 'a > {
    fn new(surround_object: &'a re_export::Node, node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, node, use_unique_path: false,
        }
    }
    #[inline]
    pub fn use_unique_path(self, value: bool) -> Self {
        Self {
            use_unique_path: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        re_export::Node::get_path_to_full(self.surround_object, self.node, self.use_unique_path,)
    }
}
#[doc = "Default-param extender for [`Node::add_to_group_ex`][super::Node::add_to_group_ex]."]
#[must_use]
pub struct ExAddToGroup < 'a > {
    surround_object: &'a mut re_export::Node, group: StringName, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddToGroup < 'a > {
    fn new(surround_object: &'a mut re_export::Node, group: StringName,) -> Self {
        Self {
            surround_object, group, persistent: false,
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
        re_export::Node::add_to_group_full(self.surround_object, self.group, self.persistent,)
    }
}
#[doc = "Default-param extender for [`Node::get_index_ex`][super::Node::get_index_ex]."]
#[must_use]
pub struct ExGetIndex < 'a > {
    surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetIndex < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        Self {
            surround_object, include_internal: false,
        }
    }
    #[inline]
    pub fn include_internal(self, value: bool) -> Self {
        Self {
            include_internal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Node::get_index_full(self.surround_object, self.include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::propagate_call_ex`][super::Node::propagate_call_ex]."]
#[must_use]
pub struct ExPropagateCall < 'a > {
    surround_object: &'a mut re_export::Node, method: StringName, args: VariantArray, parent_first: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCall < 'a > {
    fn new(surround_object: &'a mut re_export::Node, method: StringName,) -> Self {
        Self {
            surround_object, method, args: Array::new(), parent_first: false,
        }
    }
    #[inline]
    pub fn args(self, value: VariantArray) -> Self {
        Self {
            args: value, .. self
        }
    }
    #[inline]
    pub fn parent_first(self, value: bool) -> Self {
        Self {
            parent_first: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::propagate_call_full(self.surround_object, self.method, self.args, self.parent_first,)
    }
}
#[doc = "Default-param extender for [`Node::duplicate_ex`][super::Node::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'a > {
    surround_object: &'a re_export::Node, flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDuplicate < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        Self {
            surround_object, flags: 15i32,
        }
    }
    #[inline]
    pub fn flags(self, value: i32) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::Node::duplicate_full(self.surround_object, self.flags,)
    }
}
#[doc = "Default-param extender for [`Node::replace_by_ex`][super::Node::replace_by_ex]."]
#[must_use]
pub struct ExReplaceBy < 'a > {
    surround_object: &'a mut re_export::Node, node: Gd < crate::engine::Node >, keep_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReplaceBy < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, node, keep_groups: false,
        }
    }
    #[inline]
    pub fn keep_groups(self, value: bool) -> Self {
        Self {
            keep_groups: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::replace_by_full(self.surround_object, self.node, self.keep_groups,)
    }
}
#[doc = "Default-param extender for [`Node::set_multiplayer_authority_ex`][super::Node::set_multiplayer_authority_ex]."]
#[must_use]
pub struct ExSetMultiplayerAuthority < 'a > {
    surround_object: &'a mut re_export::Node, id: i32, recursive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetMultiplayerAuthority < 'a > {
    fn new(surround_object: &'a mut re_export::Node, id: i32,) -> Self {
        Self {
            surround_object, id, recursive: true,
        }
    }
    #[inline]
    pub fn recursive(self, value: bool) -> Self {
        Self {
            recursive: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node::set_multiplayer_authority_full(self.surround_object, self.id, self.recursive,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProcessMode {
    ord: i32
}
impl ProcessMode {
    pub const PROCESS_MODE_INHERIT: Self = Self {
        ord: 0i32
    };
    pub const PROCESS_MODE_PAUSABLE: Self = Self {
        ord: 1i32
    };
    pub const PROCESS_MODE_WHEN_PAUSED: Self = Self {
        ord: 2i32
    };
    pub const PROCESS_MODE_ALWAYS: Self = Self {
        ord: 3i32
    };
    pub const PROCESS_MODE_DISABLED: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ProcessMode {
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
impl crate::builtin::meta::GodotConvert for ProcessMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ProcessMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProcessThreadGroup {
    ord: i32
}
impl ProcessThreadGroup {
    pub const PROCESS_THREAD_GROUP_INHERIT: Self = Self {
        ord: 0i32
    };
    pub const PROCESS_THREAD_GROUP_MAIN_THREAD: Self = Self {
        ord: 1i32
    };
    pub const PROCESS_THREAD_GROUP_SUB_THREAD: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ProcessThreadGroup {
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
impl crate::builtin::meta::GodotConvert for ProcessThreadGroup {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ProcessThreadGroup {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ProcessThreadGroup {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ProcessThreadMessages {
    ord: u64
}
impl ProcessThreadMessages {
    pub const FLAG_PROCESS_THREAD_MESSAGES: Self = Self {
        ord: 1u64
    };
    pub const FLAG_PROCESS_THREAD_MESSAGES_PHYSICS: Self = Self {
        ord: 2u64
    };
    pub const FLAG_PROCESS_THREAD_MESSAGES_ALL: Self = Self {
        ord: 3u64
    };
    
}
impl crate::obj::EngineBitfield for ProcessThreadMessages {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for ProcessThreadMessages {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for ProcessThreadMessages {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for ProcessThreadMessages {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ProcessThreadMessages {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DuplicateFlags {
    ord: i32
}
impl DuplicateFlags {
    pub const DUPLICATE_SIGNALS: Self = Self {
        ord: 1i32
    };
    pub const DUPLICATE_GROUPS: Self = Self {
        ord: 2i32
    };
    pub const DUPLICATE_SCRIPTS: Self = Self {
        ord: 4i32
    };
    pub const DUPLICATE_USE_INSTANTIATION: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for DuplicateFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for DuplicateFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DuplicateFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DuplicateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InternalMode {
    ord: i32
}
impl InternalMode {
    pub const INTERNAL_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const INTERNAL_MODE_FRONT: Self = Self {
        ord: 1i32
    };
    pub const INTERNAL_MODE_BACK: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for InternalMode {
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
impl crate::builtin::meta::GodotConvert for InternalMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InternalMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InternalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}