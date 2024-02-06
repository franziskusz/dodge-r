#![doc = "Sidecar module for class [`ColorPicker`][crate::engine::ColorPicker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ColorPicker` enums](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ColorPicker.`\n\nInherits [`VBoxContainer`][crate::engine::VBoxContainer].\n\nRelated symbols:\n\n* [`color_picker`][crate::engine::color_picker]: sidecar module with related enum/flag types\n* [`IColorPicker`][crate::engine::IColorPicker]: virtual methods\n\n\nSee also [Godot docs for `ColorPicker`](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ColorPicker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ColorPicker`][crate::engine::ColorPicker].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ColorPicker` methods](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IColorPicker: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ColorPicker {
        pub fn set_pick_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pick_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deferred_mode(&mut self, mode: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deferred_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deferred_mode(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_deferred_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_mode(&mut self, color_mode: crate::engine::color_picker::ColorModeType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::color_picker::ColorModeType);
            let args = (color_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_mode(&self,) -> crate::engine::color_picker::ColorModeType {
            type RetMarshal = PtrcallReturnT < crate::engine::color_picker::ColorModeType >;
            type CallSig = (crate::engine::color_picker::ColorModeType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_alpha(&mut self, show: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_edit_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editing_alpha(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_editing_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_can_add_swatches(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_can_add_swatches", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_swatches_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_swatches_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_presets_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_presets_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_presets_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_presets_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modes_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modes_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_modes_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_modes_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sampler_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sampler_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sampler_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sampler_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sliders_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sliders_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_sliders_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_sliders_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hex_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hex_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hex_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hex_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_preset(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_preset(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_presets(&self,) -> PackedColorArray {
            type RetMarshal = PtrcallReturnT < PackedColorArray >;
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_presets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_recent_preset(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_recent_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_recent_preset(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_recent_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_recent_presets(&self,) -> PackedColorArray {
            type RetMarshal = PtrcallReturnT < PackedColorArray >;
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_recent_presets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_picker_shape(&mut self, shape: crate::engine::color_picker::PickerShapeType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::color_picker::PickerShapeType);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_picker_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_picker_shape(&self,) -> crate::engine::color_picker::PickerShapeType {
            type RetMarshal = PtrcallReturnT < crate::engine::color_picker::PickerShapeType >;
            type CallSig = (crate::engine::color_picker::PickerShapeType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_picker_shape", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ColorPicker {
        type Base = crate::engine::VBoxContainer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ColorPicker\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ColorPicker {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ColorPicker {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VBoxContainer > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::BoxContainer > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::Container > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ColorPicker {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ColorPicker {
        
    }
    impl crate::obj::ExportableObject for ColorPicker {
        
    }
    impl crate::obj::cap::GodotDefault for ColorPicker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ColorPicker {
        type Target = crate::engine::VBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ColorPicker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ColorPicker {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ColorPicker > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VBoxContainer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::BoxContainer > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColorModeType {
    ord: i32
}
impl ColorModeType {
    pub const MODE_RGB: Self = Self {
        ord: 0i32
    };
    pub const MODE_HSV: Self = Self {
        ord: 1i32
    };
    pub const MODE_RAW: Self = Self {
        ord: 2i32
    };
    pub const MODE_OKHSL: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ColorModeType {
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
impl crate::builtin::meta::GodotConvert for ColorModeType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ColorModeType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ColorModeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PickerShapeType {
    ord: i32
}
impl PickerShapeType {
    pub const SHAPE_HSV_RECTANGLE: Self = Self {
        ord: 0i32
    };
    pub const SHAPE_HSV_WHEEL: Self = Self {
        ord: 1i32
    };
    pub const SHAPE_VHS_CIRCLE: Self = Self {
        ord: 2i32
    };
    pub const SHAPE_OKHSL_CIRCLE: Self = Self {
        ord: 3i32
    };
    pub const SHAPE_NONE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for PickerShapeType {
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
impl crate::builtin::meta::GodotConvert for PickerShapeType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PickerShapeType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PickerShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}