#![doc = "Sidecar module for class [`GltfSkeleton`][crate::engine::GltfSkeleton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFSkeleton` enums](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFSkeleton.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IGltfSkeleton`][crate::engine::IGltfSkeleton]: virtual methods\n\n\nSee also [Godot docs for `GLTFSkeleton`](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfSkeleton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfSkeleton`][crate::engine::GltfSkeleton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFSkeleton` methods](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfSkeleton: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfSkeleton {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_joints(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joints", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_joints(&mut self, joints: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedInt32Array);
            let args = (joints,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_joints", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roots(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_roots", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roots(&mut self, roots: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedInt32Array);
            let args = (roots,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_roots", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_godot_skeleton(&mut self,) -> Option < Gd < crate::engine::Skeleton3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Skeleton3D > >;
            type CallSig = (Option < Gd < crate::engine::Skeleton3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_godot_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_names(&mut self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_names(&mut self, unique_names: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (unique_names,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_godot_bone_node(&mut self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_godot_bone_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_godot_bone_node(&mut self, godot_bone_node: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (godot_bone_node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_godot_bone_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_attachment_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_attachment_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_attachment(&mut self, idx: i32,) -> Option < Gd < crate::engine::BoneAttachment3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::BoneAttachment3D > >;
            type CallSig = (Option < Gd < crate::engine::BoneAttachment3D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone_attachment", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfSkeleton {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFSkeleton\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfSkeleton {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfSkeleton {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfSkeleton {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfSkeleton {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfSkeleton {
        
    }
    impl crate::obj::ExportableObject for GltfSkeleton {
        
    }
    impl crate::obj::cap::GodotDefault for GltfSkeleton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfSkeleton {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfSkeleton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfSkeleton {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfSkeleton > for $Class {
                
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