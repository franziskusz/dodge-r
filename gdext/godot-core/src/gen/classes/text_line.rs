#![doc = "Sidecar module for class [`TextLine`][crate::engine::TextLine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextLine` enums](https://docs.godotengine.org/en/stable/classes/class_textline.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextLine.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`text_line`][crate::engine::text_line]: sidecar module with related enum/flag types\n* [`ITextLine`][crate::engine::ITextLine]: virtual methods\n\n\nSee also [Godot docs for `TextLine`](https://docs.godotengine.org/en/stable/classes/class_textline.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextLine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextLine`][crate::engine::TextLine].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextLine` methods](https://docs.godotengine.org/en/stable/classes/class_textline.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextLine: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TextLine {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_direction(&mut self, direction: crate::engine::text_server::Direction,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::Direction);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> crate::engine::text_server::Direction {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Direction >;
            type CallSig = (crate::engine::text_server::Direction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::Orientation);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::engine::text_server::Orientation {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Orientation >;
            type CallSig = (crate::engine::text_server::Orientation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_preserve_invalid(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_preserve_invalid(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_preserve_control(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_preserve_control(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bidi_override(&mut self, override_: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (override_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_string_full(&mut self, text: GString, font: Gd < crate::engine::Font >, font_size: i32, language: GString, meta: Variant,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, Gd < crate::engine::Font >, i32, GString, Variant);
            let args = (text, font, font_size, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_string(&mut self, text: GString, font: Gd < crate::engine::Font >, font_size: i32,) -> bool {
            self.add_string_ex(text, font, font_size,) . done()
        }
        #[inline]
        pub fn add_string_ex(&mut self, text: GString, font: Gd < crate::engine::Font >, font_size: i32,) -> ExAddString < '_ > {
            ExAddString::new(self, text, font, font_size,)
        }
        pub(crate) fn add_object_full(&mut self, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, length: i32, baseline: f32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Variant, Vector2, crate::engine::global::InlineAlignment, i32, f32);
            let args = (key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_object(&mut self, key: Variant, size: Vector2,) -> bool {
            self.add_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn add_object_ex(&mut self, key: Variant, size: Vector2,) -> ExAddObject < '_ > {
            ExAddObject::new(self, key, size,)
        }
        pub(crate) fn resize_object_full(&mut self, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, baseline: f32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Variant, Vector2, crate::engine::global::InlineAlignment, f32);
            let args = (key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resize_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resize_object(&mut self, key: Variant, size: Vector2,) -> bool {
            self.resize_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn resize_object_ex(&mut self, key: Variant, size: Vector2,) -> ExResizeObject < '_ > {
            ExResizeObject::new(self, key, size,)
        }
        pub fn set_width(&mut self, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::engine::global::HorizontalAlignment,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::engine::global::HorizontalAlignment {
            type RetMarshal = PtrcallReturnT < crate::engine::global::HorizontalAlignment >;
            type CallSig = (crate::engine::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tab_align(&mut self, tab_stops: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat32Array);
            let args = (tab_stops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tab_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flags(&mut self, flags: crate::engine::text_server::JustificationFlag,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::JustificationFlag);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flags(&self,) -> crate::engine::text_server::JustificationFlag {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::JustificationFlag >;
            type CallSig = (crate::engine::text_server::JustificationFlag,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::engine::text_server::OverrunBehavior,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::engine::text_server::OverrunBehavior {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::OverrunBehavior >;
            type CallSig = (crate::engine::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_objects(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_object_rect(&self, key: Variant,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, Variant);
            let args = (key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_object_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_ascent(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_descent(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_width(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_underline_position(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_underline_thickness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_full(&self, canvas: Rid, pos: Vector2, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Color);
            let args = (canvas, pos, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw(&self, canvas: Rid, pos: Vector2,) {
            self.draw_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_ex(&self, canvas: Rid, pos: Vector2,) -> ExDraw < '_ > {
            ExDraw::new(self, canvas, pos,)
        }
        pub(crate) fn draw_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, i32, Color);
            let args = (canvas, pos, outline_size, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_outline_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_outline_ex(&self, canvas: Rid, pos: Vector2,) -> ExDrawOutline < '_ > {
            ExDrawOutline::new(self, canvas, pos,)
        }
        pub fn hit_test(&self, coords: f32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, f32);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hit_test", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextLine {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TextLine\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextLine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TextLine {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TextLine {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TextLine {
        
    }
    impl crate::obj::cap::GodotDefault for TextLine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextLine {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextLine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TextLine {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TextLine > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextLine::add_string_ex`][super::TextLine::add_string_ex]."]
#[must_use]
pub struct ExAddString < 'a > {
    surround_object: &'a mut re_export::TextLine, text: GString, font: Gd < crate::engine::Font >, font_size: i32, language: GString, meta: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, text: GString, font: Gd < crate::engine::Font >, font_size: i32,) -> Self {
        Self {
            surround_object, text, font, font_size, language: GString::from(""), meta: Variant::nil(),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn meta(self, value: Variant) -> Self {
        Self {
            meta: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextLine::add_string_full(self.surround_object, self.text, self.font, self.font_size, self.language, self.meta,)
    }
}
#[doc = "Default-param extender for [`TextLine::add_object_ex`][super::TextLine::add_object_ex]."]
#[must_use]
pub struct ExAddObject < 'a > {
    surround_object: &'a mut re_export::TextLine, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, length: i32, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: Variant, size: Vector2,) -> Self {
        Self {
            surround_object, key, size, inline_align: crate::obj::EngineEnum::from_ord(5), length: 1i32, baseline: 0f32,
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn length(self, value: i32) -> Self {
        Self {
            length: value, .. self
        }
    }
    #[inline]
    pub fn baseline(self, value: f32) -> Self {
        Self {
            baseline: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextLine::add_object_full(self.surround_object, self.key, self.size, self.inline_align, self.length, self.baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::resize_object_ex`][super::TextLine::resize_object_ex]."]
#[must_use]
pub struct ExResizeObject < 'a > {
    surround_object: &'a mut re_export::TextLine, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: Variant, size: Vector2,) -> Self {
        Self {
            surround_object, key, size, inline_align: crate::obj::EngineEnum::from_ord(5), baseline: 0f32,
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn baseline(self, value: f32) -> Self {
        Self {
            baseline: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextLine::resize_object_full(self.surround_object, self.key, self.size, self.inline_align, self.baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_ex`][super::TextLine::draw_ex]."]
#[must_use]
pub struct ExDraw < 'a > {
    surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDraw < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        Self {
            surround_object, canvas, pos, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextLine::draw_full(self.surround_object, self.canvas, self.pos, self.color,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_outline_ex`][super::TextLine::draw_outline_ex]."]
#[must_use]
pub struct ExDrawOutline < 'a > {
    surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, outline_size: i32, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        Self {
            surround_object, canvas, pos, outline_size: 1i32, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn outline_size(self, value: i32) -> Self {
        Self {
            outline_size: value, .. self
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextLine::draw_outline_full(self.surround_object, self.canvas, self.pos, self.outline_size, self.color,)
    }
}