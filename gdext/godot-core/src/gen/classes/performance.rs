#![doc = "Sidecar module for class [`Performance`][crate::engine::Performance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Performance` enums](https://docs.godotengine.org/en/stable/classes/class_performance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Performance.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`performance`][crate::engine::performance]: sidecar module with related enum/flag types\n* [`IPerformance`][crate::engine::IPerformance]: virtual methods\n\n\nSee also [Godot docs for `Performance`](https://docs.godotengine.org/en/stable/classes/class_performance.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Performance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Performance`][crate::engine::Performance].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Performance` methods](https://docs.godotengine.org/en/stable/classes/class_performance.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPerformance: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Performance {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Performance\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_monitor(&self, monitor: crate::engine::performance::Monitor,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, crate::engine::performance::Monitor);
            let args = (monitor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_custom_monitor_full(&mut self, id: StringName, callable: Callable, arguments: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Callable, VariantArray);
            let args = (id, callable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_custom_monitor(&mut self, id: StringName, callable: Callable,) {
            self.add_custom_monitor_ex(id, callable,) . done()
        }
        #[inline]
        pub fn add_custom_monitor_ex(&mut self, id: StringName, callable: Callable,) -> ExAddCustomMonitor < '_ > {
            ExAddCustomMonitor::new(self, id, callable,)
        }
        pub fn remove_custom_monitor(&mut self, id: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_custom_monitor(&mut self, id: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_monitor(&mut self, id: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_monitor_modification_time(&mut self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_monitor_modification_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_monitor_names(&mut self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_monitor_names", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Performance {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Performance\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Performance {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Performance {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Performance {
        
    }
    impl std::ops::Deref for Performance {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Performance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Performance {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Performance > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Performance::add_custom_monitor_ex`][super::Performance::add_custom_monitor_ex]."]
#[must_use]
pub struct ExAddCustomMonitor < 'a > {
    surround_object: &'a mut re_export::Performance, id: StringName, callable: Callable, arguments: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomMonitor < 'a > {
    fn new(surround_object: &'a mut re_export::Performance, id: StringName, callable: Callable,) -> Self {
        Self {
            surround_object, id, callable, arguments: Array::new(),
        }
    }
    #[inline]
    pub fn arguments(self, value: VariantArray) -> Self {
        Self {
            arguments: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Performance::add_custom_monitor_full(self.surround_object, self.id, self.callable, self.arguments,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Monitor {
    ord: i32
}
impl Monitor {
    pub const TIME_FPS: Self = Self {
        ord: 0i32
    };
    pub const TIME_PROCESS: Self = Self {
        ord: 1i32
    };
    pub const TIME_PHYSICS_PROCESS: Self = Self {
        ord: 2i32
    };
    pub const TIME_NAVIGATION_PROCESS: Self = Self {
        ord: 3i32
    };
    pub const MEMORY_STATIC: Self = Self {
        ord: 4i32
    };
    pub const MEMORY_STATIC_MAX: Self = Self {
        ord: 5i32
    };
    pub const MEMORY_MESSAGE_BUFFER_MAX: Self = Self {
        ord: 6i32
    };
    pub const OBJECT_COUNT: Self = Self {
        ord: 7i32
    };
    pub const OBJECT_RESOURCE_COUNT: Self = Self {
        ord: 8i32
    };
    pub const OBJECT_NODE_COUNT: Self = Self {
        ord: 9i32
    };
    pub const OBJECT_ORPHAN_NODE_COUNT: Self = Self {
        ord: 10i32
    };
    pub const RENDER_TOTAL_OBJECTS_IN_FRAME: Self = Self {
        ord: 11i32
    };
    pub const RENDER_TOTAL_PRIMITIVES_IN_FRAME: Self = Self {
        ord: 12i32
    };
    pub const RENDER_TOTAL_DRAW_CALLS_IN_FRAME: Self = Self {
        ord: 13i32
    };
    pub const RENDER_VIDEO_MEM_USED: Self = Self {
        ord: 14i32
    };
    pub const RENDER_TEXTURE_MEM_USED: Self = Self {
        ord: 15i32
    };
    pub const RENDER_BUFFER_MEM_USED: Self = Self {
        ord: 16i32
    };
    pub const PHYSICS_2D_ACTIVE_OBJECTS: Self = Self {
        ord: 17i32
    };
    pub const PHYSICS_2D_COLLISION_PAIRS: Self = Self {
        ord: 18i32
    };
    pub const PHYSICS_2D_ISLAND_COUNT: Self = Self {
        ord: 19i32
    };
    pub const PHYSICS_3D_ACTIVE_OBJECTS: Self = Self {
        ord: 20i32
    };
    pub const PHYSICS_3D_COLLISION_PAIRS: Self = Self {
        ord: 21i32
    };
    pub const PHYSICS_3D_ISLAND_COUNT: Self = Self {
        ord: 22i32
    };
    pub const AUDIO_OUTPUT_LATENCY: Self = Self {
        ord: 23i32
    };
    pub const NAVIGATION_ACTIVE_MAPS: Self = Self {
        ord: 24i32
    };
    pub const NAVIGATION_REGION_COUNT: Self = Self {
        ord: 25i32
    };
    pub const NAVIGATION_AGENT_COUNT: Self = Self {
        ord: 26i32
    };
    pub const NAVIGATION_LINK_COUNT: Self = Self {
        ord: 27i32
    };
    pub const NAVIGATION_POLYGON_COUNT: Self = Self {
        ord: 28i32
    };
    pub const NAVIGATION_EDGE_COUNT: Self = Self {
        ord: 29i32
    };
    pub const NAVIGATION_EDGE_MERGE_COUNT: Self = Self {
        ord: 30i32
    };
    pub const NAVIGATION_EDGE_CONNECTION_COUNT: Self = Self {
        ord: 31i32
    };
    pub const NAVIGATION_EDGE_FREE_COUNT: Self = Self {
        ord: 32i32
    };
    pub const MONITOR_MAX: Self = Self {
        ord: 33i32
    };
    
}
impl crate::obj::EngineEnum for Monitor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Monitor {
    const ENUMERATOR_COUNT: usize = 33usize;
    
}
impl crate::builtin::meta::GodotConvert for Monitor {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Monitor {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Monitor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}