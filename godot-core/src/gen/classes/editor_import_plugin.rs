#![doc = "Sidecar module for class [`EditorImportPlugin`][crate::engine::EditorImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorImportPlugin.`\n\nInherits [`ResourceImporter`][crate::engine::ResourceImporter].\n\nRelated symbols:\n\n* [`editor_import_plugin`][crate::engine::editor_import_plugin]: sidecar module with related enum/flag types\n* [`IEditorImportPlugin`][crate::engine::IEditorImportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorImportPlugin`][crate::engine::EditorImportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorImportPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_importer_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_visible_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_preset_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_preset_name(&self, preset_index: i32,) -> GString {
            unimplemented !()
        }
        fn get_recognized_extensions(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_import_options(&self, path: GString, preset_index: i32,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_save_extension(&self,) -> GString {
            unimplemented !()
        }
        fn get_resource_type(&self,) -> GString {
            unimplemented !()
        }
        fn get_priority(&self,) -> f32 {
            unimplemented !()
        }
        fn get_import_order(&self,) -> i32 {
            unimplemented !()
        }
        fn get_option_visibility(&self, path: GString, option_name: StringName, options: Dictionary,) -> bool {
            unimplemented !()
        }
        fn import(&self, source_file: GString, save_path: GString, options: Dictionary, platform_variants: Array < GString >, gen_files: Array < GString >,) -> crate::engine::global::Error {
            unimplemented !()
        }
    }
    impl EditorImportPlugin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn append_import_external_resource_full(&mut self, path: GString, custom_options: Dictionary, custom_importer: GString, generator_parameters: Variant,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, Dictionary, GString, Variant);
            let args = (path, custom_options, custom_importer, generator_parameters,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(82usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_import_external_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn append_import_external_resource(&mut self, path: GString,) -> crate::engine::global::Error {
            self.append_import_external_resource_ex(path,) . done()
        }
        #[inline]
        pub fn append_import_external_resource_ex(&mut self, path: GString,) -> ExAppendImportExternalResource < '_ > {
            ExAppendImportExternalResource::new(self, path,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for EditorImportPlugin {
        type Base = crate::engine::ResourceImporter;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorImportPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorImportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorImportPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::ResourceImporter > for EditorImportPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorImportPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorImportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorImportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorImportPlugin {
        type Target = crate::engine::ResourceImporter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorImportPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorImportPlugin > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::ResourceImporter > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorImportPlugin::append_import_external_resource_ex`][super::EditorImportPlugin::append_import_external_resource_ex]."]
#[must_use]
pub struct ExAppendImportExternalResource < 'a > {
    surround_object: &'a mut re_export::EditorImportPlugin, path: GString, custom_options: Dictionary, custom_importer: GString, generator_parameters: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendImportExternalResource < 'a > {
    fn new(surround_object: &'a mut re_export::EditorImportPlugin, path: GString,) -> Self {
        Self {
            surround_object, path, custom_options: Dictionary::new(), custom_importer: GString::from(""), generator_parameters: Variant::nil(),
        }
    }
    #[inline]
    pub fn custom_options(self, value: Dictionary) -> Self {
        Self {
            custom_options: value, .. self
        }
    }
    #[inline]
    pub fn custom_importer(self, value: GString) -> Self {
        Self {
            custom_importer: value, .. self
        }
    }
    #[inline]
    pub fn generator_parameters(self, value: Variant) -> Self {
        Self {
            generator_parameters: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::EditorImportPlugin::append_import_external_resource_full(self.surround_object, self.path, self.custom_options, self.custom_importer, self.generator_parameters,)
    }
}