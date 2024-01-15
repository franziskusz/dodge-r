#![doc = "Sidecar module for class [`TabBar`][crate::engine::TabBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabBar` enums](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabBar.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`tab_bar`][crate::engine::tab_bar]: sidecar module with related enum/flag types\n* [`ITabBar`][crate::engine::ITabBar]: virtual methods\n\n\nSee also [Godot docs for `TabBar`](https://docs.godotengine.org/en/stable/classes/class_tabbar.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TabBar`][crate::engine::TabBar].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TabBar` methods](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabBar: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TabBar {
        pub fn set_tab_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_previous_tab(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_previous_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_previous_available(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_previous_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_next_available(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_next_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_title(&mut self, tab_idx: i32, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (tab_idx, title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_text_direction(&mut self, tab_idx: i32, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::control::TextDirection);
            let args = (tab_idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_text_direction(&self, tab_idx: i32,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_language(&mut self, tab_idx: i32, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (tab_idx, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_language(&self, tab_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (tab_idx, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon_max_width(&mut self, tab_idx: i32, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (tab_idx, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon_max_width(&self, tab_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (tab_idx, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (tab_idx, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tab(&mut self, tab_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_tab_full(&mut self, title: GString, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::Texture2D >);
            let args = (title, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_tab(&mut self,) {
            self.add_tab_ex() . done()
        }
        #[inline]
        pub fn add_tab_ex(&mut self,) -> ExAddTab < '_ > {
            ExAddTab::new(self,)
        }
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_idx_at_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::engine::tab_bar::AlignmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tab_bar::AlignmentMode);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::engine::tab_bar::AlignmentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tab_bar::AlignmentMode >;
            type CallSig = (crate::engine::tab_bar::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_offset(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset_buttons_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset_buttons_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ensure_tab_visible(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ensure_tab_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_rect(&self, tab_idx: i32,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_tab(&mut self, from: i32, to: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_close_display_policy(&mut self, policy: crate::engine::tab_bar::CloseButtonDisplayPolicy,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tab_bar::CloseButtonDisplayPolicy);
            let args = (policy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_close_display_policy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_close_display_policy(&self,) -> crate::engine::tab_bar::CloseButtonDisplayPolicy {
            type RetMarshal = PtrcallReturnT < crate::engine::tab_bar::CloseButtonDisplayPolicy >;
            type CallSig = (crate::engine::tab_bar::CloseButtonDisplayPolicy,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_close_display_policy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_tab_width(&mut self, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_tab_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_tab_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_tab_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scrolling_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scrolling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scrolling_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scrolling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_to_selected(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scroll_to_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scroll_to_selected(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scroll_to_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_with_rmb(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_select_with_rmb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_with_rmb(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_select_with_rmb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tabs(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_tabs", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TabBar {
        type Base = crate::engine::Control;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TabBar\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TabBar {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Control > for TabBar {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for TabBar {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for TabBar {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TabBar {
        
    }
    impl crate::obj::ExportableObject for TabBar {
        
    }
    impl crate::obj::cap::GodotDefault for TabBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabBar {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TabBar {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TabBar > for $Class {
                
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
#[doc = "Default-param extender for [`TabBar::add_tab_ex`][super::TabBar::add_tab_ex]."]
#[must_use]
pub struct ExAddTab < 'a > {
    surround_object: &'a mut re_export::TabBar, title: GString, icon: Gd < crate::engine::Texture2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTab < 'a > {
    fn new(surround_object: &'a mut re_export::TabBar,) -> Self {
        Self {
            surround_object, title: GString::from(""), icon: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn title(self, value: GString) -> Self {
        Self {
            title: value, .. self
        }
    }
    #[inline]
    pub fn icon(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            icon: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TabBar::add_tab_full(self.surround_object, self.title, self.icon,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlignmentMode {
    ord: i32
}
impl AlignmentMode {
    pub const ALIGNMENT_LEFT: Self = Self {
        ord: 0i32
    };
    pub const ALIGNMENT_CENTER: Self = Self {
        ord: 1i32
    };
    pub const ALIGNMENT_RIGHT: Self = Self {
        ord: 2i32
    };
    pub const ALIGNMENT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AlignmentMode {
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
impl crate::obj::IndexEnum for AlignmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for AlignmentMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AlignmentMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AlignmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CloseButtonDisplayPolicy {
    ord: i32
}
impl CloseButtonDisplayPolicy {
    pub const CLOSE_BUTTON_SHOW_NEVER: Self = Self {
        ord: 0i32
    };
    pub const CLOSE_BUTTON_SHOW_ACTIVE_ONLY: Self = Self {
        ord: 1i32
    };
    pub const CLOSE_BUTTON_SHOW_ALWAYS: Self = Self {
        ord: 2i32
    };
    pub const CLOSE_BUTTON_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for CloseButtonDisplayPolicy {
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
impl crate::obj::IndexEnum for CloseButtonDisplayPolicy {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for CloseButtonDisplayPolicy {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CloseButtonDisplayPolicy {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CloseButtonDisplayPolicy {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}