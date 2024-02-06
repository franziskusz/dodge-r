#![doc = "Sidecar module for class [`WebRtcPeerConnection`][crate::engine::WebRtcPeerConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCPeerConnection` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCPeerConnection.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`web_rtc_peer_connection`][crate::engine::web_rtc_peer_connection]: sidecar module with related enum/flag types\n* [`IWebRtcPeerConnection`][crate::engine::IWebRtcPeerConnection]: virtual methods\n\n\nSee also [Godot docs for `WebRTCPeerConnection`](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcPeerConnection {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebRtcPeerConnection`][crate::engine::WebRtcPeerConnection].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebRTCPeerConnection` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcPeerConnection: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcPeerConnection {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_default_extension(extension_class: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (extension_class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_extension", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn initialize_full(&mut self, configuration: Dictionary,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Dictionary);
            let args = (configuration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "initialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn initialize(&mut self,) -> crate::engine::global::Error {
            self.initialize_ex() . done()
        }
        #[inline]
        pub fn initialize_ex(&mut self,) -> ExInitialize < '_ > {
            ExInitialize::new(self,)
        }
        pub(crate) fn create_data_channel_full(&mut self, label: GString, options: Dictionary,) -> Option < Gd < crate::engine::WebRtcDataChannel > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::WebRtcDataChannel > >;
            type CallSig = (Option < Gd < crate::engine::WebRtcDataChannel > >, GString, Dictionary);
            let args = (label, options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_data_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_data_channel(&mut self, label: GString,) -> Option < Gd < crate::engine::WebRtcDataChannel > > {
            self.create_data_channel_ex(label,) . done()
        }
        #[inline]
        pub fn create_data_channel_ex(&mut self, label: GString,) -> ExCreateDataChannel < '_ > {
            ExCreateDataChannel::new(self, label,)
        }
        pub fn create_offer(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_offer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_local_description(&mut self, type_: GString, sdp: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (type_, sdp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_local_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_remote_description(&mut self, type_: GString, sdp: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (type_, sdp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_remote_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ice_candidate(&mut self, media: GString, index: i32, name: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32, GString);
            let args = (media, index, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ice_candidate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_state(&self,) -> crate::engine::web_rtc_peer_connection::ConnectionState {
            type RetMarshal = PtrcallReturnT < crate::engine::web_rtc_peer_connection::ConnectionState >;
            type CallSig = (crate::engine::web_rtc_peer_connection::ConnectionState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connection_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gathering_state(&self,) -> crate::engine::web_rtc_peer_connection::GatheringState {
            type RetMarshal = PtrcallReturnT < crate::engine::web_rtc_peer_connection::GatheringState >;
            type CallSig = (crate::engine::web_rtc_peer_connection::GatheringState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gathering_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signaling_state(&self,) -> crate::engine::web_rtc_peer_connection::SignalingState {
            type RetMarshal = PtrcallReturnT < crate::engine::web_rtc_peer_connection::SignalingState >;
            type CallSig = (crate::engine::web_rtc_peer_connection::SignalingState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_signaling_state", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebRtcPeerConnection {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WebRTCPeerConnection\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcPeerConnection {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WebRtcPeerConnection {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for WebRtcPeerConnection {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for WebRtcPeerConnection {
        
    }
    impl crate::obj::cap::GodotDefault for WebRtcPeerConnection {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebRtcPeerConnection {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcPeerConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WebRtcPeerConnection {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WebRtcPeerConnection > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::initialize_ex`][super::WebRtcPeerConnection::initialize_ex]."]
#[must_use]
pub struct ExInitialize < 'a > {
    surround_object: &'a mut re_export::WebRtcPeerConnection, configuration: Dictionary,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInitialize < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcPeerConnection,) -> Self {
        Self {
            surround_object, configuration: Dictionary::new(),
        }
    }
    #[inline]
    pub fn configuration(self, value: Dictionary) -> Self {
        Self {
            configuration: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::WebRtcPeerConnection::initialize_full(self.surround_object, self.configuration,)
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::create_data_channel_ex`][super::WebRtcPeerConnection::create_data_channel_ex]."]
#[must_use]
pub struct ExCreateDataChannel < 'a > {
    surround_object: &'a mut re_export::WebRtcPeerConnection, label: GString, options: Dictionary,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateDataChannel < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcPeerConnection, label: GString,) -> Self {
        Self {
            surround_object, label, options: Dictionary::new(),
        }
    }
    #[inline]
    pub fn options(self, value: Dictionary) -> Self {
        Self {
            options: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::WebRtcDataChannel > > {
        re_export::WebRtcPeerConnection::create_data_channel_full(self.surround_object, self.label, self.options,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ConnectionState {
    ord: i32
}
impl ConnectionState {
    pub const STATE_NEW: Self = Self {
        ord: 0i32
    };
    pub const STATE_CONNECTING: Self = Self {
        ord: 1i32
    };
    pub const STATE_CONNECTED: Self = Self {
        ord: 2i32
    };
    pub const STATE_DISCONNECTED: Self = Self {
        ord: 3i32
    };
    pub const STATE_FAILED: Self = Self {
        ord: 4i32
    };
    pub const STATE_CLOSED: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for ConnectionState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ConnectionState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ConnectionState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ConnectionState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GatheringState {
    ord: i32
}
impl GatheringState {
    pub const GATHERING_STATE_NEW: Self = Self {
        ord: 0i32
    };
    pub const GATHERING_STATE_GATHERING: Self = Self {
        ord: 1i32
    };
    pub const GATHERING_STATE_COMPLETE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for GatheringState {
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
impl crate::builtin::meta::GodotConvert for GatheringState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GatheringState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GatheringState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignalingState {
    ord: i32
}
impl SignalingState {
    pub const SIGNALING_STATE_STABLE: Self = Self {
        ord: 0i32
    };
    pub const SIGNALING_STATE_HAVE_LOCAL_OFFER: Self = Self {
        ord: 1i32
    };
    pub const SIGNALING_STATE_HAVE_REMOTE_OFFER: Self = Self {
        ord: 2i32
    };
    pub const SIGNALING_STATE_HAVE_LOCAL_PRANSWER: Self = Self {
        ord: 3i32
    };
    pub const SIGNALING_STATE_HAVE_REMOTE_PRANSWER: Self = Self {
        ord: 4i32
    };
    pub const SIGNALING_STATE_CLOSED: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for SignalingState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SignalingState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SignalingState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SignalingState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}