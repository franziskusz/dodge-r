#![doc = "Sidecar module for class [`SceneTree`][crate::engine::SceneTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneTree` enums](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneTree.`\n\nInherits [`MainLoop`][crate::engine::MainLoop].\n\nRelated symbols:\n\n* [`scene_tree`][crate::engine::scene_tree]: sidecar module with related enum/flag types\n* [`ISceneTree`][crate::engine::ISceneTree]: virtual methods\n\n\nSee also [Godot docs for `SceneTree`](https://docs.godotengine.org/en/stable/classes/class_scenetree.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneTree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SceneTree`][crate::engine::SceneTree].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SceneTree` methods](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneTree: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: MainLoopNotification) {
            unimplemented !()
        }
        fn initialize(&mut self,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) -> bool {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) -> bool {
            unimplemented !()
        }
        fn finalize(&mut self,) {
            unimplemented !()
        }
    }
    impl SceneTree {
        pub fn get_root(&self,) -> Option < Gd < crate::engine::Window > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Window > >;
            type CallSig = (Option < Gd < crate::engine::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_group(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_accept_quit(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_auto_accept_quit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_accept_quit(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_accept_quit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_quit_on_go_back(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_quit_on_go_back", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_quit_on_go_back(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_quit_on_go_back", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_collisions_hint(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_collisions_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debugging_collisions_hint(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_debugging_collisions_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_paths_hint(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_paths_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debugging_paths_hint(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_debugging_paths_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_navigation_hint(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_navigation_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debugging_navigation_hint(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_debugging_navigation_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edited_scene_root(&mut self, scene: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_edited_scene_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited_scene_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pause(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_paused(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_timer_full(&mut self, time_sec: f64, process_always: bool, process_in_physics: bool, ignore_time_scale: bool,) -> Option < Gd < crate::engine::SceneTreeTimer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SceneTreeTimer > >;
            type CallSig = (Option < Gd < crate::engine::SceneTreeTimer > >, f64, bool, bool, bool);
            let args = (time_sec, process_always, process_in_physics, ignore_time_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_timer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_timer(&mut self, time_sec: f64,) -> Option < Gd < crate::engine::SceneTreeTimer > > {
            self.create_timer_ex(time_sec,) . done()
        }
        #[inline]
        pub fn create_timer_ex(&mut self, time_sec: f64,) -> ExCreateTimer < '_ > {
            ExCreateTimer::new(self, time_sec,)
        }
        pub fn create_tween(&mut self,) -> Option < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tween > >;
            type CallSig = (Option < Gd < crate::engine::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_tween", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_processed_tweens(&mut self,) -> Array < Gd < crate::engine::Tween > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Tween > > >;
            type CallSig = (Array < Gd < crate::engine::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_processed_tweens", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn quit_full(&mut self, exit_code: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (exit_code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "quit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn quit(&mut self,) {
            self.quit_ex() . done()
        }
        #[inline]
        pub fn quit_ex(&mut self,) -> ExQuit < '_ > {
            ExQuit::new(self,)
        }
        pub fn queue_delete(&mut self, obj: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >);
            let args = (obj,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "queue_delete", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_group_flags(&mut self, flags: i64, group: StringName, method: StringName, varargs: &[Variant]) {
            type CallSig = ((), i64, StringName, StringName);
            let args = (flags, group, method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7347usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_group_flags", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn notify_group_flags(&mut self, call_flags: u32, group: StringName, notification: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, StringName, i32);
            let args = (call_flags, group, notification,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_group_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group_flags(&mut self, call_flags: u32, group: StringName, property: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, StringName, GString, Variant);
            let args = (call_flags, group, property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_group_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_group(&mut self, group: StringName, method: StringName, varargs: &[Variant]) {
            type CallSig = ((), StringName, StringName);
            let args = (group, method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7350usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_group", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn notify_group(&mut self, group: StringName, notification: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (group, notification,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group(&mut self, group: StringName, property: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, GString, Variant);
            let args = (group, property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_nodes_in_group(&mut self, group: StringName,) -> Array < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node > > >;
            type CallSig = (Array < Gd < crate::engine::Node > >, StringName);
            let args = (group,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_nodes_in_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_first_node_in_group(&mut self, group: StringName,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, StringName);
            let args = (group,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_first_node_in_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_scene(&mut self, child_node: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (child_node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_scene(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn change_scene_to_file(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "change_scene_to_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn change_scene_to_packed(&mut self, packed_scene: Gd < crate::engine::PackedScene >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::PackedScene >);
            let args = (packed_scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "change_scene_to_packed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_current_scene(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reload_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unload_current_scene(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unload_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_multiplayer_full(&mut self, multiplayer: Gd < crate::engine::MultiplayerApi >, root_path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::MultiplayerApi >, NodePath);
            let args = (multiplayer, root_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multiplayer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_multiplayer(&mut self, multiplayer: Gd < crate::engine::MultiplayerApi >,) {
            self.set_multiplayer_ex(multiplayer,) . done()
        }
        #[inline]
        pub fn set_multiplayer_ex(&mut self, multiplayer: Gd < crate::engine::MultiplayerApi >,) -> ExSetMultiplayer < '_ > {
            ExSetMultiplayer::new(self, multiplayer,)
        }
        pub(crate) fn get_multiplayer_full(&self, for_path: NodePath,) -> Option < Gd < crate::engine::MultiplayerApi > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MultiplayerApi > >;
            type CallSig = (Option < Gd < crate::engine::MultiplayerApi > >, NodePath);
            let args = (for_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_multiplayer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::engine::MultiplayerApi > > {
            self.get_multiplayer_ex() . done()
        }
        #[inline]
        pub fn get_multiplayer_ex(&self,) -> ExGetMultiplayer < '_ > {
            ExGetMultiplayer::new(self,)
        }
        pub fn set_multiplayer_poll_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multiplayer_poll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multiplayer_poll_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_multiplayer_poll_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SceneTree {
        type Base = crate::engine::MainLoop;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SceneTree\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneTree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SceneTree {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::MainLoop > for SceneTree {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SceneTree {
        
    }
    impl crate::obj::cap::GodotDefault for SceneTree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneTree {
        type Target = crate::engine::MainLoop;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SceneTree {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SceneTree > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::MainLoop > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneTree::create_timer_ex`][super::SceneTree::create_timer_ex]."]
#[must_use]
pub struct ExCreateTimer < 'a > {
    surround_object: &'a mut re_export::SceneTree, time_sec: f64, process_always: bool, process_in_physics: bool, ignore_time_scale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTimer < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree, time_sec: f64,) -> Self {
        Self {
            surround_object, time_sec, process_always: true, process_in_physics: false, ignore_time_scale: false,
        }
    }
    #[inline]
    pub fn process_always(self, value: bool) -> Self {
        Self {
            process_always: value, .. self
        }
    }
    #[inline]
    pub fn process_in_physics(self, value: bool) -> Self {
        Self {
            process_in_physics: value, .. self
        }
    }
    #[inline]
    pub fn ignore_time_scale(self, value: bool) -> Self {
        Self {
            ignore_time_scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::SceneTreeTimer > > {
        re_export::SceneTree::create_timer_full(self.surround_object, self.time_sec, self.process_always, self.process_in_physics, self.ignore_time_scale,)
    }
}
#[doc = "Default-param extender for [`SceneTree::quit_ex`][super::SceneTree::quit_ex]."]
#[must_use]
pub struct ExQuit < 'a > {
    surround_object: &'a mut re_export::SceneTree, exit_code: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExQuit < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree,) -> Self {
        Self {
            surround_object, exit_code: 0i32,
        }
    }
    #[inline]
    pub fn exit_code(self, value: i32) -> Self {
        Self {
            exit_code: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SceneTree::quit_full(self.surround_object, self.exit_code,)
    }
}
#[doc = "Default-param extender for [`SceneTree::set_multiplayer_ex`][super::SceneTree::set_multiplayer_ex]."]
#[must_use]
pub struct ExSetMultiplayer < 'a > {
    surround_object: &'a mut re_export::SceneTree, multiplayer: Gd < crate::engine::MultiplayerApi >, root_path: NodePath,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetMultiplayer < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree, multiplayer: Gd < crate::engine::MultiplayerApi >,) -> Self {
        Self {
            surround_object, multiplayer, root_path: NodePath::from(""),
        }
    }
    #[inline]
    pub fn root_path(self, value: NodePath) -> Self {
        Self {
            root_path: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SceneTree::set_multiplayer_full(self.surround_object, self.multiplayer, self.root_path,)
    }
}
#[doc = "Default-param extender for [`SceneTree::get_multiplayer_ex`][super::SceneTree::get_multiplayer_ex]."]
#[must_use]
pub struct ExGetMultiplayer < 'a > {
    surround_object: &'a re_export::SceneTree, for_path: NodePath,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMultiplayer < 'a > {
    fn new(surround_object: &'a re_export::SceneTree,) -> Self {
        Self {
            surround_object, for_path: NodePath::from(""),
        }
    }
    #[inline]
    pub fn for_path(self, value: NodePath) -> Self {
        Self {
            for_path: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::MultiplayerApi > > {
        re_export::SceneTree::get_multiplayer_full(self.surround_object, self.for_path,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GroupCallFlags {
    ord: i32
}
impl GroupCallFlags {
    pub const GROUP_CALL_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const GROUP_CALL_REVERSE: Self = Self {
        ord: 1i32
    };
    pub const GROUP_CALL_DEFERRED: Self = Self {
        ord: 2i32
    };
    pub const GROUP_CALL_UNIQUE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for GroupCallFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for GroupCallFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GroupCallFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GroupCallFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}