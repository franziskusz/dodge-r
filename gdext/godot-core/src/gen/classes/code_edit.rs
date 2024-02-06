#![doc = "Sidecar module for class [`CodeEdit`][crate::engine::CodeEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeEdit` enums](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeEdit.`\n\nInherits [`TextEdit`][crate::engine::TextEdit].\n\nRelated symbols:\n\n* [`code_edit`][crate::engine::code_edit]: sidecar module with related enum/flag types\n* [`ICodeEdit`][crate::engine::ICodeEdit]: virtual methods\n\n\nSee also [Godot docs for `CodeEdit`](https://docs.godotengine.org/en/stable/classes/class_codeedit.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CodeEdit`][crate::engine::CodeEdit].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CodeEdit` methods](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeEdit: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn confirm_code_completion(&mut self, replace: bool,) {
            unimplemented !()
        }
        fn request_code_completion(&mut self, force: bool,) {
            unimplemented !()
        }
        fn filter_code_completion_candidates(&self, candidates: Array < Dictionary >,) -> Array < Dictionary > {
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
    impl CodeEdit {
        pub fn set_indent_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_indent_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_indent_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_indent_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indent_using_spaces(&mut self, use_spaces: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_indent_using_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_indent_using_spaces(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_indent_using_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_indent_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_indent_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_indent_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_auto_indent_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_indent_prefixes(&mut self, prefixes: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (prefixes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_indent_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_indent_prefixes(&self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_indent_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn do_indent(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "do_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn indent_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "indent_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unindent_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unindent_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn convert_indent_full(&mut self, from_line: i32, to_line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convert_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn convert_indent(&mut self,) {
            self.convert_indent_ex() . done()
        }
        #[inline]
        pub fn convert_indent_ex(&mut self,) -> ExConvertIndent < '_ > {
            ExConvertIndent::new(self,)
        }
        pub fn set_auto_brace_completion_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_brace_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_brace_completion_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_auto_brace_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_highlight_matching_braces_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_highlight_matching_braces_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_highlight_matching_braces_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_highlight_matching_braces_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_auto_brace_completion_pair(&mut self, start_key: GString, end_key: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (start_key, end_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_auto_brace_completion_pair", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_brace_completion_pairs(&mut self, pairs: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (pairs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_brace_completion_pairs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_brace_completion_pairs(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_brace_completion_pairs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_brace_completion_open_key(&self, open_key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (open_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_auto_brace_completion_open_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_brace_completion_close_key(&self, close_key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (close_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_auto_brace_completion_close_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_brace_completion_close_key(&self, open_key: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (open_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_brace_completion_close_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_breakpoints_gutter(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_breakpoints_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_breakpoints_gutter(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_breakpoints_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_bookmarks_gutter(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_bookmarks_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_bookmarks_gutter(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_bookmarks_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_executing_lines_gutter(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_executing_lines_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_executing_lines_gutter(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_executing_lines_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_breakpoint(&mut self, line: i32, breakpointed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (line, breakpointed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_breakpoint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_breakpointed(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_breakpointed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_breakpointed_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_breakpointed_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_breakpointed_lines(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_breakpointed_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_bookmarked(&mut self, line: i32, bookmarked: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (line, bookmarked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_bookmarked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_bookmarked(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_bookmarked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_bookmarked_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_bookmarked_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bookmarked_lines(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bookmarked_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_executing(&mut self, line: i32, executing: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (line, executing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_as_executing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_executing(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_executing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_executing_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_executing_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_executing_lines(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_executing_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_line_numbers(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_line_numbers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_line_numbers_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_draw_line_numbers_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_numbers_zero_padded(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_numbers_zero_padded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_numbers_zero_padded(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_numbers_zero_padded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_fold_gutter(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_fold_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_fold_gutter(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drawing_fold_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_folding_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_folding_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_fold_line(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_fold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fold_line(&mut self, line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unfold_line(&mut self, line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unfold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fold_all_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fold_all_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unfold_all_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unfold_all_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_foldable_line(&mut self, line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "toggle_foldable_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_folded(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_folded_lines(&self,) -> Array < i64 > {
            type RetMarshal = PtrcallReturnT < Array < i64 > >;
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_folded_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_code_region(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_code_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_region_start_tag(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_region_start_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_region_end_tag(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_region_end_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_code_region_tags_full(&mut self, start: GString, end: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_region_tags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_code_region_tags(&mut self,) {
            self.set_code_region_tags_ex() . done()
        }
        #[inline]
        pub fn set_code_region_tags_ex(&mut self,) -> ExSetCodeRegionTags < '_ > {
            ExSetCodeRegionTags::new(self,)
        }
        pub fn is_line_code_region_start(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_code_region_start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_code_region_end(&self, line: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_line_code_region_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_string_delimiter_full(&mut self, start_key: GString, end_key: GString, line_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, bool);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_string_delimiter(&mut self, start_key: GString, end_key: GString,) {
            self.add_string_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_string_delimiter_ex(&mut self, start_key: GString, end_key: GString,) -> ExAddStringDelimiter < '_ > {
            ExAddStringDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_string_delimiter(&mut self, start_key: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_string_delimiter(&self, start_key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_string_delimiters(&mut self, string_delimiters: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (string_delimiters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_string_delimiters(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_string_delimiters(&self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_in_string_full(&self, line: i32, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_in_string(&self, line: i32,) -> i32 {
            self.is_in_string_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_string_ex(&self, line: i32,) -> ExIsInString < '_ > {
            ExIsInString::new(self, line,)
        }
        pub(crate) fn add_comment_delimiter_full(&mut self, start_key: GString, end_key: GString, line_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, bool);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_comment_delimiter(&mut self, start_key: GString, end_key: GString,) {
            self.add_comment_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_comment_delimiter_ex(&mut self, start_key: GString, end_key: GString,) -> ExAddCommentDelimiter < '_ > {
            ExAddCommentDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_comment_delimiter(&mut self, start_key: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_comment_delimiter(&self, start_key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_comment_delimiters(&mut self, comment_delimiters: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (comment_delimiters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_comment_delimiters(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_comment_delimiters(&self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_in_comment_full(&self, line: i32, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_comment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_in_comment(&self, line: i32,) -> i32 {
            self.is_in_comment_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_comment_ex(&self, line: i32,) -> ExIsInComment < '_ > {
            ExIsInComment::new(self, line,)
        }
        pub fn get_delimiter_start_key(&self, delimiter_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_delimiter_start_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_end_key(&self, delimiter_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_delimiter_end_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_start_position(&self, line: i32, column: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_delimiter_start_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_end_position(&self, line: i32, column: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_delimiter_end_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_hint(&mut self, code_hint: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (code_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_hint_draw_below(&mut self, draw_below: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (draw_below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_hint_draw_below", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_for_code_completion(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_for_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn request_code_completion_full(&mut self, force: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn request_code_completion(&mut self,) {
            self.request_code_completion_ex() . done()
        }
        #[inline]
        pub fn request_code_completion_ex(&mut self,) -> ExRequestCodeCompletion < '_ > {
            ExRequestCodeCompletion::new(self,)
        }
        pub(crate) fn add_code_completion_option_full(&mut self, type_: crate::engine::code_edit::CodeCompletionKind, display_text: GString, insert_text: GString, text_color: Color, icon: Gd < crate::engine::Resource >, value: Variant, location: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::code_edit::CodeCompletionKind, GString, GString, Color, Gd < crate::engine::Resource >, Variant, i32);
            let args = (type_, display_text, insert_text, text_color, icon, value, location,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_code_completion_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_code_completion_option(&mut self, type_: crate::engine::code_edit::CodeCompletionKind, display_text: GString, insert_text: GString,) {
            self.add_code_completion_option_ex(type_, display_text, insert_text,) . done()
        }
        #[inline]
        pub fn add_code_completion_option_ex(&mut self, type_: crate::engine::code_edit::CodeCompletionKind, display_text: GString, insert_text: GString,) -> ExAddCodeCompletionOption < '_ > {
            ExAddCodeCompletionOption::new(self, type_, display_text, insert_text,)
        }
        pub fn update_code_completion_options(&mut self, force: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_code_completion_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_options(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_completion_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_option(&self, index: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_completion_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_selected_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_completion_selected_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_selected_index(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_completion_selected_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn confirm_code_completion_full(&mut self, replace: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (replace,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "confirm_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn confirm_code_completion(&mut self,) {
            self.confirm_code_completion_ex() . done()
        }
        #[inline]
        pub fn confirm_code_completion_ex(&mut self,) -> ExConfirmCodeCompletion < '_ > {
            ExConfirmCodeCompletion::new(self,)
        }
        pub fn cancel_code_completion(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cancel_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_code_completion_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_code_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_prefixes(&mut self, prefixes: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (prefixes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_code_completion_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_prefixes(&self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_code_completion_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_length_guidelines(&mut self, guideline_columns: Array < i64 >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < i64 >);
            let args = (guideline_columns,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_length_guidelines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_length_guidelines(&self,) -> Array < i64 > {
            type RetMarshal = PtrcallReturnT < Array < i64 > >;
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_length_guidelines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symbol_lookup_on_click_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_symbol_lookup_on_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_symbol_lookup_on_click_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_symbol_lookup_on_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_for_symbol_lookup(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_for_symbol_lookup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_with_cursor_char(&self, line: i32, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_with_cursor_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symbol_lookup_word_as_valid(&mut self, valid: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (valid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_symbol_lookup_word_as_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn duplicate_lines(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "duplicate_lines", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CodeEdit {
        type Base = crate::engine::TextEdit;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CodeEdit\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CodeEdit {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::TextEdit > for CodeEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for CodeEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for CodeEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for CodeEdit {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CodeEdit {
        
    }
    impl crate::obj::ExportableObject for CodeEdit {
        
    }
    impl crate::obj::cap::GodotDefault for CodeEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeEdit {
        type Target = crate::engine::TextEdit;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CodeEdit {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CodeEdit > for $Class {
                
            }
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
#[doc = "Default-param extender for [`CodeEdit::convert_indent_ex`][super::CodeEdit::convert_indent_ex]."]
#[must_use]
pub struct ExConvertIndent < 'a > {
    surround_object: &'a mut re_export::CodeEdit, from_line: i32, to_line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConvertIndent < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        Self {
            surround_object, from_line: - 1i32, to_line: - 1i32,
        }
    }
    #[inline]
    pub fn from_line(self, value: i32) -> Self {
        Self {
            from_line: value, .. self
        }
    }
    #[inline]
    pub fn to_line(self, value: i32) -> Self {
        Self {
            to_line: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::convert_indent_full(self.surround_object, self.from_line, self.to_line,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::set_code_region_tags_ex`][super::CodeEdit::set_code_region_tags_ex]."]
#[must_use]
pub struct ExSetCodeRegionTags < 'a > {
    surround_object: &'a mut re_export::CodeEdit, start: GString, end: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCodeRegionTags < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        Self {
            surround_object, start: GString::from("region"), end: GString::from("endregion"),
        }
    }
    #[inline]
    pub fn start(self, value: GString) -> Self {
        Self {
            start: value, .. self
        }
    }
    #[inline]
    pub fn end(self, value: GString) -> Self {
        Self {
            end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::set_code_region_tags_full(self.surround_object, self.start, self.end,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_string_delimiter_ex`][super::CodeEdit::add_string_delimiter_ex]."]
#[must_use]
pub struct ExAddStringDelimiter < 'a > {
    surround_object: &'a mut re_export::CodeEdit, start_key: GString, end_key: GString, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStringDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: GString, end_key: GString,) -> Self {
        Self {
            surround_object, start_key, end_key, line_only: false,
        }
    }
    #[inline]
    pub fn line_only(self, value: bool) -> Self {
        Self {
            line_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::add_string_delimiter_full(self.surround_object, self.start_key, self.end_key, self.line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_string_ex`][super::CodeEdit::is_in_string_ex]."]
#[must_use]
pub struct ExIsInString < 'a > {
    surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInString < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        Self {
            surround_object, line, column: - 1i32,
        }
    }
    #[inline]
    pub fn column(self, value: i32) -> Self {
        Self {
            column: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::CodeEdit::is_in_string_full(self.surround_object, self.line, self.column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_comment_delimiter_ex`][super::CodeEdit::add_comment_delimiter_ex]."]
#[must_use]
pub struct ExAddCommentDelimiter < 'a > {
    surround_object: &'a mut re_export::CodeEdit, start_key: GString, end_key: GString, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCommentDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: GString, end_key: GString,) -> Self {
        Self {
            surround_object, start_key, end_key, line_only: false,
        }
    }
    #[inline]
    pub fn line_only(self, value: bool) -> Self {
        Self {
            line_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::add_comment_delimiter_full(self.surround_object, self.start_key, self.end_key, self.line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_comment_ex`][super::CodeEdit::is_in_comment_ex]."]
#[must_use]
pub struct ExIsInComment < 'a > {
    surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInComment < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        Self {
            surround_object, line, column: - 1i32,
        }
    }
    #[inline]
    pub fn column(self, value: i32) -> Self {
        Self {
            column: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::CodeEdit::is_in_comment_full(self.surround_object, self.line, self.column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::request_code_completion_ex`][super::CodeEdit::request_code_completion_ex]."]
#[must_use]
pub struct ExRequestCodeCompletion < 'a > {
    surround_object: &'a mut re_export::CodeEdit, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequestCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        Self {
            surround_object, force: false,
        }
    }
    #[inline]
    pub fn force(self, value: bool) -> Self {
        Self {
            force: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::request_code_completion_full(self.surround_object, self.force,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_code_completion_option_ex`][super::CodeEdit::add_code_completion_option_ex]."]
#[must_use]
pub struct ExAddCodeCompletionOption < 'a > {
    surround_object: &'a mut re_export::CodeEdit, type_: crate::engine::code_edit::CodeCompletionKind, display_text: GString, insert_text: GString, text_color: Color, icon: Gd < crate::engine::Resource >, value: Variant, location: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCodeCompletionOption < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, type_: crate::engine::code_edit::CodeCompletionKind, display_text: GString, insert_text: GString,) -> Self {
        Self {
            surround_object, type_, display_text, insert_text, text_color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), icon: unimplemented !("see #156"), value: Variant::from(0), location: 1024i32,
        }
    }
    #[inline]
    pub fn text_color(self, value: Color) -> Self {
        Self {
            text_color: value, .. self
        }
    }
    #[inline]
    pub fn icon(self, value: Gd < crate::engine::Resource >) -> Self {
        Self {
            icon: value, .. self
        }
    }
    #[inline]
    pub fn value(self, value: Variant) -> Self {
        Self {
            value: value, .. self
        }
    }
    #[inline]
    pub fn location(self, value: i32) -> Self {
        Self {
            location: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::add_code_completion_option_full(self.surround_object, self.type_, self.display_text, self.insert_text, self.text_color, self.icon, self.value, self.location,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::confirm_code_completion_ex`][super::CodeEdit::confirm_code_completion_ex]."]
#[must_use]
pub struct ExConfirmCodeCompletion < 'a > {
    surround_object: &'a mut re_export::CodeEdit, replace: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConfirmCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        Self {
            surround_object, replace: false,
        }
    }
    #[inline]
    pub fn replace(self, value: bool) -> Self {
        Self {
            replace: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeEdit::confirm_code_completion_full(self.surround_object, self.replace,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    pub const KIND_CLASS: Self = Self {
        ord: 0i32
    };
    pub const KIND_FUNCTION: Self = Self {
        ord: 1i32
    };
    pub const KIND_SIGNAL: Self = Self {
        ord: 2i32
    };
    pub const KIND_VARIABLE: Self = Self {
        ord: 3i32
    };
    pub const KIND_MEMBER: Self = Self {
        ord: 4i32
    };
    pub const KIND_ENUM: Self = Self {
        ord: 5i32
    };
    pub const KIND_CONSTANT: Self = Self {
        ord: 6i32
    };
    pub const KIND_NODE_PATH: Self = Self {
        ord: 7i32
    };
    pub const KIND_FILE_PATH: Self = Self {
        ord: 8i32
    };
    pub const KIND_PLAIN_TEXT: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for CodeCompletionKind {
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