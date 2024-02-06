#![doc = "Sidecar module for class [`PacketPeer`][crate::engine::PacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_packetpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PacketPeer.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`packet_peer`][crate::engine::packet_peer]: sidecar module with related enum/flag types\n* [`IPacketPeer`][crate::engine::IPacketPeer]: virtual methods\n\n\nSee also [Godot docs for `PacketPeer`](https://docs.godotengine.org/en/stable/classes/class_packetpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PacketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PacketPeer`][crate::engine::PacketPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PacketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_packetpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPacketPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PacketPeer {
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, bool);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex(&mut self,) -> ExGetVar < '_ > {
            ExGetVar::new(self,)
        }
        pub(crate) fn put_var_full(&mut self, var: Variant, full_objects: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Variant, bool);
            let args = (var, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn put_var(&mut self, var: Variant,) -> crate::engine::global::Error {
            self.put_var_ex(var,) . done()
        }
        #[inline]
        pub fn put_var_ex(&mut self, var: Variant,) -> ExPutVar < '_ > {
            ExPutVar::new(self, var,)
        }
        pub fn get_packet(&mut self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_packet(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_packet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_error(&self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_packet_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_packet_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_available_packet_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_encode_buffer_max_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_encode_buffer_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_encode_buffer_max_size(&mut self, max_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_encode_buffer_max_size", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PacketPeer {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PacketPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PacketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PacketPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PacketPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PacketPeer {
        
    }
    impl std::ops::Deref for PacketPeer {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PacketPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PacketPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PacketPeer::get_var_ex`][super::PacketPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    surround_object: &'a mut re_export::PacketPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeer,) -> Self {
        Self {
            surround_object, allow_objects: false,
        }
    }
    #[inline]
    pub fn allow_objects(self, value: bool) -> Self {
        Self {
            allow_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::PacketPeer::get_var_full(self.surround_object, self.allow_objects,)
    }
}
#[doc = "Default-param extender for [`PacketPeer::put_var_ex`][super::PacketPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'a > {
    surround_object: &'a mut re_export::PacketPeer, var: Variant, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPutVar < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeer, var: Variant,) -> Self {
        Self {
            surround_object, var, full_objects: false,
        }
    }
    #[inline]
    pub fn full_objects(self, value: bool) -> Self {
        Self {
            full_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::PacketPeer::put_var_full(self.surround_object, self.var, self.full_objects,)
    }
}