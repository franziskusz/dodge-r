#![doc = "Sidecar module for class [`Shape2D`][crate::engine::Shape2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Shape2D` enums](https://docs.godotengine.org/en/stable/classes/class_shape2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Shape2D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IShape2D`][crate::engine::IShape2D]: virtual methods\n\n\nSee also [Godot docs for `Shape2D`](https://docs.godotengine.org/en/stable/classes/class_shape2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Shape2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Shape2D`][crate::engine::Shape2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Shape2D` methods](https://docs.godotengine.org/en/stable/classes/class_shape2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IShape2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Shape2D {
        pub fn set_custom_solver_bias(&mut self, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_solver_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_solver_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_solver_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn collide(&mut self, local_xform: Transform2D, with_shape: Gd < crate::engine::Shape2D >, shape_xform: Transform2D,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Transform2D, Gd < crate::engine::Shape2D >, Transform2D);
            let args = (local_xform, with_shape, shape_xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "collide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn collide_with_motion(&mut self, local_xform: Transform2D, local_motion: Vector2, with_shape: Gd < crate::engine::Shape2D >, shape_xform: Transform2D, shape_motion: Vector2,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Transform2D, Vector2, Gd < crate::engine::Shape2D >, Transform2D, Vector2);
            let args = (local_xform, local_motion, with_shape, shape_xform, shape_motion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "collide_with_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn collide_and_get_contacts(&mut self, local_xform: Transform2D, with_shape: Gd < crate::engine::Shape2D >, shape_xform: Transform2D,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Transform2D, Gd < crate::engine::Shape2D >, Transform2D);
            let args = (local_xform, with_shape, shape_xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "collide_and_get_contacts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn collide_with_motion_and_get_contacts(&mut self, local_xform: Transform2D, local_motion: Vector2, with_shape: Gd < crate::engine::Shape2D >, shape_xform: Transform2D, shape_motion: Vector2,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Transform2D, Vector2, Gd < crate::engine::Shape2D >, Transform2D, Vector2);
            let args = (local_xform, local_motion, with_shape, shape_xform, shape_motion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "collide_with_motion_and_get_contacts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw(&mut self, canvas_item: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (canvas_item, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rect", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Shape2D {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Shape2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Shape2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Shape2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Shape2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Shape2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Shape2D {
        
    }
    impl crate::obj::ExportableObject for Shape2D {
        
    }
    impl std::ops::Deref for Shape2D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Shape2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Shape2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Shape2D > for $Class {
                
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