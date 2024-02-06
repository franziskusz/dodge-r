#![doc = "Sidecar module for class [`InputMap`][crate::engine::InputMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputMap` enums](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputMap.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`input_map`][crate::engine::input_map]: sidecar module with related enum/flag types\n* [`IInputMap`][crate::engine::IInputMap]: virtual methods\n\n\nSee also [Godot docs for `InputMap`](https://docs.godotengine.org/en/stable/classes/class_inputmap.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputMap`][crate::engine::InputMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputMap` methods](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputMap: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputMap {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"InputMap\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_action(&self, action: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_actions(&mut self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_actions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_action_full(&mut self, action: StringName, deadzone: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f32);
            let args = (action, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_action(&mut self, action: StringName,) {
            self.add_action_ex(action,) . done()
        }
        #[inline]
        pub fn add_action_ex(&mut self, action: StringName,) -> ExAddAction < '_ > {
            ExAddAction::new(self, action,)
        }
        pub fn erase_action(&mut self, action: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_set_deadzone(&mut self, action: StringName, deadzone: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f32);
            let args = (action, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_set_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_get_deadzone(&mut self, action: StringName,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_get_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_add_event(&mut self, action: StringName, event: Gd < crate::engine::InputEvent >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::InputEvent >);
            let args = (action, event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_add_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_has_event(&mut self, action: StringName, event: Gd < crate::engine::InputEvent >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, Gd < crate::engine::InputEvent >);
            let args = (action, event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_has_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_erase_event(&mut self, action: StringName, event: Gd < crate::engine::InputEvent >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::InputEvent >);
            let args = (action, event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_erase_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_erase_events(&mut self, action: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_erase_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_get_events(&mut self, action: StringName,) -> Array < Gd < crate::engine::InputEvent > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::InputEvent > > >;
            type CallSig = (Array < Gd < crate::engine::InputEvent > >, StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_get_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn event_is_action_full(&self, event: Gd < crate::engine::InputEvent >, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::InputEvent >, StringName, bool);
            let args = (event, action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "event_is_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn event_is_action(&self, event: Gd < crate::engine::InputEvent >, action: StringName,) -> bool {
            self.event_is_action_ex(event, action,) . done()
        }
        #[inline]
        pub fn event_is_action_ex(&self, event: Gd < crate::engine::InputEvent >, action: StringName,) -> ExEventIsAction < '_ > {
            ExEventIsAction::new(self, event, action,)
        }
        pub fn load_from_project_settings(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_from_project_settings", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InputMap {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"InputMap\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for InputMap {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for InputMap {
        
    }
    impl std::ops::Deref for InputMap {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_InputMap {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::InputMap > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`InputMap::add_action_ex`][super::InputMap::add_action_ex]."]
#[must_use]
pub struct ExAddAction < 'a > {
    surround_object: &'a mut re_export::InputMap, action: StringName, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddAction < 'a > {
    fn new(surround_object: &'a mut re_export::InputMap, action: StringName,) -> Self {
        Self {
            surround_object, action, deadzone: 0.5f32,
        }
    }
    #[inline]
    pub fn deadzone(self, value: f32) -> Self {
        Self {
            deadzone: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::InputMap::add_action_full(self.surround_object, self.action, self.deadzone,)
    }
}
#[doc = "Default-param extender for [`InputMap::event_is_action_ex`][super::InputMap::event_is_action_ex]."]
#[must_use]
pub struct ExEventIsAction < 'a > {
    surround_object: &'a re_export::InputMap, event: Gd < crate::engine::InputEvent >, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEventIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputMap, event: Gd < crate::engine::InputEvent >, action: StringName,) -> Self {
        Self {
            surround_object, event, action, exact_match: false,
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
        re_export::InputMap::event_is_action_full(self.surround_object, self.event, self.action, self.exact_match,)
    }
}