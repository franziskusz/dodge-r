#![doc = "Sidecar module for class [`EditorUndoRedoManager`][crate::engine::EditorUndoRedoManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorUndoRedoManager` enums](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorUndoRedoManager.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`editor_undo_redo_manager`][crate::engine::editor_undo_redo_manager]: sidecar module with related enum/flag types\n* [`IEditorUndoRedoManager`][crate::engine::IEditorUndoRedoManager]: virtual methods\n\n\nSee also [Godot docs for `EditorUndoRedoManager`](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorUndoRedoManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorUndoRedoManager`][crate::engine::EditorUndoRedoManager].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorUndoRedoManager` methods](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorUndoRedoManager: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorUndoRedoManager {
        pub(crate) fn create_action_full(&mut self, name: GString, merge_mode: crate::engine::undo_redo::MergeMode, custom_context: Gd < crate::engine::Object >, backward_undo_ops: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, crate::engine::undo_redo::MergeMode, Gd < crate::engine::Object >, bool);
            let args = (name, merge_mode, custom_context, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_action(&mut self, name: GString,) {
            self.create_action_ex(name,) . done()
        }
        #[inline]
        pub fn create_action_ex(&mut self, name: GString,) -> ExCreateAction < '_ > {
            ExCreateAction::new(self, name,)
        }
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "commit_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[inline]
        pub fn commit_action_ex(&mut self,) -> ExCommitAction < '_ > {
            ExCommitAction::new(self,)
        }
        pub fn is_committing_action(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_committing_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_method(&mut self, object: Gd < crate::engine::Object >, method: StringName, varargs: &[Variant]) {
            type CallSig = ((), Gd < crate::engine::Object >, StringName);
            let args = (object, method,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(286usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "add_do_method", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn add_undo_method(&mut self, object: Gd < crate::engine::Object >, method: StringName, varargs: &[Variant]) {
            type CallSig = ((), Gd < crate::engine::Object >, StringName);
            let args = (object, method,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(287usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "add_undo_method", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn add_do_property(&mut self, object: Gd < crate::engine::Object >, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >, StringName, Variant);
            let args = (object, property, value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_do_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: Gd < crate::engine::Object >, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >, StringName, Variant);
            let args = (object, property, value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_do_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_object_history_id(&self, object: Gd < crate::engine::Object >,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::Object >);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_object_history_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_history_undo_redo(&self, id: i32,) -> Option < Gd < crate::engine::UndoRedo > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::UndoRedo > >;
            type CallSig = (Option < Gd < crate::engine::UndoRedo > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_history_undo_redo", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorUndoRedoManager {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorUndoRedoManager\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorUndoRedoManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorUndoRedoManager {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorUndoRedoManager {
        
    }
    impl std::ops::Deref for EditorUndoRedoManager {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorUndoRedoManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorUndoRedoManager {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorUndoRedoManager > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::create_action_ex`][super::EditorUndoRedoManager::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    surround_object: &'a mut re_export::EditorUndoRedoManager, name: GString, merge_mode: crate::engine::undo_redo::MergeMode, custom_context: Gd < crate::engine::Object >, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager, name: GString,) -> Self {
        Self {
            surround_object, name, merge_mode: crate::obj::EngineEnum::from_ord(0), custom_context: unimplemented !("see #156"), backward_undo_ops: false,
        }
    }
    #[inline]
    pub fn merge_mode(self, value: crate::engine::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: value, .. self
        }
    }
    #[inline]
    pub fn custom_context(self, value: Gd < crate::engine::Object >) -> Self {
        Self {
            custom_context: value, .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, value: bool) -> Self {
        Self {
            backward_undo_ops: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorUndoRedoManager::create_action_full(self.surround_object, self.name, self.merge_mode, self.custom_context, self.backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::commit_action_ex`][super::EditorUndoRedoManager::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    surround_object: &'a mut re_export::EditorUndoRedoManager, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager,) -> Self {
        Self {
            surround_object, execute: true,
        }
    }
    #[inline]
    pub fn execute(self, value: bool) -> Self {
        Self {
            execute: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorUndoRedoManager::commit_action_full(self.surround_object, self.execute,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpecialHistory {
    ord: i32
}
impl SpecialHistory {
    pub const GLOBAL_HISTORY: Self = Self {
        ord: 0i32
    };
    pub const REMOTE_HISTORY: Self = Self {
        ord: - 9i32
    };
    pub const INVALID_HISTORY: Self = Self {
        ord: - 99i32
    };
    
}
impl crate::obj::EngineEnum for SpecialHistory {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 99i32 | ord @ - 9i32 | ord @ 0i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SpecialHistory {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpecialHistory {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpecialHistory {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}