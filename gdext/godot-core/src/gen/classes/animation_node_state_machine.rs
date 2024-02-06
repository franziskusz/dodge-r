#![doc = "Sidecar module for class [`AnimationNodeStateMachine`][crate::engine::AnimationNodeStateMachine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachine` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachine.`\n\nInherits [`AnimationRootNode`][crate::engine::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_state_machine`][crate::engine::animation_node_state_machine]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachine`][crate::engine::IAnimationNodeStateMachine]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachine`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeStateMachine`][crate::engine::AnimationNodeStateMachine].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeStateMachine` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachine: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachine {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_node_full(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::AnimationNode >, Vector2);
            let args = (name, node, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(366usize);
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
        pub fn replace_node(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::AnimationNode >);
            let args = (name, node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "replace_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node(&self, name: StringName,) -> Option < Gd < crate::engine::AnimationNode > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationNode > >;
            type CallSig = (Option < Gd < crate::engine::AnimationNode > >, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_node(&mut self, name: StringName, new_name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, new_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_node(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_name(&self, node: Gd < crate::engine::AnimationNode >,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, Gd < crate::engine::AnimationNode >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_node_position(&mut self, name: StringName, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Vector2);
            let args = (name, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_position(&self, name: StringName,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_transition(&self, from: StringName, to: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_transition(&mut self, from: StringName, to: StringName, transition: Gd < crate::engine::AnimationNodeStateMachineTransition >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, Gd < crate::engine::AnimationNodeStateMachineTransition >);
            let args = (from, to, transition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition(&self, idx: i32,) -> Option < Gd < crate::engine::AnimationNodeStateMachineTransition > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationNodeStateMachineTransition > >;
            type CallSig = (Option < Gd < crate::engine::AnimationNodeStateMachineTransition > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_from(&self, idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transition_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_to(&self, idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transition_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transition_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_transition_by_index(&mut self, idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_transition_by_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_transition(&mut self, from: StringName, to: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_state_machine_type(&mut self, state_machine_type: crate::engine::animation_node_state_machine::StateMachineType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_node_state_machine::StateMachineType);
            let args = (state_machine_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_state_machine_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state_machine_type(&self,) -> crate::engine::animation_node_state_machine::StateMachineType {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_node_state_machine::StateMachineType >;
            type CallSig = (crate::engine::animation_node_state_machine::StateMachineType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_state_machine_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_transition_to_self(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_transition_to_self", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_allow_transition_to_self(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_allow_transition_to_self", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reset_ends(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_reset_ends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_ends_reset(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_ends_reset", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachine {
        type Base = crate::engine::AnimationRootNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNodeStateMachine\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNodeStateMachine {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AnimationRootNode > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::Inherits < crate::engine::AnimationNode > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::ExportableObject for AnimationNodeStateMachine {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachine {
        type Target = crate::engine::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNodeStateMachine {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationNodeStateMachine > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeStateMachine::add_node_ex`][super::AnimationNodeStateMachine::add_node_ex]."]
#[must_use]
pub struct ExAddNode < 'a > {
    surround_object: &'a mut re_export::AnimationNodeStateMachine, name: StringName, node: Gd < crate::engine::AnimationNode >, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNode < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachine, name: StringName, node: Gd < crate::engine::AnimationNode >,) -> Self {
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
        re_export::AnimationNodeStateMachine::add_node_full(self.surround_object, self.name, self.node, self.position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StateMachineType {
    ord: i32
}
impl StateMachineType {
    pub const STATE_MACHINE_TYPE_ROOT: Self = Self {
        ord: 0i32
    };
    pub const STATE_MACHINE_TYPE_NESTED: Self = Self {
        ord: 1i32
    };
    pub const STATE_MACHINE_TYPE_GROUPED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for StateMachineType {
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
impl crate::builtin::meta::GodotConvert for StateMachineType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for StateMachineType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for StateMachineType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}