#![doc = "Sidecar module for class [`RdPipelineDepthStencilState`][crate::engine::RdPipelineDepthStencilState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDPipelineDepthStencilState` enums](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDPipelineDepthStencilState.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IRdPipelineDepthStencilState`][crate::engine::IRdPipelineDepthStencilState]: virtual methods\n\n\nSee also [Godot docs for `RDPipelineDepthStencilState`](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdPipelineDepthStencilState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdPipelineDepthStencilState`][crate::engine::RdPipelineDepthStencilState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDPipelineDepthStencilState` methods](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdPipelineDepthStencilState: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdPipelineDepthStencilState {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_enable_depth_test(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_depth_test", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_depth_test(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_depth_test", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_depth_write(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_depth_write", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_depth_write(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_depth_write", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_compare_operator(&mut self, p_member: crate::engine::rendering_device::CompareOperator,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::CompareOperator);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth_compare_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_compare_operator(&self,) -> crate::engine::rendering_device::CompareOperator {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::CompareOperator >;
            type CallSig = (crate::engine::rendering_device::CompareOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_compare_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_depth_range(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_depth_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_depth_range(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_depth_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_range_min(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_range_min(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_range_max(&mut self, p_member: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_range_max(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_stencil(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_stencil", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_stencil(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_stencil", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_fail(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_fail(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_pass(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_pass(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_depth_fail(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_depth_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_depth_fail(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_depth_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_compare(&mut self, p_member: crate::engine::rendering_device::CompareOperator,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::CompareOperator);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_compare(&self,) -> crate::engine::rendering_device::CompareOperator {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::CompareOperator >;
            type CallSig = (crate::engine::rendering_device::CompareOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_compare_mask(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_compare_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_compare_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_compare_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_write_mask(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_write_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_write_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_write_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_op_reference(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_front_op_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_op_reference(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_front_op_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_fail(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_fail(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_pass(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_pass(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_depth_fail(&mut self, p_member: crate::engine::rendering_device::StencilOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::StencilOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_depth_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_depth_fail(&self,) -> crate::engine::rendering_device::StencilOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::StencilOperation >;
            type CallSig = (crate::engine::rendering_device::StencilOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_depth_fail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_compare(&mut self, p_member: crate::engine::rendering_device::CompareOperator,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::CompareOperator);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_compare(&self,) -> crate::engine::rendering_device::CompareOperator {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::CompareOperator >;
            type CallSig = (crate::engine::rendering_device::CompareOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_compare_mask(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_compare_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_compare_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_compare_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_write_mask(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_write_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_write_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_write_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_back_op_reference(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_back_op_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_back_op_reference(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_back_op_reference", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdPipelineDepthStencilState {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RDPipelineDepthStencilState\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdPipelineDepthStencilState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RdPipelineDepthStencilState {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RdPipelineDepthStencilState {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RdPipelineDepthStencilState {
        
    }
    impl crate::obj::cap::GodotDefault for RdPipelineDepthStencilState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdPipelineDepthStencilState {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdPipelineDepthStencilState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RdPipelineDepthStencilState {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RdPipelineDepthStencilState > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}