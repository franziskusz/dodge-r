#![doc = "Sidecar module for class [`ItemList`][crate::engine::ItemList].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ItemList` enums](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ItemList.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`item_list`][crate::engine::item_list]: sidecar module with related enum/flag types\n* [`IItemList`][crate::engine::IItemList]: virtual methods\n\n\nSee also [Godot docs for `ItemList`](https://docs.godotengine.org/en/stable/classes/class_itemlist.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ItemList {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ItemList`][crate::engine::ItemList].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ItemList` methods](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IItemList: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ItemList {
        pub(crate) fn add_item_full(&mut self, text: GString, icon: Gd < crate::engine::Texture2D >, selectable: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, Gd < crate::engine::Texture2D >, bool);
            let args = (text, icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_item(&mut self, text: GString,) -> i32 {
            self.add_item_ex(text,) . done()
        }
        #[inline]
        pub fn add_item_ex(&mut self, text: GString,) -> ExAddItem < '_ > {
            ExAddItem::new(self, text,)
        }
        pub(crate) fn add_icon_item_full(&mut self, icon: Gd < crate::engine::Texture2D >, selectable: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::Texture2D >, bool);
            let args = (icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_item(&mut self, icon: Gd < crate::engine::Texture2D >,) -> i32 {
            self.add_icon_item_ex(icon,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex(&mut self, icon: Gd < crate::engine::Texture2D >,) -> ExAddIconItem < '_ > {
            ExAddIconItem::new(self, icon,)
        }
        pub fn set_item_text(&mut self, idx: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (idx, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (idx, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_text_direction(&mut self, idx: i32, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::control::TextDirection);
            let args = (idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text_direction(&self, idx: i32,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_language(&mut self, idx: i32, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (idx, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_language(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_transposed(&mut self, idx: i32, transposed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, transposed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon_transposed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_icon_transposed(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_icon_transposed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_region(&mut self, idx: i32, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rect2);
            let args = (idx, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_region(&self, idx: i32,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_modulate(&mut self, idx: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (idx, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_modulate(&self, idx: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_selectable(&mut self, idx: i32, selectable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_selectable(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (idx, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_custom_bg_color(&mut self, idx: i32, custom_bg_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (idx, custom_bg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_custom_bg_color(&self, idx: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_custom_fg_color(&mut self, idx: i32, custom_fg_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (idx, custom_fg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_custom_fg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_custom_fg_color(&self, idx: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_custom_fg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_rect_full(&self, idx: i32, expand: bool,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, i32, bool);
            let args = (idx, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_item_rect(&self, idx: i32,) -> Rect2 {
            self.get_item_rect_ex(idx,) . done()
        }
        #[inline]
        pub fn get_item_rect_ex(&self, idx: i32,) -> ExGetItemRect < '_ > {
            ExGetItemRect::new(self, idx,)
        }
        pub fn set_item_tooltip_enabled(&mut self, idx: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_tooltip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_tooltip_enabled(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_tooltip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (idx, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn select_full(&mut self, idx: i32, single: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, single,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn select(&mut self, idx: i32,) {
            self.select_ex(idx,) . done()
        }
        #[inline]
        pub fn select_ex(&mut self, idx: i32,) -> ExSelect < '_ > {
            ExSelect::new(self, idx,)
        }
        pub fn deselect(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selected(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_items(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_items", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_item(&mut self, from_idx: i32, to_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (from_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sort_items_by_text(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sort_items_by_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_column_width(&mut self, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_column_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_same_column_width(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_same_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_same_column_width(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_same_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_text_lines(&mut self, lines: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_text_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_text_lines(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_text_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_columns(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_columns(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::engine::item_list::SelectMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::item_list::SelectMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::engine::item_list::SelectMode {
            type RetMarshal = PtrcallReturnT < crate::engine::item_list::SelectMode >;
            type CallSig = (crate::engine::item_list::SelectMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_mode(&mut self, mode: crate::engine::item_list::IconMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::item_list::IconMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_mode(&self,) -> crate::engine::item_list::IconMode {
            type RetMarshal = PtrcallReturnT < crate::engine::item_list::IconMode >;
            type CallSig = (crate::engine::item_list::IconMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_icon_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_icon_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_icon_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_icon_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_scale(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_height(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_height(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_auto_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_anything_selected(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_anything_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_at_position_full(&self, position: Vector2, exact: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2, bool);
            let args = (position, exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_item_at_position(&self, position: Vector2,) -> i32 {
            self.get_item_at_position_ex(position,) . done()
        }
        #[inline]
        pub fn get_item_at_position_ex(&self, position: Vector2,) -> ExGetItemAtPosition < '_ > {
            ExGetItemAtPosition::new(self, position,)
        }
        pub fn ensure_current_is_visible(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ensure_current_is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::engine::VScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::engine::text_server::OverrunBehavior,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::engine::text_server::OverrunBehavior {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::OverrunBehavior >;
            type CallSig = (crate::engine::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_list_size(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update_list_size", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ItemList {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ItemList\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ItemList {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ItemList {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for ItemList {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for ItemList {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ItemList {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ItemList {
        
    }
    impl crate::obj::ExportableObject for ItemList {
        
    }
    impl crate::obj::cap::GodotDefault for ItemList {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ItemList {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ItemList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ItemList {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ItemList > for $Class {
                
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
#[doc = "Default-param extender for [`ItemList::add_item_ex`][super::ItemList::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    surround_object: &'a mut re_export::ItemList, text: GString, icon: Gd < crate::engine::Texture2D >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, text: GString,) -> Self {
        Self {
            surround_object, text, icon: unimplemented !("see #156"), selectable: true,
        }
    }
    #[inline]
    pub fn icon(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            icon: value, .. self
        }
    }
    #[inline]
    pub fn selectable(self, value: bool) -> Self {
        Self {
            selectable: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::ItemList::add_item_full(self.surround_object, self.text, self.icon, self.selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::add_icon_item_ex`][super::ItemList::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    surround_object: &'a mut re_export::ItemList, icon: Gd < crate::engine::Texture2D >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, icon: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, icon, selectable: true,
        }
    }
    #[inline]
    pub fn selectable(self, value: bool) -> Self {
        Self {
            selectable: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::ItemList::add_icon_item_full(self.surround_object, self.icon, self.selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_rect_ex`][super::ItemList::get_item_rect_ex]."]
#[must_use]
pub struct ExGetItemRect < 'a > {
    surround_object: &'a re_export::ItemList, idx: i32, expand: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemRect < 'a > {
    fn new(surround_object: &'a re_export::ItemList, idx: i32,) -> Self {
        Self {
            surround_object, idx, expand: true,
        }
    }
    #[inline]
    pub fn expand(self, value: bool) -> Self {
        Self {
            expand: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        re_export::ItemList::get_item_rect_full(self.surround_object, self.idx, self.expand,)
    }
}
#[doc = "Default-param extender for [`ItemList::select_ex`][super::ItemList::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    surround_object: &'a mut re_export::ItemList, idx: i32, single: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, idx: i32,) -> Self {
        Self {
            surround_object, idx, single: true,
        }
    }
    #[inline]
    pub fn single(self, value: bool) -> Self {
        Self {
            single: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ItemList::select_full(self.surround_object, self.idx, self.single,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_at_position_ex`][super::ItemList::get_item_at_position_ex]."]
#[must_use]
pub struct ExGetItemAtPosition < 'a > {
    surround_object: &'a re_export::ItemList, position: Vector2, exact: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAtPosition < 'a > {
    fn new(surround_object: &'a re_export::ItemList, position: Vector2,) -> Self {
        Self {
            surround_object, position, exact: false,
        }
    }
    #[inline]
    pub fn exact(self, value: bool) -> Self {
        Self {
            exact: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::ItemList::get_item_at_position_full(self.surround_object, self.position, self.exact,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct IconMode {
    ord: i32
}
impl IconMode {
    pub const ICON_MODE_TOP: Self = Self {
        ord: 0i32
    };
    pub const ICON_MODE_LEFT: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for IconMode {
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
impl crate::builtin::meta::GodotConvert for IconMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for IconMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for IconMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectMode {
    ord: i32
}
impl SelectMode {
    pub const SELECT_SINGLE: Self = Self {
        ord: 0i32
    };
    pub const SELECT_MULTI: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for SelectMode {
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
impl crate::builtin::meta::GodotConvert for SelectMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SelectMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SelectMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}