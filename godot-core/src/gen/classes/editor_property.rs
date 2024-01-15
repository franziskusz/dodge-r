#![doc = "Sidecar module for class [`EditorProperty`][crate::engine::EditorProperty].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorProperty` enums](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorProperty.`\n\nInherits [`Container`][crate::engine::Container].\n\nRelated symbols:\n\n* [`editor_property`][crate::engine::editor_property]: sidecar module with related enum/flag types\n* [`IEditorProperty`][crate::engine::IEditorProperty]: virtual methods\n\n\nSee also [Godot docs for `EditorProperty`](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorProperty {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorProperty`][crate::engine::EditorProperty].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorProperty` methods](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorProperty: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn update_property(&mut self,) {
            unimplemented !()
        }
        fn set_read_only(&mut self, read_only: bool,) {
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
    impl EditorProperty {
        pub fn set_label(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_label(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_read_only(&mut self, read_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (read_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_read_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_read_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_read_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checkable(&mut self, checkable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (checkable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checkable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checked(&mut self, checked: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (checked,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checked(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_warning(&mut self, draw_warning: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (draw_warning,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_warning(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_draw_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keying(&mut self, keying: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (keying,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keying(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_keying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deletable(&mut self, deletable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (deletable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deletable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deletable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_deletable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_property(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_object(&mut self,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_property(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_focusable(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_focusable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bottom_editor(&mut self, editor: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (editor,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bottom_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn emit_changed_full(&mut self, property: StringName, value: Variant, field: StringName, changing: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant, StringName, bool);
            let args = (property, value, field, changing,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "emit_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn emit_changed(&mut self, property: StringName, value: Variant,) {
            self.emit_changed_ex(property, value,) . done()
        }
        #[inline]
        pub fn emit_changed_ex(&mut self, property: StringName, value: Variant,) -> ExEmitChanged < '_ > {
            ExEmitChanged::new(self, property, value,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for EditorProperty {
        type Base = crate::engine::Container;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorProperty\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorProperty {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorProperty {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Container > for EditorProperty {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for EditorProperty {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for EditorProperty {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for EditorProperty {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorProperty {
        
    }
    impl crate::obj::ExportableObject for EditorProperty {
        
    }
    impl crate::obj::cap::GodotDefault for EditorProperty {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorProperty {
        type Target = crate::engine::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorProperty {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorProperty {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorProperty > for $Class {
                
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
#[doc = "Default-param extender for [`EditorProperty::emit_changed_ex`][super::EditorProperty::emit_changed_ex]."]
#[must_use]
pub struct ExEmitChanged < 'a > {
    surround_object: &'a mut re_export::EditorProperty, property: StringName, value: Variant, field: StringName, changing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEmitChanged < 'a > {
    fn new(surround_object: &'a mut re_export::EditorProperty, property: StringName, value: Variant,) -> Self {
        Self {
            surround_object, property, value, field: StringName::from(""), changing: false,
        }
    }
    #[inline]
    pub fn field(self, value: StringName) -> Self {
        Self {
            field: value, .. self
        }
    }
    #[inline]
    pub fn changing(self, value: bool) -> Self {
        Self {
            changing: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorProperty::emit_changed_full(self.surround_object, self.property, self.value, self.field, self.changing,)
    }
}