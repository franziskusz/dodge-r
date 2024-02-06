#![doc = "Sidecar module for class [`RichTextLabel`][crate::engine::RichTextLabel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RichTextLabel` enums](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RichTextLabel.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`rich_text_label`][crate::engine::rich_text_label]: sidecar module with related enum/flag types\n* [`IRichTextLabel`][crate::engine::IRichTextLabel]: virtual methods\n\n\nSee also [Godot docs for `RichTextLabel`](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RichTextLabel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RichTextLabel`][crate::engine::RichTextLabel].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RichTextLabel` methods](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRichTextLabel: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RichTextLabel {
        pub fn get_parsed_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parsed_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_text(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_image_full(&mut self, image: Gd < crate::engine::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::engine::global::InlineAlignment, region: Rect2, key: Variant, pad: bool, tooltip: GString, size_in_percent: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, i32, i32, Color, crate::engine::global::InlineAlignment, Rect2, Variant, bool, GString, bool);
            let args = (image, width, height, color, inline_align, region, key, pad, tooltip, size_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_image(&mut self, image: Gd < crate::engine::Texture2D >,) {
            self.add_image_ex(image,) . done()
        }
        #[inline]
        pub fn add_image_ex(&mut self, image: Gd < crate::engine::Texture2D >,) -> ExAddImage < '_ > {
            ExAddImage::new(self, image,)
        }
        pub(crate) fn update_image_full(&mut self, key: Variant, mask: crate::engine::rich_text_label::ImageUpdateMask, image: Gd < crate::engine::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::engine::global::InlineAlignment, region: Rect2, pad: bool, tooltip: GString, size_in_percent: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant, crate::engine::rich_text_label::ImageUpdateMask, Gd < crate::engine::Texture2D >, i32, i32, Color, crate::engine::global::InlineAlignment, Rect2, bool, GString, bool);
            let args = (key, mask, image, width, height, color, inline_align, region, pad, tooltip, size_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn update_image(&mut self, key: Variant, mask: crate::engine::rich_text_label::ImageUpdateMask, image: Gd < crate::engine::Texture2D >,) {
            self.update_image_ex(key, mask, image,) . done()
        }
        #[inline]
        pub fn update_image_ex(&mut self, key: Variant, mask: crate::engine::rich_text_label::ImageUpdateMask, image: Gd < crate::engine::Texture2D >,) -> ExUpdateImage < '_ > {
            ExUpdateImage::new(self, key, mask, image,)
        }
        pub fn newline(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "newline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_paragraph(&mut self, paragraph: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_font_full(&mut self, font: Gd < crate::engine::Font >, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, i32);
            let args = (font, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_font(&mut self, font: Gd < crate::engine::Font >,) {
            self.push_font_ex(font,) . done()
        }
        #[inline]
        pub fn push_font_ex(&mut self, font: Gd < crate::engine::Font >,) -> ExPushFont < '_ > {
            ExPushFont::new(self, font,)
        }
        pub fn push_font_size(&mut self, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_normal(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bold(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_bold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bold_italics(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_bold_italics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_italics(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_italics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_mono(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_mono", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_outline_size(&mut self, outline_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_outline_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_outline_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_paragraph_full(&mut self, alignment: crate::engine::global::HorizontalAlignment, base_direction: crate::engine::control::TextDirection, language: GString, st_parser: crate::engine::text_server::StructuredTextParser, justification_flags: crate::engine::text_server::JustificationFlag, tab_stops: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::HorizontalAlignment, crate::engine::control::TextDirection, GString, crate::engine::text_server::StructuredTextParser, crate::engine::text_server::JustificationFlag, PackedFloat32Array);
            let args = (alignment, base_direction, language, st_parser, justification_flags, tab_stops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_paragraph(&mut self, alignment: crate::engine::global::HorizontalAlignment,) {
            self.push_paragraph_ex(alignment,) . done()
        }
        #[inline]
        pub fn push_paragraph_ex(&mut self, alignment: crate::engine::global::HorizontalAlignment,) -> ExPushParagraph < '_ > {
            ExPushParagraph::new(self, alignment,)
        }
        pub fn push_indent(&mut self, level: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_list_full(&mut self, level: i32, type_: crate::engine::rich_text_label::ListType, capitalize: bool, bullet: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::rich_text_label::ListType, bool, GString);
            let args = (level, type_, capitalize, bullet,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_list(&mut self, level: i32, type_: crate::engine::rich_text_label::ListType, capitalize: bool,) {
            self.push_list_ex(level, type_, capitalize,) . done()
        }
        #[inline]
        pub fn push_list_ex(&mut self, level: i32, type_: crate::engine::rich_text_label::ListType, capitalize: bool,) -> ExPushList < '_ > {
            ExPushList::new(self, level, type_, capitalize,)
        }
        pub fn push_meta(&mut self, data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_hint(&mut self, description: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_language(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_underline(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_strikethrough(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_strikethrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_table_full(&mut self, columns: i32, inline_align: crate::engine::global::InlineAlignment, align_to_row: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::global::InlineAlignment, i32);
            let args = (columns, inline_align, align_to_row,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_table", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_table(&mut self, columns: i32,) {
            self.push_table_ex(columns,) . done()
        }
        #[inline]
        pub fn push_table_ex(&mut self, columns: i32,) -> ExPushTable < '_ > {
            ExPushTable::new(self, columns,)
        }
        pub(crate) fn push_dropcap_full(&mut self, string: GString, font: Gd < crate::engine::Font >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::Font >, i32, Rect2, Color, i32, Color);
            let args = (string, font, size, dropcap_margins, color, outline_size, outline_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_dropcap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_dropcap(&mut self, string: GString, font: Gd < crate::engine::Font >, size: i32,) {
            self.push_dropcap_ex(string, font, size,) . done()
        }
        #[inline]
        pub fn push_dropcap_ex(&mut self, string: GString, font: Gd < crate::engine::Font >, size: i32,) -> ExPushDropcap < '_ > {
            ExPushDropcap::new(self, string, font, size,)
        }
        pub(crate) fn set_table_column_expand_full(&mut self, column: i32, expand: bool, ratio: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool, i32);
            let args = (column, expand, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_table_column_expand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_table_column_expand(&mut self, column: i32, expand: bool,) {
            self.set_table_column_expand_ex(column, expand,) . done()
        }
        #[inline]
        pub fn set_table_column_expand_ex(&mut self, column: i32, expand: bool,) -> ExSetTableColumnExpand < '_ > {
            ExSetTableColumnExpand::new(self, column, expand,)
        }
        pub fn set_cell_row_background_color(&mut self, odd_row_bg: Color, even_row_bg: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color, Color);
            let args = (odd_row_bg, even_row_bg,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_row_background_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_border_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size_override(&mut self, min_size: Vector2, max_size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (min_size, max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_padding(&mut self, padding: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2);
            let args = (padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_cell(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_fgcolor(&mut self, fgcolor: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (fgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_fgcolor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bgcolor(&mut self, bgcolor: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (bgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_bgcolor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_customfx(&mut self, effect: Gd < crate::engine::RichTextEffect >, env: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::RichTextEffect >, Dictionary);
            let args = (effect, env,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_customfx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_context(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop_context(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pop_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pop_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::engine::text_server::StructuredTextParser,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::engine::text_server::StructuredTextParser {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::StructuredTextParser >;
            type CallSig = (crate::engine::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (args,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::engine::text_server::AutowrapMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::engine::text_server::AutowrapMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::AutowrapMode >;
            type CallSig = (crate::engine::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta_underline(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_meta_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_meta_underlined(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_meta_underlined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hint_underline(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hint_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hint_underlined(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hint_underlined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scroll_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scroll_active(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_scroll_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_follow(&mut self, follow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scroll_follow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scroll_following(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_scroll_following", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::engine::VScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_line(&mut self, line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scroll_to_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_paragraph(&mut self, paragraph: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scroll_to_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_selection(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scroll_to_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_size(&mut self, spaces: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fit_content(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fit_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fit_content_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_fit_content_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selection_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selection_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection_from(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection_to(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selection_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse_bbcode(&mut self, bbcode: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (bbcode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn append_text(&mut self, bbcode: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (bbcode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ready(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_threaded(&mut self, threaded: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (threaded,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_threaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_threaded(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_threaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_progress_bar_delay(&mut self, delay_ms: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_progress_bar_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_progress_bar_delay(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_progress_bar_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::engine::text_server::VisibleCharactersBehavior {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::VisibleCharactersBehavior >;
            type CallSig = (crate::engine::text_server::VisibleCharactersBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::engine::text_server::VisibleCharactersBehavior,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::VisibleCharactersBehavior);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_character_line(&mut self, character: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_character_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_character_paragraph(&mut self, character: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_character_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_character_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_character_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_bbcode(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_bbcode(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_paragraph_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_paragraph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_paragraph_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_paragraph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_offset(&mut self, line: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_paragraph_offset(&mut self, paragraph: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_paragraph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse_expressions_for_values(&mut self, expressions: PackedStringArray,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, PackedStringArray);
            let args = (expressions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_expressions_for_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_effects(&mut self, effects: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (effects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_effects(&mut self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn install_effect(&mut self, effect: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant);
            let args = (effect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "install_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::engine::PopupMenu > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PopupMenu > >;
            type CallSig = (Option < Gd < crate::engine::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_menu_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7140usize);
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
    impl crate::obj::GodotClass for RichTextLabel {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RichTextLabel\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RichTextLabel {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RichTextLabel {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for RichTextLabel {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for RichTextLabel {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for RichTextLabel {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RichTextLabel {
        
    }
    impl crate::obj::ExportableObject for RichTextLabel {
        
    }
    impl crate::obj::cap::GodotDefault for RichTextLabel {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RichTextLabel {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RichTextLabel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RichTextLabel {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RichTextLabel > for $Class {
                
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
#[doc = "Default-param extender for [`RichTextLabel::add_image_ex`][super::RichTextLabel::add_image_ex]."]
#[must_use]
pub struct ExAddImage < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, image: Gd < crate::engine::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::engine::global::InlineAlignment, region: Rect2, key: Variant, pad: bool, tooltip: GString, size_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, image: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, image, width: 0i32, height: 0i32, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), inline_align: crate::obj::EngineEnum::from_ord(5), region: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), key: Variant::nil(), pad: false, tooltip: GString::from(""), size_in_percent: false,
        }
    }
    #[inline]
    pub fn width(self, value: i32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn height(self, value: i32) -> Self {
        Self {
            height: value, .. self
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn region(self, value: Rect2) -> Self {
        Self {
            region: value, .. self
        }
    }
    #[inline]
    pub fn key(self, value: Variant) -> Self {
        Self {
            key: value, .. self
        }
    }
    #[inline]
    pub fn pad(self, value: bool) -> Self {
        Self {
            pad: value, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, value: GString) -> Self {
        Self {
            tooltip: value, .. self
        }
    }
    #[inline]
    pub fn size_in_percent(self, value: bool) -> Self {
        Self {
            size_in_percent: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::add_image_full(self.surround_object, self.image, self.width, self.height, self.color, self.inline_align, self.region, self.key, self.pad, self.tooltip, self.size_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::update_image_ex`][super::RichTextLabel::update_image_ex]."]
#[must_use]
pub struct ExUpdateImage < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, key: Variant, mask: crate::engine::rich_text_label::ImageUpdateMask, image: Gd < crate::engine::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::engine::global::InlineAlignment, region: Rect2, pad: bool, tooltip: GString, size_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUpdateImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, key: Variant, mask: crate::engine::rich_text_label::ImageUpdateMask, image: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, key, mask, image, width: 0i32, height: 0i32, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), inline_align: crate::obj::EngineEnum::from_ord(5), region: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), pad: false, tooltip: GString::from(""), size_in_percent: false,
        }
    }
    #[inline]
    pub fn width(self, value: i32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn height(self, value: i32) -> Self {
        Self {
            height: value, .. self
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn region(self, value: Rect2) -> Self {
        Self {
            region: value, .. self
        }
    }
    #[inline]
    pub fn pad(self, value: bool) -> Self {
        Self {
            pad: value, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, value: GString) -> Self {
        Self {
            tooltip: value, .. self
        }
    }
    #[inline]
    pub fn size_in_percent(self, value: bool) -> Self {
        Self {
            size_in_percent: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::update_image_full(self.surround_object, self.key, self.mask, self.image, self.width, self.height, self.color, self.inline_align, self.region, self.pad, self.tooltip, self.size_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_font_ex`][super::RichTextLabel::push_font_ex]."]
#[must_use]
pub struct ExPushFont < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, font: Gd < crate::engine::Font >, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushFont < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, font: Gd < crate::engine::Font >,) -> Self {
        Self {
            surround_object, font, font_size: 0i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::push_font_full(self.surround_object, self.font, self.font_size,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_paragraph_ex`][super::RichTextLabel::push_paragraph_ex]."]
#[must_use]
pub struct ExPushParagraph < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, alignment: crate::engine::global::HorizontalAlignment, base_direction: crate::engine::control::TextDirection, language: GString, st_parser: crate::engine::text_server::StructuredTextParser, justification_flags: crate::engine::text_server::JustificationFlag, tab_stops: PackedFloat32Array,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushParagraph < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, alignment: crate::engine::global::HorizontalAlignment,) -> Self {
        Self {
            surround_object, alignment, base_direction: crate::obj::EngineEnum::from_ord(0), language: GString::from(""), st_parser: crate::obj::EngineEnum::from_ord(0), justification_flags: crate::obj::EngineBitfield::from_ord(163), tab_stops: PackedFloat32Array::new(),
        }
    }
    #[inline]
    pub fn base_direction(self, value: crate::engine::control::TextDirection) -> Self {
        Self {
            base_direction: value, .. self
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn st_parser(self, value: crate::engine::text_server::StructuredTextParser) -> Self {
        Self {
            st_parser: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn tab_stops(self, value: PackedFloat32Array) -> Self {
        Self {
            tab_stops: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::push_paragraph_full(self.surround_object, self.alignment, self.base_direction, self.language, self.st_parser, self.justification_flags, self.tab_stops,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_list_ex`][super::RichTextLabel::push_list_ex]."]
#[must_use]
pub struct ExPushList < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::engine::rich_text_label::ListType, capitalize: bool, bullet: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushList < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::engine::rich_text_label::ListType, capitalize: bool,) -> Self {
        Self {
            surround_object, level, type_, capitalize, bullet: GString::from("•"),
        }
    }
    #[inline]
    pub fn bullet(self, value: GString) -> Self {
        Self {
            bullet: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::push_list_full(self.surround_object, self.level, self.type_, self.capitalize, self.bullet,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_table_ex`][super::RichTextLabel::push_table_ex]."]
#[must_use]
pub struct ExPushTable < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, columns: i32, inline_align: crate::engine::global::InlineAlignment, align_to_row: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushTable < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, columns: i32,) -> Self {
        Self {
            surround_object, columns, inline_align: crate::obj::EngineEnum::from_ord(0), align_to_row: - 1i32,
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn align_to_row(self, value: i32) -> Self {
        Self {
            align_to_row: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::push_table_full(self.surround_object, self.columns, self.inline_align, self.align_to_row,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_dropcap_ex`][super::RichTextLabel::push_dropcap_ex]."]
#[must_use]
pub struct ExPushDropcap < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, string: GString, font: Gd < crate::engine::Font >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushDropcap < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, string: GString, font: Gd < crate::engine::Font >, size: i32,) -> Self {
        Self {
            surround_object, string, font, size, dropcap_margins: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), outline_size: 0i32, outline_color: Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn dropcap_margins(self, value: Rect2) -> Self {
        Self {
            dropcap_margins: value, .. self
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, value: i32) -> Self {
        Self {
            outline_size: value, .. self
        }
    }
    #[inline]
    pub fn outline_color(self, value: Color) -> Self {
        Self {
            outline_color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::push_dropcap_full(self.surround_object, self.string, self.font, self.size, self.dropcap_margins, self.color, self.outline_size, self.outline_color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::set_table_column_expand_ex`][super::RichTextLabel::set_table_column_expand_ex]."]
#[must_use]
pub struct ExSetTableColumnExpand < 'a > {
    surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool, ratio: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetTableColumnExpand < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool,) -> Self {
        Self {
            surround_object, column, expand, ratio: 1i32,
        }
    }
    #[inline]
    pub fn ratio(self, value: i32) -> Self {
        Self {
            ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RichTextLabel::set_table_column_expand_full(self.surround_object, self.column, self.expand, self.ratio,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ListType {
    ord: i32
}
impl ListType {
    pub const LIST_NUMBERS: Self = Self {
        ord: 0i32
    };
    pub const LIST_LETTERS: Self = Self {
        ord: 1i32
    };
    pub const LIST_ROMAN: Self = Self {
        ord: 2i32
    };
    pub const LIST_DOTS: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ListType {
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
impl crate::builtin::meta::GodotConvert for ListType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ListType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ListType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    pub const MENU_COPY: Self = Self {
        ord: 0i32
    };
    pub const MENU_SELECT_ALL: Self = Self {
        ord: 1i32
    };
    pub const MENU_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for MenuItems {
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
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 2usize;
    
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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ImageUpdateMask {
    ord: u64
}
impl ImageUpdateMask {
    pub const UPDATE_TEXTURE: Self = Self {
        ord: 1u64
    };
    pub const UPDATE_SIZE: Self = Self {
        ord: 2u64
    };
    pub const UPDATE_COLOR: Self = Self {
        ord: 4u64
    };
    pub const UPDATE_ALIGNMENT: Self = Self {
        ord: 8u64
    };
    pub const UPDATE_REGION: Self = Self {
        ord: 16u64
    };
    pub const UPDATE_PAD: Self = Self {
        ord: 32u64
    };
    pub const UPDATE_TOOLTIP: Self = Self {
        ord: 64u64
    };
    pub const UPDATE_WIDTH_IN_PERCENT: Self = Self {
        ord: 128u64
    };
    
}
impl crate::obj::EngineBitfield for ImageUpdateMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for ImageUpdateMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for ImageUpdateMask {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for ImageUpdateMask {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ImageUpdateMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}