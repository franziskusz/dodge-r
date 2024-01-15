#![doc = "Sidecar module for class [`EditorVcsInterface`][crate::engine::EditorVcsInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorVCSInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorVCSInterface.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`editor_vcs_interface`][crate::engine::editor_vcs_interface]: sidecar module with related enum/flag types\n* [`IEditorVcsInterface`][crate::engine::IEditorVcsInterface]: virtual methods\n\n\nSee also [Godot docs for `EditorVCSInterface`](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorVcsInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorVcsInterface`][crate::engine::EditorVcsInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorVCSInterface` methods](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorVcsInterface: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn initialize(&mut self, project_path: GString,) -> bool {
            unimplemented !()
        }
        fn set_credentials(&mut self, username: GString, password: GString, ssh_public_key_path: GString, ssh_private_key_path: GString, ssh_passphrase: GString,) {
            unimplemented !()
        }
        fn get_modified_files_data(&mut self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn stage_file(&mut self, file_path: GString,) {
            unimplemented !()
        }
        fn unstage_file(&mut self, file_path: GString,) {
            unimplemented !()
        }
        fn discard_file(&mut self, file_path: GString,) {
            unimplemented !()
        }
        fn commit(&mut self, msg: GString,) {
            unimplemented !()
        }
        fn get_diff(&mut self, identifier: GString, area: i32,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn shut_down(&mut self,) -> bool {
            unimplemented !()
        }
        fn get_vcs_name(&mut self,) -> GString {
            unimplemented !()
        }
        fn get_previous_commits(&mut self, max_commits: i32,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_branch_list(&mut self,) -> Array < GString > {
            unimplemented !()
        }
        fn get_remotes(&mut self,) -> Array < GString > {
            unimplemented !()
        }
        fn create_branch(&mut self, branch_name: GString,) {
            unimplemented !()
        }
        fn remove_branch(&mut self, branch_name: GString,) {
            unimplemented !()
        }
        fn create_remote(&mut self, remote_name: GString, remote_url: GString,) {
            unimplemented !()
        }
        fn remove_remote(&mut self, remote_name: GString,) {
            unimplemented !()
        }
        fn get_current_branch_name(&mut self,) -> GString {
            unimplemented !()
        }
        fn checkout_branch(&mut self, branch_name: GString,) -> bool {
            unimplemented !()
        }
        fn pull(&mut self, remote: GString,) {
            unimplemented !()
        }
        fn push(&mut self, remote: GString, force: bool,) {
            unimplemented !()
        }
        fn fetch(&mut self, remote: GString,) {
            unimplemented !()
        }
        fn get_line_diff(&mut self, file_path: GString, text: GString,) -> Array < Dictionary > {
            unimplemented !()
        }
    }
    impl EditorVcsInterface {
        pub fn create_diff_line(&mut self, new_line_no: i32, old_line_no: i32, content: GString, status: GString,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32, i32, GString, GString);
            let args = (new_line_no, old_line_no, content, status,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_diff_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_diff_hunk(&mut self, old_start: i32, new_start: i32, old_lines: i32, new_lines: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32, i32, i32, i32);
            let args = (old_start, new_start, old_lines, new_lines,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_diff_hunk", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_diff_file(&mut self, new_file: GString, old_file: GString,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, GString);
            let args = (new_file, old_file,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_diff_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_commit(&mut self, msg: GString, author: GString, id: GString, unix_timestamp: i64, offset_minutes: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, GString, GString, i64, i64);
            let args = (msg, author, id, unix_timestamp, offset_minutes,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_commit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_status_file(&mut self, file_path: GString, change_type: crate::engine::editor_vcs_interface::ChangeType, area: crate::engine::editor_vcs_interface::TreeArea,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, GString, crate::engine::editor_vcs_interface::ChangeType, crate::engine::editor_vcs_interface::TreeArea);
            let args = (file_path, change_type, area,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_status_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_diff_hunks_into_diff_file(&mut self, diff_file: Dictionary, diff_hunks: Array < Dictionary >,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Dictionary, Array < Dictionary >);
            let args = (diff_file, diff_hunks,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_diff_hunks_into_diff_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_line_diffs_into_diff_hunk(&mut self, diff_hunk: Dictionary, line_diffs: Array < Dictionary >,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Dictionary, Array < Dictionary >);
            let args = (diff_hunk, line_diffs,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_line_diffs_into_diff_hunk", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn popup_error(&mut self, msg: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (msg,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_error", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorVcsInterface {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorVCSInterface\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorVcsInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorVcsInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorVcsInterface {
        
    }
    impl crate::obj::cap::GodotDefault for EditorVcsInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorVcsInterface {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorVcsInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorVcsInterface {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorVcsInterface > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ChangeType {
    ord: i32
}
impl ChangeType {
    pub const CHANGE_TYPE_NEW: Self = Self {
        ord: 0i32
    };
    pub const CHANGE_TYPE_MODIFIED: Self = Self {
        ord: 1i32
    };
    pub const CHANGE_TYPE_RENAMED: Self = Self {
        ord: 2i32
    };
    pub const CHANGE_TYPE_DELETED: Self = Self {
        ord: 3i32
    };
    pub const CHANGE_TYPE_TYPECHANGE: Self = Self {
        ord: 4i32
    };
    pub const CHANGE_TYPE_UNMERGED: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for ChangeType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ChangeType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ChangeType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ChangeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TreeArea {
    ord: i32
}
impl TreeArea {
    pub const TREE_AREA_COMMIT: Self = Self {
        ord: 0i32
    };
    pub const TREE_AREA_STAGED: Self = Self {
        ord: 1i32
    };
    pub const TREE_AREA_UNSTAGED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TreeArea {
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
impl crate::builtin::meta::GodotConvert for TreeArea {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TreeArea {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TreeArea {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}