#![doc = "Sidecar module for class [`EditorFeatureProfile`][crate::engine::EditorFeatureProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFeatureProfile` enums](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFeatureProfile.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_feature_profile`][crate::engine::editor_feature_profile]: sidecar module with related enum/flag types\n* [`IEditorFeatureProfile`][crate::engine::IEditorFeatureProfile]: virtual methods\n\n\nSee also [Godot docs for `EditorFeatureProfile`](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFeatureProfile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorFeatureProfile`][crate::engine::EditorFeatureProfile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorFeatureProfile` methods](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFeatureProfile: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl EditorFeatureProfile {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_disable_class(&mut self, class_name: StringName, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (class_name, disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(24usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_class_disabled(&self, class_name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (class_name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(25usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_class_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_class_editor(&mut self, class_name: StringName, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (class_name, disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(26usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_class_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_class_editor_disabled(&self, class_name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (class_name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(27usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_class_editor_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_class_property(&mut self, class_name: StringName, property: StringName, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, bool);
            let args = (class_name, property, disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(28usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_class_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_class_property_disabled(&self, class_name: StringName, property: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (class_name, property,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(29usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_class_property_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_feature(&mut self, feature: crate::engine::editor_feature_profile::Feature, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_feature_profile::Feature, bool);
            let args = (feature, disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(30usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_feature_disabled(&self, feature: crate::engine::editor_feature_profile::Feature,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::editor_feature_profile::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(31usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_feature_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_feature_name(&mut self, feature: crate::engine::editor_feature_profile::Feature,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, crate::engine::editor_feature_profile::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(32usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_feature_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_to_file(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(33usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_to_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_from_file(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(34usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_from_file", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorFeatureProfile {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorFeatureProfile\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFeatureProfile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorFeatureProfile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorFeatureProfile {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorFeatureProfile {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFeatureProfile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFeatureProfile {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFeatureProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorFeatureProfile {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorFeatureProfile > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Feature {
    ord: i32
}
impl Feature {
    pub const FEATURE_3D: Self = Self {
        ord: 0i32
    };
    pub const FEATURE_SCRIPT: Self = Self {
        ord: 1i32
    };
    pub const FEATURE_ASSET_LIB: Self = Self {
        ord: 2i32
    };
    pub const FEATURE_SCENE_TREE: Self = Self {
        ord: 3i32
    };
    pub const FEATURE_NODE_DOCK: Self = Self {
        ord: 4i32
    };
    pub const FEATURE_FILESYSTEM_DOCK: Self = Self {
        ord: 5i32
    };
    pub const FEATURE_IMPORT_DOCK: Self = Self {
        ord: 6i32
    };
    pub const FEATURE_HISTORY_DOCK: Self = Self {
        ord: 7i32
    };
    pub const FEATURE_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Feature {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Feature {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}