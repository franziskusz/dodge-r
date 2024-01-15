#![doc = "Sidecar module for class [`TextEdit`][crate::engine::TextEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextEdit` enums](https://docs.godotengine.org/en/stable/classes/class_textedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextEdit.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`text_edit`][crate::engine::text_edit]: sidecar module with related enum/flag types\n* [`ITextEdit`][crate::engine::ITextEdit]: virtual methods\n\n\nSee also [Godot docs for `TextEdit`](https://docs.godotengine.org/en/stable/classes/class_textedit.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextEdit`][crate::engine::TextEdit].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextEdit` methods](https://docs.godotengine.org/en/stable/classes/class_textedit.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextEdit: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
            unimplemented !()
        }
        fn handle_unicode_input(&mut self, unicode_char: i32, caret_index: i32,) {
            unimplemented !()
        }
        fn backspace(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn cut(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn copy(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste_primary_clipboard(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::engine::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl TextEdit {
        pub fn has_ime_text(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_ime_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::engine::text_server::StructuredTextParser,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::engine::text_server::StructuredTextParser {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::StructuredTextParser >;
            type CallSig = (crate::engine::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (args,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_overtype_mode_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_overtype_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_overtype_mode_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_overtype_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_virtual_keyboard_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_virtual_keyboard_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_virtual_keyboard_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_virtual_keyboard_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_middle_mouse_paste_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_middle_mouse_paste_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_middle_mouse_paste_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_middle_mouse_paste_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_placeholder(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_placeholder(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line(&mut self, line: i32, new_text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (line, new_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line(&self, line: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_line_width_full(&self, line: i32, wrap_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_line_width(&self, line: i32,) -> i32 {
            self.get_line_width_ex(line,) . done()
        }
        #[inline]
        pub fn get_line_width_ex(&self, line: i32,) -> ExGetLineWidth < '_ > {
            ExGetLineWidth::new(self, line,)
        }
        pub fn get_line_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_indent_level(&self, line: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_indent_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_first_non_whitespace_column(&self, line: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_first_non_whitespace_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn swap_lines(&mut self, from_line: i32, to_line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "swap_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn insert_line_at(&mut self, line: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (line, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "insert_line_at", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn insert_text_at_caret_full(&mut self, text: GString, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (text, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "insert_text_at_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn insert_text_at_caret(&mut self, text: GString,) {
            self.insert_text_at_caret_ex(text,) . done()
        }
        #[inline]
        pub fn insert_text_at_caret_ex(&mut self, text: GString,) -> ExInsertTextAtCaret < '_ > {
            ExInsertTextAtCaret::new(self, text,)
        }
        pub fn remove_text(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32, i32);
            let args = (from_line, from_column, to_line, to_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_unhidden_line(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_unhidden_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_visible_line_offset_from(&self, line: i32, visible_amount: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_visible_line_offset_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_visible_line_index_offset_from(&self, line: i32, wrap_index: i32, visible_amount: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32, i32, i32);
            let args = (line, wrap_index, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_visible_line_index_offset_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn backspace_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "backspace", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn backspace(&mut self,) {
            self.backspace_ex() . done()
        }
        #[inline]
        pub fn backspace_ex(&mut self,) -> ExBackspace < '_ > {
            ExBackspace::new(self,)
        }
        pub(crate) fn cut_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn cut(&mut self,) {
            self.cut_ex() . done()
        }
        #[inline]
        pub fn cut_ex(&mut self,) -> ExCut < '_ > {
            ExCut::new(self,)
        }
        pub(crate) fn copy_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn copy(&mut self,) {
            self.copy_ex() . done()
        }
        #[inline]
        pub fn copy_ex(&mut self,) -> ExCopy < '_ > {
            ExCopy::new(self,)
        }
        pub(crate) fn paste_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "paste", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn paste(&mut self,) {
            self.paste_ex() . done()
        }
        #[inline]
        pub fn paste_ex(&mut self,) -> ExPaste < '_ > {
            ExPaste::new(self,)
        }
        pub(crate) fn paste_primary_clipboard_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "paste_primary_clipboard", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn paste_primary_clipboard(&mut self,) {
            self.paste_primary_clipboard_ex() . done()
        }
        #[inline]
        pub fn paste_primary_clipboard_ex(&mut self,) -> ExPastePrimaryClipboard < '_ > {
            ExPastePrimaryClipboard::new(self,)
        }
        pub fn start_action(&mut self, action: crate::engine::text_edit::EditAction,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_edit::EditAction);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_action(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "end_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin_complex_operation(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "begin_complex_operation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_complex_operation(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "end_complex_operation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_undo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn undo(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "undo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn redo(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_undo_history(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_undo_history", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tag_saved_version(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tag_saved_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_saved_version(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_saved_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_search_text(&mut self, search_text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (search_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_search_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_search_flags(&mut self, flags: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_search_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn search(&self, text: GString, flags: u32, from_line: i32, from_colum: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, GString, u32, i32, i32);
            let args = (text, flags, from_line, from_colum,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tooltip_request_func(&mut self, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tooltip_request_func", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_mouse_pos(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_mouse_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_word_at_pos(&self, position: Vector2,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_word_at_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_line_column_at_pos_full(&self, position: Vector2i, allow_out_of_bounds: bool,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i, bool);
            let args = (position, allow_out_of_bounds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_column_at_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_line_column_at_pos(&self, position: Vector2i,) -> Vector2i {
            self.get_line_column_at_pos_ex(position,) . done()
        }
        #[inline]
        pub fn get_line_column_at_pos_ex(&self, position: Vector2i,) -> ExGetLineColumnAtPos < '_ > {
            ExGetLineColumnAtPos::new(self, position,)
        }
        pub fn get_pos_at_line_column(&self, line: i32, column: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pos_at_line_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rect_at_line_column(&self, line: i32, column: i32,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rect_at_line_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimap_line_at_pos(&self, position: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_minimap_line_at_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dragging_cursor(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_dragging_cursor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_mouse_over_selection_full(&self, edges: bool, caret_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, bool, i32);
            let args = (edges, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_mouse_over_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_mouse_over_selection(&self, edges: bool,) -> bool {
            self.is_mouse_over_selection_ex(edges,) . done()
        }
        #[inline]
        pub fn is_mouse_over_selection_ex(&self, edges: bool,) -> ExIsMouseOverSelection < '_ > {
            ExIsMouseOverSelection::new(self, edges,)
        }
        pub fn set_caret_type(&mut self, type_: crate::engine::text_edit::CaretType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_edit::CaretType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_caret_type(&self,) -> crate::engine::text_edit::CaretType {
            type RetMarshal = PtrcallReturnT < crate::engine::text_edit::CaretType >;
            type CallSig = (crate::engine::text_edit::CaretType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_caret_blink_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_blink_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_caret_blink_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_caret_blink_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_caret_blink_interval(&mut self, interval: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_blink_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_caret_blink_interval(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_blink_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_caret_when_editable_disabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_caret_when_editable_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_caret_when_editable_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_caret_when_editable_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_move_caret_on_right_click_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_move_caret_on_right_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_move_caret_on_right_click_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_move_caret_on_right_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_caret_mid_grapheme_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_mid_grapheme_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_caret_mid_grapheme_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_caret_mid_grapheme_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multiple_carets_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multiple_carets_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multiple_carets_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_multiple_carets_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_caret(&mut self, line: i32, col: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, col,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_caret(&mut self, caret: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_secondary_carets(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_secondary_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn merge_overlapping_carets(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "merge_overlapping_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_caret_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_caret_at_carets(&mut self, below: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_caret_at_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_caret_index_edit_order(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_index_edit_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn adjust_carets_after_edit(&mut self, caret: i32, from_line: i32, from_col: i32, to_line: i32, to_col: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32, i32, i32);
            let args = (caret, from_line, from_col, to_line, to_col,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "adjust_carets_after_edit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_caret_visible_full(&self, caret_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_caret_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_caret_visible(&self,) -> bool {
            self.is_caret_visible_ex() . done()
        }
        #[inline]
        pub fn is_caret_visible_ex(&self,) -> ExIsCaretVisible < '_ > {
            ExIsCaretVisible::new(self,)
        }
        pub(crate) fn get_caret_draw_pos_full(&self, caret_index: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_draw_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_caret_draw_pos(&self,) -> Vector2 {
            self.get_caret_draw_pos_ex() . done()
        }
        #[inline]
        pub fn get_caret_draw_pos_ex(&self,) -> ExGetCaretDrawPos < '_ > {
            ExGetCaretDrawPos::new(self,)
        }
        pub(crate) fn set_caret_line_full(&mut self, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool, bool, i32, i32);
            let args = (line, adjust_viewport, can_be_hidden, wrap_index, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_caret_line(&mut self, line: i32,) {
            self.set_caret_line_ex(line,) . done()
        }
        #[inline]
        pub fn set_caret_line_ex(&mut self, line: i32,) -> ExSetCaretLine < '_ > {
            ExSetCaretLine::new(self, line,)
        }
        pub(crate) fn get_caret_line_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_caret_line(&self,) -> i32 {
            self.get_caret_line_ex() . done()
        }
        #[inline]
        pub fn get_caret_line_ex(&self,) -> ExGetCaretLine < '_ > {
            ExGetCaretLine::new(self,)
        }
        pub(crate) fn set_caret_column_full(&mut self, column: i32, adjust_viewport: bool, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool, i32);
            let args = (column, adjust_viewport, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_caret_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_caret_column(&mut self, column: i32,) {
            self.set_caret_column_ex(column,) . done()
        }
        #[inline]
        pub fn set_caret_column_ex(&mut self, column: i32,) -> ExSetCaretColumn < '_ > {
            ExSetCaretColumn::new(self, column,)
        }
        pub(crate) fn get_caret_column_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_caret_column(&self,) -> i32 {
            self.get_caret_column_ex() . done()
        }
        #[inline]
        pub fn get_caret_column_ex(&self,) -> ExGetCaretColumn < '_ > {
            ExGetCaretColumn::new(self,)
        }
        pub(crate) fn get_caret_wrap_index_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_caret_wrap_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_caret_wrap_index(&self,) -> i32 {
            self.get_caret_wrap_index_ex() . done()
        }
        #[inline]
        pub fn get_caret_wrap_index_ex(&self,) -> ExGetCaretWrapIndex < '_ > {
            ExGetCaretWrapIndex::new(self,)
        }
        pub(crate) fn get_word_under_caret_full(&self, caret_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_word_under_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_word_under_caret(&self,) -> GString {
            self.get_word_under_caret_ex() . done()
        }
        #[inline]
        pub fn get_word_under_caret_ex(&self,) -> ExGetWordUnderCaret < '_ > {
            ExGetWordUnderCaret::new(self,)
        }
        pub fn set_selecting_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_selecting_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selecting_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_selecting_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_selection_mode_full(&mut self, mode: crate::engine::text_edit::SelectionMode, line: i32, column: i32, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_edit::SelectionMode, i32, i32, i32);
            let args = (mode, line, column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_selection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_selection_mode(&mut self, mode: crate::engine::text_edit::SelectionMode,) {
            self.set_selection_mode_ex(mode,) . done()
        }
        #[inline]
        pub fn set_selection_mode_ex(&mut self, mode: crate::engine::text_edit::SelectionMode,) -> ExSetSelectionMode < '_ > {
            ExSetSelectionMode::new(self, mode,)
        }
        pub fn get_selection_mode(&self,) -> crate::engine::text_edit::SelectionMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_edit::SelectionMode >;
            type CallSig = (crate::engine::text_edit::SelectionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn select_word_under_caret_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_word_under_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn select_word_under_caret(&mut self,) {
            self.select_word_under_caret_ex() . done()
        }
        #[inline]
        pub fn select_word_under_caret_ex(&mut self,) -> ExSelectWordUnderCaret < '_ > {
            ExSelectWordUnderCaret::new(self,)
        }
        pub fn add_selection_for_next_occurrence(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_selection_for_next_occurrence", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn select_full(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32, i32, i32);
            let args = (from_line, from_column, to_line, to_column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn select(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            self.select_ex(from_line, from_column, to_line, to_column,) . done()
        }
        #[inline]
        pub fn select_ex(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> ExSelect < '_ > {
            ExSelect::new(self, from_line, from_column, to_line, to_column,)
        }
        pub(crate) fn has_selection_full(&self, caret_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_selection(&self,) -> bool {
            self.has_selection_ex() . done()
        }
        #[inline]
        pub fn has_selection_ex(&self,) -> ExHasSelection < '_ > {
            ExHasSelection::new(self,)
        }
        pub(crate) fn get_selected_text_full(&mut self, caret_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selected_text(&mut self,) -> GString {
            self.get_selected_text_ex() . done()
        }
        #[inline]
        pub fn get_selected_text_ex(&mut self,) -> ExGetSelectedText < '_ > {
            ExGetSelectedText::new(self,)
        }
        pub(crate) fn get_selection_line_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_line(&self,) -> i32 {
            self.get_selection_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_line_ex(&self,) -> ExGetSelectionLine < '_ > {
            ExGetSelectionLine::new(self,)
        }
        pub(crate) fn get_selection_column_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_column(&self,) -> i32 {
            self.get_selection_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_column_ex(&self,) -> ExGetSelectionColumn < '_ > {
            ExGetSelectionColumn::new(self,)
        }
        pub(crate) fn get_selection_from_line_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_from_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_from_line(&self,) -> i32 {
            self.get_selection_from_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_from_line_ex(&self,) -> ExGetSelectionFromLine < '_ > {
            ExGetSelectionFromLine::new(self,)
        }
        pub(crate) fn get_selection_from_column_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_from_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_from_column(&self,) -> i32 {
            self.get_selection_from_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_from_column_ex(&self,) -> ExGetSelectionFromColumn < '_ > {
            ExGetSelectionFromColumn::new(self,)
        }
        pub(crate) fn get_selection_to_line_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_to_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_to_line(&self,) -> i32 {
            self.get_selection_to_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_to_line_ex(&self,) -> ExGetSelectionToLine < '_ > {
            ExGetSelectionToLine::new(self,)
        }
        pub(crate) fn get_selection_to_column_full(&self, caret_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_to_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selection_to_column(&self,) -> i32 {
            self.get_selection_to_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_to_column_ex(&self,) -> ExGetSelectionToColumn < '_ > {
            ExGetSelectionToColumn::new(self,)
        }
        pub(crate) fn deselect_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn deselect(&mut self,) {
            self.deselect_ex() . done()
        }
        #[inline]
        pub fn deselect_ex(&mut self,) -> ExDeselect < '_ > {
            ExDeselect::new(self,)
        }
        pub(crate) fn delete_selection_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "delete_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn delete_selection(&mut self,) {
            self.delete_selection_ex() . done()
        }
        #[inline]
        pub fn delete_selection_ex(&mut self,) -> ExDeleteSelection < '_ > {
            ExDeleteSelection::new(self,)
        }
        pub fn set_line_wrapping_mode(&mut self, mode: crate::engine::text_edit::LineWrappingMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_edit::LineWrappingMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_wrapping_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_wrapping_mode(&self,) -> crate::engine::text_edit::LineWrappingMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_edit::LineWrappingMode >;
            type CallSig = (crate::engine::text_edit::LineWrappingMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_wrapping_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::engine::text_server::AutowrapMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::engine::text_server::AutowrapMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::AutowrapMode >;
            type CallSig = (crate::engine::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_wrapped(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_wrapped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_wrap_count(&self, line: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_wrap_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_wrap_index_at_column(&self, line: i32, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_wrap_index_at_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_wrapped_text(&self, line: i32,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_wrapped_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_smooth_scroll_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_smooth_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_smooth_scroll_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_smooth_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&self,) -> Option < Gd < crate::engine::VScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_scroll_bar(&self,) -> Option < Gd < crate::engine::HScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::HScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::HScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_scroll(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_scroll(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_scroll(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_past_end_of_file_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scroll_past_end_of_file_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scroll_past_end_of_file_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_scroll_past_end_of_file_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_scroll_speed(&mut self, speed: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_scroll_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_speed(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fit_content_height_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fit_content_height_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fit_content_height_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_fit_content_height_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_scroll_pos_for_line_full(&self, line: i32, wrap_index: i32,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, i32, i32);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scroll_pos_for_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_scroll_pos_for_line(&self, line: i32,) -> f64 {
            self.get_scroll_pos_for_line_ex(line,) . done()
        }
        #[inline]
        pub fn get_scroll_pos_for_line_ex(&self, line: i32,) -> ExGetScrollPosForLine < '_ > {
            ExGetScrollPosForLine::new(self, line,)
        }
        pub(crate) fn set_line_as_first_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_first_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_line_as_first_visible(&mut self, line: i32,) {
            self.set_line_as_first_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_first_visible_ex(&mut self, line: i32,) -> ExSetLineAsFirstVisible < '_ > {
            ExSetLineAsFirstVisible::new(self, line,)
        }
        pub fn get_first_visible_line(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_first_visible_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_line_as_center_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_center_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_line_as_center_visible(&mut self, line: i32,) {
            self.set_line_as_center_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_center_visible_ex(&mut self, line: i32,) -> ExSetLineAsCenterVisible < '_ > {
            ExSetLineAsCenterVisible::new(self, line,)
        }
        pub(crate) fn set_line_as_last_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_last_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_line_as_last_visible(&mut self, line: i32,) {
            self.set_line_as_last_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_last_visible_ex(&mut self, line: i32,) -> ExSetLineAsLastVisible < '_ > {
            ExSetLineAsLastVisible::new(self, line,)
        }
        pub fn get_last_full_visible_line(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_full_visible_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_full_visible_line_wrap_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_full_visible_line_wrap_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_line_count_in_range(&self, from_line: i32, to_line: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_line_count_in_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_visible_line_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_visible_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn adjust_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "adjust_viewport_to_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn adjust_viewport_to_caret(&mut self,) {
            self.adjust_viewport_to_caret_ex() . done()
        }
        #[inline]
        pub fn adjust_viewport_to_caret_ex(&mut self,) -> ExAdjustViewportToCaret < '_ > {
            ExAdjustViewportToCaret::new(self,)
        }
        pub(crate) fn center_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "center_viewport_to_caret", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn center_viewport_to_caret(&mut self,) {
            self.center_viewport_to_caret_ex() . done()
        }
        #[inline]
        pub fn center_viewport_to_caret_ex(&mut self,) -> ExCenterViewportToCaret < '_ > {
            ExCenterViewportToCaret::new(self,)
        }
        pub fn set_draw_minimap(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_minimap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_minimap(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_minimap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_minimap_width(&mut self, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_minimap_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimap_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_minimap_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimap_visible_lines(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_minimap_visible_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_gutter_full(&mut self, at: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (at,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_gutter(&mut self,) {
            self.add_gutter_ex() . done()
        }
        #[inline]
        pub fn add_gutter_ex(&mut self,) -> ExAddGutter < '_ > {
            ExAddGutter::new(self,)
        }
        pub fn remove_gutter(&mut self, gutter: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gutter_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gutter_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_name(&mut self, gutter: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (gutter, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gutter_name(&self, gutter: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gutter_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_type(&mut self, gutter: i32, type_: crate::engine::text_edit::GutterType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::text_edit::GutterType);
            let args = (gutter, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gutter_type(&self, gutter: i32,) -> crate::engine::text_edit::GutterType {
            type RetMarshal = PtrcallReturnT < crate::engine::text_edit::GutterType >;
            type CallSig = (crate::engine::text_edit::GutterType, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gutter_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_width(&mut self, gutter: i32, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (gutter, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gutter_width(&self, gutter: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gutter_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_draw(&mut self, gutter: i32, draw: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (gutter, draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_gutter_drawn(&self, gutter: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_gutter_drawn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_clickable(&mut self, gutter: i32, clickable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_clickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_gutter_clickable(&self, gutter: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_gutter_clickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_overwritable(&mut self, gutter: i32, overwritable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (gutter, overwritable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_overwritable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_gutter_overwritable(&self, gutter: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_gutter_overwritable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn merge_gutters(&mut self, from_line: i32, to_line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "merge_gutters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gutter_custom_draw(&mut self, column: i32, draw_callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Callable);
            let args = (column, draw_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gutter_custom_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_gutter_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_gutter_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_gutter_metadata(&mut self, line: i32, gutter: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Variant);
            let args = (line, gutter, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_gutter_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_gutter_metadata(&self, line: i32, gutter: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32, i32);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_gutter_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_gutter_text(&mut self, line: i32, gutter: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, GString);
            let args = (line, gutter, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_gutter_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_gutter_text(&self, line: i32, gutter: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32, i32);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_gutter_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_gutter_icon(&mut self, line: i32, gutter: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Gd < crate::engine::Texture2D >);
            let args = (line, gutter, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_gutter_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_gutter_icon(&self, line: i32, gutter: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32, i32);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_gutter_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_gutter_item_color(&mut self, line: i32, gutter: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Color);
            let args = (line, gutter, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_gutter_item_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_gutter_item_color(&self, line: i32, gutter: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32, i32);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_gutter_item_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_gutter_clickable(&mut self, line: i32, gutter: i32, clickable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (line, gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_gutter_clickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_gutter_clickable(&self, line: i32, gutter: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_gutter_clickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_background_color(&mut self, line: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (line, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_background_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_background_color(&self, line: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_background_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_syntax_highlighter(&mut self, syntax_highlighter: Gd < crate::engine::SyntaxHighlighter >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::SyntaxHighlighter >);
            let args = (syntax_highlighter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_syntax_highlighter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_syntax_highlighter(&self,) -> Option < Gd < crate::engine::SyntaxHighlighter > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SyntaxHighlighter > >;
            type CallSig = (Option < Gd < crate::engine::SyntaxHighlighter > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_syntax_highlighter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_highlight_current_line(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_highlight_current_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_highlight_current_line_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_highlight_current_line_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_highlight_all_occurrences(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_highlight_all_occurrences", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_highlight_all_occurrences_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_highlight_all_occurrences_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_control_chars(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_control_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_control_chars(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_control_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_tabs(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_tabs(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_spaces(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_spaces(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::engine::PopupMenu > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PopupMenu > >;
            type CallSig = (Option < Gd < crate::engine::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_menu_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "menu_option", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextEdit {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TextEdit\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TextEdit {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for TextEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for TextEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for TextEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TextEdit {
        
    }
    impl crate::obj::ExportableObject for TextEdit {
        
    }
    impl crate::obj::cap::GodotDefault for TextEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextEdit {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TextEdit {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TextEdit > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Control > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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
#[doc = "Default-param extender for [`TextEdit::get_line_width_ex`][super::TextEdit::get_line_width_ex]."]
#[must_use]
pub struct ExGetLineWidth < 'a > {
    surround_object: &'a re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineWidth < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, wrap_index: - 1i32,
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_line_width_full(self.surround_object, self.line, self.wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::insert_text_at_caret_ex`][super::TextEdit::insert_text_at_caret_ex]."]
#[must_use]
pub struct ExInsertTextAtCaret < 'a > {
    surround_object: &'a mut re_export::TextEdit, text: GString, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInsertTextAtCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, text: GString,) -> Self {
        Self {
            surround_object, text, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::insert_text_at_caret_full(self.surround_object, self.text, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::backspace_ex`][super::TextEdit::backspace_ex]."]
#[must_use]
pub struct ExBackspace < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBackspace < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::backspace_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::cut_ex`][super::TextEdit::cut_ex]."]
#[must_use]
pub struct ExCut < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCut < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::cut_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::copy_ex`][super::TextEdit::copy_ex]."]
#[must_use]
pub struct ExCopy < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopy < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::copy_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_ex`][super::TextEdit::paste_ex]."]
#[must_use]
pub struct ExPaste < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPaste < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::paste_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_primary_clipboard_ex`][super::TextEdit::paste_primary_clipboard_ex]."]
#[must_use]
pub struct ExPastePrimaryClipboard < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPastePrimaryClipboard < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::paste_primary_clipboard_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_line_column_at_pos_ex`][super::TextEdit::get_line_column_at_pos_ex]."]
#[must_use]
pub struct ExGetLineColumnAtPos < 'a > {
    surround_object: &'a re_export::TextEdit, position: Vector2i, allow_out_of_bounds: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineColumnAtPos < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, position: Vector2i,) -> Self {
        Self {
            surround_object, position, allow_out_of_bounds: true,
        }
    }
    #[inline]
    pub fn allow_out_of_bounds(self, value: bool) -> Self {
        Self {
            allow_out_of_bounds: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::TextEdit::get_line_column_at_pos_full(self.surround_object, self.position, self.allow_out_of_bounds,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_mouse_over_selection_ex`][super::TextEdit::is_mouse_over_selection_ex]."]
#[must_use]
pub struct ExIsMouseOverSelection < 'a > {
    surround_object: &'a re_export::TextEdit, edges: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsMouseOverSelection < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, edges: bool,) -> Self {
        Self {
            surround_object, edges, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextEdit::is_mouse_over_selection_full(self.surround_object, self.edges, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_caret_visible_ex`][super::TextEdit::is_caret_visible_ex]."]
#[must_use]
pub struct ExIsCaretVisible < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCaretVisible < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextEdit::is_caret_visible_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_draw_pos_ex`][super::TextEdit::get_caret_draw_pos_ex]."]
#[must_use]
pub struct ExGetCaretDrawPos < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretDrawPos < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        re_export::TextEdit::get_caret_draw_pos_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_line_ex`][super::TextEdit::set_caret_line_ex]."]
#[must_use]
pub struct ExSetCaretLine < 'a > {
    surround_object: &'a mut re_export::TextEdit, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCaretLine < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, adjust_viewport: true, can_be_hidden: true, wrap_index: 0i32, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, value: bool) -> Self {
        Self {
            adjust_viewport: value, .. self
        }
    }
    #[inline]
    pub fn can_be_hidden(self, value: bool) -> Self {
        Self {
            can_be_hidden: value, .. self
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_caret_line_full(self.surround_object, self.line, self.adjust_viewport, self.can_be_hidden, self.wrap_index, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_line_ex`][super::TextEdit::get_caret_line_ex]."]
#[must_use]
pub struct ExGetCaretLine < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_caret_line_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_column_ex`][super::TextEdit::set_caret_column_ex]."]
#[must_use]
pub struct ExSetCaretColumn < 'a > {
    surround_object: &'a mut re_export::TextEdit, column: i32, adjust_viewport: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCaretColumn < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, column: i32,) -> Self {
        Self {
            surround_object, column, adjust_viewport: true, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, value: bool) -> Self {
        Self {
            adjust_viewport: value, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_caret_column_full(self.surround_object, self.column, self.adjust_viewport, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_column_ex`][super::TextEdit::get_caret_column_ex]."]
#[must_use]
pub struct ExGetCaretColumn < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_caret_column_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_wrap_index_ex`][super::TextEdit::get_caret_wrap_index_ex]."]
#[must_use]
pub struct ExGetCaretWrapIndex < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretWrapIndex < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_caret_wrap_index_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_word_under_caret_ex`][super::TextEdit::get_word_under_caret_ex]."]
#[must_use]
pub struct ExGetWordUnderCaret < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetWordUnderCaret < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextEdit::get_word_under_caret_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_selection_mode_ex`][super::TextEdit::set_selection_mode_ex]."]
#[must_use]
pub struct ExSetSelectionMode < 'a > {
    surround_object: &'a mut re_export::TextEdit, mode: crate::engine::text_edit::SelectionMode, line: i32, column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetSelectionMode < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, mode: crate::engine::text_edit::SelectionMode,) -> Self {
        Self {
            surround_object, mode, line: - 1i32, column: - 1i32, caret_index: 0i32,
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
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_selection_mode_full(self.surround_object, self.mode, self.line, self.column, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_word_under_caret_ex`][super::TextEdit::select_word_under_caret_ex]."]
#[must_use]
pub struct ExSelectWordUnderCaret < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelectWordUnderCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::select_word_under_caret_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_ex`][super::TextEdit::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    surround_object: &'a mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> Self {
        Self {
            surround_object, from_line, from_column, to_line, to_column, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::select_full(self.surround_object, self.from_line, self.from_column, self.to_line, self.to_column, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::has_selection_ex`][super::TextEdit::has_selection_ex]."]
#[must_use]
pub struct ExHasSelection < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasSelection < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextEdit::has_selection_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selected_text_ex`][super::TextEdit::get_selected_text_ex]."]
#[must_use]
pub struct ExGetSelectedText < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectedText < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextEdit::get_selected_text_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_line_ex`][super::TextEdit::get_selection_line_ex]."]
#[must_use]
pub struct ExGetSelectionLine < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_line_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_column_ex`][super::TextEdit::get_selection_column_ex]."]
#[must_use]
pub struct ExGetSelectionColumn < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_column_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_line_ex`][super::TextEdit::get_selection_from_line_ex]."]
#[must_use]
pub struct ExGetSelectionFromLine < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionFromLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_from_line_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_column_ex`][super::TextEdit::get_selection_from_column_ex]."]
#[must_use]
pub struct ExGetSelectionFromColumn < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionFromColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_from_column_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_line_ex`][super::TextEdit::get_selection_to_line_ex]."]
#[must_use]
pub struct ExGetSelectionToLine < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionToLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_to_line_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_column_ex`][super::TextEdit::get_selection_to_column_ex]."]
#[must_use]
pub struct ExGetSelectionToColumn < 'a > {
    surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionToColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TextEdit::get_selection_to_column_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::deselect_ex`][super::TextEdit::deselect_ex]."]
#[must_use]
pub struct ExDeselect < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeselect < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::deselect_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::delete_selection_ex`][super::TextEdit::delete_selection_ex]."]
#[must_use]
pub struct ExDeleteSelection < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeleteSelection < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: - 1i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::delete_selection_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_scroll_pos_for_line_ex`][super::TextEdit::get_scroll_pos_for_line_ex]."]
#[must_use]
pub struct ExGetScrollPosForLine < 'a > {
    surround_object: &'a re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetScrollPosForLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, wrap_index: 0i32,
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        re_export::TextEdit::get_scroll_pos_for_line_full(self.surround_object, self.line, self.wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_first_visible_ex`][super::TextEdit::set_line_as_first_visible_ex]."]
#[must_use]
pub struct ExSetLineAsFirstVisible < 'a > {
    surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsFirstVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, wrap_index: 0i32,
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_line_as_first_visible_full(self.surround_object, self.line, self.wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_center_visible_ex`][super::TextEdit::set_line_as_center_visible_ex]."]
#[must_use]
pub struct ExSetLineAsCenterVisible < 'a > {
    surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsCenterVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, wrap_index: 0i32,
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_line_as_center_visible_full(self.surround_object, self.line, self.wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_last_visible_ex`][super::TextEdit::set_line_as_last_visible_ex]."]
#[must_use]
pub struct ExSetLineAsLastVisible < 'a > {
    surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsLastVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        Self {
            surround_object, line, wrap_index: 0i32,
        }
    }
    #[inline]
    pub fn wrap_index(self, value: i32) -> Self {
        Self {
            wrap_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::set_line_as_last_visible_full(self.surround_object, self.line, self.wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::adjust_viewport_to_caret_ex`][super::TextEdit::adjust_viewport_to_caret_ex]."]
#[must_use]
pub struct ExAdjustViewportToCaret < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAdjustViewportToCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::adjust_viewport_to_caret_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::center_viewport_to_caret_ex`][super::TextEdit::center_viewport_to_caret_ex]."]
#[must_use]
pub struct ExCenterViewportToCaret < 'a > {
    surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCenterViewportToCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, caret_index: 0i32,
        }
    }
    #[inline]
    pub fn caret_index(self, value: i32) -> Self {
        Self {
            caret_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::center_viewport_to_caret_full(self.surround_object, self.caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::add_gutter_ex`][super::TextEdit::add_gutter_ex]."]
#[must_use]
pub struct ExAddGutter < 'a > {
    surround_object: &'a mut re_export::TextEdit, at: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddGutter < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        Self {
            surround_object, at: - 1i32,
        }
    }
    #[inline]
    pub fn at(self, value: i32) -> Self {
        Self {
            at: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextEdit::add_gutter_full(self.surround_object, self.at,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    pub const MENU_CUT: Self = Self {
        ord: 0i32
    };
    pub const MENU_COPY: Self = Self {
        ord: 1i32
    };
    pub const MENU_PASTE: Self = Self {
        ord: 2i32
    };
    pub const MENU_CLEAR: Self = Self {
        ord: 3i32
    };
    pub const MENU_SELECT_ALL: Self = Self {
        ord: 4i32
    };
    pub const MENU_UNDO: Self = Self {
        ord: 5i32
    };
    pub const MENU_REDO: Self = Self {
        ord: 6i32
    };
    pub const MENU_SUBMENU_TEXT_DIR: Self = Self {
        ord: 7i32
    };
    pub const MENU_DIR_INHERITED: Self = Self {
        ord: 8i32
    };
    pub const MENU_DIR_AUTO: Self = Self {
        ord: 9i32
    };
    pub const MENU_DIR_LTR: Self = Self {
        ord: 10i32
    };
    pub const MENU_DIR_RTL: Self = Self {
        ord: 11i32
    };
    pub const MENU_DISPLAY_UCC: Self = Self {
        ord: 12i32
    };
    pub const MENU_SUBMENU_INSERT_UCC: Self = Self {
        ord: 13i32
    };
    pub const MENU_INSERT_LRM: Self = Self {
        ord: 14i32
    };
    pub const MENU_INSERT_RLM: Self = Self {
        ord: 15i32
    };
    pub const MENU_INSERT_LRE: Self = Self {
        ord: 16i32
    };
    pub const MENU_INSERT_RLE: Self = Self {
        ord: 17i32
    };
    pub const MENU_INSERT_LRO: Self = Self {
        ord: 18i32
    };
    pub const MENU_INSERT_RLO: Self = Self {
        ord: 19i32
    };
    pub const MENU_INSERT_PDF: Self = Self {
        ord: 20i32
    };
    pub const MENU_INSERT_ALM: Self = Self {
        ord: 21i32
    };
    pub const MENU_INSERT_LRI: Self = Self {
        ord: 22i32
    };
    pub const MENU_INSERT_RLI: Self = Self {
        ord: 23i32
    };
    pub const MENU_INSERT_FSI: Self = Self {
        ord: 24i32
    };
    pub const MENU_INSERT_PDI: Self = Self {
        ord: 25i32
    };
    pub const MENU_INSERT_ZWJ: Self = Self {
        ord: 26i32
    };
    pub const MENU_INSERT_ZWNJ: Self = Self {
        ord: 27i32
    };
    pub const MENU_INSERT_WJ: Self = Self {
        ord: 28i32
    };
    pub const MENU_INSERT_SHY: Self = Self {
        ord: 29i32
    };
    pub const MENU_MAX: Self = Self {
        ord: 30i32
    };
    
}
impl crate::obj::EngineEnum for MenuItems {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 30usize;
    
}
impl crate::builtin::meta::GodotConvert for MenuItems {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MenuItems {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MenuItems {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EditAction {
    ord: i32
}
impl EditAction {
    pub const ACTION_NONE: Self = Self {
        ord: 0i32
    };
    pub const ACTION_TYPING: Self = Self {
        ord: 1i32
    };
    pub const ACTION_BACKSPACE: Self = Self {
        ord: 2i32
    };
    pub const ACTION_DELETE: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EditAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for EditAction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EditAction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EditAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SearchFlags {
    ord: i32
}
impl SearchFlags {
    pub const SEARCH_MATCH_CASE: Self = Self {
        ord: 1i32
    };
    pub const SEARCH_WHOLE_WORDS: Self = Self {
        ord: 2i32
    };
    pub const SEARCH_BACKWARDS: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SearchFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SearchFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SearchFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SearchFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CaretType {
    ord: i32
}
impl CaretType {
    pub const CARET_TYPE_LINE: Self = Self {
        ord: 0i32
    };
    pub const CARET_TYPE_BLOCK: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for CaretType {
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
impl crate::builtin::meta::GodotConvert for CaretType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CaretType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CaretType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectionMode {
    ord: i32
}
impl SelectionMode {
    pub const SELECTION_MODE_NONE: Self = Self {
        ord: 0i32
    };
    pub const SELECTION_MODE_SHIFT: Self = Self {
        ord: 1i32
    };
    pub const SELECTION_MODE_POINTER: Self = Self {
        ord: 2i32
    };
    pub const SELECTION_MODE_WORD: Self = Self {
        ord: 3i32
    };
    pub const SELECTION_MODE_LINE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SelectionMode {
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
impl crate::builtin::meta::GodotConvert for SelectionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SelectionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SelectionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LineWrappingMode {
    ord: i32
}
impl LineWrappingMode {
    pub const LINE_WRAPPING_NONE: Self = Self {
        ord: 0i32
    };
    pub const LINE_WRAPPING_BOUNDARY: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for LineWrappingMode {
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
impl crate::builtin::meta::GodotConvert for LineWrappingMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LineWrappingMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LineWrappingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GutterType {
    ord: i32
}
impl GutterType {
    pub const GUTTER_TYPE_STRING: Self = Self {
        ord: 0i32
    };
    pub const GUTTER_TYPE_ICON: Self = Self {
        ord: 1i32
    };
    pub const GUTTER_TYPE_CUSTOM: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for GutterType {
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
impl crate::builtin::meta::GodotConvert for GutterType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GutterType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GutterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}