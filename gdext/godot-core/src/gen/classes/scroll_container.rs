#![doc = "Sidecar module for class [`ScrollContainer`][crate::engine::ScrollContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScrollContainer` enums](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScrollContainer.`\n\nInherits [`Container`][crate::engine::Container].\n\nRelated symbols:\n\n* [`scroll_container`][crate::engine::scroll_container]: sidecar module with related enum/flag types\n* [`IScrollContainer`][crate::engine::IScrollContainer]: virtual methods\n\n\nSee also [Godot docs for `ScrollContainer`](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScrollContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ScrollContainer`][crate::engine::ScrollContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScrollContainer` methods](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScrollContainer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ScrollContainer {
        pub fn set_h_scroll(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_scroll(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_scroll(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_horizontal_custom_step(&mut self, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_horizontal_custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_custom_step(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_horizontal_custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_custom_step(&mut self, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertical_custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_custom_step(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertical_custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_horizontal_scroll_mode(&mut self, enable: crate::engine::scroll_container::ScrollMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::scroll_container::ScrollMode);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_horizontal_scroll_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_scroll_mode(&self,) -> crate::engine::scroll_container::ScrollMode {
            type RetMarshal = PtrcallReturnT < crate::engine::scroll_container::ScrollMode >;
            type CallSig = (crate::engine::scroll_container::ScrollMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_horizontal_scroll_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_scroll_mode(&mut self, enable: crate::engine::scroll_container::ScrollMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::scroll_container::ScrollMode);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertical_scroll_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_scroll_mode(&self,) -> crate::engine::scroll_container::ScrollMode {
            type RetMarshal = PtrcallReturnT < crate::engine::scroll_container::ScrollMode >;
            type CallSig = (crate::engine::scroll_container::ScrollMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertical_scroll_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deadzone(&mut self, deadzone: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_deadzone(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_follow_focus(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_follow_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_following_focus(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_following_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_scroll_bar(&mut self,) -> Option < Gd < crate::engine::HScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::HScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::HScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::engine::VScrollBar > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VScrollBar > >;
            type CallSig = (Option < Gd < crate::engine::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ensure_control_visible(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ensure_control_visible", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ScrollContainer {
        type Base = crate::engine::Container;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ScrollContainer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScrollContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ScrollContainer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Container > for ScrollContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for ScrollContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for ScrollContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ScrollContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ScrollContainer {
        
    }
    impl crate::obj::ExportableObject for ScrollContainer {
        
    }
    impl crate::obj::cap::GodotDefault for ScrollContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScrollContainer {
        type Target = crate::engine::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScrollContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ScrollContainer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ScrollContainer > for $Class {
                
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
pub struct ScrollMode {
    ord: i32
}
impl ScrollMode {
    pub const SCROLL_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SCROLL_MODE_AUTO: Self = Self {
        ord: 1i32
    };
    pub const SCROLL_MODE_SHOW_ALWAYS: Self = Self {
        ord: 2i32
    };
    pub const SCROLL_MODE_SHOW_NEVER: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ScrollMode {
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
impl crate::builtin::meta::GodotConvert for ScrollMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ScrollMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ScrollMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}