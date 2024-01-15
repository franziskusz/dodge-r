#![doc = "Sidecar module for class [`OpenXrHand`][crate::engine::OpenXrHand].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRHand` enums](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRHand.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`open_xr_hand`][crate::engine::open_xr_hand]: sidecar module with related enum/flag types\n* [`IOpenXrHand`][crate::engine::IOpenXrHand]: virtual methods\n\n\nSee also [Godot docs for `OpenXRHand`](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrHand {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrHand`][crate::engine::OpenXrHand].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRHand` methods](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrHand: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
    impl OpenXrHand {
        pub fn set_hand(&mut self, hand: crate::engine::open_xr_hand::Hands,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::open_xr_hand::Hands);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand(&self,) -> crate::engine::open_xr_hand::Hands {
            type RetMarshal = PtrcallReturnT < crate::engine::open_xr_hand::Hands >;
            type CallSig = (crate::engine::open_xr_hand::Hands,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hand_skeleton(&mut self, hand_skeleton: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (hand_skeleton,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hand_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_skeleton(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_range(&mut self, motion_range: crate::engine::open_xr_hand::MotionRange,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::open_xr_hand::MotionRange);
            let args = (motion_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_range(&self,) -> crate::engine::open_xr_hand::MotionRange {
            type RetMarshal = PtrcallReturnT < crate::engine::open_xr_hand::MotionRange >;
            type CallSig = (crate::engine::open_xr_hand::MotionRange,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_range", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrHand {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OpenXRHand\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrHand {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OpenXrHand {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for OpenXrHand {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for OpenXrHand {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for OpenXrHand {
        
    }
    impl crate::obj::ExportableObject for OpenXrHand {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrHand {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrHand {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrHand {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OpenXrHand {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OpenXrHand > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Hands {
    ord: i32
}
impl Hands {
    pub const HAND_LEFT: Self = Self {
        ord: 0i32
    };
    pub const HAND_RIGHT: Self = Self {
        ord: 1i32
    };
    pub const HAND_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Hands {
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
impl crate::obj::IndexEnum for Hands {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for Hands {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Hands {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Hands {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MotionRange {
    ord: i32
}
impl MotionRange {
    pub const MOTION_RANGE_UNOBSTRUCTED: Self = Self {
        ord: 0i32
    };
    pub const MOTION_RANGE_CONFORM_TO_CONTROLLER: Self = Self {
        ord: 1i32
    };
    pub const MOTION_RANGE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for MotionRange {
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
impl crate::obj::IndexEnum for MotionRange {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for MotionRange {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MotionRange {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MotionRange {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}