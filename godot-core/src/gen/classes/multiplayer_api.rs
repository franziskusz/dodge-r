#![doc = "Sidecar module for class [`MultiplayerApi`][crate::engine::MultiplayerApi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerAPI` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerAPI.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`multiplayer_api`][crate::engine::multiplayer_api]: sidecar module with related enum/flag types\n* [`IMultiplayerApi`][crate::engine::IMultiplayerApi]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerAPI`](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerApi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiplayerApi`][crate::engine::MultiplayerApi].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiplayerAPI` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerApi: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerApi {
        pub fn has_multiplayer_peer(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_multiplayer_peer(&mut self,) -> Option < Gd < crate::engine::MultiplayerPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MultiplayerPeer > >;
            type CallSig = (Option < Gd < crate::engine::MultiplayerPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multiplayer_peer(&mut self, peer: Gd < crate::engine::MultiplayerPeer >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::MultiplayerPeer >);
            let args = (peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_server(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_sender_id(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_remote_sender_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn rpc_full(&mut self, peer: i32, object: Gd < crate::engine::Object >, method: StringName, arguments: VariantArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32, Gd < crate::engine::Object >, StringName, VariantArray);
            let args = (peer, object, method, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rpc", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn rpc(&mut self, peer: i32, object: Gd < crate::engine::Object >, method: StringName,) -> crate::engine::global::Error {
            self.rpc_ex(peer, object, method,) . done()
        }
        #[inline]
        pub fn rpc_ex(&mut self, peer: i32, object: Gd < crate::engine::Object >, method: StringName,) -> ExRpc < '_ > {
            ExRpc::new(self, peer, object, method,)
        }
        pub fn object_configuration_add(&mut self, object: Gd < crate::engine::Object >, configuration: Variant,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Object >, Variant);
            let args = (object, configuration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "object_configuration_add", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn object_configuration_remove(&mut self, object: Gd < crate::engine::Object >, configuration: Variant,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Object >, Variant);
            let args = (object, configuration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "object_configuration_remove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peers(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_peers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_interface(interface_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (interface_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_interface", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_default_interface() -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_interface", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn create_default_interface() -> Option < Gd < crate::engine::MultiplayerApi > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::MultiplayerApi > >;
            type CallSig = (Option < Gd < crate::engine::MultiplayerApi > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_default_interface", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for MultiplayerApi {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MultiplayerAPI\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerApi {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MultiplayerApi {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MultiplayerApi {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MultiplayerApi {
        
    }
    impl std::ops::Deref for MultiplayerApi {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerApi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MultiplayerApi {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MultiplayerApi > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerApi::rpc_ex`][super::MultiplayerApi::rpc_ex]."]
#[must_use]
pub struct ExRpc < 'a > {
    surround_object: &'a mut re_export::MultiplayerApi, peer: i32, object: Gd < crate::engine::Object >, method: StringName, arguments: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRpc < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerApi, peer: i32, object: Gd < crate::engine::Object >, method: StringName,) -> Self {
        Self {
            surround_object, peer, object, method, arguments: Array::new(),
        }
    }
    #[inline]
    pub fn arguments(self, value: VariantArray) -> Self {
        Self {
            arguments: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::MultiplayerApi::rpc_full(self.surround_object, self.peer, self.object, self.method, self.arguments,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RPCMode {
    ord: i32
}
impl RPCMode {
    pub const RPC_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const RPC_MODE_ANY_PEER: Self = Self {
        ord: 1i32
    };
    pub const RPC_MODE_AUTHORITY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for RPCMode {
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
impl crate::builtin::meta::GodotConvert for RPCMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RPCMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RPCMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}