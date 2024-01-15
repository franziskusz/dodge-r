#![doc = "Sidecar module for class [`XrServer`][crate::engine::XrServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRServer` enums](https://docs.godotengine.org/en/stable/classes/class_xrserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`xr_server`][crate::engine::xr_server]: sidecar module with related enum/flag types\n* [`IXrServer`][crate::engine::IXrServer]: virtual methods\n\n\nSee also [Godot docs for `XRServer`](https://docs.godotengine.org/en/stable/classes/class_xrserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrServer`][crate::engine::XrServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRServer` methods](https://docs.godotengine.org/en/stable/classes/class_xrserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"XRServer\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_world_scale(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_scale(&mut self, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_world_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_origin(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_origin(&mut self, world_origin: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform3D);
            let args = (world_origin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_world_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_frame(&self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_reference_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn center_on_hmd(&mut self, rotation_mode: crate::engine::xr_server::RotationMode, keep_height: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::xr_server::RotationMode, bool);
            let args = (rotation_mode, keep_height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "center_on_hmd", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hmd_transform(&mut self,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hmd_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_interface(&mut self, interface: Gd < crate::engine::XrInterface >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::XrInterface >);
            let args = (interface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interface_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_interface(&mut self, interface: Gd < crate::engine::XrInterface >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::XrInterface >);
            let args = (interface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interface(&self, idx: i32,) -> Option < Gd < crate::engine::XrInterface > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrInterface > >;
            type CallSig = (Option < Gd < crate::engine::XrInterface > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interfaces(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_interface(&self, name: GString,) -> Option < Gd < crate::engine::XrInterface > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrInterface > >;
            type CallSig = (Option < Gd < crate::engine::XrInterface > >, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tracker(&mut self, tracker: Gd < crate::engine::XrPositionalTracker >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::XrPositionalTracker >);
            let args = (tracker,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tracker(&mut self, tracker: Gd < crate::engine::XrPositionalTracker >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::XrPositionalTracker >);
            let args = (tracker,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trackers(&mut self, tracker_types: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (tracker_types,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_trackers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker(&self, tracker_name: StringName,) -> Option < Gd < crate::engine::XrPositionalTracker > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrPositionalTracker > >;
            type CallSig = (Option < Gd < crate::engine::XrPositionalTracker > >, StringName);
            let args = (tracker_name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primary_interface(&self,) -> Option < Gd < crate::engine::XrInterface > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrInterface > >;
            type CallSig = (Option < Gd < crate::engine::XrInterface > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_primary_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_primary_interface(&mut self, interface: Gd < crate::engine::XrInterface >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::XrInterface >);
            let args = (interface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_primary_interface", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrServer {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"XRServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for XrServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for XrServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for XrServer {
        
    }
    impl std::ops::Deref for XrServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_XrServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::XrServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackerType {
    ord: i32
}
impl TrackerType {
    pub const TRACKER_HEAD: Self = Self {
        ord: 1i32
    };
    pub const TRACKER_CONTROLLER: Self = Self {
        ord: 2i32
    };
    pub const TRACKER_BASESTATION: Self = Self {
        ord: 4i32
    };
    pub const TRACKER_ANCHOR: Self = Self {
        ord: 8i32
    };
    pub const TRACKER_ANY_KNOWN: Self = Self {
        ord: 127i32
    };
    pub const TRACKER_UNKNOWN: Self = Self {
        ord: 128i32
    };
    pub const TRACKER_ANY: Self = Self {
        ord: 255i32
    };
    
}
impl crate::obj::EngineEnum for TrackerType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 127i32 | ord @ 128i32 | ord @ 255i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TrackerType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TrackerType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TrackerType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RotationMode {
    ord: i32
}
impl RotationMode {
    pub const RESET_FULL_ROTATION: Self = Self {
        ord: 0i32
    };
    pub const RESET_BUT_KEEP_TILT: Self = Self {
        ord: 1i32
    };
    pub const DONT_RESET_ROTATION: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for RotationMode {
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
impl crate::builtin::meta::GodotConvert for RotationMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RotationMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RotationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}