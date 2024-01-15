#![doc = "Sidecar module for class [`Gradient`][crate::engine::Gradient].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Gradient` enums](https://docs.godotengine.org/en/stable/classes/class_gradient.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Gradient.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`gradient`][crate::engine::gradient]: sidecar module with related enum/flag types\n* [`IGradient`][crate::engine::IGradient]: virtual methods\n\n\nSee also [Godot docs for `Gradient`](https://docs.godotengine.org/en/stable/classes/class_gradient.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Gradient {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Gradient`][crate::engine::Gradient].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Gradient` methods](https://docs.godotengine.org/en/stable/classes/class_gradient.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGradient: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Gradient {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_point(&mut self, offset: f32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, Color);
            let args = (offset, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_point(&mut self, point: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, point: i32, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (point, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&mut self, point: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reverse(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reverse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, point: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (point, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&mut self, point: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample(&mut self, offset: f32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offsets(&mut self, offsets: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat32Array);
            let args = (offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offsets(&self,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_colors(&mut self, colors: PackedColorArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedColorArray);
            let args = (colors,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_colors(&self,) -> PackedColorArray {
            type RetMarshal = PtrcallReturnT < PackedColorArray >;
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolation_mode(&mut self, interpolation_mode: crate::engine::gradient::InterpolationMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gradient::InterpolationMode);
            let args = (interpolation_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interpolation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolation_mode(&mut self,) -> crate::engine::gradient::InterpolationMode {
            type RetMarshal = PtrcallReturnT < crate::engine::gradient::InterpolationMode >;
            type CallSig = (crate::engine::gradient::InterpolationMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interpolation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolation_color_space(&mut self, interpolation_color_space: crate::engine::gradient::ColorSpace,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::gradient::ColorSpace);
            let args = (interpolation_color_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_interpolation_color_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolation_color_space(&mut self,) -> crate::engine::gradient::ColorSpace {
            type RetMarshal = PtrcallReturnT < crate::engine::gradient::ColorSpace >;
            type CallSig = (crate::engine::gradient::ColorSpace,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_interpolation_color_space", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Gradient {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Gradient\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Gradient {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Gradient {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Gradient {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Gradient {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Gradient {
        
    }
    impl crate::obj::ExportableObject for Gradient {
        
    }
    impl crate::obj::cap::GodotDefault for Gradient {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Gradient {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Gradient {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Gradient {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Gradient > for $Class {
                
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
pub struct InterpolationMode {
    ord: i32
}
impl InterpolationMode {
    pub const GRADIENT_INTERPOLATE_LINEAR: Self = Self {
        ord: 0i32
    };
    pub const GRADIENT_INTERPOLATE_CONSTANT: Self = Self {
        ord: 1i32
    };
    pub const GRADIENT_INTERPOLATE_CUBIC: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for InterpolationMode {
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
impl crate::builtin::meta::GodotConvert for InterpolationMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InterpolationMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InterpolationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColorSpace {
    ord: i32
}
impl ColorSpace {
    pub const GRADIENT_COLOR_SPACE_SRGB: Self = Self {
        ord: 0i32
    };
    pub const GRADIENT_COLOR_SPACE_LINEAR_SRGB: Self = Self {
        ord: 1i32
    };
    pub const GRADIENT_COLOR_SPACE_OKLAB: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ColorSpace {
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
impl crate::builtin::meta::GodotConvert for ColorSpace {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ColorSpace {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ColorSpace {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}