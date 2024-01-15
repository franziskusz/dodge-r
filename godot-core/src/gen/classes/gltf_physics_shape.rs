#![doc = "Sidecar module for class [`GltfPhysicsShape`][crate::engine::GltfPhysicsShape].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFPhysicsShape` enums](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFPhysicsShape.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`gltf_physics_shape`][crate::engine::gltf_physics_shape]: sidecar module with related enum/flag types\n* [`IGltfPhysicsShape`][crate::engine::IGltfPhysicsShape]: virtual methods\n\n\nSee also [Godot docs for `GLTFPhysicsShape`](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfPhysicsShape {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfPhysicsShape`][crate::engine::GltfPhysicsShape].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFPhysicsShape` methods](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfPhysicsShape: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfPhysicsShape {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn from_node(shape_node: Gd < crate::engine::CollisionShape3D >,) -> Option < Gd < crate::engine::GltfPhysicsShape > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::GltfPhysicsShape > >;
            type CallSig = (Option < Gd < crate::engine::GltfPhysicsShape > >, Gd < crate::engine::CollisionShape3D >);
            let args = (shape_node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "from_node", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn to_node_full(&mut self, cache_shapes: bool,) -> Option < Gd < crate::engine::CollisionShape3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::CollisionShape3D > >;
            type CallSig = (Option < Gd < crate::engine::CollisionShape3D > >, bool);
            let args = (cache_shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn to_node(&mut self,) -> Option < Gd < crate::engine::CollisionShape3D > > {
            self.to_node_ex() . done()
        }
        #[inline]
        pub fn to_node_ex(&mut self,) -> ExToNode < '_ > {
            ExToNode::new(self,)
        }
        pub fn from_dictionary(dictionary: Dictionary,) -> Option < Gd < crate::engine::GltfPhysicsShape > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::GltfPhysicsShape > >;
            type CallSig = (Option < Gd < crate::engine::GltfPhysicsShape > >, Dictionary);
            let args = (dictionary,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "from_dictionary", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn to_dictionary(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_dictionary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape_type(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shape_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_type(&mut self, shape_type: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (shape_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shape_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_is_trigger(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_is_trigger", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_is_trigger(&mut self, is_trigger: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (is_trigger,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_is_trigger", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_index(&mut self, mesh_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (mesh_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_importer_mesh(&self,) -> Option < Gd < crate::engine::ImporterMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ImporterMesh > >;
            type CallSig = (Option < Gd < crate::engine::ImporterMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_importer_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_importer_mesh(&mut self, importer_mesh: Gd < crate::engine::ImporterMesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ImporterMesh >);
            let args = (importer_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_importer_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfPhysicsShape {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFPhysicsShape\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfPhysicsShape {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfPhysicsShape {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfPhysicsShape {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfPhysicsShape {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfPhysicsShape {
        
    }
    impl crate::obj::ExportableObject for GltfPhysicsShape {
        
    }
    impl crate::obj::cap::GodotDefault for GltfPhysicsShape {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfPhysicsShape {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfPhysicsShape {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfPhysicsShape {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfPhysicsShape > for $Class {
                
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
#[doc = "Default-param extender for [`GltfPhysicsShape::to_node_ex`][super::GltfPhysicsShape::to_node_ex]."]
#[must_use]
pub struct ExToNode < 'a > {
    surround_object: &'a mut re_export::GltfPhysicsShape, cache_shapes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToNode < 'a > {
    fn new(surround_object: &'a mut re_export::GltfPhysicsShape,) -> Self {
        Self {
            surround_object, cache_shapes: false,
        }
    }
    #[inline]
    pub fn cache_shapes(self, value: bool) -> Self {
        Self {
            cache_shapes: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::CollisionShape3D > > {
        re_export::GltfPhysicsShape::to_node_full(self.surround_object, self.cache_shapes,)
    }
}