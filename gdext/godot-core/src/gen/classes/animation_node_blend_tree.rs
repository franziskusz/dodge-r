#![doc = "Sidecar module for class [`AnimationNodeBlendTree`][crate::engine::AnimationNodeBlendTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendTree` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeBlendTree.`\n\nInherits [`AnimationRootNode`][crate::engine::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_tree`][crate::engine::animation_node_blend_tree]: sidecar module with related enum/flag types\n* [`IAnimationNodeBlendTree`][crate::engine::IAnimationNodeBlendTree]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeBlendTree`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeBlendTree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeBlendTree`][crate::engine::AnimationNodeBlendTree].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeBlendTree` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeBlendTree: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_child_nodes(&self,) -> Dictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::engine::AnimationNode > > {
            unimplemented !()
        }
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        fn process(&self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
            unimplemented !()
        }
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        fn has_filter(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AnimationNodeBlendTree {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_node_full(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::AnimationNode >, Vector2);
            let args = (name, node, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_node(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >,) {
            self.add_node_ex(name, node,) . done()
        }
        #[inline]
        pub fn add_node_ex(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >,) -> ExAddNode < '_ > {
            ExAddNode::new(self, name, node,)
        }
        pub fn get_node(&self, name: StringName,) -> Option < Gd < crate::engine::AnimationNode > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationNode > >;
            type CallSig = (Option < Gd < crate::engine::AnimationNode > >, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_node(&mut self, name: StringName, new_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, new_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_node(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_node(&mut self, input_node: StringName, input_index: i32, output_node: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32, StringName);
            let args = (input_node, input_index, output_node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_node(&mut self, input_node: StringName, input_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (input_node, input_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_node_position(&mut self, name: StringName, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Vector2);
            let args = (name, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_position(&self, name: StringName,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const CONNECTION_OK: i32 = 0i32;
        pub const CONNECTION_ERROR_NO_INPUT: i32 = 1i32;
        pub const CONNECTION_ERROR_NO_INPUT_INDEX: i32 = 2i32;
        pub const CONNECTION_ERROR_NO_OUTPUT: i32 = 3i32;
        pub const CONNECTION_ERROR_SAME_NODE: i32 = 4i32;
        pub const CONNECTION_ERROR_CONNECTION_EXISTS: i32 = 5i32;
        
    }
    impl crate::obj::GodotClass for AnimationNodeBlendTree {
        type Base = crate::engine::AnimationRootNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNodeBlendTree\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeBlendTree {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNodeBlendTree {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AnimationRootNode > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::Inherits < crate::engine::AnimationNode > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::ExportableObject for AnimationNodeBlendTree {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeBlendTree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeBlendTree {
        type Target = crate::engine::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNodeBlendTree {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationNodeBlendTree > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AnimationRootNode > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AnimationNode > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeBlendTree::add_node_ex`][super::AnimationNodeBlendTree::add_node_ex]."]
#[must_use]
pub struct ExAddNode < 'a > {
    surround_object: &'a mut re_export::AnimationNodeBlendTree, name: StringName, node: Gd < crate::engine::AnimationNode >, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNode < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendTree, name: StringName, node: Gd < crate::engine::AnimationNode >,) -> Self {
        Self {
            surround_object, name, node, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationNodeBlendTree::add_node_full(self.surround_object, self.name, self.node, self.position,)
    }
}