#![doc = "Sidecar module for class [`VoxelGi`][crate::engine::VoxelGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VoxelGI` enums](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VoxelGI.`\n\nInherits [`VisualInstance3D`][crate::engine::VisualInstance3D].\n\nRelated symbols:\n\n* [`voxel_gi`][crate::engine::voxel_gi]: sidecar module with related enum/flag types\n* [`IVoxelGi`][crate::engine::IVoxelGi]: virtual methods\n\n\nSee also [Godot docs for `VoxelGI`](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VoxelGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VoxelGi`][crate::engine::VoxelGi].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VoxelGI` methods](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVoxelGi: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
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
    impl VoxelGi {
        pub fn set_probe_data(&mut self, data: Gd < crate::engine::VoxelGiData >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::VoxelGiData >);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_probe_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_probe_data(&self,) -> Option < Gd < crate::engine::VoxelGiData > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VoxelGiData > >;
            type CallSig = (Option < Gd < crate::engine::VoxelGiData > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_probe_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subdiv(&mut self, subdiv: crate::engine::voxel_gi::Subdiv,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::voxel_gi::Subdiv);
            let args = (subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subdiv(&self,) -> crate::engine::voxel_gi::Subdiv {
            type RetMarshal = PtrcallReturnT < crate::engine::voxel_gi::Subdiv >;
            type CallSig = (crate::engine::voxel_gi::Subdiv,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: Gd < crate::engine::CameraAttributes >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CameraAttributes >);
            let args = (camera_attributes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::engine::CameraAttributes > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CameraAttributes > >;
            type CallSig = (Option < Gd < crate::engine::CameraAttributes > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bake_full(&mut self, from_node: Gd < crate::engine::Node >, create_visual_debug: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, bool);
            let args = (from_node, create_visual_debug,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bake", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bake(&mut self,) {
            self.bake_ex() . done()
        }
        #[inline]
        pub fn bake_ex(&mut self,) -> ExBake < '_ > {
            ExBake::new(self,)
        }
        pub fn debug_bake(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "debug_bake", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VoxelGi {
        type Base = crate::engine::VisualInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VoxelGI\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VoxelGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VoxelGi {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for VoxelGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for VoxelGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for VoxelGi {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VoxelGi {
        
    }
    impl crate::obj::ExportableObject for VoxelGi {
        
    }
    impl crate::obj::cap::GodotDefault for VoxelGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VoxelGi {
        type Target = crate::engine::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VoxelGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VoxelGi {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VoxelGi > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
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
#[doc = "Default-param extender for [`VoxelGi::bake_ex`][super::VoxelGi::bake_ex]."]
#[must_use]
pub struct ExBake < 'a > {
    surround_object: &'a mut re_export::VoxelGi, from_node: Gd < crate::engine::Node >, create_visual_debug: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBake < 'a > {
    fn new(surround_object: &'a mut re_export::VoxelGi,) -> Self {
        Self {
            surround_object, from_node: unimplemented !("see #156"), create_visual_debug: false,
        }
    }
    #[inline]
    pub fn from_node(self, value: Gd < crate::engine::Node >) -> Self {
        Self {
            from_node: value, .. self
        }
    }
    #[inline]
    pub fn create_visual_debug(self, value: bool) -> Self {
        Self {
            create_visual_debug: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::VoxelGi::bake_full(self.surround_object, self.from_node, self.create_visual_debug,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Subdiv {
    ord: i32
}
impl Subdiv {
    pub const SUBDIV_64: Self = Self {
        ord: 0i32
    };
    pub const SUBDIV_128: Self = Self {
        ord: 1i32
    };
    pub const SUBDIV_256: Self = Self {
        ord: 2i32
    };
    pub const SUBDIV_512: Self = Self {
        ord: 3i32
    };
    pub const SUBDIV_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Subdiv {
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
impl crate::obj::IndexEnum for Subdiv {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for Subdiv {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Subdiv {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Subdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}