#![doc = "Sidecar module for class [`EditorSettings`][crate::engine::EditorSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorSettings` enums](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorSettings.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`editor_settings`][crate::engine::editor_settings]: sidecar module with related enum/flag types\n* [`IEditorSettings`][crate::engine::IEditorSettings]: virtual methods\n* [`EditorSettingsNotification`][crate::engine::notify::EditorSettingsNotification]: notification type\n\n\nSee also [Godot docs for `EditorSettings`](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorSettings`][crate::engine::EditorSettings].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorSettings` methods](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorSettings: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: EditorSettingsNotification) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    #[doc = "Notification type for class [`EditorSettings`][crate::engine::EditorSettings]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum EditorSettingsNotification {
        EditorSettingsChanged = 10000i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for EditorSettingsNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10000i32 => Self::EditorSettingsChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < EditorSettingsNotification > for i32 {
        fn from(notification: EditorSettingsNotification) -> i32 {
            match notification {
                EditorSettingsNotification::EditorSettingsChanged => 10000i32, EditorSettingsNotification::Postinitialize => 0i32, EditorSettingsNotification::Predelete => 1i32, EditorSettingsNotification::Unknown(int) => int,
            }
        }
    }
    impl EditorSettings {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn has_setting(&self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_setting(&mut self, name: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_setting(&self, name: GString,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase(&mut self, property: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (property,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_initial_value(&mut self, name: StringName, value: Variant, update_current: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant, bool);
            let args = (name, value, update_current,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_initial_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_property_info(&mut self, info: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (info,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_property_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_project_metadata(&mut self, section: GString, key: GString, data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, Variant);
            let args = (section, key, data,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_project_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_project_metadata_full(&self, section: GString, key: GString, default: Variant,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString, GString, Variant);
            let args = (section, key, default,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_project_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_project_metadata(&self, section: GString, key: GString,) -> Variant {
            self.get_project_metadata_ex(section, key,) . done()
        }
        #[inline]
        pub fn get_project_metadata_ex(&self, section: GString, key: GString,) -> ExGetProjectMetadata < '_ > {
            ExGetProjectMetadata::new(self, section, key,)
        }
        pub fn set_favorites(&mut self, dirs: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (dirs,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_favorites", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_favorites(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_favorites", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_recent_dirs(&mut self, dirs: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (dirs,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_recent_dirs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_recent_dirs(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_recent_dirs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_builtin_action_override(&mut self, name: GString, actions_list: Array < Gd < crate::engine::InputEvent > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Array < Gd < crate::engine::InputEvent > >);
            let args = (name, actions_list,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_builtin_action_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn check_changed_settings_in_group(&self, setting_prefix: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (setting_prefix,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "check_changed_settings_in_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_changed_settings(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_changed_settings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mark_setting_changed(&mut self, setting: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (setting,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mark_setting_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: EditorSettingsNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: EditorSettingsNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_EDITOR_SETTINGS_CHANGED: i32 = 10000i32;
        
    }
    impl crate::obj::GodotClass for EditorSettings {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorSettings\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorSettings {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for EditorSettings {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorSettings {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorSettings {
        
    }
    impl crate::obj::ExportableObject for EditorSettings {
        
    }
    impl crate::obj::cap::GodotDefault for EditorSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorSettings {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorSettings {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorSettings > for $Class {
                
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
#[doc = "Default-param extender for [`EditorSettings::get_project_metadata_ex`][super::EditorSettings::get_project_metadata_ex]."]
#[must_use]
pub struct ExGetProjectMetadata < 'a > {
    surround_object: &'a re_export::EditorSettings, section: GString, key: GString, default: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetProjectMetadata < 'a > {
    fn new(surround_object: &'a re_export::EditorSettings, section: GString, key: GString,) -> Self {
        Self {
            surround_object, section, key, default: Variant::nil(),
        }
    }
    #[inline]
    pub fn default(self, value: Variant) -> Self {
        Self {
            default: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::EditorSettings::get_project_metadata_full(self.surround_object, self.section, self.key, self.default,)
    }
}