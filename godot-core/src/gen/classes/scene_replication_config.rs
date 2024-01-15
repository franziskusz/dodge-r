#![doc = "Sidecar module for class [`SceneReplicationConfig`][crate::engine::SceneReplicationConfig].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneReplicationConfig` enums](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneReplicationConfig.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`scene_replication_config`][crate::engine::scene_replication_config]: sidecar module with related enum/flag types\n* [`ISceneReplicationConfig`][crate::engine::ISceneReplicationConfig]: virtual methods\n\n\nSee also [Godot docs for `SceneReplicationConfig`](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneReplicationConfig {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SceneReplicationConfig`][crate::engine::SceneReplicationConfig].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SceneReplicationConfig` methods](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneReplicationConfig: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl SceneReplicationConfig {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_properties(&self,) -> Array < NodePath > {
            type RetMarshal = PtrcallReturnT < Array < NodePath > >;
            type CallSig = (Array < NodePath >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_properties", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_property_full(&mut self, path: NodePath, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, i32);
            let args = (path, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_property(&mut self, path: NodePath,) {
            self.add_property_ex(path,) . done()
        }
        #[inline]
        pub fn add_property_ex(&mut self, path: NodePath,) -> ExAddProperty < '_ > {
            ExAddProperty::new(self, path,)
        }
        pub fn has_property(&self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_property(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_index(&self, path: NodePath,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_spawn(&mut self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_spawn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_set_spawn(&mut self, path: NodePath, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, bool);
            let args = (path, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_set_spawn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_replication_mode(&mut self, path: NodePath,) -> crate::engine::scene_replication_config::ReplicationMode {
            type RetMarshal = PtrcallReturnT < crate::engine::scene_replication_config::ReplicationMode >;
            type CallSig = (crate::engine::scene_replication_config::ReplicationMode, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_replication_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_set_replication_mode(&mut self, path: NodePath, mode: crate::engine::scene_replication_config::ReplicationMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, crate::engine::scene_replication_config::ReplicationMode);
            let args = (path, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_set_replication_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_sync(&mut self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_set_sync(&mut self, path: NodePath, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, bool);
            let args = (path, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_set_sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_watch(&mut self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_watch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_set_watch(&mut self, path: NodePath, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, bool);
            let args = (path, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_set_watch", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SceneReplicationConfig {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SceneReplicationConfig\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneReplicationConfig {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SceneReplicationConfig {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for SceneReplicationConfig {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for SceneReplicationConfig {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SceneReplicationConfig {
        
    }
    impl crate::obj::ExportableObject for SceneReplicationConfig {
        
    }
    impl crate::obj::cap::GodotDefault for SceneReplicationConfig {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneReplicationConfig {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneReplicationConfig {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SceneReplicationConfig {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SceneReplicationConfig > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneReplicationConfig::add_property_ex`][super::SceneReplicationConfig::add_property_ex]."]
#[must_use]
pub struct ExAddProperty < 'a > {
    surround_object: &'a mut re_export::SceneReplicationConfig, path: NodePath, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddProperty < 'a > {
    fn new(surround_object: &'a mut re_export::SceneReplicationConfig, path: NodePath,) -> Self {
        Self {
            surround_object, path, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SceneReplicationConfig::add_property_full(self.surround_object, self.path, self.index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReplicationMode {
    ord: i32
}
impl ReplicationMode {
    pub const REPLICATION_MODE_NEVER: Self = Self {
        ord: 0i32
    };
    pub const REPLICATION_MODE_ALWAYS: Self = Self {
        ord: 1i32
    };
    pub const REPLICATION_MODE_ON_CHANGE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ReplicationMode {
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
impl crate::builtin::meta::GodotConvert for ReplicationMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ReplicationMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ReplicationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}