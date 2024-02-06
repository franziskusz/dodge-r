#![doc = "Sidecar module for class [`TabContainer`][crate::engine::TabContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabContainer` enums](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabContainer.`\n\nInherits [`Container`][crate::engine::Container].\n\nRelated symbols:\n\n* [`ITabContainer`][crate::engine::ITabContainer]: virtual methods\n\n\nSee also [Godot docs for `TabContainer`](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TabContainer`][crate::engine::TabContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TabContainer` methods](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabContainer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
            unimplemented !()
        }
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
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
    impl TabContainer {
        pub fn get_tab_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_previous_tab(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_previous_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_previous_available(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_previous_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_next_available(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select_next_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab_control(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_tab_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_bar(&self,) -> Option < Gd < crate::engine::TabBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TabBar > >;
            type CallSig = (Option < Gd < crate::engine::TabBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_control(&self, tab_idx: i32,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::engine::tab_bar::AlignmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::tab_bar::AlignmentMode);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::engine::tab_bar::AlignmentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::tab_bar::AlignmentMode >;
            type CallSig = (crate::engine::tab_bar::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tabs_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_tabs_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_tabs_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_all_tabs_in_front(&mut self, is_front: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (is_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_all_tabs_in_front", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_all_tabs_in_front(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_all_tabs_in_front", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_title(&mut self, tab_idx: i32, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (tab_idx, title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (tab_idx, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (tab_idx, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (tab_idx, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_idx_at_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_idx_from_control(&self, control: Gd < crate::engine::Control >,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_idx_from_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_popup(&mut self, popup: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (popup,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::engine::Popup > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Popup > >;
            type CallSig = (Option < Gd < crate::engine::Popup > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hidden_tabs_for_min_size(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_hidden_tabs_for_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_hidden_tabs_for_min_size(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_hidden_tabs_for_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_focus_mode(&mut self, focus_mode: crate::engine::control::FocusMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::FocusMode);
            let args = (focus_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tab_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_focus_mode(&self,) -> crate::engine::control::FocusMode {
            type RetMarshal = PtrcallReturnT < crate::engine::control::FocusMode >;
            type CallSig = (crate::engine::control::FocusMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tab_focus_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TabContainer {
        type Base = crate::engine::Container;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TabContainer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TabContainer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Container > for TabContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for TabContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for TabContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for TabContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TabContainer {
        
    }
    impl crate::obj::ExportableObject for TabContainer {
        
    }
    impl crate::obj::cap::GodotDefault for TabContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabContainer {
        type Target = crate::engine::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TabContainer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TabContainer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Container > for $Class {
                
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