#![doc = "Sidecar module for class [`EditorDebuggerSession`][crate::engine::EditorDebuggerSession].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorDebuggerSession` enums](https://docs.godotengine.org/en/stable/classes/class_editordebuggersession.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorDebuggerSession.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_debugger_session`][crate::engine::editor_debugger_session]: sidecar module with related enum/flag types\n* [`IEditorDebuggerSession`][crate::engine::IEditorDebuggerSession]: virtual methods\n\n\nSee also [Godot docs for `EditorDebuggerSession`](https://docs.godotengine.org/en/stable/classes/class_editordebuggersession.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorDebuggerSession {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorDebuggerSession`][crate::engine::EditorDebuggerSession].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorDebuggerSession` methods](https://docs.godotengine.org/en/stable/classes/class_editordebuggersession.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorDebuggerSession: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorDebuggerSession {
        pub(crate) fn send_message_full(&mut self, message: GString, data: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, VariantArray);
            let args = (message, data,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(4usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "send_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn send_message(&mut self, message: GString,) {
            self.send_message_ex(message,) . done()
        }
        #[inline]
        pub fn send_message_ex(&mut self, message: GString,) -> ExSendMessage < '_ > {
            ExSendMessage::new(self, message,)
        }
        pub(crate) fn toggle_profiler_full(&mut self, profiler: GString, enable: bool, data: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool, VariantArray);
            let args = (profiler, enable, data,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(5usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "toggle_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn toggle_profiler(&mut self, profiler: GString, enable: bool,) {
            self.toggle_profiler_ex(profiler, enable,) . done()
        }
        #[inline]
        pub fn toggle_profiler_ex(&mut self, profiler: GString, enable: bool,) -> ExToggleProfiler < '_ > {
            ExToggleProfiler::new(self, profiler, enable,)
        }
        pub fn is_breaked(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(6usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_breaked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debuggable(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(7usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_debuggable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_active(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(8usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_session_tab(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(9usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_session_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_session_tab(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(10usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_session_tab", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorDebuggerSession {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorDebuggerSession\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorDebuggerSession {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorDebuggerSession {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorDebuggerSession {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorDebuggerSession {
        
    }
    impl std::ops::Deref for EditorDebuggerSession {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorDebuggerSession {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorDebuggerSession {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorDebuggerSession > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorDebuggerSession::send_message_ex`][super::EditorDebuggerSession::send_message_ex]."]
#[must_use]
pub struct ExSendMessage < 'a > {
    surround_object: &'a mut re_export::EditorDebuggerSession, message: GString, data: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSendMessage < 'a > {
    fn new(surround_object: &'a mut re_export::EditorDebuggerSession, message: GString,) -> Self {
        Self {
            surround_object, message, data: Array::new(),
        }
    }
    #[inline]
    pub fn data(self, value: VariantArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorDebuggerSession::send_message_full(self.surround_object, self.message, self.data,)
    }
}
#[doc = "Default-param extender for [`EditorDebuggerSession::toggle_profiler_ex`][super::EditorDebuggerSession::toggle_profiler_ex]."]
#[must_use]
pub struct ExToggleProfiler < 'a > {
    surround_object: &'a mut re_export::EditorDebuggerSession, profiler: GString, enable: bool, data: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToggleProfiler < 'a > {
    fn new(surround_object: &'a mut re_export::EditorDebuggerSession, profiler: GString, enable: bool,) -> Self {
        Self {
            surround_object, profiler, enable, data: Array::new(),
        }
    }
    #[inline]
    pub fn data(self, value: VariantArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorDebuggerSession::toggle_profiler_full(self.surround_object, self.profiler, self.enable, self.data,)
    }
}