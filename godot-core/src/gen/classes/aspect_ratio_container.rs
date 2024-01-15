#![doc = "Sidecar module for class [`AspectRatioContainer`][crate::engine::AspectRatioContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AspectRatioContainer` enums](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AspectRatioContainer.`\n\nInherits [`Container`][crate::engine::Container].\n\nRelated symbols:\n\n* [`aspect_ratio_container`][crate::engine::aspect_ratio_container]: sidecar module with related enum/flag types\n* [`IAspectRatioContainer`][crate::engine::IAspectRatioContainer]: virtual methods\n\n\nSee also [Godot docs for `AspectRatioContainer`](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AspectRatioContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AspectRatioContainer`][crate::engine::AspectRatioContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AspectRatioContainer` methods](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAspectRatioContainer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AspectRatioContainer {
        pub fn set_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_mode(&mut self, stretch_mode: crate::engine::aspect_ratio_container::StretchMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::aspect_ratio_container::StretchMode);
            let args = (stretch_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_mode(&self,) -> crate::engine::aspect_ratio_container::StretchMode {
            type RetMarshal = PtrcallReturnT < crate::engine::aspect_ratio_container::StretchMode >;
            type CallSig = (crate::engine::aspect_ratio_container::StretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alignment_horizontal(&mut self, alignment_horizontal: crate::engine::aspect_ratio_container::AlignmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::aspect_ratio_container::AlignmentMode);
            let args = (alignment_horizontal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alignment_horizontal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alignment_horizontal(&self,) -> crate::engine::aspect_ratio_container::AlignmentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::aspect_ratio_container::AlignmentMode >;
            type CallSig = (crate::engine::aspect_ratio_container::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alignment_horizontal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alignment_vertical(&mut self, alignment_vertical: crate::engine::aspect_ratio_container::AlignmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::aspect_ratio_container::AlignmentMode);
            let args = (alignment_vertical,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alignment_vertical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alignment_vertical(&self,) -> crate::engine::aspect_ratio_container::AlignmentMode {
            type RetMarshal = PtrcallReturnT < crate::engine::aspect_ratio_container::AlignmentMode >;
            type CallSig = (crate::engine::aspect_ratio_container::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alignment_vertical", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AspectRatioContainer {
        type Base = crate::engine::Container;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AspectRatioContainer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AspectRatioContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AspectRatioContainer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Container > for AspectRatioContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Control > for AspectRatioContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for AspectRatioContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for AspectRatioContainer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AspectRatioContainer {
        
    }
    impl crate::obj::ExportableObject for AspectRatioContainer {
        
    }
    impl crate::obj::cap::GodotDefault for AspectRatioContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AspectRatioContainer {
        type Target = crate::engine::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AspectRatioContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AspectRatioContainer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AspectRatioContainer > for $Class {
                
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
pub struct StretchMode {
    ord: i32
}
impl StretchMode {
    pub const STRETCH_WIDTH_CONTROLS_HEIGHT: Self = Self {
        ord: 0i32
    };
    pub const STRETCH_HEIGHT_CONTROLS_WIDTH: Self = Self {
        ord: 1i32
    };
    pub const STRETCH_FIT: Self = Self {
        ord: 2i32
    };
    pub const STRETCH_COVER: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for StretchMode {
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
impl crate::builtin::meta::GodotConvert for StretchMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for StretchMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for StretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlignmentMode {
    ord: i32
}
impl AlignmentMode {
    pub const ALIGNMENT_BEGIN: Self = Self {
        ord: 0i32
    };
    pub const ALIGNMENT_CENTER: Self = Self {
        ord: 1i32
    };
    pub const ALIGNMENT_END: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AlignmentMode {
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