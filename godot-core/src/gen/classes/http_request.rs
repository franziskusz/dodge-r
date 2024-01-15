#![doc = "Sidecar module for class [`HttpRequest`][crate::engine::HttpRequest].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPRequest` enums](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HTTPRequest.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`http_request`][crate::engine::http_request]: sidecar module with related enum/flag types\n* [`IHttpRequest`][crate::engine::IHttpRequest]: virtual methods\n\n\nSee also [Godot docs for `HTTPRequest`](https://docs.godotengine.org/en/stable/classes/class_httprequest.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HttpRequest {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`HttpRequest`][crate::engine::HttpRequest].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `HTTPRequest` methods](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHttpRequest: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl HttpRequest {
        pub(crate) fn request_full(&mut self, url: GString, custom_headers: PackedStringArray, method: crate::engine::http_client::Method, request_data: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, PackedStringArray, crate::engine::http_client::Method, GString);
            let args = (url, custom_headers, method, request_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn request(&mut self, url: GString,) -> crate::engine::global::Error {
            self.request_ex(url,) . done()
        }
        #[inline]
        pub fn request_ex(&mut self, url: GString,) -> ExRequest < '_ > {
            ExRequest::new(self, url,)
        }
        pub(crate) fn request_raw_full(&mut self, url: GString, custom_headers: PackedStringArray, method: crate::engine::http_client::Method, request_data_raw: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, PackedStringArray, crate::engine::http_client::Method, PackedByteArray);
            let args = (url, custom_headers, method, request_data_raw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn request_raw(&mut self, url: GString,) -> crate::engine::global::Error {
            self.request_raw_ex(url,) . done()
        }
        #[inline]
        pub fn request_raw_ex(&mut self, url: GString,) -> ExRequestRaw < '_ > {
            ExRequestRaw::new(self, url,)
        }
        pub fn cancel_request(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cancel_request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tls_options(&mut self, client_options: Gd < crate::engine::TlsOptions >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TlsOptions >);
            let args = (client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tls_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_http_client_status(&self,) -> crate::engine::http_client::Status {
            type RetMarshal = PtrcallReturnT < crate::engine::http_client::Status >;
            type CallSig = (crate::engine::http_client::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_http_client_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_threads(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_threads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_threads(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_threads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accept_gzip(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_accept_gzip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_accepting_gzip(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_accepting_gzip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_body_size_limit(&mut self, bytes: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_body_size_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_size_limit(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_body_size_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_redirects(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_redirects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_redirects(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_redirects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_download_file(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_download_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_download_file(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_download_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_downloaded_bytes(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_downloaded_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_body_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_timeout(&mut self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_download_chunk_size(&mut self, chunk_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (chunk_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_download_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_download_chunk_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_download_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_http_proxy(&mut self, host: GString, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_http_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_https_proxy(&mut self, host: GString, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_https_proxy", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for HttpRequest {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"HTTPRequest\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HttpRequest {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for HttpRequest {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for HttpRequest {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for HttpRequest {
        
    }
    impl crate::obj::ExportableObject for HttpRequest {
        
    }
    impl crate::obj::cap::GodotDefault for HttpRequest {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HttpRequest {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HttpRequest {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_HttpRequest {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::HttpRequest > for $Class {
                
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
#[doc = "Default-param extender for [`HttpRequest::request_ex`][super::HttpRequest::request_ex]."]
#[must_use]
pub struct ExRequest < 'a > {
    surround_object: &'a mut re_export::HttpRequest, url: GString, custom_headers: PackedStringArray, method: crate::engine::http_client::Method, request_data: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequest < 'a > {
    fn new(surround_object: &'a mut re_export::HttpRequest, url: GString,) -> Self {
        Self {
            surround_object, url, custom_headers: PackedStringArray::new(), method: crate::obj::EngineEnum::from_ord(0), request_data: GString::from(""),
        }
    }
    #[inline]
    pub fn custom_headers(self, value: PackedStringArray) -> Self {
        Self {
            custom_headers: value, .. self
        }
    }
    #[inline]
    pub fn method(self, value: crate::engine::http_client::Method) -> Self {
        Self {
            method: value, .. self
        }
    }
    #[inline]
    pub fn request_data(self, value: GString) -> Self {
        Self {
            request_data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::HttpRequest::request_full(self.surround_object, self.url, self.custom_headers, self.method, self.request_data,)
    }
}
#[doc = "Default-param extender for [`HttpRequest::request_raw_ex`][super::HttpRequest::request_raw_ex]."]
#[must_use]
pub struct ExRequestRaw < 'a > {
    surround_object: &'a mut re_export::HttpRequest, url: GString, custom_headers: PackedStringArray, method: crate::engine::http_client::Method, request_data_raw: PackedByteArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequestRaw < 'a > {
    fn new(surround_object: &'a mut re_export::HttpRequest, url: GString,) -> Self {
        Self {
            surround_object, url, custom_headers: PackedStringArray::new(), method: crate::obj::EngineEnum::from_ord(0), request_data_raw: PackedByteArray::new(),
        }
    }
    #[inline]
    pub fn custom_headers(self, value: PackedStringArray) -> Self {
        Self {
            custom_headers: value, .. self
        }
    }
    #[inline]
    pub fn method(self, value: crate::engine::http_client::Method) -> Self {
        Self {
            method: value, .. self
        }
    }
    #[inline]
    pub fn request_data_raw(self, value: PackedByteArray) -> Self {
        Self {
            request_data_raw: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::HttpRequest::request_raw_full(self.surround_object, self.url, self.custom_headers, self.method, self.request_data_raw,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Result {
    ord: i32
}
impl Result {
    pub const RESULT_SUCCESS: Self = Self {
        ord: 0i32
    };
    pub const RESULT_CHUNKED_BODY_SIZE_MISMATCH: Self = Self {
        ord: 1i32
    };
    pub const RESULT_CANT_CONNECT: Self = Self {
        ord: 2i32
    };
    pub const RESULT_CANT_RESOLVE: Self = Self {
        ord: 3i32
    };
    pub const RESULT_CONNECTION_ERROR: Self = Self {
        ord: 4i32
    };
    pub const RESULT_TLS_HANDSHAKE_ERROR: Self = Self {
        ord: 5i32
    };
    pub const RESULT_NO_RESPONSE: Self = Self {
        ord: 6i32
    };
    pub const RESULT_BODY_SIZE_LIMIT_EXCEEDED: Self = Self {
        ord: 7i32
    };
    pub const RESULT_BODY_DECOMPRESS_FAILED: Self = Self {
        ord: 8i32
    };
    pub const RESULT_REQUEST_FAILED: Self = Self {
        ord: 9i32
    };
    pub const RESULT_DOWNLOAD_FILE_CANT_OPEN: Self = Self {
        ord: 10i32
    };
    pub const RESULT_DOWNLOAD_FILE_WRITE_ERROR: Self = Self {
        ord: 11i32
    };
    pub const RESULT_REDIRECT_LIMIT_REACHED: Self = Self {
        ord: 12i32
    };
    pub const RESULT_TIMEOUT: Self = Self {
        ord: 13i32
    };
    
}
impl crate::obj::EngineEnum for Result {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Result {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Result {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Result {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}