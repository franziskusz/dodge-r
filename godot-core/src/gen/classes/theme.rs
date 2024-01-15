#![doc = "Sidecar module for class [`Theme`][crate::engine::Theme].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Theme` enums](https://docs.godotengine.org/en/stable/classes/class_theme.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Theme.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`theme`][crate::engine::theme]: sidecar module with related enum/flag types\n* [`ITheme`][crate::engine::ITheme]: virtual methods\n\n\nSee also [Godot docs for `Theme`](https://docs.godotengine.org/en/stable/classes/class_theme.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Theme {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Theme`][crate::engine::Theme].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Theme` methods](https://docs.godotengine.org/en/stable/classes/class_theme.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITheme: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Theme {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_icon(&mut self, name: StringName, theme_type: StringName, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, Gd < crate::engine::Texture2D >);
            let args = (name, theme_type, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_icon(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_icon(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_icon(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_icon_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stylebox(&mut self, name: StringName, theme_type: StringName, texture: Gd < crate::engine::StyleBox >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, Gd < crate::engine::StyleBox >);
            let args = (name, theme_type, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stylebox(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::StyleBox > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::StyleBox > >;
            type CallSig = (Option < Gd < crate::engine::StyleBox > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_stylebox(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_stylebox(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_stylebox(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stylebox_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stylebox_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stylebox_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stylebox_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font(&mut self, name: StringName, theme_type: StringName, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, Gd < crate::engine::Font >);
            let args = (name, theme_type, font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_font(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_font(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_font(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_size(&mut self, name: StringName, theme_type: StringName, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, i32);
            let args = (name, theme_type, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_size(&self, name: StringName, theme_type: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_font_size(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_font_size(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_font_size(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_size_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_size_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_size_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_font_size_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, name: StringName, theme_type: StringName, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, Color);
            let args = (name, theme_type, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self, name: StringName, theme_type: StringName,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_color(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_color(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_color(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant(&mut self, name: StringName, theme_type: StringName, constant: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, i32);
            let args = (name, theme_type, constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant(&self, name: StringName, theme_type: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_constant(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_constant(&mut self, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_constant(&mut self, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_list(&self, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_base_scale(&mut self, base_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (base_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_base_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_default_base_scale(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_font(&mut self, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >);
            let args = (font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_font(&self,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_default_font(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_font_size(&mut self, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_font_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_default_font_size(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme_item(&mut self, data_type: crate::engine::theme::DataType, name: StringName, theme_type: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::theme::DataType, StringName, StringName, Variant);
            let args = (data_type, name, theme_type, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_theme_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_item(&self, data_type: crate::engine::theme::DataType, name: StringName, theme_type: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, crate::engine::theme::DataType, StringName, StringName);
            let args = (data_type, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_item(&self, data_type: crate::engine::theme::DataType, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::theme::DataType, StringName, StringName);
            let args = (data_type, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_theme_item(&mut self, data_type: crate::engine::theme::DataType, old_name: StringName, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::theme::DataType, StringName, StringName, StringName);
            let args = (data_type, old_name, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_theme_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_theme_item(&mut self, data_type: crate::engine::theme::DataType, name: StringName, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::theme::DataType, StringName, StringName);
            let args = (data_type, name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_theme_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_item_list(&self, data_type: crate::engine::theme::DataType, theme_type: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, crate::engine::theme::DataType, GString);
            let args = (data_type, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_item_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_item_type_list(&self, data_type: crate::engine::theme::DataType,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, crate::engine::theme::DataType);
            let args = (data_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_item_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_type_variation(&mut self, theme_type: StringName, base_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (theme_type, base_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_type_variation(&self, theme_type: StringName, base_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (theme_type, base_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_type_variation(&mut self, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_type_variation_base(&self, theme_type: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_type_variation_base", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_type_variation_list(&self, base_type: StringName,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, StringName);
            let args = (base_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_type_variation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_type(&mut self, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_type(&mut self, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_type_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_type_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn merge_with(&mut self, other: Gd < crate::engine::Theme >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Theme >);
            let args = (other,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "merge_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Theme {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Theme\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Theme {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Theme {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Theme {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Theme {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Theme {
        
    }
    impl crate::obj::ExportableObject for Theme {
        
    }
    impl crate::obj::cap::GodotDefault for Theme {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Theme {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Theme {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Theme {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Theme > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
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
pub struct DataType {
    ord: i32
}
impl DataType {
    pub const DATA_TYPE_COLOR: Self = Self {
        ord: 0i32
    };
    pub const DATA_TYPE_CONSTANT: Self = Self {
        ord: 1i32
    };
    pub const DATA_TYPE_FONT: Self = Self {
        ord: 2i32
    };
    pub const DATA_TYPE_FONT_SIZE: Self = Self {
        ord: 3i32
    };
    pub const DATA_TYPE_ICON: Self = Self {
        ord: 4i32
    };
    pub const DATA_TYPE_STYLEBOX: Self = Self {
        ord: 5i32
    };
    pub const DATA_TYPE_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for DataType {
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
impl crate::obj::IndexEnum for DataType {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for DataType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DataType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DataType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}