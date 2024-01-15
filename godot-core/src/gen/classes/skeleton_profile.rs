#![doc = "Sidecar module for class [`SkeletonProfile`][crate::engine::SkeletonProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonProfile` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonProfile.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`skeleton_profile`][crate::engine::skeleton_profile]: sidecar module with related enum/flag types\n* [`ISkeletonProfile`][crate::engine::ISkeletonProfile]: virtual methods\n\n\nSee also [Godot docs for `SkeletonProfile`](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonProfile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SkeletonProfile`][crate::engine::SkeletonProfile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SkeletonProfile` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonProfile: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SkeletonProfile {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_root_bone(&mut self, bone_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_bone(&mut self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_base_bone(&mut self, bone_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale_base_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_base_bone(&mut self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale_base_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_group_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_size(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_name(&self, group_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group_name(&mut self, group_idx: i32, group_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (group_idx, group_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_group_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, group_idx: i32,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, i32);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, group_idx: i32, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Texture2D >);
            let args = (group_idx, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_size(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_bone(&self, bone_name: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName);
            let args = (bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_name(&self, bone_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i32, bone_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bone_idx, bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i32, bone_parent: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bone_idx, bone_parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tail_direction(&self, bone_idx: i32,) -> crate::engine::skeleton_profile::TailDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::skeleton_profile::TailDirection >;
            type CallSig = (crate::engine::skeleton_profile::TailDirection, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tail_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tail_direction(&mut self, bone_idx: i32, tail_direction: crate::engine::skeleton_profile::TailDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::skeleton_profile::TailDirection);
            let args = (bone_idx, tail_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tail_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_tail(&self, bone_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_tail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_tail(&mut self, bone_idx: i32, bone_tail: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bone_idx, bone_tail,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone_tail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_pose(&self, bone_idx: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_reference_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reference_pose(&mut self, bone_idx: i32, bone_name: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_reference_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handle_offset(&self, bone_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_handle_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_offset(&mut self, bone_idx: i32, handle_offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2);
            let args = (bone_idx, handle_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handle_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group(&self, bone_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group(&mut self, bone_idx: i32, group: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bone_idx, group,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_group", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SkeletonProfile {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SkeletonProfile\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonProfile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SkeletonProfile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for SkeletonProfile {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for SkeletonProfile {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SkeletonProfile {
        
    }
    impl crate::obj::ExportableObject for SkeletonProfile {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonProfile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonProfile {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SkeletonProfile {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SkeletonProfile > for $Class {
                
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
pub struct TailDirection {
    ord: i32
}
impl TailDirection {
    pub const TAIL_DIRECTION_AVERAGE_CHILDREN: Self = Self {
        ord: 0i32
    };
    pub const TAIL_DIRECTION_SPECIFIC_CHILD: Self = Self {
        ord: 1i32
    };
    pub const TAIL_DIRECTION_END: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TailDirection {
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
impl crate::builtin::meta::GodotConvert for TailDirection {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TailDirection {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TailDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}