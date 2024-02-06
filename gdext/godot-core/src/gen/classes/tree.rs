#![doc = "Sidecar module for class [`Tree`][crate::engine::Tree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tree` enums](https://docs.godotengine.org/en/stable/classes/class_tree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tree.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`tree`][crate::engine::tree]: sidecar module with related enum/flag types\n* [`ITree`][crate::engine::ITree]: virtual methods\n\n\nSee also [Godot docs for `Tree`](https://docs.godotengine.org/en/stable/classes/class_tree.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Tree`][crate::engine::Tree].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Tree` methods](https://docs.godotengine.org/en/stable/classes/class_tree.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITree: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tree {
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_item_full(&mut self, parent: Gd < crate::engine::TreeItem >, index: i32,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, Gd < crate::engine::TreeItem >, i32);
            let args = (parent, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_item(&mut self,) -> Option < Gd < crate::engine::TreeItem > > {
            self.create_item_ex() . done()
        }
        #[inline]
        pub fn create_item_ex(&mut self,) -> ExCreateItem < '_ > {
            ExCreateItem::new(self,)
        }
        pub fn get_root(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_custom_minimum_width(&mut self, column: i32, min_width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (column, min_width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_custom_minimum_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_expand(&mut self, column: i32, expand: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_expand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_expand_ratio(&mut self, column: i32, ratio: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (column, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_expand_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_clip_content(&mut self, column: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_clip_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_column_expanding(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_column_expanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_column_clipping_content(&self, column: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_column_clipping_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_expand_ratio(&self, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_expand_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_width(&self, column: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_root(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hide_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_root_hidden(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_root_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_selected(&mut self, from: Gd < crate::engine::TreeItem >,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, Gd < crate::engine::TreeItem >);
            let args = (from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selected(&mut self, item: Gd < crate::engine::TreeItem >, column: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >, i32);
            let args = (item, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_column(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pressed_button(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pressed_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::engine::tree::SelectMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tree::SelectMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::engine::tree::SelectMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tree::SelectMode >;
            type CallSig = (crate::engine::tree::SelectMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deselect_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_columns(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_columns(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited(&self,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_column(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edited_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn edit_selected_full(&mut self, force_edit: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, bool);
            let args = (force_edit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "edit_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn edit_selected(&mut self,) -> bool {
            self.edit_selected_ex() . done()
        }
        #[inline]
        pub fn edit_selected_ex(&mut self,) -> ExEditSelected < '_ > {
            ExEditSelected::new(self,)
        }
        pub fn get_custom_popup_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_popup_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_area_rect_full(&self, item: Gd < crate::engine::TreeItem >, column: i32, button_index: i32,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, Gd < crate::engine::TreeItem >, i32, i32);
            let args = (item, column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_area_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_item_area_rect(&self, item: Gd < crate::engine::TreeItem >,) -> Rect2 {
            self.get_item_area_rect_ex(item,) . done()
        }
        #[inline]
        pub fn get_item_area_rect_ex(&self, item: Gd < crate::engine::TreeItem >,) -> ExGetItemAreaRect < '_ > {
            ExGetItemAreaRect::new(self, item,)
        }
        pub fn get_item_at_position(&self, position: Vector2,) -> Option < Gd < crate::engine::TreeItem > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TreeItem > >;
            type CallSig = (Option < Gd < crate::engine::TreeItem > >, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_at_position(&self, position: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drop_section_at_position(&self, position: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drop_section_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_id_at_position(&self, position: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_button_id_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ensure_cursor_is_visible(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ensure_cursor_is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_titles_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_titles_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_column_titles_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_column_titles_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title(&mut self, column: i32, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_alignment(&mut self, column: i32, title_alignment: crate::engine::global::HorizontalAlignment,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::global::HorizontalAlignment);
            let args = (column, title_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_title_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_alignment(&self, column: i32,) -> crate::engine::global::HorizontalAlignment {
            type RetMarshal = PtrcallReturnT < crate::engine::global::HorizontalAlignment >;
            type CallSig = (crate::engine::global::HorizontalAlignment, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_title_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_direction(&mut self, column: i32, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::control::TextDirection);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_title_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_direction(&self, column: i32,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_title_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_language(&mut self, column: i32, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (column, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_column_title_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_language(&self, column: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_column_title_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scroll(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn scroll_to_item_full(&mut self, item: Gd < crate::engine::TreeItem >, center_on_item: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TreeItem >, bool);
            let args = (item, center_on_item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scroll_to_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn scroll_to_item(&mut self, item: Gd < crate::engine::TreeItem >,) {
            self.scroll_to_item_ex(item,) . done()
        }
        #[inline]
        pub fn scroll_to_item_ex(&mut self, item: Gd < crate::engine::TreeItem >,) -> ExScrollToItem < '_ > {
            ExScrollToItem::new(self, item,)
        }
        pub fn set_h_scroll_enabled(&mut self, h_scroll: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_h_scroll_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_h_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_scroll_enabled(&mut self, h_scroll: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_v_scroll_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_v_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_folding(&mut self, hide: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (hide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hide_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_folding_hidden(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_folding_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_recursive_folding(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_recursive_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_recursive_folding_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_recursive_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drop_mode_flags(&mut self, flags: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drop_mode_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drop_mode_flags(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drop_mode_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_search", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Tree {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Tree\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Tree {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for Tree {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Tree {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Tree {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Tree {
        
    }
    impl crate::obj::ExportableObject for Tree {
        
    }
    impl crate::obj::cap::GodotDefault for Tree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tree {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Tree {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Tree > for $Class {
                
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
#[doc = "Default-param extender for [`Tree::create_item_ex`][super::Tree::create_item_ex]."]
#[must_use]
pub struct ExCreateItem < 'a > {
    surround_object: &'a mut re_export::Tree, parent: Gd < crate::engine::TreeItem >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        Self {
            surround_object, parent: unimplemented !("see #156"), index: - 1i32,
        }
    }
    #[inline]
    pub fn parent(self, value: Gd < crate::engine::TreeItem >) -> Self {
        Self {
            parent: value, .. self
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
        re_export::Tree::create_item_full(self.surround_object, self.parent, self.index,)
    }
}
#[doc = "Default-param extender for [`Tree::edit_selected_ex`][super::Tree::edit_selected_ex]."]
#[must_use]
pub struct ExEditSelected < 'a > {
    surround_object: &'a mut re_export::Tree, force_edit: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditSelected < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        Self {
            surround_object, force_edit: false,
        }
    }
    #[inline]
    pub fn force_edit(self, value: bool) -> Self {
        Self {
            force_edit: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Tree::edit_selected_full(self.surround_object, self.force_edit,)
    }
}
#[doc = "Default-param extender for [`Tree::get_item_area_rect_ex`][super::Tree::get_item_area_rect_ex]."]
#[must_use]
pub struct ExGetItemAreaRect < 'a > {
    surround_object: &'a re_export::Tree, item: Gd < crate::engine::TreeItem >, column: i32, button_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAreaRect < 'a > {
    fn new(surround_object: &'a re_export::Tree, item: Gd < crate::engine::TreeItem >,) -> Self {
        Self {
            surround_object, item, column: - 1i32, button_index: - 1i32,
        }
    }
    #[inline]
    pub fn column(self, value: i32) -> Self {
        Self {
            column: value, .. self
        }
    }
    #[inline]
    pub fn button_index(self, value: i32) -> Self {
        Self {
            button_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        re_export::Tree::get_item_area_rect_full(self.surround_object, self.item, self.column, self.button_index,)
    }
}
#[doc = "Default-param extender for [`Tree::scroll_to_item_ex`][super::Tree::scroll_to_item_ex]."]
#[must_use]
pub struct ExScrollToItem < 'a > {
    surround_object: &'a mut re_export::Tree, item: Gd < crate::engine::TreeItem >, center_on_item: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScrollToItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree, item: Gd < crate::engine::TreeItem >,) -> Self {
        Self {
            surround_object, item, center_on_item: false,
        }
    }
    #[inline]
    pub fn center_on_item(self, value: bool) -> Self {
        Self {
            center_on_item: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Tree::scroll_to_item_full(self.surround_object, self.item, self.center_on_item,)
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
    pub const SELECT_ROW: Self = Self {
        ord: 1i32
    };
    pub const SELECT_MULTI: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for SelectMode {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DropModeFlags {
    ord: i32
}
impl DropModeFlags {
    pub const DROP_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DROP_MODE_ON_ITEM: Self = Self {
        ord: 1i32
    };
    pub const DROP_MODE_INBETWEEN: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DropModeFlags {
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
impl crate::builtin::meta::GodotConvert for DropModeFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DropModeFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DropModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}