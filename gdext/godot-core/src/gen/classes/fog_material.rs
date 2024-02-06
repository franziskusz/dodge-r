#![doc = "Sidecar module for class [`FogMaterial`][crate::engine::FogMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FogMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FogMaterial.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`IFogMaterial`][crate::engine::IFogMaterial]: virtual methods\n\n\nSee also [Godot docs for `FogMaterial`](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FogMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FogMaterial`][crate::engine::FogMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FogMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFogMaterial: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_shader_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn get_shader_mode(&self,) -> crate::engine::shader::Mode {
            unimplemented !()
        }
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl FogMaterial {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_density(&mut self, density: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_density(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_albedo(&mut self, albedo: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height_falloff(&mut self, height_falloff: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height_falloff,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_height_falloff", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height_falloff(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height_falloff", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edge_fade(&mut self, edge_fade: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (edge_fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_edge_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_fade(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edge_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_density_texture(&mut self, density_texture: Gd < crate::engine::Texture3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture3D >);
            let args = (density_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_density_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_density_texture(&self,) -> Option < Gd < crate::engine::Texture3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture3D > >;
            type CallSig = (Option < Gd < crate::engine::Texture3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_density_texture", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FogMaterial {
        type Base = crate::engine::Material;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"FogMaterial\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FogMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for FogMaterial {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Material > for FogMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for FogMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for FogMaterial {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for FogMaterial {
        
    }
    impl crate::obj::ExportableObject for FogMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for FogMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FogMaterial {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FogMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_FogMaterial {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::FogMaterial > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Material > for $Class {
                
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