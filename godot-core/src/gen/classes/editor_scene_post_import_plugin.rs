#![doc = "Sidecar module for class [`EditorScenePostImportPlugin`][crate::engine::EditorScenePostImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorScenePostImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorScenePostImportPlugin.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_scene_post_import_plugin`][crate::engine::editor_scene_post_import_plugin]: sidecar module with related enum/flag types\n* [`IEditorScenePostImportPlugin`][crate::engine::IEditorScenePostImportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorScenePostImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorScenePostImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorScenePostImportPlugin`][crate::engine::EditorScenePostImportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorScenePostImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorScenePostImportPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_internal_import_options(&mut self, category: i32,) {
            unimplemented !()
        }
        fn get_internal_option_visibility(&self, category: i32, for_animation: bool, option: GString,) -> Variant {
            unimplemented !()
        }
        fn get_internal_option_update_view_required(&self, category: i32, option: GString,) -> Variant {
            unimplemented !()
        }
        fn internal_process(&mut self, category: i32, base_node: Gd < crate::engine::Node >, node: Gd < crate::engine::Node >, resource: Gd < crate::engine::Resource >,) {
            unimplemented !()
        }
        fn get_import_options(&mut self, path: GString,) {
            unimplemented !()
        }
        fn get_option_visibility(&self, path: GString, for_animation: bool, option: GString,) -> Variant {
            unimplemented !()
        }
        fn pre_process(&mut self, scene: Gd < crate::engine::Node >,) {
            unimplemented !()
        }
        fn post_process(&mut self, scene: Gd < crate::engine::Node >,) {
            unimplemented !()
        }
    }
    impl EditorScenePostImportPlugin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_option_value(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_option_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_import_option(&mut self, name: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_import_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_import_option_advanced_full(&mut self, type_: VariantType, name: GString, default_value: Variant, hint: crate::engine::global::PropertyHint, hint_string: GString, usage_flags: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantType, GString, Variant, crate::engine::global::PropertyHint, GString, i32);
            let args = (type_, name, default_value, hint, hint_string, usage_flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_import_option_advanced", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_import_option_advanced(&mut self, type_: VariantType, name: GString, default_value: Variant,) {
            self.add_import_option_advanced_ex(type_, name, default_value,) . done()
        }
        #[inline]
        pub fn add_import_option_advanced_ex(&mut self, type_: VariantType, name: GString, default_value: Variant,) -> ExAddImportOptionAdvanced < '_ > {
            ExAddImportOptionAdvanced::new(self, type_, name, default_value,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for EditorScenePostImportPlugin {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorScenePostImportPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorScenePostImportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorScenePostImportPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorScenePostImportPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorScenePostImportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorScenePostImportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorScenePostImportPlugin {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorScenePostImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorScenePostImportPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorScenePostImportPlugin > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorScenePostImportPlugin::add_import_option_advanced_ex`][super::EditorScenePostImportPlugin::add_import_option_advanced_ex]."]
#[must_use]
pub struct ExAddImportOptionAdvanced < 'a > {
    surround_object: &'a mut re_export::EditorScenePostImportPlugin, type_: VariantType, name: GString, default_value: Variant, hint: crate::engine::global::PropertyHint, hint_string: GString, usage_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImportOptionAdvanced < 'a > {
    fn new(surround_object: &'a mut re_export::EditorScenePostImportPlugin, type_: VariantType, name: GString, default_value: Variant,) -> Self {
        Self {
            surround_object, type_, name, default_value, hint: crate::obj::EngineEnum::from_ord(0), hint_string: GString::from(""), usage_flags: 6i32,
        }
    }
    #[inline]
    pub fn hint(self, value: crate::engine::global::PropertyHint) -> Self {
        Self {
            hint: value, .. self
        }
    }
    #[inline]
    pub fn hint_string(self, value: GString) -> Self {
        Self {
            hint_string: value, .. self
        }
    }
    #[inline]
    pub fn usage_flags(self, value: i32) -> Self {
        Self {
            usage_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorScenePostImportPlugin::add_import_option_advanced_full(self.surround_object, self.type_, self.name, self.default_value, self.hint, self.hint_string, self.usage_flags,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InternalImportCategory {
    ord: i32
}
impl InternalImportCategory {
    pub const INTERNAL_IMPORT_CATEGORY_NODE: Self = Self {
        ord: 0i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE: Self = Self {
        ord: 1i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_MESH: Self = Self {
        ord: 2i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_MATERIAL: Self = Self {
        ord: 3i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_ANIMATION: Self = Self {
        ord: 4i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE: Self = Self {
        ord: 5i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE: Self = Self {
        ord: 6i32
    };
    pub const INTERNAL_IMPORT_CATEGORY_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for InternalImportCategory {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for InternalImportCategory {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for InternalImportCategory {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InternalImportCategory {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InternalImportCategory {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}