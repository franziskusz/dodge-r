#![doc = "Sidecar module for class [`AnimationNodeBlendSpace1D`][crate::engine::AnimationNodeBlendSpace1D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeBlendSpace1D.`\n\nInherits [`AnimationRootNode`][crate::engine::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_space1_d`][crate::engine::animation_node_blend_space1_d]: sidecar module with related enum/flag types\n* [`IAnimationNodeBlendSpace1D`][crate::engine::IAnimationNodeBlendSpace1D]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeBlendSpace1D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeBlendSpace1D`][crate::engine::AnimationNodeBlendSpace1D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeBlendSpace1D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeBlendSpace1D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_blend_point_full(&mut self, node: Gd < crate::engine::AnimationRootNode >, pos: f32, at_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::AnimationRootNode >, f32, i32);
            let args = (node, pos, at_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_blend_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_blend_point(&mut self, node: Gd < crate::engine::AnimationRootNode >, pos: f32,) {
            self.add_blend_point_ex(node, pos,) . done()
        }
        #[inline]
        pub fn add_blend_point_ex(&mut self, node: Gd < crate::engine::AnimationRootNode >, pos: f32,) -> ExAddBlendPoint < '_ > {
            ExAddBlendPoint::new(self, node, pos,)
        }
        pub fn set_blend_point_position(&mut self, point: i32, pos: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (point, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_position(&self, point: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_point_node(&mut self, point: i32, node: Gd < crate::engine::AnimationRootNode >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::AnimationRootNode >);
            let args = (point, node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_point_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_node(&self, point: i32,) -> Option < Gd < crate::engine::AnimationRootNode > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationRootNode > >;
            type CallSig = (Option < Gd < crate::engine::AnimationRootNode > >, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_point_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_blend_point(&mut self, point: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_blend_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_space(&mut self, min_space: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (min_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_space(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_space(&mut self, max_space: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (max_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_space(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap(&mut self, snap: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (snap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_snap(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value_label(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_value_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_value_label(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_value_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_mode(&mut self, mode: crate::engine::animation_node_blend_space1_d::BlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_node_blend_space1_d::BlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::engine::animation_node_blend_space1_d::BlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_node_blend_space1_d::BlendMode >;
            type CallSig = (crate::engine::animation_node_blend_space1_d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_sync(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_sync(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_sync", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeBlendSpace1D {
        type Base = crate::engine::AnimationRootNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNodeBlendSpace1D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeBlendSpace1D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNodeBlendSpace1D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AnimationRootNode > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::Inherits < crate::engine::AnimationNode > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::ExportableObject for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeBlendSpace1D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeBlendSpace1D {
        type Target = crate::engine::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendSpace1D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNodeBlendSpace1D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationNodeBlendSpace1D > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeBlendSpace1D::add_blend_point_ex`][super::AnimationNodeBlendSpace1D::add_blend_point_ex]."]
#[must_use]
pub struct ExAddBlendPoint < 'a > {
    surround_object: &'a mut re_export::AnimationNodeBlendSpace1D, node: Gd < crate::engine::AnimationRootNode >, pos: f32, at_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBlendPoint < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendSpace1D, node: Gd < crate::engine::AnimationRootNode >, pos: f32,) -> Self {
        Self {
            surround_object, node, pos, at_index: - 1i32,
        }
    }
    #[inline]
    pub fn at_index(self, value: i32) -> Self {
        Self {
            at_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationNodeBlendSpace1D::add_blend_point_full(self.surround_object, self.node, self.pos, self.at_index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    pub const BLEND_MODE_INTERPOLATED: Self = Self {
        ord: 0i32
    };
    pub const BLEND_MODE_DISCRETE: Self = Self {
        ord: 1i32
    };
    pub const BLEND_MODE_DISCRETE_CARRY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for BlendMode {
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
impl crate::builtin::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}