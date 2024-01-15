#![doc = "Sidecar module for class [`AnimationNode`][crate::engine::AnimationNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNode` enums](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNode.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation_node`][crate::engine::animation_node]: sidecar module with related enum/flag types\n* [`IAnimationNode`][crate::engine::IAnimationNode]: virtual methods\n\n\nSee also [Godot docs for `AnimationNode`](https://docs.godotengine.org/en/stable/classes/class_animationnode.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNode`][crate::engine::AnimationNode].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNode` methods](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNode: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNode {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_input(&mut self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_input(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_name(&mut self, input: i32, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, GString);
            let args = (input, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_name(&self, input: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (input,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_input(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_filter_path(&mut self, path: NodePath, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, bool);
            let args = (path, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_filter_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_filtered(&self, path: NodePath,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_path_filtered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_filter_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_filter_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_filter_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_filter_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn blend_animation_full(&mut self, animation: StringName, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32, looped_flag: crate::engine::animation::LoopedFlag,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f64, f64, bool, bool, f32, crate::engine::animation::LoopedFlag);
            let args = (animation, time, delta, seeked, is_external_seeking, blend, looped_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn blend_animation(&mut self, animation: StringName, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) {
            self.blend_animation_ex(animation, time, delta, seeked, is_external_seeking, blend,) . done()
        }
        #[inline]
        pub fn blend_animation_ex(&mut self, animation: StringName, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) -> ExBlendAnimation < '_ > {
            ExBlendAnimation::new(self, animation, time, delta, seeked, is_external_seeking, blend,)
        }
        pub(crate) fn blend_node_full(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::engine::animation_node::FilterAction, sync: bool, test_only: bool,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, StringName, Gd < crate::engine::AnimationNode >, f64, bool, bool, f32, crate::engine::animation_node::FilterAction, bool, bool);
            let args = (name, node, time, seek, is_external_seeking, blend, filter, sync, test_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn blend_node(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> f64 {
            self.blend_node_ex(name, node, time, seek, is_external_seeking, blend,) . done()
        }
        #[inline]
        pub fn blend_node_ex(&mut self, name: StringName, node: Gd < crate::engine::AnimationNode >, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> ExBlendNode < '_ > {
            ExBlendNode::new(self, name, node, time, seek, is_external_seeking, blend,)
        }
        pub(crate) fn blend_input_full(&mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::engine::animation_node::FilterAction, sync: bool, test_only: bool,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, i32, f64, bool, bool, f32, crate::engine::animation_node::FilterAction, bool, bool);
            let args = (input_index, time, seek, is_external_seeking, blend, filter, sync, test_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn blend_input(&mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> f64 {
            self.blend_input_ex(input_index, time, seek, is_external_seeking, blend,) . done()
        }
        #[inline]
        pub fn blend_input_ex(&mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> ExBlendInput < '_ > {
            ExBlendInput::new(self, input_index, time, seek, is_external_seeking, blend,)
        }
        pub fn set_parameter(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parameter(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parameter", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNode {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNode\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNode {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNode {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNode {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNode {
        
    }
    impl crate::obj::ExportableObject for AnimationNode {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNode {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNode {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNode {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`AnimationNode::blend_animation_ex`][super::AnimationNode::blend_animation_ex]."]
#[must_use]
pub struct ExBlendAnimation < 'a > {
    surround_object: &'a mut re_export::AnimationNode, animation: StringName, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32, looped_flag: crate::engine::animation::LoopedFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBlendAnimation < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNode, animation: StringName, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) -> Self {
        Self {
            surround_object, animation, time, delta, seeked, is_external_seeking, blend, looped_flag: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn looped_flag(self, value: crate::engine::animation::LoopedFlag) -> Self {
        Self {
            looped_flag: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationNode::blend_animation_full(self.surround_object, self.animation, self.time, self.delta, self.seeked, self.is_external_seeking, self.blend, self.looped_flag,)
    }
}
#[doc = "Default-param extender for [`AnimationNode::blend_node_ex`][super::AnimationNode::blend_node_ex]."]
#[must_use]
pub struct ExBlendNode < 'a > {
    surround_object: &'a mut re_export::AnimationNode, name: StringName, node: Gd < crate::engine::AnimationNode >, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::engine::animation_node::FilterAction, sync: bool, test_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBlendNode < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNode, name: StringName, node: Gd < crate::engine::AnimationNode >, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> Self {
        Self {
            surround_object, name, node, time, seek, is_external_seeking, blend, filter: crate::obj::EngineEnum::from_ord(0), sync: true, test_only: false,
        }
    }
    #[inline]
    pub fn filter(self, value: crate::engine::animation_node::FilterAction) -> Self {
        Self {
            filter: value, .. self
        }
    }
    #[inline]
    pub fn sync(self, value: bool) -> Self {
        Self {
            sync: value, .. self
        }
    }
    #[inline]
    pub fn test_only(self, value: bool) -> Self {
        Self {
            test_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        re_export::AnimationNode::blend_node_full(self.surround_object, self.name, self.node, self.time, self.seek, self.is_external_seeking, self.blend, self.filter, self.sync, self.test_only,)
    }
}
#[doc = "Default-param extender for [`AnimationNode::blend_input_ex`][super::AnimationNode::blend_input_ex]."]
#[must_use]
pub struct ExBlendInput < 'a > {
    surround_object: &'a mut re_export::AnimationNode, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::engine::animation_node::FilterAction, sync: bool, test_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBlendInput < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNode, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> Self {
        Self {
            surround_object, input_index, time, seek, is_external_seeking, blend, filter: crate::obj::EngineEnum::from_ord(0), sync: true, test_only: false,
        }
    }
    #[inline]
    pub fn filter(self, value: crate::engine::animation_node::FilterAction) -> Self {
        Self {
            filter: value, .. self
        }
    }
    #[inline]
    pub fn sync(self, value: bool) -> Self {
        Self {
            sync: value, .. self
        }
    }
    #[inline]
    pub fn test_only(self, value: bool) -> Self {
        Self {
            test_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        re_export::AnimationNode::blend_input_full(self.surround_object, self.input_index, self.time, self.seek, self.is_external_seeking, self.blend, self.filter, self.sync, self.test_only,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilterAction {
    ord: i32
}
impl FilterAction {
    pub const FILTER_IGNORE: Self = Self {
        ord: 0i32
    };
    pub const FILTER_PASS: Self = Self {
        ord: 1i32
    };
    pub const FILTER_STOP: Self = Self {
        ord: 2i32
    };
    pub const FILTER_BLEND: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for FilterAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for FilterAction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FilterAction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FilterAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}