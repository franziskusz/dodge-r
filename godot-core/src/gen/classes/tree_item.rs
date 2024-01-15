#![doc = "Sidecar module for class [`TreeItem`][crate::engine::TreeItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TreeItem` enums](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TreeItem.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`tree_item`][crate::engine::tree_item]: sidecar module with related enum/flag types\n* [`ITreeItem`][crate::engine::ITreeItem]: virtual methods\n\n\nSee also [Godot docs for `TreeItem`](https://docs.godotengine.org/en/stable/classes/class_treeitem.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TreeItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TreeItem`][crate::engine::TreeItem].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TreeItem` methods](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITreeItem: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TreeItem {
        pub fn set_cell_mode(&mut self, column: i32, mode: crate::engine::tree_item::TreeCellMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::tree_item::TreeCellMode);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_mode(&self, column: i32,) -> crate::engine::tree_item::TreeCellMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tree_item::TreeCellMode >;
            type CallSig = (crate::engine::tree_item::TreeCellMode, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_multiline(&mut self, column: i32, multiline: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, multiline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_edit_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_edit_multiline(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_edit_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checked(&mut self, column: i32, checked: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indeterminate(&mut self, column: i32, indeterminate: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_indeterminate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checked(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_indeterminate(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_indeterminate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn propagate_check_full(&mut self, column: i32, emit_signal: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, emit_signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "propagate_check", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn propagate_check(&mut self, column: i32,) {
            self.propagate_check_ex(column,) . done()
        }
        #[inline]
        pub fn propagate_check_ex(&mut self, column: i32,) -> ExPropagateCheck < '_ > {
            ExPropagateCheck::new(self, column,)
        }
        pub fn set_text(&mut self, column: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, column: i32, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::control::TextDirection);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self, column: i32,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, column: i32, autowrap_mode: crate::engine::text_server::AutowrapMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::text_server::AutowrapMode);
            let args = (column, autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self, column: i32,) -> crate::engine::text_server::AutowrapMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::AutowrapMode >;
            type CallSig = (crate::engine::text_server::AutowrapMode, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, column: i32, overrun_behavior: crate::engine::text_server::OverrunBehavior,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::text_server::OverrunBehavior);
            let args = (column, overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self, column: i32,) -> crate::engine::text_server::OverrunBehavior {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::OverrunBehavior >;
            type CallSig = (crate::engine::text_server::OverrunBehavior, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, column: i32, parser: crate::engine::text_server::StructuredTextParser,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::text_server::StructuredTextParser);
            let args = (column, parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self, column: i32,) -> crate::engine::text_server::StructuredTextParser {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::StructuredTextParser >;
            type CallSig = (crate::engine::text_server::StructuredTextParser, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, column: i32, args: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, VariantArray);
            let args = (column, args,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self, column: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, column: i32, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suffix(&mut self, column: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_suffix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suffix(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_suffix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon(&mut self, column: i32, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (column, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon(&self, column: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_region(&mut self, column: i32, region: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rect2);
            let args = (column, region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_region(&self, column: i32,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_max_width(&mut self, column: i32, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (column, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_max_width(&self, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_modulate(&mut self, column: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (column, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_modulate(&self, column: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_range(&mut self, column: i32, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f64);
            let args = (column, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_range(&self, column: i32,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_range_config_full(&mut self, column: i32, min: f64, max: f64, step: f64, expr: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f64, f64, f64, bool);
            let args = (column, min, max, step, expr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_range_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_range_config(&mut self, column: i32, min: f64, max: f64, step: f64,) {
            self.set_range_config_ex(column, min, max, step,) . done()
        }
        #[inline]
        pub fn set_range_config_ex(&mut self, column: i32, min: f64, max: f64, step: f64,) -> ExSetRangeConfig < '_ > {
            ExSetRangeConfig::new(self, column, min, max, step,)
        }
        pub fn get_range_config(&mut self, column: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_range_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metadata(&mut self, column: i32, meta: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (column, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metadata(&self, column: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_draw(&mut self, column: i32, object: Gd < crate::engine::Object >, callback: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Object >, StringName);
            let args = (column, object, callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collapsed(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collapsed(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collapsed_recursive(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collapsed_recursive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_any_collapsed_full(&mut self, only_visible: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, bool);
            let args = (only_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_any_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_any_collapsed(&mut self,) -> bool {
            self.is_any_collapsed_ex() . done()
        }
        #[inline]
        pub fn is_any_collapsed_ex(&mut self,) -> ExIsAnyCollapsed < '_ > {
            ExIsAnyCollapsed::new(self,)
        }
        pub fn set_visible(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uncollapse_tree(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "uncollapse_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_minimum_height(&mut self, height: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_minimum_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_minimum_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_minimum_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selectable(&mut self, column: i32, selectable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selectable(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selected(&mut self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select(&mut self, column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect(&mut self, column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable(&mut self, column: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable(&mut self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_color(&mut self, column: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (column, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_color(&self, column: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_custom_color(&mut self, column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_font(&mut self, column: i32, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Font >);
            let args = (column, font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_font(&self, column: i32,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_font_size(&mut self, column: i32, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (column, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_font_size(&self, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_custom_bg_color_full(&mut self, column: i32, color: Color, just_outline: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color, bool);
            let args = (column, color, just_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_custom_bg_color(&mut self, column: i32, color: Color,) {
            self.set_custom_bg_color_ex(column, color,) . done()
        }
        #[inline]
        pub fn set_custom_bg_color_ex(&mut self, column: i32, color: Color,) -> ExSetCustomBgColor < '_ > {
            ExSetCustomBgColor::new(self, column, color,)
        }
        pub fn clear_custom_bg_color(&mut self, column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_bg_color(&self, column: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_as_button(&mut self, column: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_as_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_custom_set_as_button(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_custom_set_as_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_button_full(&mut self, column: i32, button: Gd < crate::engine::Texture2D >, id: i32, disabled: bool, tooltip_text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >, i32, bool, GString);
            let args = (column, button, id, disabled, tooltip_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_button(&mut self, column: i32, button: Gd < crate::engine::Texture2D >,) {
            self.add_button_ex(column, button,) . done()
        }
        #[inline]
        pub fn add_button_ex(&mut self, column: i32, button: Gd < crate::engine::Texture2D >,) -> ExAddButton < '_ > {
            ExAddButton::new(self, column, button,)
        }
        pub fn get_button_count(&self, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_tooltip_text(&self, column: i32, button_index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_id(&self, column: i32, button_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_by_id(&self, column: i32, id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button_by_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button(&self, column: i32, button_index: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_tooltip_text(&mut self, column: i32, button_index: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, GString);
            let args = (column, button_index, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_button_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button(&mut self, column: i32, button_index: i32, button: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Gd < crate::engine::Texture2D >);
            let args = (column, button_index, button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_button(&mut self, column: i32, button_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_disabled(&mut self, column: i32, button_index: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (column, button_index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_button_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_color(&mut self, column: i32, button_index: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Color);
            let args = (column, button_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_button_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_button_disabled(&self, column: i32, button_index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_button_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, column: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tooltip_text(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_alignment(&mut self, column: i32, text_alignment: crate::engine::global::HorizontalAlignment,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::global::HorizontalAlignment);
            let args = (column, text_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_alignment(&self, column: i32,) -> crate::engine::global::HorizontalAlignment {
            type RetMarshal = PtrcallReturnT < crate::engine::global::HorizontalAlignment >;
            type CallSig = (crate::engine::global::HorizontalAlignment, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_right(&mut self, column: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_expand_right", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_expand_right(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_expand_right", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_folding(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_folding_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_folding_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_child_full(&mut self, index: i32,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_child(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.create_child_ex() . done()
        }
        #[inline]
        pub fn create_child_ex(&mut self,) -> ExCreateChild < '_ > {
            ExCreateChild::new(self,)
        }
        pub fn add_child(&mut self, child: Gd < crate::engine::TreeItem >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >);
            let args = (child,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_child(&mut self, child: Gd < crate::engine::TreeItem >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >);
            let args = (child,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::engine::Tree > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Tree > >;
            type CallSig = (Option < Gd < crate::engine::Tree > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_prev(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_prev", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_first_child(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_first_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_next_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_next_in_tree(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.get_next_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_next_in_tree_ex(&mut self,) -> ExGetNextInTree < '_ > {
            ExGetNextInTree::new(self,)
        }
        pub(crate) fn get_prev_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_prev_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_prev_in_tree(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.get_prev_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_prev_in_tree_ex(&mut self,) -> ExGetPrevInTree < '_ > {
            ExGetPrevInTree::new(self,)
        }
        pub(crate) fn get_next_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_next_visible(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.get_next_visible_ex() . done()
        }
        #[inline]
        pub fn get_next_visible_ex(&mut self,) -> ExGetNextVisible < '_ > {
            ExGetNextVisible::new(self,)
        }
        pub(crate) fn get_prev_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_prev_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_prev_visible(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.get_prev_visible_ex() . done()
        }
        #[inline]
        pub fn get_prev_visible_ex(&mut self,) -> ExGetPrevVisible < '_ > {
            ExGetPrevVisible::new(self,)
        }
        pub fn get_child(&mut self, index: i32,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_child_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_child_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_children(&mut self,) -> Array < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::TreeItem > > >;
            type CallSig = (Array < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_index(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_before(&mut self, item: Gd < crate::engine::TreeItem >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_before", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_after(&mut self, item: Gd < crate::engine::TreeItem >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_after", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_recursive(&mut self, method: StringName, varargs: &[Variant]) {
            type CallSig = ((), StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9125usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_recursive", self.object_ptr, self.__checked_id(), args, varargs)
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
    impl crate::obj::GodotClass for TreeItem {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TreeItem\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TreeItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TreeItem {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for TreeItem {
        
    }
    impl std::ops::Deref for TreeItem {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TreeItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TreeItem {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TreeItem > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TreeItem::propagate_check_ex`][super::TreeItem::propagate_check_ex]."]
#[must_use]
pub struct ExPropagateCheck < 'a > {
    surround_object: &'a mut re_export::TreeItem, column: i32, emit_signal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCheck < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32,) -> Self {
        Self {
            surround_object, column, emit_signal: true,
        }
    }
    #[inline]
    pub fn emit_signal(self, value: bool) -> Self {
        Self {
            emit_signal: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TreeItem::propagate_check_full(self.surround_object, self.column, self.emit_signal,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_range_config_ex`][super::TreeItem::set_range_config_ex]."]
#[must_use]
pub struct ExSetRangeConfig < 'a > {
    surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64, expr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRangeConfig < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64,) -> Self {
        Self {
            surround_object, column, min, max, step, expr: false,
        }
    }
    #[inline]
    pub fn expr(self, value: bool) -> Self {
        Self {
            expr: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TreeItem::set_range_config_full(self.surround_object, self.column, self.min, self.max, self.step, self.expr,)
    }
}
#[doc = "Default-param extender for [`TreeItem::is_any_collapsed_ex`][super::TreeItem::is_any_collapsed_ex]."]
#[must_use]
pub struct ExIsAnyCollapsed < 'a > {
    surround_object: &'a mut re_export::TreeItem, only_visible: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAnyCollapsed < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, only_visible: false,
        }
    }
    #[inline]
    pub fn only_visible(self, value: bool) -> Self {
        Self {
            only_visible: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TreeItem::is_any_collapsed_full(self.surround_object, self.only_visible,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_custom_bg_color_ex`][super::TreeItem::set_custom_bg_color_ex]."]
#[must_use]
pub struct ExSetCustomBgColor < 'a > {
    surround_object: &'a mut re_export::TreeItem, column: i32, color: Color, just_outline: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomBgColor < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, color: Color,) -> Self {
        Self {
            surround_object, column, color, just_outline: false,
        }
    }
    #[inline]
    pub fn just_outline(self, value: bool) -> Self {
        Self {
            just_outline: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TreeItem::set_custom_bg_color_full(self.surround_object, self.column, self.color, self.just_outline,)
    }
}
#[doc = "Default-param extender for [`TreeItem::add_button_ex`][super::TreeItem::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'a > {
    surround_object: &'a mut re_export::TreeItem, column: i32, button: Gd < crate::engine::Texture2D >, id: i32, disabled: bool, tooltip_text: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddButton < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, button: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, column, button, id: - 1i32, disabled: false, tooltip_text: GString::from(""),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn disabled(self, value: bool) -> Self {
        Self {
            disabled: value, .. self
        }
    }
    #[inline]
    pub fn tooltip_text(self, value: GString) -> Self {
        Self {
            tooltip_text: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TreeItem::add_button_full(self.surround_object, self.column, self.button, self.id, self.disabled, self.tooltip_text,)
    }
}
#[doc = "Default-param extender for [`TreeItem::create_child_ex`][super::TreeItem::create_child_ex]."]
#[must_use]
pub struct ExCreateChild < 'a > {
    surround_object: &'a mut re_export::TreeItem, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateChild < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TreeItem > > {
        re_export::TreeItem::create_child_full(self.surround_object, self.index,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_in_tree_ex`][super::TreeItem::get_next_in_tree_ex]."]
#[must_use]
pub struct ExGetNextInTree < 'a > {
    surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, wrap: false,
        }
    }
    #[inline]
    pub fn wrap(self, value: bool) -> Self {
        Self {
            wrap: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TreeItem > > {
        re_export::TreeItem::get_next_in_tree_full(self.surround_object, self.wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_in_tree_ex`][super::TreeItem::get_prev_in_tree_ex]."]
#[must_use]
pub struct ExGetPrevInTree < 'a > {
    surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, wrap: false,
        }
    }
    #[inline]
    pub fn wrap(self, value: bool) -> Self {
        Self {
            wrap: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TreeItem > > {
        re_export::TreeItem::get_prev_in_tree_full(self.surround_object, self.wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_visible_ex`][super::TreeItem::get_next_visible_ex]."]
#[must_use]
pub struct ExGetNextVisible < 'a > {
    surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, wrap: false,
        }
    }
    #[inline]
    pub fn wrap(self, value: bool) -> Self {
        Self {
            wrap: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TreeItem > > {
        re_export::TreeItem::get_next_visible_full(self.surround_object, self.wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_visible_ex`][super::TreeItem::get_prev_visible_ex]."]
#[must_use]
pub struct ExGetPrevVisible < 'a > {
    surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        Self {
            surround_object, wrap: false,
        }
    }
    #[inline]
    pub fn wrap(self, value: bool) -> Self {
        Self {
            wrap: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::TreeItem > > {
        re_export::TreeItem::get_prev_visible_full(self.surround_object, self.wrap,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TreeCellMode {
    ord: i32
}
impl TreeCellMode {
    pub const CELL_MODE_STRING: Self = Self {
        ord: 0i32
    };
    pub const CELL_MODE_CHECK: Self = Self {
        ord: 1i32
    };
    pub const CELL_MODE_RANGE: Self = Self {
        ord: 2i32
    };
    pub const CELL_MODE_ICON: Self = Self {
        ord: 3i32
    };
    pub const CELL_MODE_CUSTOM: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TreeCellMode {
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
impl crate::builtin::meta::GodotConvert for TreeCellMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TreeCellMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TreeCellMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}