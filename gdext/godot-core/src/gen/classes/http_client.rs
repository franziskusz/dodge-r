#![doc = "Sidecar module for class [`HttpClient`][crate::engine::HttpClient].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPClient` enums](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HTTPClient.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`http_client`][crate::engine::http_client]: sidecar module with related enum/flag types\n* [`IHttpClient`][crate::engine::IHttpClient]: virtual methods\n\n\nSee also [Godot docs for `HTTPClient`](https://docs.godotengine.org/en/stable/classes/class_httpclient.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HttpClient {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`HttpClient`][crate::engine::HttpClient].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `HTTPClient` methods](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHttpClient: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl HttpClient {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn connect_to_host_full(&mut self, host: GString, port: i32, tls_options: Gd < crate::engine::TlsOptions >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32, Gd < crate::engine::TlsOptions >);
            let args = (host, port, tls_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_to_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect_to_host(&mut self, host: GString,) -> crate::engine::global::Error {
            self.connect_to_host_ex(host,) . done()
        }
        #[inline]
        pub fn connect_to_host_ex(&mut self, host: GString,) -> ExConnectToHost < '_ > {
            ExConnectToHost::new(self, host,)
        }
        pub fn set_connection(&mut self, connection: Gd < crate::engine::StreamPeer >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::StreamPeer >);
            let args = (connection,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection(&self,) -> Option < Gd < crate::engine::StreamPeer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::StreamPeer > >;
            type CallSig = (Option < Gd < crate::engine::StreamPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_raw(&mut self, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray, body: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, crate::engine::http_client::Method, GString, PackedStringArray, PackedByteArray);
            let args = (method, url, headers, body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn request_full(&mut self, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray, body: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, crate::engine::http_client::Method, GString, PackedStringArray, GString);
            let args = (method, url, headers, body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn request(&mut self, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray,) -> crate::engine::global::Error {
            self.request_ex(method, url, headers,) . done()
        }
        #[inline]
        pub fn request_ex(&mut self, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray,) -> ExRequest < '_ > {
            ExRequest::new(self, method, url, headers,)
        }
        pub fn close(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_response(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_response", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_response_chunked(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_response_chunked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_code(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_response_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_headers(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_response_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_headers_as_dictionary(&mut self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_response_headers_as_dictionary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_body_length(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_response_body_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn read_response_body_chunk(&mut self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "read_response_body_chunk", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_read_chunk_size(&mut self, bytes: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_read_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_read_chunk_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_read_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blocking_mode(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blocking_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_blocking_mode_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_blocking_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_status(&self,) -> crate::engine::http_client::Status {
            type RetMarshal = PtrcallReturnT < crate::engine::http_client::Status >;
            type CallSig = (crate::engine::http_client::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_http_proxy(&mut self, host: GString, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_http_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_https_proxy(&mut self, host: GString, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (host, port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_https_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn query_string_from_dict(&mut self, fields: Dictionary,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Dictionary);
            let args = (fields,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "query_string_from_dict", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for HttpClient {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"HTTPClient\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HttpClient {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for HttpClient {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for HttpClient {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for HttpClient {
        
    }
    impl crate::obj::cap::GodotDefault for HttpClient {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HttpClient {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HttpClient {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_HttpClient {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::HttpClient > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`HttpClient::connect_to_host_ex`][super::HttpClient::connect_to_host_ex]."]
#[must_use]
pub struct ExConnectToHost < 'a > {
    surround_object: &'a mut re_export::HttpClient, host: GString, port: i32, tls_options: Gd < crate::engine::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToHost < 'a > {
    fn new(surround_object: &'a mut re_export::HttpClient, host: GString,) -> Self {
        Self {
            surround_object, host, port: - 1i32, tls_options: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn port(self, value: i32) -> Self {
        Self {
            port: value, .. self
        }
    }
    #[inline]
    pub fn tls_options(self, value: Gd < crate::engine::TlsOptions >) -> Self {
        Self {
            tls_options: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::HttpClient::connect_to_host_full(self.surround_object, self.host, self.port, self.tls_options,)
    }
}
#[doc = "Default-param extender for [`HttpClient::request_ex`][super::HttpClient::request_ex]."]
#[must_use]
pub struct ExRequest < 'a > {
    surround_object: &'a mut re_export::HttpClient, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray, body: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequest < 'a > {
    fn new(surround_object: &'a mut re_export::HttpClient, method: crate::engine::http_client::Method, url: GString, headers: PackedStringArray,) -> Self {
        Self {
            surround_object, method, url, headers, body: GString::from(""),
        }
    }
    #[inline]
    pub fn body(self, value: GString) -> Self {
        Self {
            body: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::HttpClient::request_full(self.surround_object, self.method, self.url, self.headers, self.body,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Method {
    ord: i32
}
impl Method {
    pub const METHOD_GET: Self = Self {
        ord: 0i32
    };
    pub const METHOD_HEAD: Self = Self {
        ord: 1i32
    };
    pub const METHOD_POST: Self = Self {
        ord: 2i32
    };
    pub const METHOD_PUT: Self = Self {
        ord: 3i32
    };
    pub const METHOD_DELETE: Self = Self {
        ord: 4i32
    };
    pub const METHOD_OPTIONS: Self = Self {
        ord: 5i32
    };
    pub const METHOD_TRACE: Self = Self {
        ord: 6i32
    };
    pub const METHOD_CONNECT: Self = Self {
        ord: 7i32
    };
    pub const METHOD_PATCH: Self = Self {
        ord: 8i32
    };
    pub const METHOD_MAX: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for Method {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Method {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::builtin::meta::GodotConvert for Method {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Method {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Method {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Status {
    ord: i32
}
impl Status {
    pub const STATUS_DISCONNECTED: Self = Self {
        ord: 0i32
    };
    pub const STATUS_RESOLVING: Self = Self {
        ord: 1i32
    };
    pub const STATUS_CANT_RESOLVE: Self = Self {
        ord: 2i32
    };
    pub const STATUS_CONNECTING: Self = Self {
        ord: 3i32
    };
    pub const STATUS_CANT_CONNECT: Self = Self {
        ord: 4i32
    };
    pub const STATUS_CONNECTED: Self = Self {
        ord: 5i32
    };
    pub const STATUS_REQUESTING: Self = Self {
        ord: 6i32
    };
    pub const STATUS_BODY: Self = Self {
        ord: 7i32
    };
    pub const STATUS_CONNECTION_ERROR: Self = Self {
        ord: 8i32
    };
    pub const STATUS_TLS_HANDSHAKE_ERROR: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for Status {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Status {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Status {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Status {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ResponseCode {
    ord: i32
}
impl ResponseCode {
    pub const RESPONSE_CONTINUE: Self = Self {
        ord: 100i32
    };
    pub const RESPONSE_SWITCHING_PROTOCOLS: Self = Self {
        ord: 101i32
    };
    pub const RESPONSE_PROCESSING: Self = Self {
        ord: 102i32
    };
    pub const RESPONSE_OK: Self = Self {
        ord: 200i32
    };
    pub const RESPONSE_CREATED: Self = Self {
        ord: 201i32
    };
    pub const RESPONSE_ACCEPTED: Self = Self {
        ord: 202i32
    };
    pub const RESPONSE_NON_AUTHORITATIVE_INFORMATION: Self = Self {
        ord: 203i32
    };
    pub const RESPONSE_NO_CONTENT: Self = Self {
        ord: 204i32
    };
    pub const RESPONSE_RESET_CONTENT: Self = Self {
        ord: 205i32
    };
    pub const RESPONSE_PARTIAL_CONTENT: Self = Self {
        ord: 206i32
    };
    pub const RESPONSE_MULTI_STATUS: Self = Self {
        ord: 207i32
    };
    pub const RESPONSE_ALREADY_REPORTED: Self = Self {
        ord: 208i32
    };
    pub const RESPONSE_IM_USED: Self = Self {
        ord: 226i32
    };
    pub const RESPONSE_MULTIPLE_CHOICES: Self = Self {
        ord: 300i32
    };
    pub const RESPONSE_MOVED_PERMANENTLY: Self = Self {
        ord: 301i32
    };
    pub const RESPONSE_FOUND: Self = Self {
        ord: 302i32
    };
    pub const RESPONSE_SEE_OTHER: Self = Self {
        ord: 303i32
    };
    pub const RESPONSE_NOT_MODIFIED: Self = Self {
        ord: 304i32
    };
    pub const RESPONSE_USE_PROXY: Self = Self {
        ord: 305i32
    };
    pub const RESPONSE_SWITCH_PROXY: Self = Self {
        ord: 306i32
    };
    pub const RESPONSE_TEMPORARY_REDIRECT: Self = Self {
        ord: 307i32
    };
    pub const RESPONSE_PERMANENT_REDIRECT: Self = Self {
        ord: 308i32
    };
    pub const RESPONSE_BAD_REQUEST: Self = Self {
        ord: 400i32
    };
    pub const RESPONSE_UNAUTHORIZED: Self = Self {
        ord: 401i32
    };
    pub const RESPONSE_PAYMENT_REQUIRED: Self = Self {
        ord: 402i32
    };
    pub const RESPONSE_FORBIDDEN: Self = Self {
        ord: 403i32
    };
    pub const RESPONSE_NOT_FOUND: Self = Self {
        ord: 404i32
    };
    pub const RESPONSE_METHOD_NOT_ALLOWED: Self = Self {
        ord: 405i32
    };
    pub const RESPONSE_NOT_ACCEPTABLE: Self = Self {
        ord: 406i32
    };
    pub const RESPONSE_PROXY_AUTHENTICATION_REQUIRED: Self = Self {
        ord: 407i32
    };
    pub const RESPONSE_REQUEST_TIMEOUT: Self = Self {
        ord: 408i32
    };
    pub const RESPONSE_CONFLICT: Self = Self {
        ord: 409i32
    };
    pub const RESPONSE_GONE: Self = Self {
        ord: 410i32
    };
    pub const RESPONSE_LENGTH_REQUIRED: Self = Self {
        ord: 411i32
    };
    pub const RESPONSE_PRECONDITION_FAILED: Self = Self {
        ord: 412i32
    };
    pub const RESPONSE_REQUEST_ENTITY_TOO_LARGE: Self = Self {
        ord: 413i32
    };
    pub const RESPONSE_REQUEST_URI_TOO_LONG: Self = Self {
        ord: 414i32
    };
    pub const RESPONSE_UNSUPPORTED_MEDIA_TYPE: Self = Self {
        ord: 415i32
    };
    pub const RESPONSE_REQUESTED_RANGE_NOT_SATISFIABLE: Self = Self {
        ord: 416i32
    };
    pub const RESPONSE_EXPECTATION_FAILED: Self = Self {
        ord: 417i32
    };
    pub const RESPONSE_IM_A_TEAPOT: Self = Self {
        ord: 418i32
    };
    pub const RESPONSE_MISDIRECTED_REQUEST: Self = Self {
        ord: 421i32
    };
    pub const RESPONSE_UNPROCESSABLE_ENTITY: Self = Self {
        ord: 422i32
    };
    pub const RESPONSE_LOCKED: Self = Self {
        ord: 423i32
    };
    pub const RESPONSE_FAILED_DEPENDENCY: Self = Self {
        ord: 424i32
    };
    pub const RESPONSE_UPGRADE_REQUIRED: Self = Self {
        ord: 426i32
    };
    pub const RESPONSE_PRECONDITION_REQUIRED: Self = Self {
        ord: 428i32
    };
    pub const RESPONSE_TOO_MANY_REQUESTS: Self = Self {
        ord: 429i32
    };
    pub const RESPONSE_REQUEST_HEADER_FIELDS_TOO_LARGE: Self = Self {
        ord: 431i32
    };
    pub const RESPONSE_UNAVAILABLE_FOR_LEGAL_REASONS: Self = Self {
        ord: 451i32
    };
    pub const RESPONSE_INTERNAL_SERVER_ERROR: Self = Self {
        ord: 500i32
    };
    pub const RESPONSE_NOT_IMPLEMENTED: Self = Self {
        ord: 501i32
    };
    pub const RESPONSE_BAD_GATEWAY: Self = Self {
        ord: 502i32
    };
    pub const RESPONSE_SERVICE_UNAVAILABLE: Self = Self {
        ord: 503i32
    };
    pub const RESPONSE_GATEWAY_TIMEOUT: Self = Self {
        ord: 504i32
    };
    pub const RESPONSE_HTTP_VERSION_NOT_SUPPORTED: Self = Self {
        ord: 505i32
    };
    pub const RESPONSE_VARIANT_ALSO_NEGOTIATES: Self = Self {
        ord: 506i32
    };
    pub const RESPONSE_INSUFFICIENT_STORAGE: Self = Self {
        ord: 507i32
    };
    pub const RESPONSE_LOOP_DETECTED: Self = Self {
        ord: 508i32
    };
    pub const RESPONSE_NOT_EXTENDED: Self = Self {
        ord: 510i32
    };
    pub const RESPONSE_NETWORK_AUTH_REQUIRED: Self = Self {
        ord: 511i32
    };
    
}
impl crate::obj::EngineEnum for ResponseCode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 100i32 | ord @ 101i32 | ord @ 102i32 | ord @ 200i32 | ord @ 201i32 | ord @ 202i32 | ord @ 203i32 | ord @ 204i32 | ord @ 205i32 | ord @ 206i32 | ord @ 207i32 | ord @ 208i32 | ord @ 226i32 | ord @ 300i32 | ord @ 301i32 | ord @ 302i32 | ord @ 303i32 | ord @ 304i32 | ord @ 305i32 | ord @ 306i32 | ord @ 307i32 | ord @ 308i32 | ord @ 400i32 | ord @ 401i32 | ord @ 402i32 | ord @ 403i32 | ord @ 404i32 | ord @ 405i32 | ord @ 406i32 | ord @ 407i32 | ord @ 408i32 | ord @ 409i32 | ord @ 410i32 | ord @ 411i32 | ord @ 412i32 | ord @ 413i32 | ord @ 414i32 | ord @ 415i32 | ord @ 416i32 | ord @ 417i32 | ord @ 418i32 | ord @ 421i32 | ord @ 422i32 | ord @ 423i32 | ord @ 424i32 | ord @ 426i32 | ord @ 428i32 | ord @ 429i32 | ord @ 431i32 | ord @ 451i32 | ord @ 500i32 | ord @ 501i32 | ord @ 502i32 | ord @ 503i32 | ord @ 504i32 | ord @ 505i32 | ord @ 506i32 | ord @ 507i32 | ord @ 508i32 | ord @ 510i32 | ord @ 511i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ResponseCode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ResponseCode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ResponseCode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}