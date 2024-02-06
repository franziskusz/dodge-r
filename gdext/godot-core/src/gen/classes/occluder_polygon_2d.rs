#![doc = "Sidecar module for class [`OccluderPolygon2D`][crate::engine::OccluderPolygon2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OccluderPolygon2D` enums](https://docs.godotengine.org/en/stable/classes/class_occluderpolygon2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OccluderPolygon2D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`occluder_polygon_2d`][crate::engine::occluder_polygon_2d]: sidecar module with related enum/flag types\n* [`IOccluderPolygon2D`][crate::engine::IOccluderPolygon2D]: virtual methods\n\n\nSee also [Godot docs for `OccluderPolygon2D`](https://docs.godotengine.org/en/stable/classes/class_occluderpolygon2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OccluderPolygon2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OccluderPolygon2D`][crate::engine::OccluderPolygon2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OccluderPolygon2D` methods](https://docs.godotengine.org/en/stable/classes/class_occluderpolygon2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOccluderPolygon2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OccluderPolygon2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_closed(&mut self, closed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (closed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_closed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_closed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_closed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mode(&mut self, cull_mode: crate::engine::occluder_polygon_2d::CullMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::occluder_polygon_2d::CullMode);
            let args = (cull_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mode(&self,) -> crate::engine::occluder_polygon_2d::CullMode {
            type RetMarshal = PtrcallReturnT < crate::engine::occluder_polygon_2d::CullMode >;
            type CallSig = (crate::engine::occluder_polygon_2d::CullMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_polygon(&mut self, polygon: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_polygon(&self,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_polygon", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OccluderPolygon2D {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OccluderPolygon2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OccluderPolygon2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OccluderPolygon2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for OccluderPolygon2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for OccluderPolygon2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for OccluderPolygon2D {
        
    }
    impl crate::obj::ExportableObject for OccluderPolygon2D {
        
    }
    impl crate::obj::cap::GodotDefault for OccluderPolygon2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OccluderPolygon2D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OccluderPolygon2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OccluderPolygon2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OccluderPolygon2D > for $Class {
                
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
pub struct CullMode {
    ord: i32
}
impl CullMode {
    pub const CULL_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CULL_CLOCKWISE: Self = Self {
        ord: 1i32
    };
    pub const CULL_COUNTER_CLOCKWISE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CullMode {
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
impl crate::builtin::meta::GodotConvert for CullMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CullMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}