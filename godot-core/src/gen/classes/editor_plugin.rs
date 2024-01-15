#![doc = "Sidecar module for class [`EditorPlugin`][crate::engine::EditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorPlugin.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`editor_plugin`][crate::engine::editor_plugin]: sidecar module with related enum/flag types\n* [`IEditorPlugin`][crate::engine::IEditorPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorPlugin`][crate::engine::EditorPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented !()
        }
        fn forward_canvas_gui_input(&mut self, event: Gd < crate::engine::InputEvent >,) -> bool {
            unimplemented !()
        }
        fn forward_canvas_draw_over_viewport(&mut self, viewport_control: Gd < crate::engine::Control >,) {
            unimplemented !()
        }
        fn forward_canvas_force_draw_over_viewport(&mut self, viewport_control: Gd < crate::engine::Control >,) {
            unimplemented !()
        }
        fn forward_3d_gui_input(&mut self, viewport_camera: Gd < crate::engine::Camera3D >, event: Gd < crate::engine::InputEvent >,) -> i32 {
            unimplemented !()
        }
        fn forward_3d_draw_over_viewport(&mut self, viewport_control: Gd < crate::engine::Control >,) {
            unimplemented !()
        }
        fn forward_3d_force_draw_over_viewport(&mut self, viewport_control: Gd < crate::engine::Control >,) {
            unimplemented !()
        }
        fn get_plugin_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_plugin_icon(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            unimplemented !()
        }
        fn has_main_screen(&self,) -> bool {
            unimplemented !()
        }
        fn make_visible(&mut self, visible: bool,) {
            unimplemented !()
        }
        fn edit(&mut self, object: Gd < crate::engine::Object >,) {
            unimplemented !()
        }
        fn handles(&self, object: Gd < crate::engine::Object >,) -> bool {
            unimplemented !()
        }
        fn get_state(&self,) -> Dictionary {
            unimplemented !()
        }
        fn set_state(&mut self, state: Dictionary,) {
            unimplemented !()
        }
        fn clear(&mut self,) {
            unimplemented !()
        }
        fn get_unsaved_status(&self, for_scene: GString,) -> GString {
            unimplemented !()
        }
        fn save_external_data(&mut self,) {
            unimplemented !()
        }
        fn apply_changes(&mut self,) {
            unimplemented !()
        }
        fn get_breakpoints(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn set_window_layout(&mut self, configuration: Gd < crate::engine::ConfigFile >,) {
            unimplemented !()
        }
        fn get_window_layout(&mut self, configuration: Gd < crate::engine::ConfigFile >,) {
            unimplemented !()
        }
        fn build(&mut self,) -> bool {
            unimplemented !()
        }
        fn enable_plugin(&mut self,) {
            unimplemented !()
        }
        fn disable_plugin(&mut self,) {
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
    impl EditorPlugin {
        pub fn add_control_to_container(&mut self, container: crate::engine::editor_plugin::CustomControlContainer, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_plugin::CustomControlContainer, Gd < crate::engine::Control >);
            let args = (container, control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_control_to_container", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_control_to_bottom_panel(&mut self, control: Gd < crate::engine::Control >, title: GString,) -> Option < Gd < crate::engine::Button > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Button > >;
            type CallSig = (Option < Gd < crate::engine::Button > >, Gd < crate::engine::Control >, GString);
            let args = (control, title,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_control_to_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_control_to_dock(&mut self, slot: crate::engine::editor_plugin::DockSlot, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_plugin::DockSlot, Gd < crate::engine::Control >);
            let args = (slot, control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_control_to_dock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_control_from_docks(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_control_from_docks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_control_from_bottom_panel(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_control_from_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_control_from_container(&mut self, container: crate::engine::editor_plugin::CustomControlContainer, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::editor_plugin::CustomControlContainer, Gd < crate::engine::Control >);
            let args = (container, control,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_control_from_container", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tool_menu_item(&mut self, name: GString, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Callable);
            let args = (name, callable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_tool_menu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tool_submenu_item(&mut self, name: GString, submenu: Gd < crate::engine::PopupMenu >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::PopupMenu >);
            let args = (name, submenu,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_tool_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tool_menu_item(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_tool_menu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_export_as_menu(&mut self,) -> Option < Gd < crate::engine::PopupMenu > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PopupMenu > >;
            type CallSig = (Option < Gd < crate::engine::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_export_as_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_custom_type(&mut self, type_: GString, base: GString, script: Gd < crate::engine::Script >, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, Gd < crate::engine::Script >, Gd < crate::engine::Texture2D >);
            let args = (type_, base, script, icon,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_custom_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_custom_type(&mut self, type_: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_custom_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_autoload_singleton(&mut self, name: GString, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (name, path,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_autoload_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_autoload_singleton(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_autoload_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_overlays(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_overlays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_bottom_panel_item_visible(&mut self, item: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_bottom_panel_item_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide_bottom_panel(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hide_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_undo_redo(&mut self,) -> Option < Gd < crate::engine::EditorUndoRedoManager > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorUndoRedoManager > >;
            type CallSig = (Option < Gd < crate::engine::EditorUndoRedoManager > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_undo_redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_redo_inspector_hook_callback(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_undo_redo_inspector_hook_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_undo_redo_inspector_hook_callback(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_undo_redo_inspector_hook_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue_save_layout(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "queue_save_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_translation_parser_plugin(&mut self, parser: Gd < crate::engine::EditorTranslationParserPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorTranslationParserPlugin >);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_translation_parser_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_translation_parser_plugin(&mut self, parser: Gd < crate::engine::EditorTranslationParserPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorTranslationParserPlugin >);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_translation_parser_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_import_plugin_full(&mut self, importer: Gd < crate::engine::EditorImportPlugin >, first_priority: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorImportPlugin >, bool);
            let args = (importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_import_plugin(&mut self, importer: Gd < crate::engine::EditorImportPlugin >,) {
            self.add_import_plugin_ex(importer,) . done()
        }
        #[inline]
        pub fn add_import_plugin_ex(&mut self, importer: Gd < crate::engine::EditorImportPlugin >,) -> ExAddImportPlugin < '_ > {
            ExAddImportPlugin::new(self, importer,)
        }
        pub fn remove_import_plugin(&mut self, importer: Gd < crate::engine::EditorImportPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorImportPlugin >);
            let args = (importer,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_scene_format_importer_plugin_full(&mut self, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >, first_priority: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorSceneFormatImporter >, bool);
            let args = (scene_format_importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_scene_format_importer_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_scene_format_importer_plugin(&mut self, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >,) {
            self.add_scene_format_importer_plugin_ex(scene_format_importer,) . done()
        }
        #[inline]
        pub fn add_scene_format_importer_plugin_ex(&mut self, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >,) -> ExAddSceneFormatImporterPlugin < '_ > {
            ExAddSceneFormatImporterPlugin::new(self, scene_format_importer,)
        }
        pub fn remove_scene_format_importer_plugin(&mut self, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorSceneFormatImporter >);
            let args = (scene_format_importer,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_scene_format_importer_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_scene_post_import_plugin_full(&mut self, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >, first_priority: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorScenePostImportPlugin >, bool);
            let args = (scene_import_plugin, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_scene_post_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_scene_post_import_plugin(&mut self, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >,) {
            self.add_scene_post_import_plugin_ex(scene_import_plugin,) . done()
        }
        #[inline]
        pub fn add_scene_post_import_plugin_ex(&mut self, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >,) -> ExAddScenePostImportPlugin < '_ > {
            ExAddScenePostImportPlugin::new(self, scene_import_plugin,)
        }
        pub fn remove_scene_post_import_plugin(&mut self, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorScenePostImportPlugin >);
            let args = (scene_import_plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_scene_post_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_export_plugin(&mut self, plugin: Gd < crate::engine::EditorExportPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorExportPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_export_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_export_plugin(&mut self, plugin: Gd < crate::engine::EditorExportPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorExportPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_export_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_node_3d_gizmo_plugin(&mut self, plugin: Gd < crate::engine::EditorNode3DGizmoPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorNode3DGizmoPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_node_3d_gizmo_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node_3d_gizmo_plugin(&mut self, plugin: Gd < crate::engine::EditorNode3DGizmoPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorNode3DGizmoPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_node_3d_gizmo_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_inspector_plugin(&mut self, plugin: Gd < crate::engine::EditorInspectorPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorInspectorPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_inspector_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_inspector_plugin(&mut self, plugin: Gd < crate::engine::EditorInspectorPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorInspectorPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_inspector_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_resource_conversion_plugin(&mut self, plugin: Gd < crate::engine::EditorResourceConversionPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorResourceConversionPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_resource_conversion_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_resource_conversion_plugin(&mut self, plugin: Gd < crate::engine::EditorResourceConversionPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorResourceConversionPlugin >);
            let args = (plugin,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_resource_conversion_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_event_forwarding_always_enabled(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_event_forwarding_always_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_draw_over_forwarding_enabled(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_force_draw_over_forwarding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_interface(&mut self,) -> Option < Gd < crate::engine::EditorInterface > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorInterface > >;
            type CallSig = (Option < Gd < crate::engine::EditorInterface > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_editor_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_create_dialog(&mut self,) -> Option < Gd < crate::engine::ScriptCreateDialog > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ScriptCreateDialog > >;
            type CallSig = (Option < Gd < crate::engine::ScriptCreateDialog > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script_create_dialog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_debugger_plugin(&mut self, script: Gd < crate::engine::EditorDebuggerPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorDebuggerPlugin >);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_debugger_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_debugger_plugin(&mut self, script: Gd < crate::engine::EditorDebuggerPlugin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::EditorDebuggerPlugin >);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_debugger_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_plugin_version(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_plugin_version", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorPlugin {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorPlugin {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for EditorPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorPlugin {
        
    }
    impl crate::obj::ExportableObject for EditorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorPlugin {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorPlugin > for $Class {
                
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
#[doc = "Default-param extender for [`EditorPlugin::add_import_plugin_ex`][super::EditorPlugin::add_import_plugin_ex]."]
#[must_use]
pub struct ExAddImportPlugin < 'a > {
    surround_object: &'a mut re_export::EditorPlugin, importer: Gd < crate::engine::EditorImportPlugin >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, importer: Gd < crate::engine::EditorImportPlugin >,) -> Self {
        Self {
            surround_object, importer, first_priority: false,
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
        re_export::EditorPlugin::add_import_plugin_full(self.surround_object, self.importer, self.first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_format_importer_plugin_ex`][super::EditorPlugin::add_scene_format_importer_plugin_ex]."]
#[must_use]
pub struct ExAddSceneFormatImporterPlugin < 'a > {
    surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSceneFormatImporterPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: Gd < crate::engine::EditorSceneFormatImporter >,) -> Self {
        Self {
            surround_object, scene_format_importer, first_priority: false,
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
        re_export::EditorPlugin::add_scene_format_importer_plugin_full(self.surround_object, self.scene_format_importer, self.first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_post_import_plugin_ex`][super::EditorPlugin::add_scene_post_import_plugin_ex]."]
#[must_use]
pub struct ExAddScenePostImportPlugin < 'a > {
    surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddScenePostImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: Gd < crate::engine::EditorScenePostImportPlugin >,) -> Self {
        Self {
            surround_object, scene_import_plugin, first_priority: false,
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
        re_export::EditorPlugin::add_scene_post_import_plugin_full(self.surround_object, self.scene_import_plugin, self.first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomControlContainer {
    ord: i32
}
impl CustomControlContainer {
    pub const CONTAINER_TOOLBAR: Self = Self {
        ord: 0i32
    };
    pub const CONTAINER_SPATIAL_EDITOR_MENU: Self = Self {
        ord: 1i32
    };
    pub const CONTAINER_SPATIAL_EDITOR_SIDE_LEFT: Self = Self {
        ord: 2i32
    };
    pub const CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT: Self = Self {
        ord: 3i32
    };
    pub const CONTAINER_SPATIAL_EDITOR_BOTTOM: Self = Self {
        ord: 4i32
    };
    pub const CONTAINER_CANVAS_EDITOR_MENU: Self = Self {
        ord: 5i32
    };
    pub const CONTAINER_CANVAS_EDITOR_SIDE_LEFT: Self = Self {
        ord: 6i32
    };
    pub const CONTAINER_CANVAS_EDITOR_SIDE_RIGHT: Self = Self {
        ord: 7i32
    };
    pub const CONTAINER_CANVAS_EDITOR_BOTTOM: Self = Self {
        ord: 8i32
    };
    pub const CONTAINER_INSPECTOR_BOTTOM: Self = Self {
        ord: 9i32
    };
    pub const CONTAINER_PROJECT_SETTING_TAB_LEFT: Self = Self {
        ord: 10i32
    };
    pub const CONTAINER_PROJECT_SETTING_TAB_RIGHT: Self = Self {
        ord: 11i32
    };
    
}
impl crate::obj::EngineEnum for CustomControlContainer {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CustomControlContainer {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CustomControlContainer {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CustomControlContainer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DockSlot {
    ord: i32
}
impl DockSlot {
    pub const DOCK_SLOT_LEFT_UL: Self = Self {
        ord: 0i32
    };
    pub const DOCK_SLOT_LEFT_BL: Self = Self {
        ord: 1i32
    };
    pub const DOCK_SLOT_LEFT_UR: Self = Self {
        ord: 2i32
    };
    pub const DOCK_SLOT_LEFT_BR: Self = Self {
        ord: 3i32
    };
    pub const DOCK_SLOT_RIGHT_UL: Self = Self {
        ord: 4i32
    };
    pub const DOCK_SLOT_RIGHT_BL: Self = Self {
        ord: 5i32
    };
    pub const DOCK_SLOT_RIGHT_UR: Self = Self {
        ord: 6i32
    };
    pub const DOCK_SLOT_RIGHT_BR: Self = Self {
        ord: 7i32
    };
    pub const DOCK_SLOT_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for DockSlot {
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
impl crate::obj::IndexEnum for DockSlot {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for DockSlot {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DockSlot {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DockSlot {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AfterGUIInput {
    ord: i32
}
impl AfterGUIInput {
    pub const AFTER_GUI_INPUT_PASS: Self = Self {
        ord: 0i32
    };
    pub const AFTER_GUI_INPUT_STOP: Self = Self {
        ord: 1i32
    };
    pub const AFTER_GUI_INPUT_CUSTOM: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AfterGUIInput {
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
impl crate::builtin::meta::GodotConvert for AfterGUIInput {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AfterGUIInput {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AfterGUIInput {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}