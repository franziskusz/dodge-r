#![doc = "Sidecar module for class [`EditorFileDialog`][crate::engine::EditorFileDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFileDialog` enums](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFileDialog.`\n\nInherits [`ConfirmationDialog`][crate::engine::ConfirmationDialog].\n\nRelated symbols:\n\n* [`editor_file_dialog`][crate::engine::editor_file_dialog]: sidecar module with related enum/flag types\n* [`IEditorFileDialog`][crate::engine::IEditorFileDialog]: virtual methods\n\n\nSee also [Godot docs for `EditorFileDialog`](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFileDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorFileDialog`][crate::engine::EditorFileDialog].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorFileDialog` methods](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFileDialog: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
            unimplemented !()
        }
        fn get_contents_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl EditorFileDialog {
        pub fn clear_filters(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(35usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_filter_full(&mut self, filter: GString, description: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (filter, description,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(36usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_filter(&mut self, filter: GString,) {
            self.add_filter_ex(filter,) . done()
        }
        #[inline]
        pub fn add_filter_ex(&mut self, filter: GString,) -> ExAddFilter < '_ > {
            ExAddFilter::new(self, filter,)
        }
        pub fn set_filters(&mut self, filters: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray);
            let args = (filters,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(37usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_filters(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(38usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_dir(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(39usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_file(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(40usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(41usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_dir(&mut self, dir: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (dir,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(42usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_file(&mut self, file: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(43usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_path(&mut self, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(44usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_file_mode(&mut self, mode: crate::engine::editor_file_dialog::FileMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_file_dialog::FileMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(45usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_file_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_mode(&self,) -> crate::engine::editor_file_dialog::FileMode {
            type RetMarshal = PtrcallReturnT < crate::engine::editor_file_dialog::FileMode >;
            type CallSig = (crate::engine::editor_file_dialog::FileMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(46usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vbox(&mut self,) -> Option < Gd < crate::engine::VBoxContainer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VBoxContainer > >;
            type CallSig = (Option < Gd < crate::engine::VBoxContainer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(47usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vbox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_edit(&mut self,) -> Option < Gd < crate::engine::LineEdit > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::LineEdit > >;
            type CallSig = (Option < Gd < crate::engine::LineEdit > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(48usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_edit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_access(&mut self, access: crate::engine::editor_file_dialog::Access,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_file_dialog::Access);
            let args = (access,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(49usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_access", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_access(&self,) -> crate::engine::editor_file_dialog::Access {
            type RetMarshal = PtrcallReturnT < crate::engine::editor_file_dialog::Access >;
            type CallSig = (crate::engine::editor_file_dialog::Access,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(50usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_access", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_show_hidden_files(&mut self, show: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(51usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_show_hidden_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_showing_hidden_files(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(52usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_showing_hidden_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_mode(&mut self, mode: crate::engine::editor_file_dialog::DisplayMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_file_dialog::DisplayMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(53usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_display_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_mode(&self,) -> crate::engine::editor_file_dialog::DisplayMode {
            type RetMarshal = PtrcallReturnT < crate::engine::editor_file_dialog::DisplayMode >;
            type CallSig = (crate::engine::editor_file_dialog::DisplayMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(54usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_display_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_overwrite_warning(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(55usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_overwrite_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_overwrite_warning_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(56usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_overwrite_warning_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_side_menu_full(&mut self, menu: Gd < crate::engine::Control >, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >, GString);
            let args = (menu, title,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(57usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_side_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_side_menu(&mut self, menu: Gd < crate::engine::Control >,) {
            self.add_side_menu_ex(menu,) . done()
        }
        #[inline]
        pub fn add_side_menu_ex(&mut self, menu: Gd < crate::engine::Control >,) -> ExAddSideMenu < '_ > {
            ExAddSideMenu::new(self, menu,)
        }
        pub fn invalidate(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(58usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "invalidate", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorFileDialog {
        type Base = crate::engine::ConfirmationDialog;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorFileDialog\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFileDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorFileDialog {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::ConfirmationDialog > for EditorFileDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::AcceptDialog > for EditorFileDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Window > for EditorFileDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Viewport > for EditorFileDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for EditorFileDialog {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorFileDialog {
        
    }
    impl crate::obj::ExportableObject for EditorFileDialog {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFileDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFileDialog {
        type Target = crate::engine::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFileDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorFileDialog {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorFileDialog > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::ConfirmationDialog > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AcceptDialog > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Window > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Viewport > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_filter_ex`][super::EditorFileDialog::add_filter_ex]."]
#[must_use]
pub struct ExAddFilter < 'a > {
    surround_object: &'a mut re_export::EditorFileDialog, filter: GString, description: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFilter < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, filter: GString,) -> Self {
        Self {
            surround_object, filter, description: GString::from(""),
        }
    }
    #[inline]
    pub fn description(self, value: GString) -> Self {
        Self {
            description: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorFileDialog::add_filter_full(self.surround_object, self.filter, self.description,)
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_side_menu_ex`][super::EditorFileDialog::add_side_menu_ex]."]
#[must_use]
pub struct ExAddSideMenu < 'a > {
    surround_object: &'a mut re_export::EditorFileDialog, menu: Gd < crate::engine::Control >, title: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSideMenu < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, menu: Gd < crate::engine::Control >,) -> Self {
        Self {
            surround_object, menu, title: GString::from(""),
        }
    }
    #[inline]
    pub fn title(self, value: GString) -> Self {
        Self {
            title: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorFileDialog::add_side_menu_full(self.surround_object, self.menu, self.title,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileMode {
    ord: i32
}
impl FileMode {
    pub const FILE_MODE_OPEN_FILE: Self = Self {
        ord: 0i32
    };
    pub const FILE_MODE_OPEN_FILES: Self = Self {
        ord: 1i32
    };
    pub const FILE_MODE_OPEN_DIR: Self = Self {
        ord: 2i32
    };
    pub const FILE_MODE_OPEN_ANY: Self = Self {
        ord: 3i32
    };
    pub const FILE_MODE_SAVE_FILE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for FileMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for FileMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FileMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FileMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Access {
    ord: i32
}
impl Access {
    pub const ACCESS_RESOURCES: Self = Self {
        ord: 0i32
    };
    pub const ACCESS_USERDATA: Self = Self {
        ord: 1i32
    };
    pub const ACCESS_FILESYSTEM: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Access {
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
impl crate::builtin::meta::GodotConvert for Access {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Access {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Access {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DisplayMode {
    ord: i32
}
impl DisplayMode {
    pub const DISPLAY_THUMBNAILS: Self = Self {
        ord: 0i32
    };
    pub const DISPLAY_LIST: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for DisplayMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for DisplayMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DisplayMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DisplayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}