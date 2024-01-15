#![doc = "Sidecar module for class [`TlsOptions`][crate::engine::TlsOptions].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TLSOptions` enums](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TLSOptions.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`tls_options`][crate::engine::tls_options]: sidecar module with related enum/flag types\n* [`ITlsOptions`][crate::engine::ITlsOptions]: virtual methods\n\n\nSee also [Godot docs for `TLSOptions`](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TlsOptions {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TlsOptions`][crate::engine::TlsOptions].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TLSOptions` methods](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITlsOptions: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TlsOptions {
        pub(crate) fn client_full(trusted_chain: Gd < crate::engine::X509Certificate >, common_name_override: GString,) -> Option < Gd < crate::engine::TlsOptions > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TlsOptions > >;
            type CallSig = (Option < Gd < crate::engine::TlsOptions > >, Gd < crate::engine::X509Certificate >, GString);
            let args = (trusted_chain, common_name_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "client", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn client() -> Option < Gd < crate::engine::TlsOptions > > {
            Self::client_ex() . done()
        }
        #[inline]
        pub fn client_ex() -> ExClient {
            ExClient::new()
        }
        pub(crate) fn client_unsafe_full(trusted_chain: Gd < crate::engine::X509Certificate >,) -> Option < Gd < crate::engine::TlsOptions > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TlsOptions > >;
            type CallSig = (Option < Gd < crate::engine::TlsOptions > >, Gd < crate::engine::X509Certificate >);
            let args = (trusted_chain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "client_unsafe", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn client_unsafe() -> Option < Gd < crate::engine::TlsOptions > > {
            Self::client_unsafe_ex() . done()
        }
        #[inline]
        pub fn client_unsafe_ex() -> ExClientUnsafe {
            ExClientUnsafe::new()
        }
        pub fn server(key: Gd < crate::engine::CryptoKey >, certificate: Gd < crate::engine::X509Certificate >,) -> Option < Gd < crate::engine::TlsOptions > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TlsOptions > >;
            type CallSig = (Option < Gd < crate::engine::TlsOptions > >, Gd < crate::engine::CryptoKey >, Gd < crate::engine::X509Certificate >);
            let args = (key, certificate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "server", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for TlsOptions {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TLSOptions\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TlsOptions {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TlsOptions {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TlsOptions {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TlsOptions {
        
    }
    impl std::ops::Deref for TlsOptions {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TlsOptions {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TlsOptions {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TlsOptions > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_ex`][super::TlsOptions::client_ex]."]
#[must_use]
pub struct ExClient {
    trusted_chain: Gd < crate::engine::X509Certificate >, common_name_override: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExClient {
    fn new() -> Self {
        Self {
            trusted_chain: unimplemented !("see #156"), common_name_override: GString::from(""),
        }
    }
    #[inline]
    pub fn trusted_chain(self, value: Gd < crate::engine::X509Certificate >) -> Self {
        Self {
            trusted_chain: value, .. self
        }
    }
    #[inline]
    pub fn common_name_override(self, value: GString) -> Self {
        Self {
            common_name_override: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TlsOptions > > {
        re_export::TlsOptions::client_full(self.trusted_chain, self.common_name_override,)
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_unsafe_ex`][super::TlsOptions::client_unsafe_ex]."]
#[must_use]
pub struct ExClientUnsafe {
    trusted_chain: Gd < crate::engine::X509Certificate >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExClientUnsafe {
    fn new() -> Self {
        Self {
            trusted_chain: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn trusted_chain(self, value: Gd < crate::engine::X509Certificate >) -> Self {
        Self {
            trusted_chain: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TlsOptions > > {
        re_export::TlsOptions::client_unsafe_full(self.trusted_chain,)
    }
}