#![doc = "Sidecar module for class [`WorkerThreadPool`][crate::engine::WorkerThreadPool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WorkerThreadPool` enums](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WorkerThreadPool.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`worker_thread_pool`][crate::engine::worker_thread_pool]: sidecar module with related enum/flag types\n* [`IWorkerThreadPool`][crate::engine::IWorkerThreadPool]: virtual methods\n\n\nSee also [Godot docs for `WorkerThreadPool`](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WorkerThreadPool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WorkerThreadPool`][crate::engine::WorkerThreadPool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WorkerThreadPool` methods](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWorkerThreadPool: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WorkerThreadPool {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"WorkerThreadPool\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn add_task_full(&mut self, action: Callable, high_priority: bool, description: GString,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Callable, bool, GString);
            let args = (action, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_task", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_task(&mut self, action: Callable,) -> i64 {
            self.add_task_ex(action,) . done()
        }
        #[inline]
        pub fn add_task_ex(&mut self, action: Callable,) -> ExAddTask < '_ > {
            ExAddTask::new(self, action,)
        }
        pub fn is_task_completed(&self, task_id: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_task_completed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait_for_task_completion(&mut self, task_id: i64,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i64);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "wait_for_task_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_group_task_full(&mut self, action: Callable, elements: i32, tasks_needed: i32, high_priority: bool, description: GString,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Callable, i32, i32, bool, GString);
            let args = (action, elements, tasks_needed, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_group_task", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_group_task(&mut self, action: Callable, elements: i32,) -> i64 {
            self.add_group_task_ex(action, elements,) . done()
        }
        #[inline]
        pub fn add_group_task_ex(&mut self, action: Callable, elements: i32,) -> ExAddGroupTask < '_ > {
            ExAddGroupTask::new(self, action, elements,)
        }
        pub fn is_group_task_completed(&self, group_id: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_group_task_completed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_processed_element_count(&self, group_id: i64,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group_processed_element_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait_for_group_task_completion(&mut self, group_id: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "wait_for_group_task_completion", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WorkerThreadPool {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WorkerThreadPool\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WorkerThreadPool {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WorkerThreadPool {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for WorkerThreadPool {
        
    }
    impl std::ops::Deref for WorkerThreadPool {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WorkerThreadPool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WorkerThreadPool {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WorkerThreadPool > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_task_ex`][super::WorkerThreadPool::add_task_ex]."]
#[must_use]
pub struct ExAddTask < 'a > {
    surround_object: &'a mut re_export::WorkerThreadPool, action: Callable, high_priority: bool, description: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: Callable,) -> Self {
        Self {
            surround_object, action, high_priority: false, description: GString::from(""),
        }
    }
    #[inline]
    pub fn high_priority(self, value: bool) -> Self {
        Self {
            high_priority: value, .. self
        }
    }
    #[inline]
    pub fn description(self, value: GString) -> Self {
        Self {
            description: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::WorkerThreadPool::add_task_full(self.surround_object, self.action, self.high_priority, self.description,)
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_group_task_ex`][super::WorkerThreadPool::add_group_task_ex]."]
#[must_use]
pub struct ExAddGroupTask < 'a > {
    surround_object: &'a mut re_export::WorkerThreadPool, action: Callable, elements: i32, tasks_needed: i32, high_priority: bool, description: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddGroupTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: Callable, elements: i32,) -> Self {
        Self {
            surround_object, action, elements, tasks_needed: - 1i32, high_priority: false, description: GString::from(""),
        }
    }
    #[inline]
    pub fn tasks_needed(self, value: i32) -> Self {
        Self {
            tasks_needed: value, .. self
        }
    }
    #[inline]
    pub fn high_priority(self, value: bool) -> Self {
        Self {
            high_priority: value, .. self
        }
    }
    #[inline]
    pub fn description(self, value: GString) -> Self {
        Self {
            description: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::WorkerThreadPool::add_group_task_full(self.surround_object, self.action, self.elements, self.tasks_needed, self.high_priority, self.description,)
    }
}