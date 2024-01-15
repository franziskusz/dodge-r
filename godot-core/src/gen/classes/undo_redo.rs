#![doc = "Sidecar module for class [`UndoRedo`][crate::engine::UndoRedo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UndoRedo` enums](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UndoRedo.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`undo_redo`][crate::engine::undo_redo]: sidecar module with related enum/flag types\n* [`IUndoRedo`][crate::engine::IUndoRedo]: virtual methods\n\n\nSee also [Godot docs for `UndoRedo`](https://docs.godotengine.org/en/stable/classes/class_undoredo.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UndoRedo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`UndoRedo`][crate::engine::UndoRedo].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UndoRedo` methods](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUndoRedo: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl UndoRedo {
        pub(crate) fn create_action_full(&mut self, name: GString, merge_mode: crate::engine::undo_redo::MergeMode, backward_undo_ops: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, crate::engine::undo_redo::MergeMode, bool);
            let args = (name, merge_mode, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9199usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9200usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_committing_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_method(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_do_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_method(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_property(&mut self, object: Gd < crate::engine::Object >, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >, StringName, Variant);
            let args = (object, property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_do_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: Gd < crate::engine::Object >, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >, StringName, Variant);
            let args = (object, property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_do_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: Gd < crate::engine::Object >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn start_force_keep_in_merge_ends(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start_force_keep_in_merge_ends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_force_keep_in_merge_ends(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "end_force_keep_in_merge_ends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_history_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_history_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_action(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_name(&mut self, id: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn clear_history_full(&mut self, increase_version: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (increase_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_history", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn clear_history(&mut self,) {
            self.clear_history_ex() . done()
        }
        #[inline]
        pub fn clear_history_ex(&mut self,) -> ExClearHistory < '_ > {
            ExClearHistory::new(self,)
        }
        pub fn get_current_action_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_action_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_undo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn redo(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn undo(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "undo", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for UndoRedo {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"UndoRedo\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UndoRedo {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for UndoRedo {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for UndoRedo {
        
    }
    impl crate::obj::cap::GodotDefault for UndoRedo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UndoRedo {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UndoRedo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_UndoRedo {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::UndoRedo > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UndoRedo::create_action_ex`][super::UndoRedo::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    surround_object: &'a mut re_export::UndoRedo, name: GString, merge_mode: crate::engine::undo_redo::MergeMode, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo, name: GString,) -> Self {
        Self {
            surround_object, name, merge_mode: crate::obj::EngineEnum::from_ord(0), backward_undo_ops: false,
        }
    }
    #[inline]
    pub fn merge_mode(self, value: crate::engine::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: value, .. self
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
        re_export::UndoRedo::create_action_full(self.surround_object, self.name, self.merge_mode, self.backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::commit_action_ex`][super::UndoRedo::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    surround_object: &'a mut re_export::UndoRedo, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
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
        re_export::UndoRedo::commit_action_full(self.surround_object, self.execute,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::clear_history_ex`][super::UndoRedo::clear_history_ex]."]
#[must_use]
pub struct ExClearHistory < 'a > {
    surround_object: &'a mut re_export::UndoRedo, increase_version: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearHistory < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
        Self {
            surround_object, increase_version: true,
        }
    }
    #[inline]
    pub fn increase_version(self, value: bool) -> Self {
        Self {
            increase_version: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::UndoRedo::clear_history_full(self.surround_object, self.increase_version,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MergeMode {
    ord: i32
}
impl MergeMode {
    pub const MERGE_DISABLE: Self = Self {
        ord: 0i32
    };
    pub const MERGE_ENDS: Self = Self {
        ord: 1i32
    };
    pub const MERGE_ALL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for MergeMode {
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
impl crate::builtin::meta::GodotConvert for MergeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MergeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MergeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}