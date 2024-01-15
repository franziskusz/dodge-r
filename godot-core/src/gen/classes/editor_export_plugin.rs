#![doc = "Sidecar module for class [`EditorExportPlugin`][crate::engine::EditorExportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlugin.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IEditorExportPlugin`][crate::engine::IEditorExportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorExportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorExportPlugin`][crate::engine::EditorExportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorExportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorExportPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn export_file(&mut self, path: GString, type_: GString, features: PackedStringArray,) {
            unimplemented !()
        }
        fn export_begin(&mut self, features: PackedStringArray, is_debug: bool, path: GString, flags: u32,) {
            unimplemented !()
        }
        fn export_end(&mut self,) {
            unimplemented !()
        }
        fn begin_customize_resources(&self, platform: Gd < crate::engine::EditorExportPlatform >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        fn customize_resource(&mut self, resource: Gd < crate::engine::Resource >, path: GString,) -> Option < Gd < crate::engine::Resource > > {
            unimplemented !()
        }
        fn begin_customize_scenes(&self, platform: Gd < crate::engine::EditorExportPlatform >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        fn customize_scene(&mut self, scene: Gd < crate::engine::Node >, path: GString,) -> Option < Gd < crate::engine::Node > > {
            unimplemented !()
        }
        fn get_customization_configuration_hash(&self,) -> u64 {
            unimplemented !()
        }
        fn end_customize_scenes(&mut self,) {
            unimplemented !()
        }
        fn end_customize_resources(&mut self,) {
            unimplemented !()
        }
        fn get_export_options(&self, platform: Gd < crate::engine::EditorExportPlatform >,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn should_update_export_options(&self, platform: Gd < crate::engine::EditorExportPlatform >,) -> bool {
            unimplemented !()
        }
        fn get_export_option_warning(&self, platform: Gd < crate::engine::EditorExportPlatform >, option: GString,) -> GString {
            unimplemented !()
        }
        fn get_export_features(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_name(&self,) -> GString {
            unimplemented !()
        }
        fn supports_platform(&self, platform: Gd < crate::engine::EditorExportPlatform >,) -> bool {
            unimplemented !()
        }
        fn get_android_dependencies(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_dependencies_maven_repos(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_libraries(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_manifest_activity_element_contents(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> GString {
            unimplemented !()
        }
        fn get_android_manifest_application_element_contents(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> GString {
            unimplemented !()
        }
        fn get_android_manifest_element_contents(&self, platform: Gd < crate::engine::EditorExportPlatform >, debug: bool,) -> GString {
            unimplemented !()
        }
    }
    impl EditorExportPlugin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_shared_object(&mut self, path: GString, tags: PackedStringArray, target: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, PackedStringArray, GString);
            let args = (path, tags, target,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(12usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_shared_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_project_static_lib(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(13usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_project_static_lib", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_file(&mut self, path: GString, file: PackedByteArray, remap: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, PackedByteArray, bool);
            let args = (path, file, remap,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(14usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_framework(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(15usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_framework", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_embedded_framework(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(16usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_embedded_framework", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_plist_content(&mut self, plist_content: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (plist_content,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(17usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_plist_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_linker_flags(&mut self, flags: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(18usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_linker_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_bundle_file(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(19usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_bundle_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_cpp_code(&mut self, code: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(20usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_ios_cpp_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_macos_plugin_file(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(21usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_macos_plugin_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skip(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(22usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(23usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_option", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorExportPlugin {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorExportPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorExportPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorExportPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorExportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorExportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorExportPlugin {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorExportPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorExportPlugin > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}