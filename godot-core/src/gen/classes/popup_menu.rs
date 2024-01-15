#![doc = "Sidecar module for class [`PopupMenu`][crate::engine::PopupMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PopupMenu` enums](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PopupMenu.`\n\nInherits [`Popup`][crate::engine::Popup].\n\nRelated symbols:\n\n* [`popup_menu`][crate::engine::popup_menu]: sidecar module with related enum/flag types\n* [`IPopupMenu`][crate::engine::IPopupMenu]: virtual methods\n\n\nSee also [Godot docs for `PopupMenu`](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PopupMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PopupMenu`][crate::engine::PopupMenu].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PopupMenu` methods](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPopupMenu: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
            unimplemented !()
        }
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl PopupMenu {
        pub(crate) fn activate_item_by_event_full(&mut self, event: Gd < crate::engine::InputEvent >, for_global_only: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::InputEvent >, bool);
            let args = (event, for_global_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "activate_item_by_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn activate_item_by_event(&mut self, event: Gd < crate::engine::InputEvent >,) -> bool {
            self.activate_item_by_event_ex(event,) . done()
        }
        #[inline]
        pub fn activate_item_by_event_ex(&mut self, event: Gd < crate::engine::InputEvent >,) -> ExActivateItemByEvent < '_ > {
            ExActivateItemByEvent::new(self, event,)
        }
        pub(crate) fn add_item_full(&mut self, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, crate::engine::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_item(&mut self, label: GString,) {
            self.add_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_item_ex(&mut self, label: GString,) -> ExAddItem < '_ > {
            ExAddItem::new(self, label,)
        }
        pub(crate) fn add_icon_item_full(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, GString, i32, crate::engine::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_item(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) {
            self.add_icon_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) -> ExAddIconItem < '_ > {
            ExAddIconItem::new(self, texture, label,)
        }
        pub(crate) fn add_check_item_full(&mut self, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, crate::engine::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_check_item(&mut self, label: GString,) {
            self.add_check_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_check_item_ex(&mut self, label: GString,) -> ExAddCheckItem < '_ > {
            ExAddCheckItem::new(self, label,)
        }
        pub(crate) fn add_icon_check_item_full(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, GString, i32, crate::engine::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_check_item(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) {
            self.add_icon_check_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_check_item_ex(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) -> ExAddIconCheckItem < '_ > {
            ExAddIconCheckItem::new(self, texture, label,)
        }
        pub(crate) fn add_radio_check_item_full(&mut self, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, crate::engine::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_radio_check_item(&mut self, label: GString,) {
            self.add_radio_check_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_radio_check_item_ex(&mut self, label: GString,) -> ExAddRadioCheckItem < '_ > {
            ExAddRadioCheckItem::new(self, label,)
        }
        pub(crate) fn add_icon_radio_check_item_full(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, GString, i32, crate::engine::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) {
            self.add_icon_radio_check_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_item_ex(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString,) -> ExAddIconRadioCheckItem < '_ > {
            ExAddIconRadioCheckItem::new(self, texture, label,)
        }
        pub(crate) fn add_multistate_item_full(&mut self, label: GString, max_states: i32, default_state: i32, id: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, i32, i32, crate::engine::global::Key);
            let args = (label, max_states, default_state, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_multistate_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_multistate_item(&mut self, label: GString, max_states: i32,) {
            self.add_multistate_item_ex(label, max_states,) . done()
        }
        #[inline]
        pub fn add_multistate_item_ex(&mut self, label: GString, max_states: i32,) -> ExAddMultistateItem < '_ > {
            ExAddMultistateItem::new(self, label, max_states,)
        }
        pub(crate) fn add_shortcut_full(&mut self, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool, allow_echo: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Shortcut >, i32, bool, bool);
            let args = (shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_shortcut(&mut self, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_shortcut_ex(&mut self, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddShortcut < '_ > {
            ExAddShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_shortcut_full(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool, allow_echo: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Gd < crate::engine::Shortcut >, i32, bool, bool);
            let args = (texture, shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_shortcut(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_icon_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_shortcut_ex(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddIconShortcut < '_ > {
            ExAddIconShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_check_shortcut_full(&mut self, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Shortcut >, i32, bool);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_check_shortcut(&mut self, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_check_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_check_shortcut_ex(&mut self, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddCheckShortcut < '_ > {
            ExAddCheckShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_check_shortcut_full(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Gd < crate::engine::Shortcut >, i32, bool);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_check_shortcut(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_icon_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_check_shortcut_ex(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddIconCheckShortcut < '_ > {
            ExAddIconCheckShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_radio_check_shortcut_full(&mut self, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Shortcut >, i32, bool);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_radio_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_radio_check_shortcut(&mut self, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_radio_check_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_radio_check_shortcut_ex(&mut self, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddRadioCheckShortcut < '_ > {
            ExAddRadioCheckShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_radio_check_shortcut_full(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Gd < crate::engine::Shortcut >, i32, bool);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_icon_radio_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_icon_radio_check_shortcut(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) {
            self.add_icon_radio_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_shortcut_ex(&mut self, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> ExAddIconRadioCheckShortcut < '_ > {
            ExAddIconRadioCheckShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_submenu_item_full(&mut self, label: GString, submenu: GString, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, i32);
            let args = (label, submenu, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_submenu_item(&mut self, label: GString, submenu: GString,) {
            self.add_submenu_item_ex(label, submenu,) . done()
        }
        #[inline]
        pub fn add_submenu_item_ex(&mut self, label: GString, submenu: GString,) -> ExAddSubmenuItem < '_ > {
            ExAddSubmenuItem::new(self, label, submenu,)
        }
        pub fn set_item_text(&mut self, index: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (index, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_text_direction(&mut self, index: i32, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::control::TextDirection);
            let args = (index, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_language(&mut self, index: i32, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (index, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, index: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (index, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_max_width(&mut self, index: i32, width: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_modulate(&mut self, index: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (index, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_checked(&mut self, index: i32, checked: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_id(&mut self, index: i32, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_accelerator(&mut self, index: i32, accel: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::global::Key);
            let args = (index, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, index: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (index, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, index: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_submenu(&mut self, index: i32, submenu: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (index, submenu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_separator(&mut self, index: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_as_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_checkable(&mut self, index: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_as_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_radio_checkable(&mut self, index: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_as_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, index: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (index, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_item_shortcut_full(&mut self, index: i32, shortcut: Gd < crate::engine::Shortcut >, global: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Shortcut >, bool);
            let args = (index, shortcut, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_item_shortcut(&mut self, index: i32, shortcut: Gd < crate::engine::Shortcut >,) {
            self.set_item_shortcut_ex(index, shortcut,) . done()
        }
        #[inline]
        pub fn set_item_shortcut_ex(&mut self, index: i32, shortcut: Gd < crate::engine::Shortcut >,) -> ExSetItemShortcut < '_ > {
            ExSetItemShortcut::new(self, index, shortcut,)
        }
        pub fn set_item_indent(&mut self, index: i32, indent: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index, indent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_multistate(&mut self, index: i32, state: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_multistate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_shortcut_disabled(&mut self, index: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_shortcut_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_item_checked(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "toggle_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_item_multistate(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "toggle_item_multistate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text_direction(&self, index: i32,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_language(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, index: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_max_width(&self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_modulate(&self, index: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checked(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_id(&self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_accelerator(&self, index: i32,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, index: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_submenu(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_separator(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checkable(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_radio_checkable(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_shortcut_disabled(&self, index: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_shortcut_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_shortcut(&self, index: i32,) -> Option < Gd < crate::engine::Shortcut > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Shortcut > >;
            type CallSig = (Option < Gd < crate::engine::Shortcut > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_indent(&self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focused_item(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focused_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focused_item(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focused_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_item(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scroll_to_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_separator_full(&mut self, label: GString, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_separator(&mut self,) {
            self.add_separator_ex() . done()
        }
        #[inline]
        pub fn add_separator_ex(&mut self,) -> ExAddSeparator < '_ > {
            ExAddSeparator::new(self,)
        }
        pub(crate) fn clear_full(&mut self, free_submenus: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (free_submenus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn clear(&mut self,) {
            self.clear_ex() . done()
        }
        #[inline]
        pub fn clear_ex(&mut self,) -> ExClear < '_ > {
            ExClear::new(self,)
        }
        pub fn set_hide_on_item_selection(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hide_on_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_item_selection(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hide_on_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_on_checkable_item_selection(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hide_on_checkable_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_checkable_item_selection(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hide_on_checkable_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_on_state_item_selection(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hide_on_state_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_state_item_selection(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hide_on_state_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_submenu_popup_delay(&mut self, seconds: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_submenu_popup_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_submenu_popup_delay(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_submenu_popup_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6362usize);
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
    impl crate::obj::GodotClass for PopupMenu {
        type Base = crate::engine::Popup;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PopupMenu\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PopupMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PopupMenu {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Popup > for PopupMenu {
        
    }
    impl crate::obj::Inherits < crate::engine::Window > for PopupMenu {
        
    }
    impl crate::obj::Inherits < crate::engine::Viewport > for PopupMenu {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for PopupMenu {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PopupMenu {
        
    }
    impl crate::obj::ExportableObject for PopupMenu {
        
    }
    impl crate::obj::cap::GodotDefault for PopupMenu {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PopupMenu {
        type Target = crate::engine::Popup;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PopupMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PopupMenu {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PopupMenu > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Popup > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Window > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Viewport > for $Class {
                
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
#[doc = "Default-param extender for [`PopupMenu::activate_item_by_event_ex`][super::PopupMenu::activate_item_by_event_ex]."]
#[must_use]
pub struct ExActivateItemByEvent < 'a > {
    surround_object: &'a mut re_export::PopupMenu, event: Gd < crate::engine::InputEvent >, for_global_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExActivateItemByEvent < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, event: Gd < crate::engine::InputEvent >,) -> Self {
        Self {
            surround_object, event, for_global_only: false,
        }
    }
    #[inline]
    pub fn for_global_only(self, value: bool) -> Self {
        Self {
            for_global_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::PopupMenu::activate_item_by_event_full(self.surround_object, self.event, self.for_global_only,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_item_ex`][super::PopupMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: GString,) -> Self {
        Self {
            surround_object, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_item_full(self.surround_object, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_item_ex`][super::PopupMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, texture, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_item_full(self.surround_object, self.texture, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_item_ex`][super::PopupMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: GString,) -> Self {
        Self {
            surround_object, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_check_item_full(self.surround_object, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_item_ex`][super::PopupMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, texture, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_check_item_full(self.surround_object, self.texture, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_item_ex`][super::PopupMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: GString,) -> Self {
        Self {
            surround_object, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_radio_check_item_full(self.surround_object, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_item_ex`][super::PopupMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, texture, label, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_radio_check_item_full(self.surround_object, self.texture, self.label, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_multistate_item_ex`][super::PopupMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, max_states: i32, default_state: i32, id: i32, accel: crate::engine::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: GString, max_states: i32,) -> Self {
        Self {
            surround_object, label, max_states, default_state: 0i32, id: - 1i32, accel: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn default_state(self, value: i32) -> Self {
        Self {
            default_state: value, .. self
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn accel(self, value: crate::engine::global::Key) -> Self {
        Self {
            accel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_multistate_item_full(self.surround_object, self.label, self.max_states, self.default_state, self.id, self.accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_shortcut_ex`][super::PopupMenu::add_shortcut_ex]."]
#[must_use]
pub struct ExAddShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, shortcut, id: - 1i32, global: false, allow_echo: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, value: bool) -> Self {
        Self {
            allow_echo: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_shortcut_full(self.surround_object, self.shortcut, self.id, self.global, self.allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_shortcut_ex`][super::PopupMenu::add_icon_shortcut_ex]."]
#[must_use]
pub struct ExAddIconShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, texture, shortcut, id: - 1i32, global: false, allow_echo: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, value: bool) -> Self {
        Self {
            allow_echo: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_shortcut_full(self.surround_object, self.texture, self.shortcut, self.id, self.global, self.allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_shortcut_ex`][super::PopupMenu::add_check_shortcut_ex]."]
#[must_use]
pub struct ExAddCheckShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, shortcut, id: - 1i32, global: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_check_shortcut_full(self.surround_object, self.shortcut, self.id, self.global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_shortcut_ex`][super::PopupMenu::add_icon_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconCheckShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, texture, shortcut, id: - 1i32, global: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_check_shortcut_full(self.surround_object, self.texture, self.shortcut, self.id, self.global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_shortcut_ex`][super::PopupMenu::add_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddRadioCheckShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, shortcut, id: - 1i32, global: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_radio_check_shortcut_full(self.surround_object, self.shortcut, self.id, self.global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_shortcut_ex`][super::PopupMenu::add_icon_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: Gd < crate::engine::Texture2D >, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, texture, shortcut, id: - 1i32, global: false,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_icon_radio_check_shortcut_full(self.surround_object, self.texture, self.shortcut, self.id, self.global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_submenu_item_ex`][super::PopupMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, submenu: GString, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: GString, submenu: GString,) -> Self {
        Self {
            surround_object, label, submenu, id: - 1i32,
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_submenu_item_full(self.surround_object, self.label, self.submenu, self.id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::set_item_shortcut_ex`][super::PopupMenu::set_item_shortcut_ex]."]
#[must_use]
pub struct ExSetItemShortcut < 'a > {
    surround_object: &'a mut re_export::PopupMenu, index: i32, shortcut: Gd < crate::engine::Shortcut >, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetItemShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, index: i32, shortcut: Gd < crate::engine::Shortcut >,) -> Self {
        Self {
            surround_object, index, shortcut, global: false,
        }
    }
    #[inline]
    pub fn global(self, value: bool) -> Self {
        Self {
            global: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::set_item_shortcut_full(self.surround_object, self.index, self.shortcut, self.global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_separator_ex`][super::PopupMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    surround_object: &'a mut re_export::PopupMenu, label: GString, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu,) -> Self {
        Self {
            surround_object, label: GString::from(""), id: - 1i32,
        }
    }
    #[inline]
    pub fn label(self, value: GString) -> Self {
        Self {
            label: value, .. self
        }
    }
    #[inline]
    pub fn id(self, value: i32) -> Self {
        Self {
            id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::add_separator_full(self.surround_object, self.label, self.id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::clear_ex`][super::PopupMenu::clear_ex]."]
#[must_use]
pub struct ExClear < 'a > {
    surround_object: &'a mut re_export::PopupMenu, free_submenus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClear < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu,) -> Self {
        Self {
            surround_object, free_submenus: false,
        }
    }
    #[inline]
    pub fn free_submenus(self, value: bool) -> Self {
        Self {
            free_submenus: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PopupMenu::clear_full(self.surround_object, self.free_submenus,)
    }
}