#![doc = "Sidecar module for class [`InputEvent`][crate::engine::InputEvent].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEvent` enums](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEvent.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`input_event`][crate::engine::input_event]: sidecar module with related enum/flag types\n* [`IInputEvent`][crate::engine::IInputEvent]: virtual methods\n\n\nSee also [Godot docs for `InputEvent`](https://docs.godotengine.org/en/stable/classes/class_inputevent.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEvent {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputEvent`][crate::engine::InputEvent].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputEvent` methods](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEvent: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEvent {
        pub fn set_device(&mut self, device: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_action_full(&self, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action(&self, action: StringName,) -> bool {
            self.is_action_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_ex(&self, action: StringName,) -> ExIsAction < '_ > {
            ExIsAction::new(self, action,)
        }
        pub(crate) fn is_action_pressed_full(&self, action: StringName, allow_echo: bool, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool, bool);
            let args = (action, allow_echo, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action_pressed(&self, action: StringName,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_pressed_ex(&self, action: StringName,) -> ExIsActionPressed < '_ > {
            ExIsActionPressed::new(self, action,)
        }
        pub(crate) fn is_action_released_full(&self, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action_released(&self, action: StringName,) -> bool {
            self.is_action_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_released_ex(&self, action: StringName,) -> ExIsActionReleased < '_ > {
            ExIsActionReleased::new(self, action,)
        }
        pub(crate) fn get_action_strength_full(&self, action: StringName, exact_match: bool,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_action_strength(&self, action: StringName,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_strength_ex(&self, action: StringName,) -> ExGetActionStrength < '_ > {
            ExGetActionStrength::new(self, action,)
        }
        pub fn is_canceled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_canceled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_released(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_echo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_echo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "as_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_match_full(&self, event: Gd < crate::engine::InputEvent >, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::InputEvent >, bool);
            let args = (event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_match", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_match(&self, event: Gd < crate::engine::InputEvent >,) -> bool {
            self.is_match_ex(event,) . done()
        }
        #[inline]
        pub fn is_match_ex(&self, event: Gd < crate::engine::InputEvent >,) -> ExIsMatch < '_ > {
            ExIsMatch::new(self, event,)
        }
        pub fn is_action_type(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn accumulate(&mut self, with_event: Gd < crate::engine::InputEvent >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::InputEvent >);
            let args = (with_event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "accumulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn xformed_by_full(&self, xform: Transform2D, local_ofs: Vector2,) -> Option < Gd < crate::engine::InputEvent > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::InputEvent > >;
            type CallSig = (Option < Gd < crate::engine::InputEvent > >, Transform2D, Vector2);
            let args = (xform, local_ofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "xformed_by", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn xformed_by(&self, xform: Transform2D,) -> Option < Gd < crate::engine::InputEvent > > {
            self.xformed_by_ex(xform,) . done()
        }
        #[inline]
        pub fn xformed_by_ex(&self, xform: Transform2D,) -> ExXformedBy < '_ > {
            ExXformedBy::new(self, xform,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for InputEvent {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"InputEvent\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEvent {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for InputEvent {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for InputEvent {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for InputEvent {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for InputEvent {
        
    }
    impl crate::obj::ExportableObject for InputEvent {
        
    }
    impl std::ops::Deref for InputEvent {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEvent {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_InputEvent {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::InputEvent > for $Class {
                
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
#[doc = "Default-param extender for [`InputEvent::is_action_ex`][super::InputEvent::is_action_ex]."]
#[must_use]
pub struct ExIsAction < 'a > {
    surround_object: &'a re_export::InputEvent, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::InputEvent::is_action_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_pressed_ex`][super::InputEvent::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    surround_object: &'a re_export::InputEvent, action: StringName, allow_echo: bool, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: StringName,) -> Self {
        Self {
            surround_object, action, allow_echo: false, exact_match: false,
        }
    }
    #[inline]
    pub fn allow_echo(self, value: bool) -> Self {
        Self {
            allow_echo: value, .. self
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::InputEvent::is_action_pressed_full(self.surround_object, self.action, self.allow_echo, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_released_ex`][super::InputEvent::is_action_released_ex]."]
#[must_use]
pub struct ExIsActionReleased < 'a > {
    surround_object: &'a re_export::InputEvent, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionReleased < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::InputEvent::is_action_released_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::get_action_strength_ex`][super::InputEvent::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    surround_object: &'a re_export::InputEvent, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::InputEvent::get_action_strength_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_match_ex`][super::InputEvent::is_match_ex]."]
#[must_use]
pub struct ExIsMatch < 'a > {
    surround_object: &'a re_export::InputEvent, event: Gd < crate::engine::InputEvent >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsMatch < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, event: Gd < crate::engine::InputEvent >,) -> Self {
        Self {
            surround_object, event, exact_match: true,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::InputEvent::is_match_full(self.surround_object, self.event, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::xformed_by_ex`][super::InputEvent::xformed_by_ex]."]
#[must_use]
pub struct ExXformedBy < 'a > {
    surround_object: &'a re_export::InputEvent, xform: Transform2D, local_ofs: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExXformedBy < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, xform: Transform2D,) -> Self {
        Self {
            surround_object, xform, local_ofs: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn local_ofs(self, value: Vector2) -> Self {
        Self {
            local_ofs: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::InputEvent > > {
        re_export::InputEvent::xformed_by_full(self.surround_object, self.xform, self.local_ofs,)
    }
}