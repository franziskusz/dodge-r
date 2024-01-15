#![doc = "Sidecar module for class [`EditorInterface`][crate::engine::EditorInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInterface.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`editor_interface`][crate::engine::editor_interface]: sidecar module with related enum/flag types\n* [`IEditorInterface`][crate::engine::IEditorInterface]: virtual methods\n\n\nSee also [Godot docs for `EditorInterface`](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorInterface`][crate::engine::EditorInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorInterface` methods](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorInterface: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorInterface {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"EditorInterface\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn restart_editor_full(&mut self, save: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (save,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(88usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "restart_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn restart_editor(&mut self,) {
            self.restart_editor_ex() . done()
        }
        #[inline]
        pub fn restart_editor_ex(&mut self,) -> ExRestartEditor < '_ > {
            ExRestartEditor::new(self,)
        }
        pub fn get_command_palette(&self,) -> Option < Gd < crate::engine::EditorCommandPalette > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorCommandPalette > >;
            type CallSig = (Option < Gd < crate::engine::EditorCommandPalette > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(89usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_command_palette", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resource_filesystem(&self,) -> Option < Gd < crate::engine::EditorFileSystem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorFileSystem > >;
            type CallSig = (Option < Gd < crate::engine::EditorFileSystem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(90usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resource_filesystem", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_paths(&self,) -> Option < Gd < crate::engine::EditorPaths > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorPaths > >;
            type CallSig = (Option < Gd < crate::engine::EditorPaths > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(91usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_paths", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resource_previewer(&self,) -> Option < Gd < crate::engine::EditorResourcePreview > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorResourcePreview > >;
            type CallSig = (Option < Gd < crate::engine::EditorResourcePreview > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(92usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resource_previewer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection(&self,) -> Option < Gd < crate::engine::EditorSelection > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorSelection > >;
            type CallSig = (Option < Gd < crate::engine::EditorSelection > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(93usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_settings(&self,) -> Option < Gd < crate::engine::EditorSettings > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorSettings > >;
            type CallSig = (Option < Gd < crate::engine::EditorSettings > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(94usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_settings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_mesh_previews(&mut self, meshes: Array < Gd < crate::engine::Mesh > >, preview_size: i32,) -> Array < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Texture2D > > >;
            type CallSig = (Array < Gd < crate::engine::Texture2D > >, Array < Gd < crate::engine::Mesh > >, i32);
            let args = (meshes, preview_size,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(95usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_mesh_previews", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_plugin_enabled(&mut self, plugin: GString, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (plugin, enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(96usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_plugin_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_plugin_enabled(&self, plugin: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(97usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_plugin_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_theme(&self,) -> Option < Gd < crate::engine::Theme > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Theme > >;
            type CallSig = (Option < Gd < crate::engine::Theme > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(98usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_control(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(99usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_base_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_main_screen(&self,) -> Option < Gd < crate::engine::VBoxContainer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VBoxContainer > >;
            type CallSig = (Option < Gd < crate::engine::VBoxContainer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_main_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_editor(&self,) -> Option < Gd < crate::engine::ScriptEditor > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ScriptEditor > >;
            type CallSig = (Option < Gd < crate::engine::ScriptEditor > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_viewport_2d(&self,) -> Option < Gd < crate::engine::SubViewport > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SubViewport > >;
            type CallSig = (Option < Gd < crate::engine::SubViewport > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_viewport_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_editor_viewport_3d_full(&self, idx: i32,) -> Option < Gd < crate::engine::SubViewport > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SubViewport > >;
            type CallSig = (Option < Gd < crate::engine::SubViewport > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_viewport_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_editor_viewport_3d(&self,) -> Option < Gd < crate::engine::SubViewport > > {
            self.get_editor_viewport_3d_ex() . done()
        }
        #[inline]
        pub fn get_editor_viewport_3d_ex(&self,) -> ExGetEditorViewport3d < '_ > {
            ExGetEditorViewport3d::new(self,)
        }
        pub fn set_main_screen_editor(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_main_screen_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distraction_free_mode(&mut self, enter: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_distraction_free_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_distraction_free_mode_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_distraction_free_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_dialog_full(&mut self, dialog: Gd < crate::engine::Window >, rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Window >, Rect2i);
            let args = (dialog, rect,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_dialog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_dialog(&mut self, dialog: Gd < crate::engine::Window >,) {
            self.popup_dialog_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_ex(&mut self, dialog: Gd < crate::engine::Window >,) -> ExPopupDialog < '_ > {
            ExPopupDialog::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_full(&mut self, dialog: Gd < crate::engine::Window >, minsize: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Window >, Vector2i);
            let args = (dialog, minsize,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_dialog_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_dialog_centered(&mut self, dialog: Gd < crate::engine::Window >,) {
            self.popup_dialog_centered_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ex(&mut self, dialog: Gd < crate::engine::Window >,) -> ExPopupDialogCentered < '_ > {
            ExPopupDialogCentered::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_ratio_full(&mut self, dialog: Gd < crate::engine::Window >, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Window >, f32);
            let args = (dialog, ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_dialog_centered_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_dialog_centered_ratio(&mut self, dialog: Gd < crate::engine::Window >,) {
            self.popup_dialog_centered_ratio_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ratio_ex(&mut self, dialog: Gd < crate::engine::Window >,) -> ExPopupDialogCenteredRatio < '_ > {
            ExPopupDialogCenteredRatio::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_clamped_full(&mut self, dialog: Gd < crate::engine::Window >, minsize: Vector2i, fallback_ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Window >, Vector2i, f32);
            let args = (dialog, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_dialog_centered_clamped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_dialog_centered_clamped(&mut self, dialog: Gd < crate::engine::Window >,) {
            self.popup_dialog_centered_clamped_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_clamped_ex(&mut self, dialog: Gd < crate::engine::Window >,) -> ExPopupDialogCenteredClamped < '_ > {
            ExPopupDialogCenteredClamped::new(self, dialog,)
        }
        pub fn get_current_feature_profile(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_feature_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_feature_profile(&mut self, profile_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (profile_name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_feature_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_system_dock(&self,) -> Option < Gd < crate::engine::FileSystemDock > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::FileSystemDock > >;
            type CallSig = (Option < Gd < crate::engine::FileSystemDock > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_system_dock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_file(&mut self, file: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_paths(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_paths", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_directory(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_directory", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inspector(&self,) -> Option < Gd < crate::engine::EditorInspector > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorInspector > >;
            type CallSig = (Option < Gd < crate::engine::EditorInspector > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inspector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn inspect_object_full(&mut self, object: Gd < crate::engine::Object >, for_property: GString, inspector_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Object >, GString, bool);
            let args = (object, for_property, inspector_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "inspect_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn inspect_object(&mut self, object: Gd < crate::engine::Object >,) {
            self.inspect_object_ex(object,) . done()
        }
        #[inline]
        pub fn inspect_object_ex(&mut self, object: Gd < crate::engine::Object >,) -> ExInspectObject < '_ > {
            ExInspectObject::new(self, object,)
        }
        pub fn edit_resource(&mut self, resource: Gd < crate::engine::Resource >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Resource >);
            let args = (resource,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "edit_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn edit_node(&mut self, node: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "edit_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn edit_script_full(&mut self, script: Gd < crate::engine::Script >, line: i32, column: i32, grab_focus: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Script >, i32, i32, bool);
            let args = (script, line, column, grab_focus,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "edit_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn edit_script(&mut self, script: Gd < crate::engine::Script >,) {
            self.edit_script_ex(script,) . done()
        }
        #[inline]
        pub fn edit_script_ex(&mut self, script: Gd < crate::engine::Script >,) -> ExEditScript < '_ > {
            ExEditScript::new(self, script,)
        }
        pub fn open_scene_from_path(&mut self, scene_filepath: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (scene_filepath,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_scene_from_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_scene_from_path(&mut self, scene_filepath: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (scene_filepath,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reload_scene_from_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_open_scenes(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_open_scenes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited_scene_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_scene(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn save_scene_as_full(&mut self, path: GString, with_preview: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (path, with_preview,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_scene_as", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_scene_as(&mut self, path: GString,) {
            self.save_scene_as_ex(path,) . done()
        }
        #[inline]
        pub fn save_scene_as_ex(&mut self, path: GString,) -> ExSaveSceneAs < '_ > {
            ExSaveSceneAs::new(self, path,)
        }
        pub fn save_all_scenes(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_all_scenes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mark_scene_as_unsaved(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mark_scene_as_unsaved", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_main_scene(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_main_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_current_scene(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_custom_scene(&mut self, scene_filepath: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (scene_filepath,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_custom_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_playing_scene(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing_scene(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_scene(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_movie_maker_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_movie_maker_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_movie_maker_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_movie_maker_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorInterface {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorInterface\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorInterface {
        
    }
    impl std::ops::Deref for EditorInterface {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorInterface {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorInterface > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorInterface::restart_editor_ex`][super::EditorInterface::restart_editor_ex]."]
#[must_use]
pub struct ExRestartEditor < 'a > {
    surround_object: &'a mut re_export::EditorInterface, save: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRestartEditor < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface,) -> Self {
        Self {
            surround_object, save: true,
        }
    }
    #[inline]
    pub fn save(self, value: bool) -> Self {
        Self {
            save: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::restart_editor_full(self.surround_object, self.save,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::get_editor_viewport_3d_ex`][super::EditorInterface::get_editor_viewport_3d_ex]."]
#[must_use]
pub struct ExGetEditorViewport3d < 'a > {
    surround_object: &'a re_export::EditorInterface, idx: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetEditorViewport3d < 'a > {
    fn new(surround_object: &'a re_export::EditorInterface,) -> Self {
        Self {
            surround_object, idx: 0i32,
        }
    }
    #[inline]
    pub fn idx(self, value: i32) -> Self {
        Self {
            idx: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::SubViewport > > {
        re_export::EditorInterface::get_editor_viewport_3d_full(self.surround_object, self.idx,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_ex`][super::EditorInterface::popup_dialog_ex]."]
#[must_use]
pub struct ExPopupDialog < 'a > {
    surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialog < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >,) -> Self {
        Self {
            surround_object, dialog, rect: Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn rect(self, value: Rect2i) -> Self {
        Self {
            rect: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::popup_dialog_full(self.surround_object, self.dialog, self.rect,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ex`][super::EditorInterface::popup_dialog_centered_ex]."]
#[must_use]
pub struct ExPopupDialogCentered < 'a > {
    surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCentered < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >,) -> Self {
        Self {
            surround_object, dialog, minsize: Vector2i::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::popup_dialog_centered_full(self.surround_object, self.dialog, self.minsize,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ratio_ex`][super::EditorInterface::popup_dialog_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredRatio < 'a > {
    surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >,) -> Self {
        Self {
            surround_object, dialog, ratio: 0.8f32,
        }
    }
    #[inline]
    pub fn ratio(self, value: f32) -> Self {
        Self {
            ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::popup_dialog_centered_ratio_full(self.surround_object, self.dialog, self.ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_clamped_ex`][super::EditorInterface::popup_dialog_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredClamped < 'a > {
    surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: Gd < crate::engine::Window >,) -> Self {
        Self {
            surround_object, dialog, minsize: Vector2i::new(0 as _, 0 as _), fallback_ratio: 0.75f32,
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, value: f32) -> Self {
        Self {
            fallback_ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::popup_dialog_centered_clamped_full(self.surround_object, self.dialog, self.minsize, self.fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::inspect_object_ex`][super::EditorInterface::inspect_object_ex]."]
#[must_use]
pub struct ExInspectObject < 'a > {
    surround_object: &'a mut re_export::EditorInterface, object: Gd < crate::engine::Object >, for_property: GString, inspector_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInspectObject < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: Gd < crate::engine::Object >,) -> Self {
        Self {
            surround_object, object, for_property: GString::from(""), inspector_only: false,
        }
    }
    #[inline]
    pub fn for_property(self, value: GString) -> Self {
        Self {
            for_property: value, .. self
        }
    }
    #[inline]
    pub fn inspector_only(self, value: bool) -> Self {
        Self {
            inspector_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::inspect_object_full(self.surround_object, self.object, self.for_property, self.inspector_only,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::edit_script_ex`][super::EditorInterface::edit_script_ex]."]
#[must_use]
pub struct ExEditScript < 'a > {
    surround_object: &'a mut re_export::EditorInterface, script: Gd < crate::engine::Script >, line: i32, column: i32, grab_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditScript < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, script: Gd < crate::engine::Script >,) -> Self {
        Self {
            surround_object, script, line: - 1i32, column: 0i32, grab_focus: true,
        }
    }
    #[inline]
    pub fn line(self, value: i32) -> Self {
        Self {
            line: value, .. self
        }
    }
    #[inline]
    pub fn column(self, value: i32) -> Self {
        Self {
            column: value, .. self
        }
    }
    #[inline]
    pub fn grab_focus(self, value: bool) -> Self {
        Self {
            grab_focus: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::edit_script_full(self.surround_object, self.script, self.line, self.column, self.grab_focus,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::save_scene_as_ex`][super::EditorInterface::save_scene_as_ex]."]
#[must_use]
pub struct ExSaveSceneAs < 'a > {
    surround_object: &'a mut re_export::EditorInterface, path: GString, with_preview: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveSceneAs < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, path: GString,) -> Self {
        Self {
            surround_object, path, with_preview: true,
        }
    }
    #[inline]
    pub fn with_preview(self, value: bool) -> Self {
        Self {
            with_preview: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorInterface::save_scene_as_full(self.surround_object, self.path, self.with_preview,)
    }
}