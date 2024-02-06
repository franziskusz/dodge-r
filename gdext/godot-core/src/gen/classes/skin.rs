#![doc = "Sidecar module for class [`Skin`][crate::engine::Skin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Skin` enums](https://docs.godotengine.org/en/stable/classes/class_skin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Skin.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`ISkin`][crate::engine::ISkin]: virtual methods\n\n\nSee also [Godot docs for `Skin`](https://docs.godotengine.org/en/stable/classes/class_skin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Skin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Skin`][crate::engine::Skin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Skin` methods](https://docs.godotengine.org/en/stable/classes/class_skin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Skin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_bind_count(&mut self, bind_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bind_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bind_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bind_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bind_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_bind(&mut self, bone: i32, pose: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (bone, pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_bind", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_named_bind(&mut self, name: GString, pose: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Transform3D);
            let args = (name, pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_named_bind", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bind_pose(&mut self, bind_index: i32, pose: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (bind_index, pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bind_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bind_pose(&self, bind_index: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bind_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bind_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bind_name(&mut self, bind_index: i32, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bind_index, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bind_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bind_name(&self, bind_index: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bind_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bind_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bind_bone(&mut self, bind_index: i32, bone: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (bind_index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bind_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bind_bone(&self, bind_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (bind_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bind_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_binds(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_binds", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Skin {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Skin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Skin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Skin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Skin {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Skin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Skin {
        
    }
    impl crate::obj::ExportableObject for Skin {
        
    }
    impl crate::obj::cap::GodotDefault for Skin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Skin {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Skin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Skin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Skin > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}