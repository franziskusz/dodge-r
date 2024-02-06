#![doc = "Sidecar module for class [`Label3D`][crate::engine::Label3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label3D` enums](https://docs.godotengine.org/en/stable/classes/class_label3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Label3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`label_3d`][crate::engine::label_3d]: sidecar module with related enum/flag types\n* [`ILabel3D`][crate::engine::ILabel3D]: virtual methods\n\n\nSee also [Godot docs for `Label3D`](https://docs.godotengine.org/en/stable/classes/class_label3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Label3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Label3D`][crate::engine::Label3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Label3D` methods](https://docs.godotengine.org/en/stable/classes/class_label3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabel3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
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
    impl Label3D {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::engine::global::HorizontalAlignment,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::engine::global::HorizontalAlignment {
            type RetMarshal = PtrcallReturnT < crate::engine::global::HorizontalAlignment >;
            type CallSig = (crate::engine::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::engine::global::VerticalAlignment,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::VerticalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::engine::global::VerticalAlignment {
            type RetMarshal = PtrcallReturnT < crate::engine::global::VerticalAlignment >;
            type CallSig = (crate::engine::global::VerticalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outline_modulate(&mut self, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outline_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outline_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outline_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::engine::text_server::Direction,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::Direction);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::engine::text_server::Direction {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Direction >;
            type CallSig = (crate::engine::text_server::Direction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::engine::text_server::StructuredTextParser,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::engine::text_server::StructuredTextParser {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::StructuredTextParser >;
            type CallSig = (crate::engine::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (args,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uppercase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_uppercase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outline_render_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outline_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outline_render_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outline_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font(&mut self, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >);
            let args = (font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outline_size(&mut self, outline_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outline_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (line_spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_line_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::engine::text_server::AutowrapMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::engine::text_server::AutowrapMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::AutowrapMode >;
            type CallSig = (crate::engine::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::engine::text_server::JustificationFlag,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::JustificationFlag);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::engine::text_server::JustificationFlag {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::JustificationFlag >;
            type CallSig = (crate::engine::text_server::JustificationFlag,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_flag(&mut self, flag: crate::engine::label_3d::DrawFlags, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::label_3d::DrawFlags, bool);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_flag(&self, flag: crate::engine::label_3d::DrawFlags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::label_3d::DrawFlags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::engine::base_material_3d::BillboardMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::BillboardMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::engine::base_material_3d::BillboardMode {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::BillboardMode >;
            type CallSig = (crate::engine::base_material_3d::BillboardMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: crate::engine::label_3d::AlphaCutMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::label_3d::AlphaCutMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_cut_mode(&self,) -> crate::engine::label_3d::AlphaCutMode {
            type RetMarshal = PtrcallReturnT < crate::engine::label_3d::AlphaCutMode >;
            type CallSig = (crate::engine::label_3d::AlphaCutMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::engine::base_material_3d::AlphaAntiAliasing,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::AlphaAntiAliasing);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::engine::base_material_3d::AlphaAntiAliasing {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::AlphaAntiAliasing >;
            type CallSig = (crate::engine::base_material_3d::AlphaAntiAliasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::engine::base_material_3d::TextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::base_material_3d::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::engine::base_material_3d::TextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::base_material_3d::TextureFilter >;
            type CallSig = (crate::engine::base_material_3d::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::engine::TriangleMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TriangleMesh > >;
            type CallSig = (Option < Gd < crate::engine::TriangleMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_triangle_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Label3D {
        type Base = crate::engine::GeometryInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Label3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Label3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Label3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for Label3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for Label3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for Label3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Label3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Label3D {
        
    }
    impl crate::obj::ExportableObject for Label3D {
        
    }
    impl crate::obj::cap::GodotDefault for Label3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Label3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Label3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Label3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Label3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::GeometryInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DrawFlags {
    ord: i32
}
impl DrawFlags {
    pub const FLAG_SHADED: Self = Self {
        ord: 0i32
    };
    pub const FLAG_DOUBLE_SIDED: Self = Self {
        ord: 1i32
    };
    pub const FLAG_DISABLE_DEPTH_TEST: Self = Self {
        ord: 2i32
    };
    pub const FLAG_FIXED_SIZE: Self = Self {
        ord: 3i32
    };
    pub const FLAG_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for DrawFlags {
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
impl crate::obj::IndexEnum for DrawFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for DrawFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DrawFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlphaCutMode {
    ord: i32
}
impl AlphaCutMode {
    pub const ALPHA_CUT_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const ALPHA_CUT_DISCARD: Self = Self {
        ord: 1i32
    };
    pub const ALPHA_CUT_OPAQUE_PREPASS: Self = Self {
        ord: 2i32
    };
    pub const ALPHA_CUT_HASH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AlphaCutMode {
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
impl crate::builtin::meta::GodotConvert for AlphaCutMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AlphaCutMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AlphaCutMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}