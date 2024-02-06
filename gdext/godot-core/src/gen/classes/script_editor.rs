#![doc = "Sidecar module for class [`ScriptEditor`][crate::engine::ScriptEditor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptEditor` enums](https://docs.godotengine.org/en/stable/classes/class_scripteditor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptEditor.`\n\nInherits [`PanelContainer`][crate::engine::PanelContainer].\n\nRelated symbols:\n\n* [`IScriptEditor`][crate::engine::IScriptEditor]: virtual methods\n\n\nSee also [Godot docs for `ScriptEditor`](https://docs.godotengine.org/en/stable/classes/class_scripteditor.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptEditor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ScriptEditor`][crate::engine::ScriptEditor].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScriptEditor` methods](https://docs.godotengine.org/en/stable/classes/class_scripteditor.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptEditor: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
            unimplemented !()
        }
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
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
    impl ScriptEditor {
        pub fn get_current_editor(&self,) -> Option < Gd < crate::engine::ScriptEditorBase > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ScriptEditorBase > >;
            type CallSig = (Option < Gd < crate::engine::ScriptEditorBase > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_open_script_editors(&self,) -> Array < Gd < crate::engine::ScriptEditorBase > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::ScriptEditorBase > > >;
            type CallSig = (Array < Gd < crate::engine::ScriptEditorBase > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_open_script_editors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_syntax_highlighter(&mut self, syntax_highlighter: Gd < crate::engine::EditorSyntaxHighlighter >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorSyntaxHighlighter >);
            let args = (syntax_highlighter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_syntax_highlighter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_syntax_highlighter(&mut self, syntax_highlighter: Gd < crate::engine::EditorSyntaxHighlighter >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorSyntaxHighlighter >);
            let args = (syntax_highlighter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unregister_syntax_highlighter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn goto_line(&mut self, line_number: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (line_number,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "goto_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_script(&mut self,) -> Option < Gd < crate::engine::Script > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Script > >;
            type CallSig = (Option < Gd < crate::engine::Script > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_open_scripts(&self,) -> Array < Gd < crate::engine::Script > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Script > > >;
            type CallSig = (Array < Gd < crate::engine::Script > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_open_scripts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn open_script_create_dialog(&mut self, base_name: GString, base_path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (base_name, base_path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_script_create_dialog", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ScriptEditor {
        type Base = crate::engine::PanelContainer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ScriptEditor\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for ScriptEditor {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ScriptEditor {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PanelContainer > for ScriptEditor {
        
    }
    impl crate::obj::Inherits < crate::engine::Container > for ScriptEditor {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for ScriptEditor {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for ScriptEditor {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ScriptEditor {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ScriptEditor {
        
    }
    impl crate::obj::ExportableObject for ScriptEditor {
        
    }
    impl std::ops::Deref for ScriptEditor {
        type Target = crate::engine::PanelContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptEditor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ScriptEditor {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ScriptEditor > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PanelContainer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Container > for $Class {
                
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