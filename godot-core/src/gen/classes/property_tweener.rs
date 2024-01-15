#![doc = "Sidecar module for class [`PropertyTweener`][crate::engine::PropertyTweener].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PropertyTweener` enums](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PropertyTweener.`\n\nInherits [`Tweener`][crate::engine::Tweener].\n\nRelated symbols:\n\n* [`IPropertyTweener`][crate::engine::IPropertyTweener]: virtual methods\n\n\nSee also [Godot docs for `PropertyTweener`](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PropertyTweener {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PropertyTweener`][crate::engine::PropertyTweener].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PropertyTweener` methods](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPropertyTweener: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PropertyTweener {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn from(&mut self, value: Variant,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >, Variant);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn from_current(&mut self,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "from_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_relative(&mut self,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "as_relative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trans(&mut self, trans: crate::engine::tween::TransitionType,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >, crate::engine::tween::TransitionType);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_trans", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ease(&mut self, ease: crate::engine::tween::EaseType,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >, crate::engine::tween::EaseType);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ease", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_delay(&mut self, delay: f64,) -> Option < Gd < crate::engine::PropertyTweener > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PropertyTweener > >;
            type CallSig = (Option < Gd < crate::engine::PropertyTweener > >, f64);
            let args = (delay,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_delay", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PropertyTweener {
        type Base = crate::engine::Tweener;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PropertyTweener\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PropertyTweener {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PropertyTweener {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Tweener > for PropertyTweener {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PropertyTweener {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PropertyTweener {
        
    }
    impl crate::obj::cap::GodotDefault for PropertyTweener {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PropertyTweener {
        type Target = crate::engine::Tweener;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PropertyTweener {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PropertyTweener {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PropertyTweener > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Tweener > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}