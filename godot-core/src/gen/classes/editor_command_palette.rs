#![doc = "Sidecar module for class [`EditorCommandPalette`][crate::engine::EditorCommandPalette].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorCommandPalette` enums](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorCommandPalette.`\n\nInherits [`ConfirmationDialog`][crate::engine::ConfirmationDialog].\n\nRelated symbols:\n\n* [`editor_command_palette`][crate::engine::editor_command_palette]: sidecar module with related enum/flag types\n* [`IEditorCommandPalette`][crate::engine::IEditorCommandPalette]: virtual methods\n\n\nSee also [Godot docs for `EditorCommandPalette`](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorCommandPalette {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorCommandPalette`][crate::engine::EditorCommandPalette].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorCommandPalette` methods](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorCommandPalette: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
            unimplemented !()
        }
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl EditorCommandPalette {
        pub(crate) fn add_command_full(&mut self, command_name: GString, key_name: GString, binded_callable: Callable, shortcut_text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, Callable, GString);
            let args = (command_name, key_name, binded_callable, shortcut_text,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(0usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_command", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_command(&mut self, command_name: GString, key_name: GString, binded_callable: Callable,) {
            self.add_command_ex(command_name, key_name, binded_callable,) . done()
        }
        #[inline]
        pub fn add_command_ex(&mut self, command_name: GString, key_name: GString, binded_callable: Callable,) -> ExAddCommand < '_ > {
            ExAddCommand::new(self, command_name, key_name, binded_callable,)
        }
        pub fn remove_command(&mut self, key_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (key_name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(1usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_command", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorCommandPalette {
        type Base = crate::engine::ConfirmationDialog;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorCommandPalette\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorCommandPalette {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorCommandPalette {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::ConfirmationDialog > for EditorCommandPalette {
        
    }
    impl crate::obj::Inherits < crate::engine::AcceptDialog > for EditorCommandPalette {
        
    }
    impl crate::obj::Inherits < crate::engine::Window > for EditorCommandPalette {
        
    }
    impl crate::obj::Inherits < crate::engine::Viewport > for EditorCommandPalette {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for EditorCommandPalette {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorCommandPalette {
        
    }
    impl crate::obj::ExportableObject for EditorCommandPalette {
        
    }
    impl crate::obj::cap::GodotDefault for EditorCommandPalette {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorCommandPalette {
        type Target = crate::engine::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorCommandPalette {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorCommandPalette {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorCommandPalette > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::ConfirmationDialog > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AcceptDialog > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Window > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Viewport > for $Class {
                
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
#[doc = "Default-param extender for [`EditorCommandPalette::add_command_ex`][super::EditorCommandPalette::add_command_ex]."]
#[must_use]
pub struct ExAddCommand < 'a > {
    surround_object: &'a mut re_export::EditorCommandPalette, command_name: GString, key_name: GString, binded_callable: Callable, shortcut_text: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCommand < 'a > {
    fn new(surround_object: &'a mut re_export::EditorCommandPalette, command_name: GString, key_name: GString, binded_callable: Callable,) -> Self {
        Self {
            surround_object, command_name, key_name, binded_callable, shortcut_text: GString::from("None"),
        }
    }
    #[inline]
    pub fn shortcut_text(self, value: GString) -> Self {
        Self {
            shortcut_text: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorCommandPalette::add_command_full(self.surround_object, self.command_name, self.key_name, self.binded_callable, self.shortcut_text,)
    }
}