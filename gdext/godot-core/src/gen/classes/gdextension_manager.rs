#![doc = "Sidecar module for class [`GDExtensionManager`][crate::engine::GDExtensionManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GDExtensionManager` enums](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GDExtensionManager.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`gdextension_manager`][crate::engine::gdextension_manager]: sidecar module with related enum/flag types\n* [`IGDExtensionManager`][crate::engine::IGDExtensionManager]: virtual methods\n\n\nSee also [Godot docs for `GDExtensionManager`](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GDExtensionManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GDExtensionManager`][crate::engine::GDExtensionManager].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GDExtensionManager` methods](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGDExtensionManager: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GDExtensionManager {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"GDExtensionManager\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn load_extension(&mut self, path: GString,) -> crate::engine::gdextension_manager::LoadStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::gdextension_manager::LoadStatus >;
            type CallSig = (crate::engine::gdextension_manager::LoadStatus, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_extension(&mut self, path: GString,) -> crate::engine::gdextension_manager::LoadStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::gdextension_manager::LoadStatus >;
            type CallSig = (crate::engine::gdextension_manager::LoadStatus, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reload_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unload_extension(&mut self, path: GString,) -> crate::engine::gdextension_manager::LoadStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::gdextension_manager::LoadStatus >;
            type CallSig = (crate::engine::gdextension_manager::LoadStatus, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unload_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_extension_loaded(&self, path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_extension_loaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loaded_extensions(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loaded_extensions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extension(&mut self, path: GString,) -> Option < Gd < crate::engine::GDExtension > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::GDExtension > >;
            type CallSig = (Option < Gd < crate::engine::GDExtension > >, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_extension", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GDExtensionManager {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GDExtensionManager\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GDExtensionManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GDExtensionManager {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for GDExtensionManager {
        
    }
    impl std::ops::Deref for GDExtensionManager {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GDExtensionManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GDExtensionManager {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GDExtensionManager > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LoadStatus {
    ord: i32
}
impl LoadStatus {
    pub const LOAD_STATUS_OK: Self = Self {
        ord: 0i32
    };
    pub const LOAD_STATUS_FAILED: Self = Self {
        ord: 1i32
    };
    pub const LOAD_STATUS_ALREADY_LOADED: Self = Self {
        ord: 2i32
    };
    pub const LOAD_STATUS_NOT_LOADED: Self = Self {
        ord: 3i32
    };
    pub const LOAD_STATUS_NEEDS_RESTART: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for LoadStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for LoadStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LoadStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}