#![doc = "Sidecar module for class [`Crypto`][crate::engine::Crypto].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Crypto` enums](https://docs.godotengine.org/en/stable/classes/class_crypto.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Crypto.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`crypto`][crate::engine::crypto]: sidecar module with related enum/flag types\n* [`ICrypto`][crate::engine::ICrypto]: virtual methods\n\n\nSee also [Godot docs for `Crypto`](https://docs.godotengine.org/en/stable/classes/class_crypto.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Crypto {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Crypto`][crate::engine::Crypto].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Crypto` methods](https://docs.godotengine.org/en/stable/classes/class_crypto.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICrypto: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Crypto {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn generate_random_bytes(&mut self, size: i32,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_random_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_rsa(&mut self, size: i32,) -> Option < Gd < crate::engine::CryptoKey > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CryptoKey > >;
            type CallSig = (Option < Gd < crate::engine::CryptoKey > >, i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_rsa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_self_signed_certificate_full(&mut self, key: Gd < crate::engine::CryptoKey >, issuer_name: GString, not_before: GString, not_after: GString,) -> Option < Gd < crate::engine::X509Certificate > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::X509Certificate > >;
            type CallSig = (Option < Gd < crate::engine::X509Certificate > >, Gd < crate::engine::CryptoKey >, GString, GString, GString);
            let args = (key, issuer_name, not_before, not_after,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_self_signed_certificate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn generate_self_signed_certificate(&mut self, key: Gd < crate::engine::CryptoKey >,) -> Option < Gd < crate::engine::X509Certificate > > {
            self.generate_self_signed_certificate_ex(key,) . done()
        }
        #[inline]
        pub fn generate_self_signed_certificate_ex(&mut self, key: Gd < crate::engine::CryptoKey >,) -> ExGenerateSelfSignedCertificate < '_ > {
            ExGenerateSelfSignedCertificate::new(self, key,)
        }
        pub fn sign(&mut self, hash_type: crate::engine::hashing_context::HashType, hash: PackedByteArray, key: Gd < crate::engine::CryptoKey >,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, crate::engine::hashing_context::HashType, PackedByteArray, Gd < crate::engine::CryptoKey >);
            let args = (hash_type, hash, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sign", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn verify(&mut self, hash_type: crate::engine::hashing_context::HashType, hash: PackedByteArray, signature: PackedByteArray, key: Gd < crate::engine::CryptoKey >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::hashing_context::HashType, PackedByteArray, PackedByteArray, Gd < crate::engine::CryptoKey >);
            let args = (hash_type, hash, signature, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "verify", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn encrypt(&mut self, key: Gd < crate::engine::CryptoKey >, plaintext: PackedByteArray,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Gd < crate::engine::CryptoKey >, PackedByteArray);
            let args = (key, plaintext,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "encrypt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decrypt(&mut self, key: Gd < crate::engine::CryptoKey >, ciphertext: PackedByteArray,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Gd < crate::engine::CryptoKey >, PackedByteArray);
            let args = (key, ciphertext,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decrypt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hmac_digest(&mut self, hash_type: crate::engine::hashing_context::HashType, key: PackedByteArray, msg: PackedByteArray,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, crate::engine::hashing_context::HashType, PackedByteArray, PackedByteArray);
            let args = (hash_type, key, msg,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hmac_digest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn constant_time_compare(&mut self, trusted: PackedByteArray, received: PackedByteArray,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, PackedByteArray, PackedByteArray);
            let args = (trusted, received,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "constant_time_compare", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Crypto {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Crypto\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Crypto {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Crypto {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Crypto {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Crypto {
        
    }
    impl crate::obj::cap::GodotDefault for Crypto {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Crypto {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Crypto {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Crypto {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Crypto > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Crypto::generate_self_signed_certificate_ex`][super::Crypto::generate_self_signed_certificate_ex]."]
#[must_use]
pub struct ExGenerateSelfSignedCertificate < 'a > {
    surround_object: &'a mut re_export::Crypto, key: Gd < crate::engine::CryptoKey >, issuer_name: GString, not_before: GString, not_after: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateSelfSignedCertificate < 'a > {
    fn new(surround_object: &'a mut re_export::Crypto, key: Gd < crate::engine::CryptoKey >,) -> Self {
        Self {
            surround_object, key, issuer_name: GString::from("CN=myserver,O=myorganisation,C=IT"), not_before: GString::from("20140101000000"), not_after: GString::from("20340101000000"),
        }
    }
    #[inline]
    pub fn issuer_name(self, value: GString) -> Self {
        Self {
            issuer_name: value, .. self
        }
    }
    #[inline]
    pub fn not_before(self, value: GString) -> Self {
        Self {
            not_before: value, .. self
        }
    }
    #[inline]
    pub fn not_after(self, value: GString) -> Self {
        Self {
            not_after: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::X509Certificate > > {
        re_export::Crypto::generate_self_signed_certificate_full(self.surround_object, self.key, self.issuer_name, self.not_before, self.not_after,)
    }
}