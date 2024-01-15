#![doc = "Sidecar module for class [`OptionButton`][crate::engine::OptionButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OptionButton` enums](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OptionButton.`\n\nInherits [`Button`][crate::engine::Button].\n\nRelated symbols:\n\n* [`option_button`][crate::engine::option_button]: sidecar module with related enum/flag types\n* [`IOptionButton`][crate::engine::IOptionButton]: virtual methods\n\n\nSee also [Godot docs for `OptionButton`](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OptionButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OptionButton`][crate::engine::OptionButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OptionButton` methods](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOptionButton: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl OptionButton {
        pub(crate) fn add_item_full(&mut self, label: GString, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5367usize);
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
        pub(crate) fn add_icon_item_full(&mut self, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, GString, i32);
            let args = (texture, label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5368usize);
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
        pub fn set_item_text(&mut self, idx: i32, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (idx, text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (idx, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_id(&mut self, idx: i32, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (idx, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (idx, metadata,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (idx, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_id(&self, idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_separator(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_item_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_separator_full(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5383usize);
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
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_metadata(&self,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selected_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::engine::PopupMenu > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PopupMenu > >;
            type CallSig = (Option < Gd < crate::engine::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show_popup(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "show_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_selectable_items(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_selectable_items", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_selectable_item_full(&self, from_last: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, bool);
            let args = (from_last,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_selectable_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_selectable_item(&self,) -> i32 {
            self.get_selectable_item_ex() . done()
        }
        #[inline]
        pub fn get_selectable_item_ex(&self,) -> ExGetSelectableItem < '_ > {
            ExGetSelectableItem::new(self,)
        }
        pub fn set_fit_to_longest_item(&mut self, fit: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (fit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fit_to_longest_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fit_to_longest_item(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_fit_to_longest_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_shortcuts", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OptionButton {
        type Base = crate::engine::Button;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OptionButton\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OptionButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OptionButton {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Button > for OptionButton {
        
    }
    impl crate::obj::Inherits < crate::engine::BaseButton > for OptionButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for OptionButton {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for OptionButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for OptionButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for OptionButton {
        
    }
    impl crate::obj::ExportableObject for OptionButton {
        
    }
    impl crate::obj::cap::GodotDefault for OptionButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OptionButton {
        type Target = crate::engine::Button;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OptionButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OptionButton {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OptionButton > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Button > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::BaseButton > for $Class {
                
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
#[doc = "Default-param extender for [`OptionButton::add_item_ex`][super::OptionButton::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    surround_object: &'a mut re_export::OptionButton, label: GString, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, label: GString,) -> Self {
        Self {
            surround_object, label, id: - 1i32,
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
        re_export::OptionButton::add_item_full(self.surround_object, self.label, self.id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_icon_item_ex`][super::OptionButton::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    surround_object: &'a mut re_export::OptionButton, texture: Gd < crate::engine::Texture2D >, label: GString, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, texture: Gd < crate::engine::Texture2D >, label: GString,) -> Self {
        Self {
            surround_object, texture, label, id: - 1i32,
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
        re_export::OptionButton::add_icon_item_full(self.surround_object, self.texture, self.label, self.id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_separator_ex`][super::OptionButton::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    surround_object: &'a mut re_export::OptionButton, text: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton,) -> Self {
        Self {
            surround_object, text: GString::from(""),
        }
    }
    #[inline]
    pub fn text(self, value: GString) -> Self {
        Self {
            text: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::OptionButton::add_separator_full(self.surround_object, self.text,)
    }
}
#[doc = "Default-param extender for [`OptionButton::get_selectable_item_ex`][super::OptionButton::get_selectable_item_ex]."]
#[must_use]
pub struct ExGetSelectableItem < 'a > {
    surround_object: &'a re_export::OptionButton, from_last: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectableItem < 'a > {
    fn new(surround_object: &'a re_export::OptionButton,) -> Self {
        Self {
            surround_object, from_last: false,
        }
    }
    #[inline]
    pub fn from_last(self, value: bool) -> Self {
        Self {
            from_last: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::OptionButton::get_selectable_item_full(self.surround_object, self.from_last,)
    }
}