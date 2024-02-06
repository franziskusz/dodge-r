#![doc = "Sidecar module for class [`MultiplayerSynchronizer`][crate::engine::MultiplayerSynchronizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerSynchronizer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerSynchronizer.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`multiplayer_synchronizer`][crate::engine::multiplayer_synchronizer]: sidecar module with related enum/flag types\n* [`IMultiplayerSynchronizer`][crate::engine::IMultiplayerSynchronizer]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerSynchronizer`](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerSynchronizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiplayerSynchronizer`][crate::engine::MultiplayerSynchronizer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiplayerSynchronizer` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerSynchronizer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerSynchronizer {
        pub fn set_root_path(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_path(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_replication_interval(&mut self, milliseconds: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_replication_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_replication_interval(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_replication_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_delta_interval(&mut self, milliseconds: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_delta_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delta_interval(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_delta_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_replication_config(&mut self, config: Gd < crate::engine::SceneReplicationConfig >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::SceneReplicationConfig >);
            let args = (config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_replication_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_replication_config(&mut self,) -> Option < Gd < crate::engine::SceneReplicationConfig > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SceneReplicationConfig > >;
            type CallSig = (Option < Gd < crate::engine::SceneReplicationConfig > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_replication_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_update_mode(&mut self, mode: crate::engine::multiplayer_synchronizer::VisibilityUpdateMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::multiplayer_synchronizer::VisibilityUpdateMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_update_mode(&self,) -> crate::engine::multiplayer_synchronizer::VisibilityUpdateMode {
            type RetMarshal = PtrcallReturnT < crate::engine::multiplayer_synchronizer::VisibilityUpdateMode >;
            type CallSig = (crate::engine::multiplayer_synchronizer::VisibilityUpdateMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn update_visibility_full(&mut self, for_peer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (for_peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_visibility", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn update_visibility(&mut self,) {
            self.update_visibility_ex() . done()
        }
        #[inline]
        pub fn update_visibility_ex(&mut self,) -> ExUpdateVisibility < '_ > {
            ExUpdateVisibility::new(self,)
        }
        pub fn set_visibility_public(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_public", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visibility_public(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visibility_public", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_visibility_filter(&mut self, filter: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_visibility_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_visibility_filter(&mut self, filter: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_visibility_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_for(&mut self, peer: i32, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (peer, visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_for", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_for(&self, peer: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_for", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MultiplayerSynchronizer {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MultiplayerSynchronizer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerSynchronizer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MultiplayerSynchronizer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for MultiplayerSynchronizer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MultiplayerSynchronizer {
        
    }
    impl crate::obj::ExportableObject for MultiplayerSynchronizer {
        
    }
    impl crate::obj::cap::GodotDefault for MultiplayerSynchronizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiplayerSynchronizer {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerSynchronizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MultiplayerSynchronizer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MultiplayerSynchronizer > for $Class {
                
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
#[doc = "Default-param extender for [`MultiplayerSynchronizer::update_visibility_ex`][super::MultiplayerSynchronizer::update_visibility_ex]."]
#[must_use]
pub struct ExUpdateVisibility < 'a > {
    surround_object: &'a mut re_export::MultiplayerSynchronizer, for_peer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUpdateVisibility < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerSynchronizer,) -> Self {
        Self {
            surround_object, for_peer: 0i32,
        }
    }
    #[inline]
    pub fn for_peer(self, value: i32) -> Self {
        Self {
            for_peer: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::MultiplayerSynchronizer::update_visibility_full(self.surround_object, self.for_peer,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VisibilityUpdateMode {
    ord: i32
}
impl VisibilityUpdateMode {
    pub const VISIBILITY_PROCESS_IDLE: Self = Self {
        ord: 0i32
    };
    pub const VISIBILITY_PROCESS_PHYSICS: Self = Self {
        ord: 1i32
    };
    pub const VISIBILITY_PROCESS_NONE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for VisibilityUpdateMode {
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
impl crate::builtin::meta::GodotConvert for VisibilityUpdateMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibilityUpdateMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibilityUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}