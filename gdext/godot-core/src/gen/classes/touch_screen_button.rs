#![doc = "Sidecar module for class [`TouchScreenButton`][crate::engine::TouchScreenButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TouchScreenButton` enums](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TouchScreenButton.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`touch_screen_button`][crate::engine::touch_screen_button]: sidecar module with related enum/flag types\n* [`ITouchScreenButton`][crate::engine::ITouchScreenButton]: virtual methods\n\n\nSee also [Godot docs for `TouchScreenButton`](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TouchScreenButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TouchScreenButton`][crate::engine::TouchScreenButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TouchScreenButton` methods](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITouchScreenButton: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    impl TouchScreenButton {
        pub fn set_texture_normal(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_normal(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_pressed(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_pressed(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bitmask(&mut self, bitmask: Gd < crate::engine::BitMap >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::BitMap >);
            let args = (bitmask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bitmask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bitmask(&self,) -> Option < Gd < crate::engine::BitMap > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::BitMap > >;
            type CallSig = (Option < Gd < crate::engine::BitMap > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bitmask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape(&mut self, shape: Gd < crate::engine::Shape2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Shape2D >);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape(&self,) -> Option < Gd < crate::engine::Shape2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Shape2D > >;
            type CallSig = (Option < Gd < crate::engine::Shape2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_centered(&mut self, bool: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (bool,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shape_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_centered(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shape_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_visible(&mut self, bool: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (bool,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shape_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shape_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_action(&mut self, action: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_mode(&mut self, mode: crate::engine::touch_screen_button::VisibilityMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::touch_screen_button::VisibilityMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_mode(&self,) -> crate::engine::touch_screen_button::VisibilityMode {
            type RetMarshal = PtrcallReturnT < crate::engine::touch_screen_button::VisibilityMode >;
            type CallSig = (crate::engine::touch_screen_button::VisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_passby_press(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_passby_press", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passby_press_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_passby_press_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_pressed", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TouchScreenButton {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TouchScreenButton\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TouchScreenButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TouchScreenButton {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for TouchScreenButton {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for TouchScreenButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for TouchScreenButton {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TouchScreenButton {
        
    }
    impl crate::obj::ExportableObject for TouchScreenButton {
        
    }
    impl crate::obj::cap::GodotDefault for TouchScreenButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TouchScreenButton {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TouchScreenButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TouchScreenButton {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TouchScreenButton > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node2D > for $Class {
                
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
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    pub const VISIBILITY_ALWAYS: Self = Self {
        ord: 0i32
    };
    pub const VISIBILITY_TOUCHSCREEN_ONLY: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for VisibilityMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibilityMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}