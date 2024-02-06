#![doc = "Sidecar module for class [`EngineDebugger`][crate::engine::EngineDebugger].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EngineDebugger` enums](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EngineDebugger.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`engine_debugger`][crate::engine::engine_debugger]: sidecar module with related enum/flag types\n* [`IEngineDebugger`][crate::engine::IEngineDebugger]: virtual methods\n\n\nSee also [Godot docs for `EngineDebugger`](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EngineDebugger {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EngineDebugger`][crate::engine::EngineDebugger].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EngineDebugger` methods](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEngineDebugger: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EngineDebugger {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"EngineDebugger\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_active(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_profiler(&mut self, name: StringName, profiler: Gd < crate::engine::EngineProfiler >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::EngineProfiler >);
            let args = (name, profiler,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_profiler(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unregister_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_profiling(&mut self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_profiling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_profiler(&mut self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn profiler_add_frame_data(&mut self, name: StringName, data: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, VariantArray);
            let args = (name, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "profiler_add_frame_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn profiler_enable_full(&mut self, name: StringName, enable: bool, arguments: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool, VariantArray);
            let args = (name, enable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "profiler_enable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn profiler_enable(&mut self, name: StringName, enable: bool,) {
            self.profiler_enable_ex(name, enable,) . done()
        }
        #[inline]
        pub fn profiler_enable_ex(&mut self, name: StringName, enable: bool,) -> ExProfilerEnable < '_ > {
            ExProfilerEnable::new(self, name, enable,)
        }
        pub fn register_message_capture(&mut self, name: StringName, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Callable);
            let args = (name, callable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_message_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_message_capture(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unregister_message_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_capture(&mut self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn send_message(&mut self, message: GString, data: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, VariantArray);
            let args = (message, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send_message", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EngineDebugger {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EngineDebugger\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for EngineDebugger {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EngineDebugger {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for EngineDebugger {
        
    }
    impl std::ops::Deref for EngineDebugger {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EngineDebugger {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EngineDebugger {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EngineDebugger > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EngineDebugger::profiler_enable_ex`][super::EngineDebugger::profiler_enable_ex]."]
#[must_use]
pub struct ExProfilerEnable < 'a > {
    surround_object: &'a mut re_export::EngineDebugger, name: StringName, enable: bool, arguments: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExProfilerEnable < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger, name: StringName, enable: bool,) -> Self {
        Self {
            surround_object, name, enable, arguments: Array::new(),
        }
    }
    #[inline]
    pub fn arguments(self, value: VariantArray) -> Self {
        Self {
            arguments: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EngineDebugger::profiler_enable_full(self.surround_object, self.name, self.enable, self.arguments,)
    }
}