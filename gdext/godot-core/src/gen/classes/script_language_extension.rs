#![doc = "Sidecar module for class [`ScriptLanguageExtension`][crate::engine::ScriptLanguageExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptLanguageExtension` enums](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptLanguageExtension.`\n\nInherits [`ScriptLanguage`][crate::engine::ScriptLanguage].\n\nRelated symbols:\n\n* [`script_language_extension`][crate::engine::script_language_extension]: sidecar module with related enum/flag types\n* [`IScriptLanguageExtension`][crate::engine::IScriptLanguageExtension]: virtual methods\n\n\nSee also [Godot docs for `ScriptLanguageExtension`](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptLanguageExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ScriptLanguageExtension`][crate::engine::ScriptLanguageExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScriptLanguageExtension` methods](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptLanguageExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_name(&self,) -> GString {
            unimplemented !()
        }
        fn init_ext(&mut self,) {
            unimplemented !()
        }
        fn get_type(&self,) -> GString {
            unimplemented !()
        }
        fn get_extension(&self,) -> GString {
            unimplemented !()
        }
        fn finish(&mut self,) {
            unimplemented !()
        }
        fn get_reserved_words(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn is_control_flow_keyword(&self, keyword: GString,) -> bool {
            unimplemented !()
        }
        fn get_comment_delimiters(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_doc_comment_delimiters(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_string_delimiters(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn make_template(&self, template: GString, class_name: GString, base_class_name: GString,) -> Option < Gd < crate::engine::Script > > {
            unimplemented !()
        }
        fn get_built_in_templates(&self, object: StringName,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn is_using_templates(&mut self,) -> bool {
            unimplemented !()
        }
        fn validate(&self, script: GString, path: GString, validate_functions: bool, validate_errors: bool, validate_warnings: bool, validate_safe_lines: bool,) -> Dictionary {
            unimplemented !()
        }
        fn validate_path(&self, path: GString,) -> GString {
            unimplemented !()
        }
        fn create_script(&self,) -> Option < Gd < crate::engine::Object > > {
            unimplemented !()
        }
        fn has_named_classes(&self,) -> bool {
            unimplemented !()
        }
        fn supports_builtin_mode(&self,) -> bool {
            unimplemented !()
        }
        fn supports_documentation(&self,) -> bool {
            unimplemented !()
        }
        fn can_inherit_from_file(&self,) -> bool {
            unimplemented !()
        }
        fn find_function(&self, class_name: GString, function_name: GString,) -> i32 {
            unimplemented !()
        }
        fn make_function(&self, class_name: GString, function_name: GString, function_args: PackedStringArray,) -> GString {
            unimplemented !()
        }
        fn open_in_external_editor(&mut self, script: Gd < crate::engine::Script >, line: i32, column: i32,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn overrides_external_editor(&mut self,) -> bool {
            unimplemented !()
        }
        fn complete_code(&self, code: GString, path: GString, owner: Gd < crate::engine::Object >,) -> Dictionary {
            unimplemented !()
        }
        fn lookup_code(&self, code: GString, symbol: GString, path: GString, owner: Gd < crate::engine::Object >,) -> Dictionary {
            unimplemented !()
        }
        fn auto_indent_code(&self, code: GString, from_line: i32, to_line: i32,) -> GString {
            unimplemented !()
        }
        fn add_global_constant(&mut self, name: StringName, value: Variant,) {
            unimplemented !()
        }
        fn add_named_global_constant(&mut self, name: StringName, value: Variant,) {
            unimplemented !()
        }
        fn remove_named_global_constant(&mut self, name: StringName,) {
            unimplemented !()
        }
        fn thread_enter(&mut self,) {
            unimplemented !()
        }
        fn thread_exit(&mut self,) {
            unimplemented !()
        }
        fn debug_get_error(&self,) -> GString {
            unimplemented !()
        }
        fn debug_get_stack_level_count(&self,) -> i32 {
            unimplemented !()
        }
        fn debug_get_stack_level_line(&self, level: i32,) -> i32 {
            unimplemented !()
        }
        fn debug_get_stack_level_function(&self, level: i32,) -> GString {
            unimplemented !()
        }
        fn debug_get_stack_level_locals(&mut self, level: i32, max_subitems: i32, max_depth: i32,) -> Dictionary {
            unimplemented !()
        }
        fn debug_get_stack_level_members(&mut self, level: i32, max_subitems: i32, max_depth: i32,) -> Dictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn debug_get_stack_level_instance(&mut self, level: i32,) -> * mut c_void {
            unimplemented !()
        }
        fn debug_get_globals(&mut self, max_subitems: i32, max_depth: i32,) -> Dictionary {
            unimplemented !()
        }
        fn debug_parse_stack_level_expression(&mut self, level: i32, expression: GString, max_subitems: i32, max_depth: i32,) -> GString {
            unimplemented !()
        }
        fn debug_get_current_stack_info(&mut self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn reload_all_scripts(&mut self,) {
            unimplemented !()
        }
        fn reload_tool_script(&mut self, script: Gd < crate::engine::Script >, soft_reload: bool,) {
            unimplemented !()
        }
        fn get_recognized_extensions(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_public_functions(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_public_constants(&self,) -> Dictionary {
            unimplemented !()
        }
        fn get_public_annotations(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn profiling_start(&mut self,) {
            unimplemented !()
        }
        fn profiling_stop(&mut self,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn profiling_get_accumulated_data(&mut self, info_array: * mut ScriptLanguageExtensionProfilingInfo, info_max: i32,) -> i32 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn profiling_get_frame_data(&mut self, info_array: * mut ScriptLanguageExtensionProfilingInfo, info_max: i32,) -> i32 {
            unimplemented !()
        }
        fn frame(&mut self,) {
            unimplemented !()
        }
        fn handles_global_class_type(&self, type_: GString,) -> bool {
            unimplemented !()
        }
        fn get_global_class_name(&self, path: GString,) -> Dictionary {
            unimplemented !()
        }
    }
    impl ScriptLanguageExtension {
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for ScriptLanguageExtension {
        type Base = crate::engine::ScriptLanguage;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ScriptLanguageExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScriptLanguageExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ScriptLanguageExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::ScriptLanguage > for ScriptLanguageExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ScriptLanguageExtension {
        
    }
    impl crate::obj::cap::GodotDefault for ScriptLanguageExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScriptLanguageExtension {
        type Target = crate::engine::ScriptLanguage;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptLanguageExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ScriptLanguageExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ScriptLanguageExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::ScriptLanguage > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LookupResultType {
    ord: i32
}
impl LookupResultType {
    pub const LOOKUP_RESULT_SCRIPT_LOCATION: Self = Self {
        ord: 0i32
    };
    pub const LOOKUP_RESULT_CLASS: Self = Self {
        ord: 1i32
    };
    pub const LOOKUP_RESULT_CLASS_CONSTANT: Self = Self {
        ord: 2i32
    };
    pub const LOOKUP_RESULT_CLASS_PROPERTY: Self = Self {
        ord: 3i32
    };
    pub const LOOKUP_RESULT_CLASS_METHOD: Self = Self {
        ord: 4i32
    };
    pub const LOOKUP_RESULT_CLASS_SIGNAL: Self = Self {
        ord: 5i32
    };
    pub const LOOKUP_RESULT_CLASS_ENUM: Self = Self {
        ord: 6i32
    };
    pub const LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE: Self = Self {
        ord: 7i32
    };
    pub const LOOKUP_RESULT_CLASS_ANNOTATION: Self = Self {
        ord: 8i32
    };
    pub const LOOKUP_RESULT_MAX: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for LookupResultType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for LookupResultType {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::builtin::meta::GodotConvert for LookupResultType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LookupResultType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LookupResultType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CodeCompletionLocation {
    ord: i32
}
impl CodeCompletionLocation {
    pub const LOCATION_LOCAL: Self = Self {
        ord: 0i32
    };
    pub const LOCATION_PARENT_MASK: Self = Self {
        ord: 256i32
    };
    pub const LOCATION_OTHER_USER_CODE: Self = Self {
        ord: 512i32
    };
    pub const LOCATION_OTHER: Self = Self {
        ord: 1024i32
    };
    
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CodeCompletionLocation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CodeCompletionLocation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CodeCompletionLocation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    pub const CODE_COMPLETION_KIND_CLASS: Self = Self {
        ord: 0i32
    };
    pub const CODE_COMPLETION_KIND_FUNCTION: Self = Self {
        ord: 1i32
    };
    pub const CODE_COMPLETION_KIND_SIGNAL: Self = Self {
        ord: 2i32
    };
    pub const CODE_COMPLETION_KIND_VARIABLE: Self = Self {
        ord: 3i32
    };
    pub const CODE_COMPLETION_KIND_MEMBER: Self = Self {
        ord: 4i32
    };
    pub const CODE_COMPLETION_KIND_ENUM: Self = Self {
        ord: 5i32
    };
    pub const CODE_COMPLETION_KIND_CONSTANT: Self = Self {
        ord: 6i32
    };
    pub const CODE_COMPLETION_KIND_NODE_PATH: Self = Self {
        ord: 7i32
    };
    pub const CODE_COMPLETION_KIND_FILE_PATH: Self = Self {
        ord: 8i32
    };
    pub const CODE_COMPLETION_KIND_PLAIN_TEXT: Self = Self {
        ord: 9i32
    };
    pub const CODE_COMPLETION_KIND_MAX: Self = Self {
        ord: 10i32
    };
    
}
impl crate::obj::EngineEnum for CodeCompletionKind {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for CodeCompletionKind {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::builtin::meta::GodotConvert for CodeCompletionKind {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CodeCompletionKind {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CodeCompletionKind {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}