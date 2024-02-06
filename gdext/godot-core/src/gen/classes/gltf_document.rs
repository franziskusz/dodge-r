#![doc = "Sidecar module for class [`GltfDocument`][crate::engine::GltfDocument].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocument` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocument.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`gltf_document`][crate::engine::gltf_document]: sidecar module with related enum/flag types\n* [`IGltfDocument`][crate::engine::IGltfDocument]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocument`](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocument {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfDocument`][crate::engine::GltfDocument].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFDocument` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocument: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl GltfDocument {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn append_from_file_full(&mut self, path: GString, state: Gd < crate::engine::GltfState >, flags: u32, base_path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, Gd < crate::engine::GltfState >, u32, GString);
            let args = (path, state, flags, base_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_from_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn append_from_file(&mut self, path: GString, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            self.append_from_file_ex(path, state,) . done()
        }
        #[inline]
        pub fn append_from_file_ex(&mut self, path: GString, state: Gd < crate::engine::GltfState >,) -> ExAppendFromFile < '_ > {
            ExAppendFromFile::new(self, path, state,)
        }
        pub(crate) fn append_from_buffer_full(&mut self, bytes: PackedByteArray, base_path: GString, state: Gd < crate::engine::GltfState >, flags: u32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray, GString, Gd < crate::engine::GltfState >, u32);
            let args = (bytes, base_path, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn append_from_buffer(&mut self, bytes: PackedByteArray, base_path: GString, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            self.append_from_buffer_ex(bytes, base_path, state,) . done()
        }
        #[inline]
        pub fn append_from_buffer_ex(&mut self, bytes: PackedByteArray, base_path: GString, state: Gd < crate::engine::GltfState >,) -> ExAppendFromBuffer < '_ > {
            ExAppendFromBuffer::new(self, bytes, base_path, state,)
        }
        pub(crate) fn append_from_scene_full(&mut self, node: Gd < crate::engine::Node >, state: Gd < crate::engine::GltfState >, flags: u32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Node >, Gd < crate::engine::GltfState >, u32);
            let args = (node, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_from_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn append_from_scene(&mut self, node: Gd < crate::engine::Node >, state: Gd < crate::engine::GltfState >,) -> crate::engine::global::Error {
            self.append_from_scene_ex(node, state,) . done()
        }
        #[inline]
        pub fn append_from_scene_ex(&mut self, node: Gd < crate::engine::Node >, state: Gd < crate::engine::GltfState >,) -> ExAppendFromScene < '_ > {
            ExAppendFromScene::new(self, node, state,)
        }
        pub(crate) fn generate_scene_full(&mut self, state: Gd < crate::engine::GltfState >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, Gd < crate::engine::GltfState >, f32, bool, bool);
            let args = (state, bake_fps, trimming, remove_immutable_tracks,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn generate_scene(&mut self, state: Gd < crate::engine::GltfState >,) -> Option < Gd < crate::engine::Node > > {
            self.generate_scene_ex(state,) . done()
        }
        #[inline]
        pub fn generate_scene_ex(&mut self, state: Gd < crate::engine::GltfState >,) -> ExGenerateScene < '_ > {
            ExGenerateScene::new(self, state,)
        }
        pub fn generate_buffer(&mut self, state: Gd < crate::engine::GltfState >,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Gd < crate::engine::GltfState >);
            let args = (state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn write_to_filesystem(&mut self, state: Gd < crate::engine::GltfState >, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::GltfState >, GString);
            let args = (state, path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "write_to_filesystem", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_image_format(&mut self, image_format: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (image_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_image_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_image_format(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_image_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lossy_quality(&mut self, lossy_quality: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (lossy_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lossy_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lossy_quality(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lossy_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_node_mode(&mut self, root_node_mode: crate::engine::gltf_document::RootNodeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gltf_document::RootNodeMode);
            let args = (root_node_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_node_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_node_mode(&self,) -> crate::engine::gltf_document::RootNodeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::gltf_document::RootNodeMode >;
            type CallSig = (crate::engine::gltf_document::RootNodeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_node_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn register_gltf_document_extension_full(extension: Gd < crate::engine::GltfDocumentExtension >, first_priority: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::GltfDocumentExtension >, bool);
            let args = (extension, first_priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_gltf_document_extension", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn register_gltf_document_extension(extension: Gd < crate::engine::GltfDocumentExtension >,) {
            Self::register_gltf_document_extension_ex(extension,) . done()
        }
        #[inline]
        pub fn register_gltf_document_extension_ex(extension: Gd < crate::engine::GltfDocumentExtension >,) -> ExRegisterGltfDocumentExtension {
            ExRegisterGltfDocumentExtension::new(extension,)
        }
        pub fn unregister_gltf_document_extension(extension: Gd < crate::engine::GltfDocumentExtension >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::GltfDocumentExtension >);
            let args = (extension,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unregister_gltf_document_extension", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for GltfDocument {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFDocument\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocument {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfDocument {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfDocument {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfDocument {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfDocument {
        
    }
    impl crate::obj::ExportableObject for GltfDocument {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocument {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocument {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocument {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfDocument {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfDocument > for $Class {
                
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
#[doc = "Default-param extender for [`GltfDocument::append_from_file_ex`][super::GltfDocument::append_from_file_ex]."]
#[must_use]
pub struct ExAppendFromFile < 'a > {
    surround_object: &'a mut re_export::GltfDocument, path: GString, state: Gd < crate::engine::GltfState >, flags: u32, base_path: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromFile < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, path: GString, state: Gd < crate::engine::GltfState >,) -> Self {
        Self {
            surround_object, path, state, flags: 0u32, base_path: GString::from(""),
        }
    }
    #[inline]
    pub fn flags(self, value: u32) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn base_path(self, value: GString) -> Self {
        Self {
            base_path: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::GltfDocument::append_from_file_full(self.surround_object, self.path, self.state, self.flags, self.base_path,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_buffer_ex`][super::GltfDocument::append_from_buffer_ex]."]
#[must_use]
pub struct ExAppendFromBuffer < 'a > {
    surround_object: &'a mut re_export::GltfDocument, bytes: PackedByteArray, base_path: GString, state: Gd < crate::engine::GltfState >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromBuffer < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, bytes: PackedByteArray, base_path: GString, state: Gd < crate::engine::GltfState >,) -> Self {
        Self {
            surround_object, bytes, base_path, state, flags: 0u32,
        }
    }
    #[inline]
    pub fn flags(self, value: u32) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::GltfDocument::append_from_buffer_full(self.surround_object, self.bytes, self.base_path, self.state, self.flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_scene_ex`][super::GltfDocument::append_from_scene_ex]."]
#[must_use]
pub struct ExAppendFromScene < 'a > {
    surround_object: &'a mut re_export::GltfDocument, node: Gd < crate::engine::Node >, state: Gd < crate::engine::GltfState >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromScene < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, node: Gd < crate::engine::Node >, state: Gd < crate::engine::GltfState >,) -> Self {
        Self {
            surround_object, node, state, flags: 0u32,
        }
    }
    #[inline]
    pub fn flags(self, value: u32) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::GltfDocument::append_from_scene_full(self.surround_object, self.node, self.state, self.flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::generate_scene_ex`][super::GltfDocument::generate_scene_ex]."]
#[must_use]
pub struct ExGenerateScene < 'a > {
    surround_object: &'a mut re_export::GltfDocument, state: Gd < crate::engine::GltfState >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateScene < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, state: Gd < crate::engine::GltfState >,) -> Self {
        Self {
            surround_object, state, bake_fps: 30f32, trimming: false, remove_immutable_tracks: true,
        }
    }
    #[inline]
    pub fn bake_fps(self, value: f32) -> Self {
        Self {
            bake_fps: value, .. self
        }
    }
    #[inline]
    pub fn trimming(self, value: bool) -> Self {
        Self {
            trimming: value, .. self
        }
    }
    #[inline]
    pub fn remove_immutable_tracks(self, value: bool) -> Self {
        Self {
            remove_immutable_tracks: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::GltfDocument::generate_scene_full(self.surround_object, self.state, self.bake_fps, self.trimming, self.remove_immutable_tracks,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::register_gltf_document_extension_ex`][super::GltfDocument::register_gltf_document_extension_ex]."]
#[must_use]
pub struct ExRegisterGltfDocumentExtension {
    extension: Gd < crate::engine::GltfDocumentExtension >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExRegisterGltfDocumentExtension {
    fn new(extension: Gd < crate::engine::GltfDocumentExtension >,) -> Self {
        Self {
            extension, first_priority: false,
        }
    }
    #[inline]
    pub fn first_priority(self, value: bool) -> Self {
        Self {
            first_priority: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::GltfDocument::register_gltf_document_extension_full(self.extension, self.first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RootNodeMode {
    ord: i32
}
impl RootNodeMode {
    pub const ROOT_NODE_MODE_SINGLE_ROOT: Self = Self {
        ord: 0i32
    };
    pub const ROOT_NODE_MODE_KEEP_ROOT: Self = Self {
        ord: 1i32
    };
    pub const ROOT_NODE_MODE_MULTI_ROOT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for RootNodeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for RootNodeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RootNodeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RootNodeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}