#![doc = "Sidecar module for class [`DisplayServer`][crate::engine::DisplayServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DisplayServer` enums](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DisplayServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`display_server`][crate::engine::display_server]: sidecar module with related enum/flag types\n* [`IDisplayServer`][crate::engine::IDisplayServer]: virtual methods\n\n\nSee also [Godot docs for `DisplayServer`](https://docs.godotengine.org/en/stable/classes/class_displayserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DisplayServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`DisplayServer`][crate::engine::DisplayServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `DisplayServer` methods](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDisplayServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DisplayServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"DisplayServer\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_feature(&self, feature: crate::engine::display_server::Feature,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::display_server::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(54usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(55usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_popup_callbacks(&mut self, menu_root: GString, open_callback: Callable, close_callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Callable, Callable);
            let args = (menu_root, open_callback, close_callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(56usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_popup_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn global_menu_add_submenu_item_full(&mut self, menu_root: GString, label: GString, submenu: GString, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString, GString, i32);
            let args = (menu_root, label, submenu, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(57usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_submenu_item(&mut self, menu_root: GString, label: GString, submenu: GString,) -> i32 {
            self.global_menu_add_submenu_item_ex(menu_root, label, submenu,) . done()
        }
        #[inline]
        pub fn global_menu_add_submenu_item_ex(&mut self, menu_root: GString, label: GString, submenu: GString,) -> ExGlobalMenuAddSubmenuItem < '_ > {
            ExGlobalMenuAddSubmenuItem::new(self, menu_root, label, submenu,)
        }
        pub(crate) fn global_menu_add_item_full(&mut self, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(58usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_item(&mut self, menu_root: GString, label: GString,) -> i32 {
            self.global_menu_add_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_item_ex(&mut self, menu_root: GString, label: GString,) -> ExGlobalMenuAddItem < '_ > {
            ExGlobalMenuAddItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_check_item_full(&mut self, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(59usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_check_item(&mut self, menu_root: GString, label: GString,) -> i32 {
            self.global_menu_add_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_check_item_ex(&mut self, menu_root: GString, label: GString,) -> ExGlobalMenuAddCheckItem < '_ > {
            ExGlobalMenuAddCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_item_full(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, Gd < crate::engine::Texture2D >, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(60usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_icon_item(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> i32 {
            self.global_menu_add_icon_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_item_ex(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> ExGlobalMenuAddIconItem < '_ > {
            ExGlobalMenuAddIconItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_icon_check_item_full(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, Gd < crate::engine::Texture2D >, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(61usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_icon_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_icon_check_item(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> i32 {
            self.global_menu_add_icon_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_check_item_ex(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> ExGlobalMenuAddIconCheckItem < '_ > {
            ExGlobalMenuAddIconCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_radio_check_item_full(&mut self, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(62usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_radio_check_item(&mut self, menu_root: GString, label: GString,) -> i32 {
            self.global_menu_add_radio_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_radio_check_item_ex(&mut self, menu_root: GString, label: GString,) -> ExGlobalMenuAddRadioCheckItem < '_ > {
            ExGlobalMenuAddRadioCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_radio_check_item_full(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, Gd < crate::engine::Texture2D >, GString, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(63usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_icon_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_icon_radio_check_item(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> i32 {
            self.global_menu_add_icon_radio_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_radio_check_item_ex(&mut self, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> ExGlobalMenuAddIconRadioCheckItem < '_ > {
            ExGlobalMenuAddIconRadioCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_multistate_item_full(&mut self, menu_root: GString, label: GString, max_states: i32, default_state: i32, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString, i32, i32, Callable, Callable, Variant, crate::engine::global::Key, i32);
            let args = (menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(64usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_multistate_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_multistate_item(&mut self, menu_root: GString, label: GString, max_states: i32, default_state: i32,) -> i32 {
            self.global_menu_add_multistate_item_ex(menu_root, label, max_states, default_state,) . done()
        }
        #[inline]
        pub fn global_menu_add_multistate_item_ex(&mut self, menu_root: GString, label: GString, max_states: i32, default_state: i32,) -> ExGlobalMenuAddMultistateItem < '_ > {
            ExGlobalMenuAddMultistateItem::new(self, menu_root, label, max_states, default_state,)
        }
        pub(crate) fn global_menu_add_separator_full(&mut self, menu_root: GString, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, i32);
            let args = (menu_root, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(65usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_add_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn global_menu_add_separator(&mut self, menu_root: GString,) -> i32 {
            self.global_menu_add_separator_ex(menu_root,) . done()
        }
        #[inline]
        pub fn global_menu_add_separator_ex(&mut self, menu_root: GString,) -> ExGlobalMenuAddSeparator < '_ > {
            ExGlobalMenuAddSeparator::new(self, menu_root,)
        }
        pub fn global_menu_get_item_index_from_text(&self, menu_root: GString, text: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString);
            let args = (menu_root, text,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(66usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_index_from_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_index_from_tag(&self, menu_root: GString, tag: Variant,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, Variant);
            let args = (menu_root, tag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(67usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_index_from_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_checked(&self, menu_root: GString, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(68usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_is_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_checkable(&self, menu_root: GString, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(69usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_is_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_radio_checkable(&self, menu_root: GString, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(70usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_is_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_callback(&self, menu_root: GString, idx: i32,) -> Callable {
            type RetMarshal = PtrcallReturnT < Callable >;
            type CallSig = (Callable, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(71usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_key_callback(&self, menu_root: GString, idx: i32,) -> Callable {
            type RetMarshal = PtrcallReturnT < Callable >;
            type CallSig = (Callable, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(72usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_tag(&self, menu_root: GString, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(73usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_text(&self, menu_root: GString, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(74usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_submenu(&self, menu_root: GString, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(75usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_accelerator(&self, menu_root: GString, idx: i32,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(76usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_disabled(&self, menu_root: GString, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(77usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_hidden(&self, menu_root: GString, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(78usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_is_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_tooltip(&self, menu_root: GString, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(79usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_state(&self, menu_root: GString, idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(80usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_max_states(&self, menu_root: GString, idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(81usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_icon(&self, menu_root: GString, idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(82usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_indentation_level(&self, menu_root: GString, idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(83usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_checked(&mut self, menu_root: GString, idx: i32, checked: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, bool);
            let args = (menu_root, idx, checked,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(84usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_checkable(&mut self, menu_root: GString, idx: i32, checkable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, bool);
            let args = (menu_root, idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(85usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_radio_checkable(&mut self, menu_root: GString, idx: i32, checkable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, bool);
            let args = (menu_root, idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(86usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_callback(&mut self, menu_root: GString, idx: i32, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, Callable);
            let args = (menu_root, idx, callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(87usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_hover_callbacks(&mut self, menu_root: GString, idx: i32, callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, Callable);
            let args = (menu_root, idx, callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(88usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_hover_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_key_callback(&mut self, menu_root: GString, idx: i32, key_callback: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, Callable);
            let args = (menu_root, idx, key_callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(89usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_tag(&mut self, menu_root: GString, idx: i32, tag: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, Variant);
            let args = (menu_root, idx, tag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(90usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_text(&mut self, menu_root: GString, idx: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, GString);
            let args = (menu_root, idx, text,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(91usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_submenu(&mut self, menu_root: GString, idx: i32, submenu: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, GString);
            let args = (menu_root, idx, submenu,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(92usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_accelerator(&mut self, menu_root: GString, idx: i32, keycode: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, crate::engine::global::Key);
            let args = (menu_root, idx, keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(93usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_disabled(&mut self, menu_root: GString, idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, bool);
            let args = (menu_root, idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(94usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_hidden(&mut self, menu_root: GString, idx: i32, hidden: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, bool);
            let args = (menu_root, idx, hidden,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(95usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_tooltip(&mut self, menu_root: GString, idx: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, GString);
            let args = (menu_root, idx, tooltip,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(96usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_state(&mut self, menu_root: GString, idx: i32, state: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, i32);
            let args = (menu_root, idx, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(97usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_max_states(&mut self, menu_root: GString, idx: i32, max_states: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, i32);
            let args = (menu_root, idx, max_states,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(98usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_icon(&mut self, menu_root: GString, idx: i32, icon: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, Gd < crate::engine::Texture2D >);
            let args = (menu_root, idx, icon,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(99usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_indentation_level(&mut self, menu_root: GString, idx: i32, level: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32, i32);
            let args = (menu_root, idx, level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_set_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_count(&self, menu_root: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (menu_root,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_remove_item(&mut self, menu_root: GString, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (menu_root, idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_clear(&mut self, menu_root: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (menu_root,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_menu_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_is_speaking(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_is_speaking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_is_paused(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_is_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_get_voices(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_get_voices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_get_voices_for_language(&self, language: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_get_voices_for_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn tts_speak_full(&mut self, text: GString, voice: GString, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, i32, f32, f32, i32, bool);
            let args = (text, voice, volume, pitch, rate, utterance_id, interrupt,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_speak", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn tts_speak(&mut self, text: GString, voice: GString,) {
            self.tts_speak_ex(text, voice,) . done()
        }
        #[inline]
        pub fn tts_speak_ex(&mut self, text: GString, voice: GString,) -> ExTtsSpeak < '_ > {
            ExTtsSpeak::new(self, text, voice,)
        }
        pub fn tts_pause(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_resume(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_resume", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_set_utterance_callback(&mut self, event: crate::engine::display_server::TTSUtteranceEvent, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::TTSUtteranceEvent, Callable);
            let args = (event, callable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tts_set_utterance_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dark_mode_supported(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_dark_mode_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dark_mode(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_dark_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accent_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_accent_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_set_mode(&mut self, mouse_mode: crate::engine::display_server::MouseMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::MouseMode);
            let args = (mouse_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mouse_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_mode(&self,) -> crate::engine::display_server::MouseMode {
            type RetMarshal = PtrcallReturnT < crate::engine::display_server::MouseMode >;
            type CallSig = (crate::engine::display_server::MouseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mouse_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_position(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mouse_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_button_state(&self,) -> crate::engine::global::MouseButtonMask {
            type RetMarshal = PtrcallReturnT < crate::engine::global::MouseButtonMask >;
            type CallSig = (crate::engine::global::MouseButtonMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mouse_get_button_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_set(&mut self, clipboard: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (clipboard,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get_image(&self,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_has(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_has", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_has_image(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_has_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_set_primary(&mut self, clipboard_primary: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (clipboard_primary,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_set_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get_primary(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clipboard_get_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_cutouts(&self,) -> Array < Rect2 > {
            type RetMarshal = PtrcallReturnT < Array < Rect2 > >;
            type CallSig = (Array < Rect2 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_display_cutouts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_safe_area(&self,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_display_safe_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primary_screen(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_primary_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keyboard_focus_screen(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keyboard_focus_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_from_rect(&self, rect: Rect2,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rect2);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_from_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_position_full(&self, screen: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_position(&self,) -> Vector2i {
            self.screen_get_position_ex() . done()
        }
        #[inline]
        pub fn screen_get_position_ex(&self,) -> ExScreenGetPosition < '_ > {
            ExScreenGetPosition::new(self,)
        }
        pub(crate) fn screen_get_size_full(&self, screen: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_size(&self,) -> Vector2i {
            self.screen_get_size_ex() . done()
        }
        #[inline]
        pub fn screen_get_size_ex(&self,) -> ExScreenGetSize < '_ > {
            ExScreenGetSize::new(self,)
        }
        pub(crate) fn screen_get_usable_rect_full(&self, screen: i32,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_usable_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_usable_rect(&self,) -> Rect2i {
            self.screen_get_usable_rect_ex() . done()
        }
        #[inline]
        pub fn screen_get_usable_rect_ex(&self,) -> ExScreenGetUsableRect < '_ > {
            ExScreenGetUsableRect::new(self,)
        }
        pub(crate) fn screen_get_dpi_full(&self, screen: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_dpi", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_dpi(&self,) -> i32 {
            self.screen_get_dpi_ex() . done()
        }
        #[inline]
        pub fn screen_get_dpi_ex(&self,) -> ExScreenGetDpi < '_ > {
            ExScreenGetDpi::new(self,)
        }
        pub(crate) fn screen_get_scale_full(&self, screen: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_scale(&self,) -> f32 {
            self.screen_get_scale_ex() . done()
        }
        #[inline]
        pub fn screen_get_scale_ex(&self,) -> ExScreenGetScale < '_ > {
            ExScreenGetScale::new(self,)
        }
        pub fn is_touchscreen_available(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_touchscreen_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn screen_get_max_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_max_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_refresh_rate_full(&self, screen: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_refresh_rate(&self,) -> f32 {
            self.screen_get_refresh_rate_ex() . done()
        }
        #[inline]
        pub fn screen_get_refresh_rate_ex(&self,) -> ExScreenGetRefreshRate < '_ > {
            ExScreenGetRefreshRate::new(self,)
        }
        pub fn screen_get_pixel(&self, position: Vector2i,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_image_full(&self, screen: i32,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_image(&self,) -> Option < Gd < crate::engine::Image > > {
            self.screen_get_image_ex() . done()
        }
        #[inline]
        pub fn screen_get_image_ex(&self,) -> ExScreenGetImage < '_ > {
            ExScreenGetImage::new(self,)
        }
        pub(crate) fn screen_set_orientation_full(&mut self, orientation: crate::engine::display_server::ScreenOrientation, screen: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::ScreenOrientation, i32);
            let args = (orientation, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_set_orientation(&mut self, orientation: crate::engine::display_server::ScreenOrientation,) {
            self.screen_set_orientation_ex(orientation,) . done()
        }
        #[inline]
        pub fn screen_set_orientation_ex(&mut self, orientation: crate::engine::display_server::ScreenOrientation,) -> ExScreenSetOrientation < '_ > {
            ExScreenSetOrientation::new(self, orientation,)
        }
        pub(crate) fn screen_get_orientation_full(&self, screen: i32,) -> crate::engine::display_server::ScreenOrientation {
            type RetMarshal = PtrcallReturnT < crate::engine::display_server::ScreenOrientation >;
            type CallSig = (crate::engine::display_server::ScreenOrientation, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_orientation(&self,) -> crate::engine::display_server::ScreenOrientation {
            self.screen_get_orientation_ex() . done()
        }
        #[inline]
        pub fn screen_get_orientation_ex(&self,) -> ExScreenGetOrientation < '_ > {
            ExScreenGetOrientation::new(self,)
        }
        pub fn screen_set_keep_on(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_set_keep_on", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn screen_is_kept_on(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_is_kept_on", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window_list(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_window_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window_at_screen_position(&self, position: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_window_at_screen_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_get_native_handle_full(&self, handle_type: crate::engine::display_server::HandleType, window_id: i32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, crate::engine::display_server::HandleType, i32);
            let args = (handle_type, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_native_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_native_handle(&self, handle_type: crate::engine::display_server::HandleType,) -> i64 {
            self.window_get_native_handle_ex(handle_type,) . done()
        }
        #[inline]
        pub fn window_get_native_handle_ex(&self, handle_type: crate::engine::display_server::HandleType,) -> ExWindowGetNativeHandle < '_ > {
            ExWindowGetNativeHandle::new(self, handle_type,)
        }
        pub fn window_get_active_popup(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_active_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_set_popup_safe_rect(&mut self, window: i32, rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Rect2i);
            let args = (window, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_popup_safe_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_get_popup_safe_rect(&self, window: i32,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i, i32);
            let args = (window,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_popup_safe_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_set_title_full(&mut self, title: GString, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_title(&mut self, title: GString,) {
            self.window_set_title_ex(title,) . done()
        }
        #[inline]
        pub fn window_set_title_ex(&mut self, title: GString,) -> ExWindowSetTitle < '_ > {
            ExWindowSetTitle::new(self, title,)
        }
        pub(crate) fn window_get_title_size_full(&self, title: GString, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, GString, i32);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_title_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_title_size(&self, title: GString,) -> Vector2i {
            self.window_get_title_size_ex(title,) . done()
        }
        #[inline]
        pub fn window_get_title_size_ex(&self, title: GString,) -> ExWindowGetTitleSize < '_ > {
            ExWindowGetTitleSize::new(self, title,)
        }
        pub(crate) fn window_set_mouse_passthrough_full(&mut self, region: PackedVector2Array, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, i32);
            let args = (region, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_mouse_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_mouse_passthrough(&mut self, region: PackedVector2Array,) {
            self.window_set_mouse_passthrough_ex(region,) . done()
        }
        #[inline]
        pub fn window_set_mouse_passthrough_ex(&mut self, region: PackedVector2Array,) -> ExWindowSetMousePassthrough < '_ > {
            ExWindowSetMousePassthrough::new(self, region,)
        }
        pub(crate) fn window_get_current_screen_full(&self, window_id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_current_screen(&self,) -> i32 {
            self.window_get_current_screen_ex() . done()
        }
        #[inline]
        pub fn window_get_current_screen_ex(&self,) -> ExWindowGetCurrentScreen < '_ > {
            ExWindowGetCurrentScreen::new(self,)
        }
        pub(crate) fn window_set_current_screen_full(&mut self, screen: i32, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (screen, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_current_screen(&mut self, screen: i32,) {
            self.window_set_current_screen_ex(screen,) . done()
        }
        #[inline]
        pub fn window_set_current_screen_ex(&mut self, screen: i32,) -> ExWindowSetCurrentScreen < '_ > {
            ExWindowSetCurrentScreen::new(self, screen,)
        }
        pub(crate) fn window_get_position_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_position(&self,) -> Vector2i {
            self.window_get_position_ex() . done()
        }
        #[inline]
        pub fn window_get_position_ex(&self,) -> ExWindowGetPosition < '_ > {
            ExWindowGetPosition::new(self,)
        }
        pub(crate) fn window_get_position_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_position_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_position_with_decorations(&self,) -> Vector2i {
            self.window_get_position_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_position_with_decorations_ex(&self,) -> ExWindowGetPositionWithDecorations < '_ > {
            ExWindowGetPositionWithDecorations::new(self,)
        }
        pub(crate) fn window_set_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_position(&mut self, position: Vector2i,) {
            self.window_set_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_position_ex(&mut self, position: Vector2i,) -> ExWindowSetPosition < '_ > {
            ExWindowSetPosition::new(self, position,)
        }
        pub(crate) fn window_get_size_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_size(&self,) -> Vector2i {
            self.window_get_size_ex() . done()
        }
        #[inline]
        pub fn window_get_size_ex(&self,) -> ExWindowGetSize < '_ > {
            ExWindowGetSize::new(self,)
        }
        pub(crate) fn window_set_size_full(&mut self, size: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_size(&mut self, size: Vector2i,) {
            self.window_set_size_ex(size,) . done()
        }
        #[inline]
        pub fn window_set_size_ex(&mut self, size: Vector2i,) -> ExWindowSetSize < '_ > {
            ExWindowSetSize::new(self, size,)
        }
        pub(crate) fn window_set_rect_changed_callback_full(&mut self, callback: Callable, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_rect_changed_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_rect_changed_callback(&mut self, callback: Callable,) {
            self.window_set_rect_changed_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_rect_changed_callback_ex(&mut self, callback: Callable,) -> ExWindowSetRectChangedCallback < '_ > {
            ExWindowSetRectChangedCallback::new(self, callback,)
        }
        pub(crate) fn window_set_window_event_callback_full(&mut self, callback: Callable, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_window_event_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_window_event_callback(&mut self, callback: Callable,) {
            self.window_set_window_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_window_event_callback_ex(&mut self, callback: Callable,) -> ExWindowSetWindowEventCallback < '_ > {
            ExWindowSetWindowEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_event_callback_full(&mut self, callback: Callable, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_input_event_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_input_event_callback(&mut self, callback: Callable,) {
            self.window_set_input_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_event_callback_ex(&mut self, callback: Callable,) -> ExWindowSetInputEventCallback < '_ > {
            ExWindowSetInputEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_text_callback_full(&mut self, callback: Callable, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_input_text_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_input_text_callback(&mut self, callback: Callable,) {
            self.window_set_input_text_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_text_callback_ex(&mut self, callback: Callable,) -> ExWindowSetInputTextCallback < '_ > {
            ExWindowSetInputTextCallback::new(self, callback,)
        }
        pub(crate) fn window_set_drop_files_callback_full(&mut self, callback: Callable, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_drop_files_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_drop_files_callback(&mut self, callback: Callable,) {
            self.window_set_drop_files_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_drop_files_callback_ex(&mut self, callback: Callable,) -> ExWindowSetDropFilesCallback < '_ > {
            ExWindowSetDropFilesCallback::new(self, callback,)
        }
        pub(crate) fn window_get_attached_instance_id_full(&self, window_id: i32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_attached_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_attached_instance_id(&self,) -> u64 {
            self.window_get_attached_instance_id_ex() . done()
        }
        #[inline]
        pub fn window_get_attached_instance_id_ex(&self,) -> ExWindowGetAttachedInstanceId < '_ > {
            ExWindowGetAttachedInstanceId::new(self,)
        }
        pub(crate) fn window_get_max_size_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_max_size(&self,) -> Vector2i {
            self.window_get_max_size_ex() . done()
        }
        #[inline]
        pub fn window_get_max_size_ex(&self,) -> ExWindowGetMaxSize < '_ > {
            ExWindowGetMaxSize::new(self,)
        }
        pub(crate) fn window_set_max_size_full(&mut self, max_size: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (max_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_max_size(&mut self, max_size: Vector2i,) {
            self.window_set_max_size_ex(max_size,) . done()
        }
        #[inline]
        pub fn window_set_max_size_ex(&mut self, max_size: Vector2i,) -> ExWindowSetMaxSize < '_ > {
            ExWindowSetMaxSize::new(self, max_size,)
        }
        pub(crate) fn window_get_min_size_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_min_size(&self,) -> Vector2i {
            self.window_get_min_size_ex() . done()
        }
        #[inline]
        pub fn window_get_min_size_ex(&self,) -> ExWindowGetMinSize < '_ > {
            ExWindowGetMinSize::new(self,)
        }
        pub(crate) fn window_set_min_size_full(&mut self, min_size: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (min_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_min_size(&mut self, min_size: Vector2i,) {
            self.window_set_min_size_ex(min_size,) . done()
        }
        #[inline]
        pub fn window_set_min_size_ex(&mut self, min_size: Vector2i,) -> ExWindowSetMinSize < '_ > {
            ExWindowSetMinSize::new(self, min_size,)
        }
        pub(crate) fn window_get_size_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_size_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_size_with_decorations(&self,) -> Vector2i {
            self.window_get_size_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_size_with_decorations_ex(&self,) -> ExWindowGetSizeWithDecorations < '_ > {
            ExWindowGetSizeWithDecorations::new(self,)
        }
        pub(crate) fn window_get_mode_full(&self, window_id: i32,) -> crate::engine::display_server::WindowMode {
            type RetMarshal = PtrcallReturnT < crate::engine::display_server::WindowMode >;
            type CallSig = (crate::engine::display_server::WindowMode, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_mode(&self,) -> crate::engine::display_server::WindowMode {
            self.window_get_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_mode_ex(&self,) -> ExWindowGetMode < '_ > {
            ExWindowGetMode::new(self,)
        }
        pub(crate) fn window_set_mode_full(&mut self, mode: crate::engine::display_server::WindowMode, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::WindowMode, i32);
            let args = (mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_mode(&mut self, mode: crate::engine::display_server::WindowMode,) {
            self.window_set_mode_ex(mode,) . done()
        }
        #[inline]
        pub fn window_set_mode_ex(&mut self, mode: crate::engine::display_server::WindowMode,) -> ExWindowSetMode < '_ > {
            ExWindowSetMode::new(self, mode,)
        }
        pub(crate) fn window_set_flag_full(&mut self, flag: crate::engine::display_server::WindowFlags, enabled: bool, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::WindowFlags, bool, i32);
            let args = (flag, enabled, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_flag(&mut self, flag: crate::engine::display_server::WindowFlags, enabled: bool,) {
            self.window_set_flag_ex(flag, enabled,) . done()
        }
        #[inline]
        pub fn window_set_flag_ex(&mut self, flag: crate::engine::display_server::WindowFlags, enabled: bool,) -> ExWindowSetFlag < '_ > {
            ExWindowSetFlag::new(self, flag, enabled,)
        }
        pub(crate) fn window_get_flag_full(&self, flag: crate::engine::display_server::WindowFlags, window_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::display_server::WindowFlags, i32);
            let args = (flag, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_flag(&self, flag: crate::engine::display_server::WindowFlags,) -> bool {
            self.window_get_flag_ex(flag,) . done()
        }
        #[inline]
        pub fn window_get_flag_ex(&self, flag: crate::engine::display_server::WindowFlags,) -> ExWindowGetFlag < '_ > {
            ExWindowGetFlag::new(self, flag,)
        }
        pub(crate) fn window_set_window_buttons_offset_full(&mut self, offset: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (offset, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_window_buttons_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_window_buttons_offset(&mut self, offset: Vector2i,) {
            self.window_set_window_buttons_offset_ex(offset,) . done()
        }
        #[inline]
        pub fn window_set_window_buttons_offset_ex(&mut self, offset: Vector2i,) -> ExWindowSetWindowButtonsOffset < '_ > {
            ExWindowSetWindowButtonsOffset::new(self, offset,)
        }
        pub(crate) fn window_get_safe_title_margins_full(&self, window_id: i32,) -> Vector3i {
            type RetMarshal = PtrcallReturnT < Vector3i >;
            type CallSig = (Vector3i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_safe_title_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_safe_title_margins(&self,) -> Vector3i {
            self.window_get_safe_title_margins_ex() . done()
        }
        #[inline]
        pub fn window_get_safe_title_margins_ex(&self,) -> ExWindowGetSafeTitleMargins < '_ > {
            ExWindowGetSafeTitleMargins::new(self,)
        }
        pub(crate) fn window_request_attention_full(&mut self, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_request_attention", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_request_attention(&mut self,) {
            self.window_request_attention_ex() . done()
        }
        #[inline]
        pub fn window_request_attention_ex(&mut self,) -> ExWindowRequestAttention < '_ > {
            ExWindowRequestAttention::new(self,)
        }
        pub(crate) fn window_move_to_foreground_full(&mut self, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_move_to_foreground", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_move_to_foreground(&mut self,) {
            self.window_move_to_foreground_ex() . done()
        }
        #[inline]
        pub fn window_move_to_foreground_ex(&mut self,) -> ExWindowMoveToForeground < '_ > {
            ExWindowMoveToForeground::new(self,)
        }
        pub(crate) fn window_is_focused_full(&self, window_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_is_focused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_is_focused(&self,) -> bool {
            self.window_is_focused_ex() . done()
        }
        #[inline]
        pub fn window_is_focused_ex(&self,) -> ExWindowIsFocused < '_ > {
            ExWindowIsFocused::new(self,)
        }
        pub(crate) fn window_can_draw_full(&self, window_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_can_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_can_draw(&self,) -> bool {
            self.window_can_draw_ex() . done()
        }
        #[inline]
        pub fn window_can_draw_ex(&self,) -> ExWindowCanDraw < '_ > {
            ExWindowCanDraw::new(self,)
        }
        pub fn window_set_transient(&mut self, window_id: i32, parent_window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (window_id, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_transient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_set_exclusive(&mut self, window_id: i32, exclusive: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (window_id, exclusive,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_exclusive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_set_ime_active_full(&mut self, active: bool, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, i32);
            let args = (active, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_ime_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_ime_active(&mut self, active: bool,) {
            self.window_set_ime_active_ex(active,) . done()
        }
        #[inline]
        pub fn window_set_ime_active_ex(&mut self, active: bool,) -> ExWindowSetImeActive < '_ > {
            ExWindowSetImeActive::new(self, active,)
        }
        pub(crate) fn window_set_ime_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_ime_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_ime_position(&mut self, position: Vector2i,) {
            self.window_set_ime_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_ime_position_ex(&mut self, position: Vector2i,) -> ExWindowSetImePosition < '_ > {
            ExWindowSetImePosition::new(self, position,)
        }
        pub(crate) fn window_set_vsync_mode_full(&mut self, vsync_mode: crate::engine::display_server::VSyncMode, window_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::VSyncMode, i32);
            let args = (vsync_mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_set_vsync_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_set_vsync_mode(&mut self, vsync_mode: crate::engine::display_server::VSyncMode,) {
            self.window_set_vsync_mode_ex(vsync_mode,) . done()
        }
        #[inline]
        pub fn window_set_vsync_mode_ex(&mut self, vsync_mode: crate::engine::display_server::VSyncMode,) -> ExWindowSetVsyncMode < '_ > {
            ExWindowSetVsyncMode::new(self, vsync_mode,)
        }
        pub(crate) fn window_get_vsync_mode_full(&self, window_id: i32,) -> crate::engine::display_server::VSyncMode {
            type RetMarshal = PtrcallReturnT < crate::engine::display_server::VSyncMode >;
            type CallSig = (crate::engine::display_server::VSyncMode, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_get_vsync_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_get_vsync_mode(&self,) -> crate::engine::display_server::VSyncMode {
            self.window_get_vsync_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_vsync_mode_ex(&self,) -> ExWindowGetVsyncMode < '_ > {
            ExWindowGetVsyncMode::new(self,)
        }
        pub(crate) fn window_is_maximize_allowed_full(&self, window_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_is_maximize_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn window_is_maximize_allowed(&self,) -> bool {
            self.window_is_maximize_allowed_ex() . done()
        }
        #[inline]
        pub fn window_is_maximize_allowed_ex(&self,) -> ExWindowIsMaximizeAllowed < '_ > {
            ExWindowIsMaximizeAllowed::new(self,)
        }
        pub fn window_maximize_on_title_dbl_click(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_maximize_on_title_dbl_click", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_minimize_on_title_dbl_click(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "window_minimize_on_title_dbl_click", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ime_get_selection(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ime_get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ime_get_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ime_get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn virtual_keyboard_show_full(&mut self, existing_text: GString, position: Rect2, type_: crate::engine::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Rect2, crate::engine::display_server::VirtualKeyboardType, i32, i32, i32);
            let args = (existing_text, position, type_, max_length, cursor_start, cursor_end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "virtual_keyboard_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn virtual_keyboard_show(&mut self, existing_text: GString,) {
            self.virtual_keyboard_show_ex(existing_text,) . done()
        }
        #[inline]
        pub fn virtual_keyboard_show_ex(&mut self, existing_text: GString,) -> ExVirtualKeyboardShow < '_ > {
            ExVirtualKeyboardShow::new(self, existing_text,)
        }
        pub fn virtual_keyboard_hide(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "virtual_keyboard_hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn virtual_keyboard_get_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "virtual_keyboard_get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cursor_set_shape(&mut self, shape: crate::engine::display_server::CursorShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::display_server::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cursor_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cursor_get_shape(&self,) -> crate::engine::display_server::CursorShape {
            type RetMarshal = PtrcallReturnT < crate::engine::display_server::CursorShape >;
            type CallSig = (crate::engine::display_server::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cursor_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn cursor_set_custom_image_full(&mut self, cursor: Gd < crate::engine::Resource >, shape: crate::engine::display_server::CursorShape, hotspot: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Resource >, crate::engine::display_server::CursorShape, Vector2);
            let args = (cursor, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cursor_set_custom_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn cursor_set_custom_image(&mut self, cursor: Gd < crate::engine::Resource >,) {
            self.cursor_set_custom_image_ex(cursor,) . done()
        }
        #[inline]
        pub fn cursor_set_custom_image_ex(&mut self, cursor: Gd < crate::engine::Resource >,) -> ExCursorSetCustomImage < '_ > {
            ExCursorSetCustomImage::new(self, cursor,)
        }
        pub fn get_swap_cancel_ok(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_swap_cancel_ok", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn enable_for_stealing_focus(&mut self, process_id: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (process_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "enable_for_stealing_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dialog_show(&mut self, title: GString, description: GString, buttons: PackedStringArray, callback: Callable,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, PackedStringArray, Callable);
            let args = (title, description, buttons, callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dialog_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dialog_input_text(&mut self, title: GString, description: GString, existing_text: GString, callback: Callable,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, GString, Callable);
            let args = (title, description, existing_text, callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dialog_input_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn file_dialog_show(&mut self, title: GString, current_directory: GString, filename: GString, show_hidden: bool, mode: crate::engine::display_server::FileDialogMode, filters: PackedStringArray, callback: Callable,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, GString, bool, crate::engine::display_server::FileDialogMode, PackedStringArray, Callable);
            let args = (title, current_directory, filename, show_hidden, mode, filters, callback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "file_dialog_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_layout_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_current_layout(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_current_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_set_current_layout(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_set_current_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_language(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_layout_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_name(&self, index: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_layout_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_keycode_from_physical(&self, keycode: crate::engine::global::Key,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key, crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_keycode_from_physical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_label_from_physical(&self, keycode: crate::engine::global::Key,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key, crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "keyboard_get_label_from_physical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn process_events(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "process_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_process_and_drop_events(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_process_and_drop_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_native_icon(&mut self, filename: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (filename,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_native_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon(&mut self, image: Gd < crate::engine::Image >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >);
            let args = (image,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_driver_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tablet_get_driver_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_driver_name(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tablet_get_driver_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_current_driver(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tablet_get_current_driver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_set_current_driver(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tablet_set_current_driver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const SCREEN_WITH_MOUSE_FOCUS: i32 = - 4i32;
        pub const SCREEN_WITH_KEYBOARD_FOCUS: i32 = - 3i32;
        pub const SCREEN_PRIMARY: i32 = - 2i32;
        pub const SCREEN_OF_MAIN_WINDOW: i32 = - 1i32;
        pub const MAIN_WINDOW_ID: i32 = 0i32;
        pub const INVALID_WINDOW_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for DisplayServer {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"DisplayServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for DisplayServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for DisplayServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for DisplayServer {
        
    }
    impl std::ops::Deref for DisplayServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DisplayServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_DisplayServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::DisplayServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_submenu_item_ex`][super::DisplayServer::global_menu_add_submenu_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSubmenuItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, submenu: GString, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, submenu: GString,) -> Self {
        Self {
            surround_object, menu_root, label, submenu, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_submenu_item_full(self.surround_object, self.menu_root, self.label, self.submenu, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_item_ex`][super::DisplayServer::global_menu_add_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString,) -> Self {
        Self {
            surround_object, menu_root, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_item_full(self.surround_object, self.menu_root, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_check_item_ex`][super::DisplayServer::global_menu_add_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddCheckItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString,) -> Self {
        Self {
            surround_object, menu_root, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_check_item_full(self.surround_object, self.menu_root, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_item_ex`][super::DisplayServer::global_menu_add_icon_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, menu_root, icon, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_icon_item_full(self.surround_object, self.menu_root, self.icon, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_check_item_ex`][super::DisplayServer::global_menu_add_icon_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconCheckItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, menu_root, icon, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_icon_check_item_full(self.surround_object, self.menu_root, self.icon, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_radio_check_item_ex`][super::DisplayServer::global_menu_add_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddRadioCheckItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString,) -> Self {
        Self {
            surround_object, menu_root, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_radio_check_item_full(self.surround_object, self.menu_root, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_radio_check_item_ex`][super::DisplayServer::global_menu_add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconRadioCheckItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, icon: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, menu_root, icon, label, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_icon_radio_check_item_full(self.surround_object, self.menu_root, self.icon, self.label, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_multistate_item_ex`][super::DisplayServer::global_menu_add_multistate_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddMultistateItem < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, max_states: i32, default_state: i32, callback: Callable, key_callback: Callable, tag: Variant, accelerator: crate::engine::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString, label: GString, max_states: i32, default_state: i32,) -> Self {
        Self {
            surround_object, menu_root, label, max_states, default_state, callback: Callable::invalid(), key_callback: Callable::invalid(), tag: Variant::nil(), accelerator: crate::obj::EngineEnum::from_ord(0), index: - 1i32,
        }
    }
    #[inline]
    pub fn callback(self, value: Callable) -> Self {
        Self {
            callback: value, .. self
        }
    }
    #[inline]
    pub fn key_callback(self, value: Callable) -> Self {
        Self {
            key_callback: value, .. self
        }
    }
    #[inline]
    pub fn tag(self, value: Variant) -> Self {
        Self {
            tag: value, .. self
        }
    }
    #[inline]
    pub fn accelerator(self, value: crate::engine::global::Key) -> Self {
        Self {
            accelerator: value, .. self
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_multistate_item_full(self.surround_object, self.menu_root, self.label, self.max_states, self.default_state, self.callback, self.key_callback, self.tag, self.accelerator, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_separator_ex`][super::DisplayServer::global_menu_add_separator_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSeparator < 'a > {
    surround_object: &'a mut re_export::DisplayServer, menu_root: GString, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: GString,) -> Self {
        Self {
            surround_object, menu_root, index: - 1i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::global_menu_add_separator_full(self.surround_object, self.menu_root, self.index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::tts_speak_ex`][super::DisplayServer::tts_speak_ex]."]
#[must_use]
pub struct ExTtsSpeak < 'a > {
    surround_object: &'a mut re_export::DisplayServer, text: GString, voice: GString, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTtsSpeak < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, text: GString, voice: GString,) -> Self {
        Self {
            surround_object, text, voice, volume: 50i32, pitch: 1f32, rate: 1f32, utterance_id: 0i32, interrupt: false,
        }
    }
    #[inline]
    pub fn volume(self, value: i32) -> Self {
        Self {
            volume: value, .. self
        }
    }
    #[inline]
    pub fn pitch(self, value: f32) -> Self {
        Self {
            pitch: value, .. self
        }
    }
    #[inline]
    pub fn rate(self, value: f32) -> Self {
        Self {
            rate: value, .. self
        }
    }
    #[inline]
    pub fn utterance_id(self, value: i32) -> Self {
        Self {
            utterance_id: value, .. self
        }
    }
    #[inline]
    pub fn interrupt(self, value: bool) -> Self {
        Self {
            interrupt: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::tts_speak_full(self.surround_object, self.text, self.voice, self.volume, self.pitch, self.rate, self.utterance_id, self.interrupt,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_position_ex`][super::DisplayServer::screen_get_position_ex]."]
#[must_use]
pub struct ExScreenGetPosition < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::screen_get_position_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_size_ex`][super::DisplayServer::screen_get_size_ex]."]
#[must_use]
pub struct ExScreenGetSize < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::screen_get_size_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_usable_rect_ex`][super::DisplayServer::screen_get_usable_rect_ex]."]
#[must_use]
pub struct ExScreenGetUsableRect < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetUsableRect < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        re_export::DisplayServer::screen_get_usable_rect_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_dpi_ex`][super::DisplayServer::screen_get_dpi_ex]."]
#[must_use]
pub struct ExScreenGetDpi < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetDpi < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::screen_get_dpi_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_scale_ex`][super::DisplayServer::screen_get_scale_ex]."]
#[must_use]
pub struct ExScreenGetScale < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetScale < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::DisplayServer::screen_get_scale_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_refresh_rate_ex`][super::DisplayServer::screen_get_refresh_rate_ex]."]
#[must_use]
pub struct ExScreenGetRefreshRate < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetRefreshRate < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::DisplayServer::screen_get_refresh_rate_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_image_ex`][super::DisplayServer::screen_get_image_ex]."]
#[must_use]
pub struct ExScreenGetImage < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetImage < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Image > > {
        re_export::DisplayServer::screen_get_image_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_set_orientation_ex`][super::DisplayServer::screen_set_orientation_ex]."]
#[must_use]
pub struct ExScreenSetOrientation < 'a > {
    surround_object: &'a mut re_export::DisplayServer, orientation: crate::engine::display_server::ScreenOrientation, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, orientation: crate::engine::display_server::ScreenOrientation,) -> Self {
        Self {
            surround_object, orientation, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::screen_set_orientation_full(self.surround_object, self.orientation, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_orientation_ex`][super::DisplayServer::screen_get_orientation_ex]."]
#[must_use]
pub struct ExScreenGetOrientation < 'a > {
    surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetOrientation < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, screen: - 1i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::display_server::ScreenOrientation {
        re_export::DisplayServer::screen_get_orientation_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_native_handle_ex`][super::DisplayServer::window_get_native_handle_ex]."]
#[must_use]
pub struct ExWindowGetNativeHandle < 'a > {
    surround_object: &'a re_export::DisplayServer, handle_type: crate::engine::display_server::HandleType, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetNativeHandle < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, handle_type: crate::engine::display_server::HandleType,) -> Self {
        Self {
            surround_object, handle_type, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::DisplayServer::window_get_native_handle_full(self.surround_object, self.handle_type, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_title_ex`][super::DisplayServer::window_set_title_ex]."]
#[must_use]
pub struct ExWindowSetTitle < 'a > {
    surround_object: &'a mut re_export::DisplayServer, title: GString, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetTitle < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, title: GString,) -> Self {
        Self {
            surround_object, title, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_title_full(self.surround_object, self.title, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_title_size_ex`][super::DisplayServer::window_get_title_size_ex]."]
#[must_use]
pub struct ExWindowGetTitleSize < 'a > {
    surround_object: &'a re_export::DisplayServer, title: GString, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetTitleSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, title: GString,) -> Self {
        Self {
            surround_object, title, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_title_size_full(self.surround_object, self.title, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mouse_passthrough_ex`][super::DisplayServer::window_set_mouse_passthrough_ex]."]
#[must_use]
pub struct ExWindowSetMousePassthrough < 'a > {
    surround_object: &'a mut re_export::DisplayServer, region: PackedVector2Array, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMousePassthrough < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, region: PackedVector2Array,) -> Self {
        Self {
            surround_object, region, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_mouse_passthrough_full(self.surround_object, self.region, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_current_screen_ex`][super::DisplayServer::window_get_current_screen_ex]."]
#[must_use]
pub struct ExWindowGetCurrentScreen < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetCurrentScreen < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::DisplayServer::window_get_current_screen_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_current_screen_ex`][super::DisplayServer::window_set_current_screen_ex]."]
#[must_use]
pub struct ExWindowSetCurrentScreen < 'a > {
    surround_object: &'a mut re_export::DisplayServer, screen: i32, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetCurrentScreen < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, screen: i32,) -> Self {
        Self {
            surround_object, screen, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_current_screen_full(self.surround_object, self.screen, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_ex`][super::DisplayServer::window_get_position_ex]."]
#[must_use]
pub struct ExWindowGetPosition < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_position_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_with_decorations_ex`][super::DisplayServer::window_get_position_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetPositionWithDecorations < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPositionWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_position_with_decorations_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_position_ex`][super::DisplayServer::window_set_position_ex]."]
#[must_use]
pub struct ExWindowSetPosition < 'a > {
    surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetPosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        Self {
            surround_object, position, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_position_full(self.surround_object, self.position, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_ex`][super::DisplayServer::window_get_size_ex]."]
#[must_use]
pub struct ExWindowGetSize < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_size_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_size_ex`][super::DisplayServer::window_set_size_ex]."]
#[must_use]
pub struct ExWindowSetSize < 'a > {
    surround_object: &'a mut re_export::DisplayServer, size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, size: Vector2i,) -> Self {
        Self {
            surround_object, size, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_size_full(self.surround_object, self.size, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_rect_changed_callback_ex`][super::DisplayServer::window_set_rect_changed_callback_ex]."]
#[must_use]
pub struct ExWindowSetRectChangedCallback < 'a > {
    surround_object: &'a mut re_export::DisplayServer, callback: Callable, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetRectChangedCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: Callable,) -> Self {
        Self {
            surround_object, callback, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_rect_changed_callback_full(self.surround_object, self.callback, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_event_callback_ex`][super::DisplayServer::window_set_window_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetWindowEventCallback < 'a > {
    surround_object: &'a mut re_export::DisplayServer, callback: Callable, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: Callable,) -> Self {
        Self {
            surround_object, callback, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_window_event_callback_full(self.surround_object, self.callback, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_event_callback_ex`][super::DisplayServer::window_set_input_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputEventCallback < 'a > {
    surround_object: &'a mut re_export::DisplayServer, callback: Callable, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: Callable,) -> Self {
        Self {
            surround_object, callback, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_input_event_callback_full(self.surround_object, self.callback, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_text_callback_ex`][super::DisplayServer::window_set_input_text_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputTextCallback < 'a > {
    surround_object: &'a mut re_export::DisplayServer, callback: Callable, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputTextCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: Callable,) -> Self {
        Self {
            surround_object, callback, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_input_text_callback_full(self.surround_object, self.callback, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_drop_files_callback_ex`][super::DisplayServer::window_set_drop_files_callback_ex]."]
#[must_use]
pub struct ExWindowSetDropFilesCallback < 'a > {
    surround_object: &'a mut re_export::DisplayServer, callback: Callable, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetDropFilesCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: Callable,) -> Self {
        Self {
            surround_object, callback, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_drop_files_callback_full(self.surround_object, self.callback, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_attached_instance_id_ex`][super::DisplayServer::window_get_attached_instance_id_ex]."]
#[must_use]
pub struct ExWindowGetAttachedInstanceId < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetAttachedInstanceId < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        re_export::DisplayServer::window_get_attached_instance_id_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_max_size_ex`][super::DisplayServer::window_get_max_size_ex]."]
#[must_use]
pub struct ExWindowGetMaxSize < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMaxSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_max_size_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_max_size_ex`][super::DisplayServer::window_set_max_size_ex]."]
#[must_use]
pub struct ExWindowSetMaxSize < 'a > {
    surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMaxSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i,) -> Self {
        Self {
            surround_object, max_size, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_max_size_full(self.surround_object, self.max_size, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_min_size_ex`][super::DisplayServer::window_get_min_size_ex]."]
#[must_use]
pub struct ExWindowGetMinSize < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMinSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_min_size_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_min_size_ex`][super::DisplayServer::window_set_min_size_ex]."]
#[must_use]
pub struct ExWindowSetMinSize < 'a > {
    surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMinSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i,) -> Self {
        Self {
            surround_object, min_size, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_min_size_full(self.surround_object, self.min_size, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_with_decorations_ex`][super::DisplayServer::window_get_size_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetSizeWithDecorations < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSizeWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        re_export::DisplayServer::window_get_size_with_decorations_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_mode_ex`][super::DisplayServer::window_get_mode_ex]."]
#[must_use]
pub struct ExWindowGetMode < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::display_server::WindowMode {
        re_export::DisplayServer::window_get_mode_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mode_ex`][super::DisplayServer::window_set_mode_ex]."]
#[must_use]
pub struct ExWindowSetMode < 'a > {
    surround_object: &'a mut re_export::DisplayServer, mode: crate::engine::display_server::WindowMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, mode: crate::engine::display_server::WindowMode,) -> Self {
        Self {
            surround_object, mode, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_mode_full(self.surround_object, self.mode, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_flag_ex`][super::DisplayServer::window_set_flag_ex]."]
#[must_use]
pub struct ExWindowSetFlag < 'a > {
    surround_object: &'a mut re_export::DisplayServer, flag: crate::engine::display_server::WindowFlags, enabled: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetFlag < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, flag: crate::engine::display_server::WindowFlags, enabled: bool,) -> Self {
        Self {
            surround_object, flag, enabled, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_flag_full(self.surround_object, self.flag, self.enabled, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_flag_ex`][super::DisplayServer::window_get_flag_ex]."]
#[must_use]
pub struct ExWindowGetFlag < 'a > {
    surround_object: &'a re_export::DisplayServer, flag: crate::engine::display_server::WindowFlags, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetFlag < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, flag: crate::engine::display_server::WindowFlags,) -> Self {
        Self {
            surround_object, flag, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::DisplayServer::window_get_flag_full(self.surround_object, self.flag, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_buttons_offset_ex`][super::DisplayServer::window_set_window_buttons_offset_ex]."]
#[must_use]
pub struct ExWindowSetWindowButtonsOffset < 'a > {
    surround_object: &'a mut re_export::DisplayServer, offset: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowButtonsOffset < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, offset: Vector2i,) -> Self {
        Self {
            surround_object, offset, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_window_buttons_offset_full(self.surround_object, self.offset, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_safe_title_margins_ex`][super::DisplayServer::window_get_safe_title_margins_ex]."]
#[must_use]
pub struct ExWindowGetSafeTitleMargins < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSafeTitleMargins < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3i {
        re_export::DisplayServer::window_get_safe_title_margins_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_request_attention_ex`][super::DisplayServer::window_request_attention_ex]."]
#[must_use]
pub struct ExWindowRequestAttention < 'a > {
    surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowRequestAttention < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_request_attention_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_move_to_foreground_ex`][super::DisplayServer::window_move_to_foreground_ex]."]
#[must_use]
pub struct ExWindowMoveToForeground < 'a > {
    surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowMoveToForeground < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_move_to_foreground_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_focused_ex`][super::DisplayServer::window_is_focused_ex]."]
#[must_use]
pub struct ExWindowIsFocused < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsFocused < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::DisplayServer::window_is_focused_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_can_draw_ex`][super::DisplayServer::window_can_draw_ex]."]
#[must_use]
pub struct ExWindowCanDraw < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowCanDraw < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::DisplayServer::window_can_draw_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_active_ex`][super::DisplayServer::window_set_ime_active_ex]."]
#[must_use]
pub struct ExWindowSetImeActive < 'a > {
    surround_object: &'a mut re_export::DisplayServer, active: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImeActive < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, active: bool,) -> Self {
        Self {
            surround_object, active, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_ime_active_full(self.surround_object, self.active, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_position_ex`][super::DisplayServer::window_set_ime_position_ex]."]
#[must_use]
pub struct ExWindowSetImePosition < 'a > {
    surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImePosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        Self {
            surround_object, position, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_ime_position_full(self.surround_object, self.position, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_vsync_mode_ex`][super::DisplayServer::window_set_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowSetVsyncMode < 'a > {
    surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::engine::display_server::VSyncMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetVsyncMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::engine::display_server::VSyncMode,) -> Self {
        Self {
            surround_object, vsync_mode, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::window_set_vsync_mode_full(self.surround_object, self.vsync_mode, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_vsync_mode_ex`][super::DisplayServer::window_get_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowGetVsyncMode < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetVsyncMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::display_server::VSyncMode {
        re_export::DisplayServer::window_get_vsync_mode_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_maximize_allowed_ex`][super::DisplayServer::window_is_maximize_allowed_ex]."]
#[must_use]
pub struct ExWindowIsMaximizeAllowed < 'a > {
    surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsMaximizeAllowed < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        Self {
            surround_object, window_id: 0i32,
        }
    }
    #[inline]
    pub fn window_id(self, value: i32) -> Self {
        Self {
            window_id: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::DisplayServer::window_is_maximize_allowed_full(self.surround_object, self.window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::virtual_keyboard_show_ex`][super::DisplayServer::virtual_keyboard_show_ex]."]
#[must_use]
pub struct ExVirtualKeyboardShow < 'a > {
    surround_object: &'a mut re_export::DisplayServer, existing_text: GString, position: Rect2, type_: crate::engine::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVirtualKeyboardShow < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, existing_text: GString,) -> Self {
        Self {
            surround_object, existing_text, position: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), type_: crate::obj::EngineEnum::from_ord(0), max_length: - 1i32, cursor_start: - 1i32, cursor_end: - 1i32,
        }
    }
    #[inline]
    pub fn position(self, value: Rect2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn type_(self, value: crate::engine::display_server::VirtualKeyboardType) -> Self {
        Self {
            type_: value, .. self
        }
    }
    #[inline]
    pub fn max_length(self, value: i32) -> Self {
        Self {
            max_length: value, .. self
        }
    }
    #[inline]
    pub fn cursor_start(self, value: i32) -> Self {
        Self {
            cursor_start: value, .. self
        }
    }
    #[inline]
    pub fn cursor_end(self, value: i32) -> Self {
        Self {
            cursor_end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::virtual_keyboard_show_full(self.surround_object, self.existing_text, self.position, self.type_, self.max_length, self.cursor_start, self.cursor_end,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::cursor_set_custom_image_ex`][super::DisplayServer::cursor_set_custom_image_ex]."]
#[must_use]
pub struct ExCursorSetCustomImage < 'a > {
    surround_object: &'a mut re_export::DisplayServer, cursor: Gd < crate::engine::Resource >, shape: crate::engine::display_server::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCursorSetCustomImage < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, cursor: Gd < crate::engine::Resource >,) -> Self {
        Self {
            surround_object, cursor, shape: crate::obj::EngineEnum::from_ord(0), hotspot: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn shape(self, value: crate::engine::display_server::CursorShape) -> Self {
        Self {
            shape: value, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, value: Vector2) -> Self {
        Self {
            hotspot: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::DisplayServer::cursor_set_custom_image_full(self.surround_object, self.cursor, self.shape, self.hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Feature {
    ord: i32
}
impl Feature {
    pub const FEATURE_GLOBAL_MENU: Self = Self {
        ord: 0i32
    };
    pub const FEATURE_SUBWINDOWS: Self = Self {
        ord: 1i32
    };
    pub const FEATURE_TOUCHSCREEN: Self = Self {
        ord: 2i32
    };
    pub const FEATURE_MOUSE: Self = Self {
        ord: 3i32
    };
    pub const FEATURE_MOUSE_WARP: Self = Self {
        ord: 4i32
    };
    pub const FEATURE_CLIPBOARD: Self = Self {
        ord: 5i32
    };
    pub const FEATURE_VIRTUAL_KEYBOARD: Self = Self {
        ord: 6i32
    };
    pub const FEATURE_CURSOR_SHAPE: Self = Self {
        ord: 7i32
    };
    pub const FEATURE_CUSTOM_CURSOR_SHAPE: Self = Self {
        ord: 8i32
    };
    pub const FEATURE_NATIVE_DIALOG: Self = Self {
        ord: 9i32
    };
    pub const FEATURE_IME: Self = Self {
        ord: 10i32
    };
    pub const FEATURE_WINDOW_TRANSPARENCY: Self = Self {
        ord: 11i32
    };
    pub const FEATURE_HIDPI: Self = Self {
        ord: 12i32
    };
    pub const FEATURE_ICON: Self = Self {
        ord: 13i32
    };
    pub const FEATURE_NATIVE_ICON: Self = Self {
        ord: 14i32
    };
    pub const FEATURE_ORIENTATION: Self = Self {
        ord: 15i32
    };
    pub const FEATURE_SWAP_BUFFERS: Self = Self {
        ord: 16i32
    };
    pub const FEATURE_CLIPBOARD_PRIMARY: Self = Self {
        ord: 18i32
    };
    pub const FEATURE_TEXT_TO_SPEECH: Self = Self {
        ord: 19i32
    };
    pub const FEATURE_EXTEND_TO_TITLE: Self = Self {
        ord: 20i32
    };
    pub const FEATURE_SCREEN_CAPTURE: Self = Self {
        ord: 21i32
    };
    
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Feature {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    pub const MOUSE_MODE_VISIBLE: Self = Self {
        ord: 0i32
    };
    pub const MOUSE_MODE_HIDDEN: Self = Self {
        ord: 1i32
    };
    pub const MOUSE_MODE_CAPTURED: Self = Self {
        ord: 2i32
    };
    pub const MOUSE_MODE_CONFINED: Self = Self {
        ord: 3i32
    };
    pub const MOUSE_MODE_CONFINED_HIDDEN: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for MouseMode {
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
impl crate::builtin::meta::GodotConvert for MouseMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MouseMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScreenOrientation {
    ord: i32
}
impl ScreenOrientation {
    pub const SCREEN_LANDSCAPE: Self = Self {
        ord: 0i32
    };
    pub const SCREEN_PORTRAIT: Self = Self {
        ord: 1i32
    };
    pub const SCREEN_REVERSE_LANDSCAPE: Self = Self {
        ord: 2i32
    };
    pub const SCREEN_REVERSE_PORTRAIT: Self = Self {
        ord: 3i32
    };
    pub const SCREEN_SENSOR_LANDSCAPE: Self = Self {
        ord: 4i32
    };
    pub const SCREEN_SENSOR_PORTRAIT: Self = Self {
        ord: 5i32
    };
    pub const SCREEN_SENSOR: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for ScreenOrientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ScreenOrientation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ScreenOrientation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ScreenOrientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    pub const KEYBOARD_TYPE_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const KEYBOARD_TYPE_MULTILINE: Self = Self {
        ord: 1i32
    };
    pub const KEYBOARD_TYPE_NUMBER: Self = Self {
        ord: 2i32
    };
    pub const KEYBOARD_TYPE_NUMBER_DECIMAL: Self = Self {
        ord: 3i32
    };
    pub const KEYBOARD_TYPE_PHONE: Self = Self {
        ord: 4i32
    };
    pub const KEYBOARD_TYPE_EMAIL_ADDRESS: Self = Self {
        ord: 5i32
    };
    pub const KEYBOARD_TYPE_PASSWORD: Self = Self {
        ord: 6i32
    };
    pub const KEYBOARD_TYPE_URL: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VirtualKeyboardType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VirtualKeyboardType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    pub const CURSOR_ARROW: Self = Self {
        ord: 0i32
    };
    pub const CURSOR_IBEAM: Self = Self {
        ord: 1i32
    };
    pub const CURSOR_POINTING_HAND: Self = Self {
        ord: 2i32
    };
    pub const CURSOR_CROSS: Self = Self {
        ord: 3i32
    };
    pub const CURSOR_WAIT: Self = Self {
        ord: 4i32
    };
    pub const CURSOR_BUSY: Self = Self {
        ord: 5i32
    };
    pub const CURSOR_DRAG: Self = Self {
        ord: 6i32
    };
    pub const CURSOR_CAN_DROP: Self = Self {
        ord: 7i32
    };
    pub const CURSOR_FORBIDDEN: Self = Self {
        ord: 8i32
    };
    pub const CURSOR_VSIZE: Self = Self {
        ord: 9i32
    };
    pub const CURSOR_HSIZE: Self = Self {
        ord: 10i32
    };
    pub const CURSOR_BDIAGSIZE: Self = Self {
        ord: 11i32
    };
    pub const CURSOR_FDIAGSIZE: Self = Self {
        ord: 12i32
    };
    pub const CURSOR_MOVE: Self = Self {
        ord: 13i32
    };
    pub const CURSOR_VSPLIT: Self = Self {
        ord: 14i32
    };
    pub const CURSOR_HSPLIT: Self = Self {
        ord: 15i32
    };
    pub const CURSOR_HELP: Self = Self {
        ord: 16i32
    };
    pub const CURSOR_MAX: Self = Self {
        ord: 17i32
    };
    
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for CursorShape {
    const ENUMERATOR_COUNT: usize = 17usize;
    
}
impl crate::builtin::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CursorShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileDialogMode {
    ord: i32
}
impl FileDialogMode {
    pub const FILE_DIALOG_MODE_OPEN_FILE: Self = Self {
        ord: 0i32
    };
    pub const FILE_DIALOG_MODE_OPEN_FILES: Self = Self {
        ord: 1i32
    };
    pub const FILE_DIALOG_MODE_OPEN_DIR: Self = Self {
        ord: 2i32
    };
    pub const FILE_DIALOG_MODE_OPEN_ANY: Self = Self {
        ord: 3i32
    };
    pub const FILE_DIALOG_MODE_SAVE_FILE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for FileDialogMode {
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
impl crate::builtin::meta::GodotConvert for FileDialogMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FileDialogMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FileDialogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WindowMode {
    ord: i32
}
impl WindowMode {
    pub const WINDOW_MODE_WINDOWED: Self = Self {
        ord: 0i32
    };
    pub const WINDOW_MODE_MINIMIZED: Self = Self {
        ord: 1i32
    };
    pub const WINDOW_MODE_MAXIMIZED: Self = Self {
        ord: 2i32
    };
    pub const WINDOW_MODE_FULLSCREEN: Self = Self {
        ord: 3i32
    };
    pub const WINDOW_MODE_EXCLUSIVE_FULLSCREEN: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for WindowMode {
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
impl crate::builtin::meta::GodotConvert for WindowMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for WindowMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for WindowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WindowFlags {
    ord: i32
}
impl WindowFlags {
    pub const WINDOW_FLAG_RESIZE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const WINDOW_FLAG_BORDERLESS: Self = Self {
        ord: 1i32
    };
    pub const WINDOW_FLAG_ALWAYS_ON_TOP: Self = Self {
        ord: 2i32
    };
    pub const WINDOW_FLAG_TRANSPARENT: Self = Self {
        ord: 3i32
    };
    pub const WINDOW_FLAG_NO_FOCUS: Self = Self {
        ord: 4i32
    };
    pub const WINDOW_FLAG_POPUP: Self = Self {
        ord: 5i32
    };
    pub const WINDOW_FLAG_EXTEND_TO_TITLE: Self = Self {
        ord: 6i32
    };
    pub const WINDOW_FLAG_MOUSE_PASSTHROUGH: Self = Self {
        ord: 7i32
    };
    pub const WINDOW_FLAG_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for WindowFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for WindowFlags {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for WindowFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for WindowFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for WindowFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WindowEvent {
    ord: i32
}
impl WindowEvent {
    pub const WINDOW_EVENT_MOUSE_ENTER: Self = Self {
        ord: 0i32
    };
    pub const WINDOW_EVENT_MOUSE_EXIT: Self = Self {
        ord: 1i32
    };
    pub const WINDOW_EVENT_FOCUS_IN: Self = Self {
        ord: 2i32
    };
    pub const WINDOW_EVENT_FOCUS_OUT: Self = Self {
        ord: 3i32
    };
    pub const WINDOW_EVENT_CLOSE_REQUEST: Self = Self {
        ord: 4i32
    };
    pub const WINDOW_EVENT_GO_BACK_REQUEST: Self = Self {
        ord: 5i32
    };
    pub const WINDOW_EVENT_DPI_CHANGE: Self = Self {
        ord: 6i32
    };
    pub const WINDOW_EVENT_TITLEBAR_CHANGE: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for WindowEvent {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for WindowEvent {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for WindowEvent {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for WindowEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VSyncMode {
    ord: i32
}
impl VSyncMode {
    pub const VSYNC_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VSYNC_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const VSYNC_ADAPTIVE: Self = Self {
        ord: 2i32
    };
    pub const VSYNC_MAILBOX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for VSyncMode {
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
impl crate::builtin::meta::GodotConvert for VSyncMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VSyncMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VSyncMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HandleType {
    ord: i32
}
impl HandleType {
    pub const DISPLAY_HANDLE: Self = Self {
        ord: 0i32
    };
    pub const WINDOW_HANDLE: Self = Self {
        ord: 1i32
    };
    pub const WINDOW_VIEW: Self = Self {
        ord: 2i32
    };
    pub const OPENGL_CONTEXT: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for HandleType {
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
impl crate::builtin::meta::GodotConvert for HandleType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HandleType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TTSUtteranceEvent {
    ord: i32
}
impl TTSUtteranceEvent {
    pub const TTS_UTTERANCE_STARTED: Self = Self {
        ord: 0i32
    };
    pub const TTS_UTTERANCE_ENDED: Self = Self {
        ord: 1i32
    };
    pub const TTS_UTTERANCE_CANCELED: Self = Self {
        ord: 2i32
    };
    pub const TTS_UTTERANCE_BOUNDARY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for TTSUtteranceEvent {
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
impl crate::builtin::meta::GodotConvert for TTSUtteranceEvent {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TTSUtteranceEvent {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TTSUtteranceEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}