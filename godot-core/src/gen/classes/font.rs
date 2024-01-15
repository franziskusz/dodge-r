#![doc = "Sidecar module for class [`Font`][crate::engine::Font].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Font` enums](https://docs.godotengine.org/en/stable/classes/class_font.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Font.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`font`][crate::engine::font]: sidecar module with related enum/flag types\n* [`IFont`][crate::engine::IFont]: virtual methods\n\n\nSee also [Godot docs for `Font`](https://docs.godotengine.org/en/stable/classes/class_font.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Font {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Font`][crate::engine::Font].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Font` methods](https://docs.godotengine.org/en/stable/classes/class_font.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFont: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Font {
        pub fn set_fallbacks(&mut self, fallbacks: Array < Gd < crate::engine::Font > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::Font > >);
            let args = (fallbacks,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fallbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fallbacks(&self,) -> Array < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Font > > >;
            type CallSig = (Array < Gd < crate::engine::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fallbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn find_variation_full(&self, variation_coordinates: Dictionary, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Dictionary, i32, f32, Transform2D, i32, i32, i32, i32);
            let args = (variation_coordinates, face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn find_variation(&self, variation_coordinates: Dictionary,) -> Rid {
            self.find_variation_ex(variation_coordinates,) . done()
        }
        #[inline]
        pub fn find_variation_ex(&self, variation_coordinates: Dictionary,) -> ExFindVariation < '_ > {
            ExFindVariation::new(self, variation_coordinates,)
        }
        pub fn get_rids(&self,) -> Array < Rid > {
            type RetMarshal = PtrcallReturnT < Array < Rid > >;
            type CallSig = (Array < Rid >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rids", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_height_full(&self, font_size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_height(&self,) -> f32 {
            self.get_height_ex() . done()
        }
        #[inline]
        pub fn get_height_ex(&self,) -> ExGetHeight < '_ > {
            ExGetHeight::new(self,)
        }
        pub(crate) fn get_ascent_full(&self, font_size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_ascent(&self,) -> f32 {
            self.get_ascent_ex() . done()
        }
        #[inline]
        pub fn get_ascent_ex(&self,) -> ExGetAscent < '_ > {
            ExGetAscent::new(self,)
        }
        pub(crate) fn get_descent_full(&self, font_size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_descent(&self,) -> f32 {
            self.get_descent_ex() . done()
        }
        #[inline]
        pub fn get_descent_ex(&self,) -> ExGetDescent < '_ > {
            ExGetDescent::new(self,)
        }
        pub(crate) fn get_underline_position_full(&self, font_size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_underline_position(&self,) -> f32 {
            self.get_underline_position_ex() . done()
        }
        #[inline]
        pub fn get_underline_position_ex(&self,) -> ExGetUnderlinePosition < '_ > {
            ExGetUnderlinePosition::new(self,)
        }
        pub(crate) fn get_underline_thickness_full(&self, font_size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_underline_thickness(&self,) -> f32 {
            self.get_underline_thickness_ex() . done()
        }
        #[inline]
        pub fn get_underline_thickness_ex(&self,) -> ExGetUnderlineThickness < '_ > {
            ExGetUnderlineThickness::new(self,)
        }
        pub fn get_font_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_style_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ot_name_strings(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ot_name_strings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_style(&self,) -> crate::engine::text_server::FontStyle {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FontStyle >;
            type CallSig = (crate::engine::text_server::FontStyle,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_weight(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_stretch(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spacing(&self, spacing: crate::engine::text_server::SpacingType,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::text_server::SpacingType);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_opentype_features(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_opentype_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_capacity(&mut self, single_line: i32, multi_line: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (single_line, multi_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_capacity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_size_full(&self, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (text, alignment, width, font_size, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_string_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_string_size(&self, text: GString,) -> Vector2 {
            self.get_string_size_ex(text,) . done()
        }
        #[inline]
        pub fn get_string_size_ex(&self, text: GString,) -> ExGetStringSize < '_ > {
            ExGetStringSize::new(self, text,)
        }
        pub(crate) fn get_multiline_string_size_full(&self, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, crate::engine::text_server::LineBreakFlag, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_multiline_string_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_multiline_string_size(&self, text: GString,) -> Vector2 {
            self.get_multiline_string_size_ex(text,) . done()
        }
        #[inline]
        pub fn get_multiline_string_size_ex(&self, text: GString,) -> ExGetMultilineStringSize < '_ > {
            ExGetMultilineStringSize::new(self, text,)
        }
        pub(crate) fn draw_string_full(&self, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, Color, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_string(&self, canvas_item: Rid, pos: Vector2, text: GString,) {
            self.draw_string_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_ex(&self, canvas_item: Rid, pos: Vector2, text: GString,) -> ExDrawString < '_ > {
            ExDrawString::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_multiline_string_full(&self, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, Color, crate::engine::text_server::LineBreakFlag, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline_string(&self, canvas_item: Rid, pos: Vector2, text: GString,) {
            self.draw_multiline_string_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_ex(&self, canvas_item: Rid, pos: Vector2, text: GString,) -> ExDrawMultilineString < '_ > {
            ExDrawMultilineString::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, Color, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_string_outline(&self, canvas_item: Rid, pos: Vector2, text: GString,) {
            self.draw_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_outline_ex(&self, canvas_item: Rid, pos: Vector2, text: GString,) -> ExDrawStringOutline < '_ > {
            ExDrawStringOutline::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_multiline_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::engine::text_server::LineBreakFlag, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline_string_outline(&self, canvas_item: Rid, pos: Vector2, text: GString,) {
            self.draw_multiline_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_outline_ex(&self, canvas_item: Rid, pos: Vector2, text: GString,) -> ExDrawMultilineStringOutline < '_ > {
            ExDrawMultilineStringOutline::new(self, canvas_item, pos, text,)
        }
        pub fn get_char_size(&self, char: i64, font_size: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i64, i32);
            let args = (char, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_char_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_char_full(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, modulate: Color,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, Vector2, i64, i32, Color);
            let args = (canvas_item, pos, char, font_size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_char(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> f32 {
            self.draw_char_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[inline]
        pub fn draw_char_ex(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> ExDrawChar < '_ > {
            ExDrawChar::new(self, canvas_item, pos, char, font_size,)
        }
        pub(crate) fn draw_char_outline_full(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, size: i32, modulate: Color,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Rid, Vector2, i64, i32, i32, Color);
            let args = (canvas_item, pos, char, font_size, size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_char_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_char_outline(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> f32 {
            self.draw_char_outline_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[inline]
        pub fn draw_char_outline_ex(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> ExDrawCharOutline < '_ > {
            ExDrawCharOutline::new(self, canvas_item, pos, char, font_size,)
        }
        pub fn has_char(&self, char: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_chars(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_language_supported(&self, language: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_language_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_script_supported(&self, script: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_script_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_feature_list(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_feature_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_variation_list(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_variation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_count(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_count", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Font {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Font\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Font {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Font {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Font {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Font {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Font {
        
    }
    impl crate::obj::ExportableObject for Font {
        
    }
    impl std::ops::Deref for Font {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Font {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Font {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Font > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Font::find_variation_ex`][super::Font::find_variation_ex]."]
#[must_use]
pub struct ExFindVariation < 'a > {
    surround_object: &'a re_export::Font, variation_coordinates: Dictionary, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindVariation < 'a > {
    fn new(surround_object: &'a re_export::Font, variation_coordinates: Dictionary,) -> Self {
        Self {
            surround_object, variation_coordinates, face_index: 0i32, strength: 0f32, transform: Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _), spacing_top: 0i32, spacing_bottom: 0i32, spacing_space: 0i32, spacing_glyph: 0i32,
        }
    }
    #[inline]
    pub fn face_index(self, value: i32) -> Self {
        Self {
            face_index: value, .. self
        }
    }
    #[inline]
    pub fn strength(self, value: f32) -> Self {
        Self {
            strength: value, .. self
        }
    }
    #[inline]
    pub fn transform(self, value: Transform2D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn spacing_top(self, value: i32) -> Self {
        Self {
            spacing_top: value, .. self
        }
    }
    #[inline]
    pub fn spacing_bottom(self, value: i32) -> Self {
        Self {
            spacing_bottom: value, .. self
        }
    }
    #[inline]
    pub fn spacing_space(self, value: i32) -> Self {
        Self {
            spacing_space: value, .. self
        }
    }
    #[inline]
    pub fn spacing_glyph(self, value: i32) -> Self {
        Self {
            spacing_glyph: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::Font::find_variation_full(self.surround_object, self.variation_coordinates, self.face_index, self.strength, self.transform, self.spacing_top, self.spacing_bottom, self.spacing_space, self.spacing_glyph,)
    }
}
#[doc = "Default-param extender for [`Font::get_height_ex`][super::Font::get_height_ex]."]
#[must_use]
pub struct ExGetHeight < 'a > {
    surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetHeight < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        Self {
            surround_object, font_size: 16i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::get_height_full(self.surround_object, self.font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_ascent_ex`][super::Font::get_ascent_ex]."]
#[must_use]
pub struct ExGetAscent < 'a > {
    surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAscent < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        Self {
            surround_object, font_size: 16i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::get_ascent_full(self.surround_object, self.font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_descent_ex`][super::Font::get_descent_ex]."]
#[must_use]
pub struct ExGetDescent < 'a > {
    surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDescent < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        Self {
            surround_object, font_size: 16i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::get_descent_full(self.surround_object, self.font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_position_ex`][super::Font::get_underline_position_ex]."]
#[must_use]
pub struct ExGetUnderlinePosition < 'a > {
    surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUnderlinePosition < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        Self {
            surround_object, font_size: 16i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::get_underline_position_full(self.surround_object, self.font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_thickness_ex`][super::Font::get_underline_thickness_ex]."]
#[must_use]
pub struct ExGetUnderlineThickness < 'a > {
    surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUnderlineThickness < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        Self {
            surround_object, font_size: 16i32,
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::get_underline_thickness_full(self.surround_object, self.font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_string_size_ex`][super::Font::get_string_size_ex]."]
#[must_use]
pub struct ExGetStringSize < 'a > {
    surround_object: &'a re_export::Font, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStringSize < 'a > {
    fn new(surround_object: &'a re_export::Font, text: GString,) -> Self {
        Self {
            surround_object, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        re_export::Font::get_string_size_full(self.surround_object, self.text, self.alignment, self.width, self.font_size, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::get_multiline_string_size_ex`][super::Font::get_multiline_string_size_ex]."]
#[must_use]
pub struct ExGetMultilineStringSize < 'a > {
    surround_object: &'a re_export::Font, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMultilineStringSize < 'a > {
    fn new(surround_object: &'a re_export::Font, text: GString,) -> Self {
        Self {
            surround_object, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, max_lines: - 1i32, brk_flags: crate::obj::EngineBitfield::from_ord(3), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, value: i32) -> Self {
        Self {
            max_lines: value, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        re_export::Font::get_multiline_string_size_full(self.surround_object, self.text, self.alignment, self.width, self.font_size, self.max_lines, self.brk_flags, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_ex`][super::Font::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawString < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, canvas_item, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Font::draw_string_full(self.surround_object, self.canvas_item, self.pos, self.text, self.alignment, self.width, self.font_size, self.modulate, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_ex`][super::Font::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineString < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, canvas_item, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, max_lines: - 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), brk_flags: crate::obj::EngineBitfield::from_ord(3), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, value: i32) -> Self {
        Self {
            max_lines: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Font::draw_multiline_string_full(self.surround_object, self.canvas_item, self.pos, self.text, self.alignment, self.width, self.font_size, self.max_lines, self.modulate, self.brk_flags, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_outline_ex`][super::Font::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawStringOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, canvas_item, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, size: 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Font::draw_string_outline_full(self.surround_object, self.canvas_item, self.pos, self.text, self.alignment, self.width, self.font_size, self.size, self.modulate, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_outline_ex`][super::Font::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineStringOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, canvas_item, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, max_lines: - 1i32, size: 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), brk_flags: crate::obj::EngineBitfield::from_ord(3), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, value: i32) -> Self {
        Self {
            max_lines: value, .. self
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Font::draw_multiline_string_outline_full(self.surround_object, self.canvas_item, self.pos, self.text, self.alignment, self.width, self.font_size, self.max_lines, self.size, self.modulate, self.brk_flags, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_ex`][super::Font::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawChar < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> Self {
        Self {
            surround_object, canvas_item, pos, char, font_size, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::draw_char_full(self.surround_object, self.canvas_item, self.pos, self.char, self.font_size, self.modulate,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_outline_ex`][super::Font::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'a > {
    surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawCharOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> Self {
        Self {
            surround_object, canvas_item, pos, char, font_size, size: - 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Font::draw_char_outline_full(self.surround_object, self.canvas_item, self.pos, self.char, self.font_size, self.size, self.modulate,)
    }
}