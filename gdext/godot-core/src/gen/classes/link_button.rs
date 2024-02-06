#![doc = "Sidecar module for class [`LinkButton`][crate::engine::LinkButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LinkButton` enums](https://docs.godotengine.org/en/stable/classes/class_linkbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LinkButton.`\n\nInherits [`BaseButton`][crate::engine::BaseButton].\n\nRelated symbols:\n\n* [`link_button`][crate::engine::link_button]: sidecar module with related enum/flag types\n* [`ILinkButton`][crate::engine::ILinkButton]: virtual methods\n\n\nSee also [Godot docs for `LinkButton`](https://docs.godotengine.org/en/stable/classes/class_linkbutton.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LinkButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`LinkButton`][crate::engine::LinkButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `LinkButton` methods](https://docs.godotengine.org/en/stable/classes/class_linkbutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILinkButton: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LinkButton {
        pub fn set_text(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::engine::control::TextDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::engine::control::TextDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::TextDirection >;
            type CallSig = (crate::engine::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uri(&mut self, uri: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (uri,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uri", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uri(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_uri", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_underline_mode(&mut self, underline_mode: crate::engine::link_button::UnderlineMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::link_button::UnderlineMode);
            let args = (underline_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_underline_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_underline_mode(&self,) -> crate::engine::link_button::UnderlineMode {
            type RetMarshal = PtrcallReturnT < crate::engine::link_button::UnderlineMode >;
            type CallSig = (crate::engine::link_button::UnderlineMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_underline_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::engine::text_server::StructuredTextParser,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::engine::text_server::StructuredTextParser {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::StructuredTextParser >;
            type CallSig = (crate::engine::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (args,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for LinkButton {
        type Base = crate::engine::BaseButton;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"LinkButton\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LinkButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for LinkButton {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::BaseButton > for LinkButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for LinkButton {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for LinkButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for LinkButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for LinkButton {
        
    }
    impl crate::obj::ExportableObject for LinkButton {
        
    }
    impl crate::obj::cap::GodotDefault for LinkButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LinkButton {
        type Target = crate::engine::BaseButton;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LinkButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_LinkButton {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::LinkButton > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UnderlineMode {
    ord: i32
}
impl UnderlineMode {
    pub const UNDERLINE_MODE_ALWAYS: Self = Self {
        ord: 0i32
    };
    pub const UNDERLINE_MODE_ON_HOVER: Self = Self {
        ord: 1i32
    };
    pub const UNDERLINE_MODE_NEVER: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for UnderlineMode {
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
impl crate::builtin::meta::GodotConvert for UnderlineMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for UnderlineMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UnderlineMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}