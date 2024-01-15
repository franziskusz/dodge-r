#![doc = "Sidecar module for class [`GltfDocumentExtensionConvertImporterMesh`][crate::engine::GltfDocumentExtensionConvertImporterMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocumentExtensionConvertImporterMesh.`\n\nInherits [`GltfDocumentExtension`][crate::engine::GltfDocumentExtension].\n\nRelated symbols:\n\n* [`IGltfDocumentExtensionConvertImporterMesh`][crate::engine::IGltfDocumentExtensionConvertImporterMesh]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh`](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocumentExtensionConvertImporterMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfDocumentExtensionConvertImporterMesh`][crate::engine::GltfDocumentExtensionConvertImporterMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocumentExtensionConvertImporterMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn import_preflight(&mut self, state: Gd < crate::engine::GltfState >, extensions: PackedStringArray,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn get_supported_extensions(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn parse_node_extensions(&mut self, state: Gd < crate::engine::GltfState >, gltf_node: Gd < crate::engine::GltfNode >, extensions: Dictionary,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn parse_image_data(&mut self, state: Gd < crate::engine::GltfState >, image_data: PackedByteArray, mime_type: GString, ret_image: Gd < crate::engine::Image >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn get_image_file_extension(&mut self,) -> GString {
            unimplemented !()
        }
        fn parse_texture_json(&mut self, state: Gd < crate::engine::GltfState >, texture_json: Dictionary, ret_gltf_texture: Gd < crate::engine::GltfTexture >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn generate_scene_node(&mut self, state: Gd < crate::engine::GltfState >, gltf_node: Gd < crate::engine::GltfNode >, scene_parent: Gd < crate::engine::Node >,) -> Option < Gd < crate::engine::Node3D > > {
            unimplemented !()
        }
        fn import_post_parse(&mut self, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn import_node(&mut self, state: Gd < crate::engine::GltfState >, gltf_node: Gd < crate::engine::GltfNode >, json: Dictionary, node: Gd < crate::engine::Node >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn import_post(&mut self, state: Gd < crate::engine::GltfState >, root: Gd < crate::engine::Node >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn export_preflight(&mut self, state: Gd < crate::engine::GltfState >, root: Gd < crate::engine::Node >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn convert_scene_node(&mut self, state: Gd < crate::engine::GltfState >, gltf_node: Gd < crate::engine::GltfNode >, scene_node: Gd < crate::engine::Node >,) {
            unimplemented !()
        }
        fn export_preserialize(&mut self, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn get_saveable_image_formats(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn serialize_image_to_bytes(&mut self, state: Gd < crate::engine::GltfState >, image: Gd < crate::engine::Image >, image_dict: Dictionary, image_format: GString, lossy_quality: f32,) -> PackedByteArray {
            unimplemented !()
        }
        fn save_image_at_path(&mut self, state: Gd < crate::engine::GltfState >, image: Gd < crate::engine::Image >, file_path: GString, image_format: GString, lossy_quality: f32,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn serialize_texture_json(&mut self, state: Gd < crate::engine::GltfState >, texture_json: Dictionary, gltf_texture: Gd < crate::engine::GltfTexture >, image_format: GString,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn export_node(&mut self, state: Gd < crate::engine::GltfState >, gltf_node: Gd < crate::engine::GltfNode >, json: Dictionary, node: Gd < crate::engine::Node >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn export_post(&mut self, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl GltfDocumentExtensionConvertImporterMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for GltfDocumentExtensionConvertImporterMesh {
        type Base = crate::engine::GltfDocumentExtension;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFDocumentExtensionConvertImporterMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocumentExtensionConvertImporterMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfDocumentExtensionConvertImporterMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GltfDocumentExtension > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::ExportableObject for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocumentExtensionConvertImporterMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocumentExtensionConvertImporterMesh {
        type Target = crate::engine::GltfDocumentExtension;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocumentExtensionConvertImporterMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfDocumentExtensionConvertImporterMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfDocumentExtensionConvertImporterMesh > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::GltfDocumentExtension > for $Class {
                
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