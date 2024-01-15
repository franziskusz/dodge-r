#![doc = "Sidecar module for class [`ScriptCreateDialog`][crate::engine::ScriptCreateDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptCreateDialog` enums](https://docs.godotengine.org/en/stable/classes/class_scriptcreatedialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptCreateDialog.`\n\nInherits [`ConfirmationDialog`][crate::engine::ConfirmationDialog].\n\nRelated symbols:\n\n* [`script_create_dialog`][crate::engine::script_create_dialog]: sidecar module with related enum/flag types\n* [`IScriptCreateDialog`][crate::engine::IScriptCreateDialog]: virtual methods\n\n\nSee also [Godot docs for `ScriptCreateDialog`](https://docs.godotengine.org/en/stable/classes/class_scriptcreatedialog.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptCreateDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ScriptCreateDialog`][crate::engine::ScriptCreateDialog].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScriptCreateDialog` methods](https://docs.godotengine.org/en/stable/classes/class_scriptcreatedialog.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptCreateDialog: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ScriptCreateDialog {
        pub(crate) fn config_full(&mut self, inherits: GString, path: GString, built_in_enabled: bool, load_enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, bool, bool);
            let args = (inherits, path, built_in_enabled, load_enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn config(&mut self, inherits: GString, path: GString,) {
            self.config_ex(inherits, path,) . done()
        }
        #[inline]
        pub fn config_ex(&mut self, inherits: GString, path: GString,) -> ExConfig < '_ > {
            ExConfig::new(self, inherits, path,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for ScriptCreateDialog {
        type Base = crate::engine::ConfirmationDialog;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ScriptCreateDialog\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for ScriptCreateDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ScriptCreateDialog {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::ConfirmationDialog > for ScriptCreateDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::AcceptDialog > for ScriptCreateDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Window > for ScriptCreateDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Viewport > for ScriptCreateDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ScriptCreateDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ScriptCreateDialog {
        
    }
    impl crate::obj::ExportableObject for ScriptCreateDialog {
        
    }
    impl crate::obj::cap::GodotDefault for ScriptCreateDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScriptCreateDialog {
        type Target = crate::engine::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptCreateDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ScriptCreateDialog {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ScriptCreateDialog > for $Class {
                
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
#[doc = "Default-param extender for [`ScriptCreateDialog::config_ex`][super::ScriptCreateDialog::config_ex]."]
#[must_use]
pub struct ExConfig < 'a > {
    surround_object: &'a mut re_export::ScriptCreateDialog, inherits: GString, path: GString, built_in_enabled: bool, load_enabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConfig < 'a > {
    fn new(surround_object: &'a mut re_export::ScriptCreateDialog, inherits: GString, path: GString,) -> Self {
        Self {
            surround_object, inherits, path, built_in_enabled: true, load_enabled: true,
        }
    }
    #[inline]
    pub fn built_in_enabled(self, value: bool) -> Self {
        Self {
            built_in_enabled: value, .. self
        }
    }
    #[inline]
    pub fn load_enabled(self, value: bool) -> Self {
        Self {
            load_enabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ScriptCreateDialog::config_full(self.surround_object, self.inherits, self.path, self.built_in_enabled, self.load_enabled,)
    }
}