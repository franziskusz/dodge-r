#![doc = "Sidecar module for class [`SceneMultiplayer`][crate::engine::SceneMultiplayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneMultiplayer` enums](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneMultiplayer.`\n\nInherits [`MultiplayerApi`][crate::engine::MultiplayerApi].\n\nRelated symbols:\n\n* [`scene_multiplayer`][crate::engine::scene_multiplayer]: sidecar module with related enum/flag types\n* [`ISceneMultiplayer`][crate::engine::ISceneMultiplayer]: virtual methods\n\n\nSee also [Godot docs for `SceneMultiplayer`](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneMultiplayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SceneMultiplayer`][crate::engine::SceneMultiplayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SceneMultiplayer` methods](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneMultiplayer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SceneMultiplayer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_root_path(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_path(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_peer(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_authenticating_peers(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_authenticating_peers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn send_auth(&mut self, id: i32, data: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, PackedByteArray);
            let args = (id, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send_auth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn complete_auth(&mut self, id: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "complete_auth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auth_callback(&mut self, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auth_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auth_callback(&self,) -> Callable {
            type RetMarshal = PtrcallReturnT < Callable >;
            type CallSig = (Callable,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auth_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auth_timeout(&mut self, timeout: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auth_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auth_timeout(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auth_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refuse_new_connections(&mut self, refuse: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (refuse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_refuse_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_refusing_new_connections(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_refusing_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_object_decoding(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_object_decoding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_object_decoding_allowed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_object_decoding_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_server_relay_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_server_relay_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_server_relay_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_server_relay_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn send_bytes_full(&mut self, bytes: PackedByteArray, id: i32, mode: crate::engine::multiplayer_peer::TransferMode, channel: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray, i32, crate::engine::multiplayer_peer::TransferMode, i32);
            let args = (bytes, id, mode, channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn send_bytes(&mut self, bytes: PackedByteArray,) -> crate::engine::global::Error {
            self.send_bytes_ex(bytes,) . done()
        }
        #[inline]
        pub fn send_bytes_ex(&mut self, bytes: PackedByteArray,) -> ExSendBytes < '_ > {
            ExSendBytes::new(self, bytes,)
        }
        pub fn get_max_sync_packet_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_sync_packet_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_sync_packet_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_sync_packet_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_delta_packet_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_delta_packet_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_delta_packet_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_delta_packet_size", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SceneMultiplayer {
        type Base = crate::engine::MultiplayerApi;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SceneMultiplayer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneMultiplayer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SceneMultiplayer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::MultiplayerApi > for SceneMultiplayer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for SceneMultiplayer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SceneMultiplayer {
        
    }
    impl crate::obj::cap::GodotDefault for SceneMultiplayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneMultiplayer {
        type Target = crate::engine::MultiplayerApi;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneMultiplayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SceneMultiplayer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SceneMultiplayer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::MultiplayerApi > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneMultiplayer::send_bytes_ex`][super::SceneMultiplayer::send_bytes_ex]."]
#[must_use]
pub struct ExSendBytes < 'a > {
    surround_object: &'a mut re_export::SceneMultiplayer, bytes: PackedByteArray, id: i32, mode: crate::engine::multiplayer_peer::TransferMode, channel: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSendBytes < 'a > {
    fn new(surround_object: &'a mut re_export::SceneMultiplayer, bytes: PackedByteArray,) -> Self {
        Self {
            surround_object, bytes, id: 0i32, mode: crate::obj::EngineEnum::from_ord(2), channel: 0i32,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn mode(self, value: crate::engine::multiplayer_peer::TransferMode) -> Self {
        Self {
            mode: value, .. self
        }
    }
    #[inline]
    pub fn channel(self, value: i32) -> Self {
        Self {
            channel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::SceneMultiplayer::send_bytes_full(self.surround_object, self.bytes, self.id, self.mode, self.channel,)
    }
}