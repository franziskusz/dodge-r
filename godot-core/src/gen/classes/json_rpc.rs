#![doc = "Sidecar module for class [`JsonRpc`][crate::engine::JsonRpc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `JSONRPC` enums](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `JSONRPC.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`json_rpc`][crate::engine::json_rpc]: sidecar module with related enum/flag types\n* [`IJsonRpc`][crate::engine::IJsonRpc]: virtual methods\n\n\nSee also [Godot docs for `JSONRPC`](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct JsonRpc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`JsonRpc`][crate::engine::JsonRpc].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `JSONRPC` methods](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IJsonRpc: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl JsonRpc {
        pub fn set_scope(&mut self, scope: GString, target: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::Object >);
            let args = (scope, target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scope", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn process_action_full(&mut self, action: Variant, recurse: bool,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Variant, bool);
            let args = (action, recurse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "process_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn process_action(&mut self, action: Variant,) -> Variant {
            self.process_action_ex(action,) . done()
        }
        #[inline]
        pub fn process_action_ex(&mut self, action: Variant,) -> ExProcessAction < '_ > {
            ExProcessAction::new(self, action,)
        }
        pub fn process_string(&mut self, action: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "process_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_request(&mut self, method: GString, params: Variant, id: Variant,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, Variant, Variant);
            let args = (method, params, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_response(&mut self, result: Variant, id: Variant,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Variant, Variant);
            let args = (result, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_response", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_notification(&mut self, method: GString, params: Variant,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, Variant);
            let args = (method, params,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn make_response_error_full(&self, code: i32, message: GString, id: Variant,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32, GString, Variant);
            let args = (code, message, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_response_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn make_response_error(&self, code: i32, message: GString,) -> Dictionary {
            self.make_response_error_ex(code, message,) . done()
        }
        #[inline]
        pub fn make_response_error_ex(&self, code: i32, message: GString,) -> ExMakeResponseError < '_ > {
            ExMakeResponseError::new(self, code, message,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for JsonRpc {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"JSONRPC\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for JsonRpc {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for JsonRpc {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for JsonRpc {
        
    }
    impl crate::obj::cap::GodotDefault for JsonRpc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for JsonRpc {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for JsonRpc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_JsonRpc {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::JsonRpc > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`JsonRpc::process_action_ex`][super::JsonRpc::process_action_ex]."]
#[must_use]
pub struct ExProcessAction < 'a > {
    surround_object: &'a mut re_export::JsonRpc, action: Variant, recurse: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExProcessAction < 'a > {
    fn new(surround_object: &'a mut re_export::JsonRpc, action: Variant,) -> Self {
        Self {
            surround_object, action, recurse: false,
        }
    }
    #[inline]
    pub fn recurse(self, value: bool) -> Self {
        Self {
            recurse: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::JsonRpc::process_action_full(self.surround_object, self.action, self.recurse,)
    }
}
#[doc = "Default-param extender for [`JsonRpc::make_response_error_ex`][super::JsonRpc::make_response_error_ex]."]
#[must_use]
pub struct ExMakeResponseError < 'a > {
    surround_object: &'a re_export::JsonRpc, code: i32, message: GString, id: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMakeResponseError < 'a > {
    fn new(surround_object: &'a re_export::JsonRpc, code: i32, message: GString,) -> Self {
        Self {
            surround_object, code, message, id: Variant::nil(),
        }
    }
    #[inline]
    pub fn id(self, value: Variant) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        re_export::JsonRpc::make_response_error_full(self.surround_object, self.code, self.message, self.id,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ErrorCode {
    ord: i32
}
impl ErrorCode {
    pub const PARSE_ERROR: Self = Self {
        ord: - 32700i32
    };
    pub const INVALID_REQUEST: Self = Self {
        ord: - 32600i32
    };
    pub const METHOD_NOT_FOUND: Self = Self {
        ord: - 32601i32
    };
    pub const INVALID_PARAMS: Self = Self {
        ord: - 32602i32
    };
    pub const INTERNAL_ERROR: Self = Self {
        ord: - 32603i32
    };
    
}
impl crate::obj::EngineEnum for ErrorCode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 32700i32 | ord @ - 32603i32 | ord @ - 32602i32 | ord @ - 32601i32 | ord @ - 32600i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ErrorCode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ErrorCode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ErrorCode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}