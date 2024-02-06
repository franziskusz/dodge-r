#![doc = "Sidecar module for class [`Range`][crate::engine::Range].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Range` enums](https://docs.godotengine.org/en/stable/classes/class_range.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Range.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`IRange`][crate::engine::IRange]: virtual methods\n\n\nSee also [Godot docs for `Range`](https://docs.godotengine.org/en/stable/classes/class_range.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Range {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Range`][crate::engine::Range].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Range` methods](https://docs.godotengine.org/en/stable/classes/class_range.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRange: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
            unimplemented !()
        }
        fn value_changed(&mut self, new_value: f64,) {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::engine::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl Range {
        pub fn get_value(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_page(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_page", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_as_ratio(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_as_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value_no_signal(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_value_no_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, minimum: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (minimum,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, maximum: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (maximum,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, step: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (step,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_page(&mut self, pagesize: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (pagesize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_page", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_ratio(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_rounded_values(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_rounded_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_rounded_values(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_rounded_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exp_ratio(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exp_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ratio_exp(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ratio_exp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_greater(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_greater", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_greater_allowed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_greater_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_lesser(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_lesser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_lesser_allowed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_lesser_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn share(&mut self, with: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (with,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "share", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unshare(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unshare", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Range {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Range\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Range {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Range {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for Range {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Range {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Range {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Range {
        
    }
    impl crate::obj::ExportableObject for Range {
        
    }
    impl crate::obj::cap::GodotDefault for Range {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Range {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Range {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Range {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Range > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Control > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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