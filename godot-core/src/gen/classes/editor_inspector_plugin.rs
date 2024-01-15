#![doc = "Sidecar module for class [`EditorInspectorPlugin`][crate::engine::EditorInspectorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInspectorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInspectorPlugin.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_inspector_plugin`][crate::engine::editor_inspector_plugin]: sidecar module with related enum/flag types\n* [`IEditorInspectorPlugin`][crate::engine::IEditorInspectorPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorInspectorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInspectorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorInspectorPlugin`][crate::engine::EditorInspectorPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorInspectorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorInspectorPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn can_handle(&self, object: Gd < crate::engine::Object >,) -> bool {
            unimplemented !()
        }
        fn parse_begin(&mut self, object: Gd < crate::engine::Object >,) {
            unimplemented !()
        }
        fn parse_category(&mut self, object: Gd < crate::engine::Object >, category: GString,) {
            unimplemented !()
        }
        fn parse_group(&mut self, object: Gd < crate::engine::Object >, group: GString,) {
            unimplemented !()
        }
        fn parse_property(&mut self, object: Gd < crate::engine::Object >, type_: VariantType, name: GString, hint_type: crate::engine::global::PropertyHint, hint_string: GString, usage_flags: crate::engine::global::PropertyUsageFlags, wide: bool,) -> bool {
            unimplemented !()
        }
        fn parse_end(&mut self, object: Gd < crate::engine::Object >,) {
            unimplemented !()
        }
    }
    impl EditorInspectorPlugin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_custom_control(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(85usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_custom_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_property_editor_full(&mut self, property: GString, editor: Gd < crate::engine::Control >, add_to_end: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::Control >, bool);
            let args = (property, editor, add_to_end,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(86usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_property_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_property_editor(&mut self, property: GString, editor: Gd < crate::engine::Control >,) {
            self.add_property_editor_ex(property, editor,) . done()
        }
        #[inline]
        pub fn add_property_editor_ex(&mut self, property: GString, editor: Gd < crate::engine::Control >,) -> ExAddPropertyEditor < '_ > {
            ExAddPropertyEditor::new(self, property, editor,)
        }
        pub fn add_property_editor_for_multiple_properties(&mut self, label: GString, properties: PackedStringArray, editor: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, PackedStringArray, Gd < crate::engine::Control >);
            let args = (label, properties, editor,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(87usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_property_editor_for_multiple_properties", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorInspectorPlugin {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorInspectorPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInspectorPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorInspectorPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorInspectorPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorInspectorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorInspectorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorInspectorPlugin {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInspectorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorInspectorPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorInspectorPlugin > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorInspectorPlugin::add_property_editor_ex`][super::EditorInspectorPlugin::add_property_editor_ex]."]
#[must_use]
pub struct ExAddPropertyEditor < 'a > {
    surround_object: &'a mut re_export::EditorInspectorPlugin, property: GString, editor: Gd < crate::engine::Control >, add_to_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPropertyEditor < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInspectorPlugin, property: GString, editor: Gd < crate::engine::Control >,) -> Self {
        Self {
            surround_object, property, editor, add_to_end: false,
        }
    }
    #[inline]
    pub fn add_to_end(self, value: bool) -> Self {
        Self {
            add_to_end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInspectorPlugin::add_property_editor_full(self.surround_object, self.property, self.editor, self.add_to_end,)
    }
}