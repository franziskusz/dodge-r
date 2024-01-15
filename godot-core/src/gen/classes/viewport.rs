#![doc = "Sidecar module for class [`Viewport`][crate::engine::Viewport].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Viewport` enums](https://docs.godotengine.org/en/stable/classes/class_viewport.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Viewport.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`viewport`][crate::engine::viewport]: sidecar module with related enum/flag types\n* [`IViewport`][crate::engine::IViewport]: virtual methods\n\n\nSee also [Godot docs for `Viewport`](https://docs.godotengine.org/en/stable/classes/class_viewport.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Viewport {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Viewport`][crate::engine::Viewport].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Viewport` methods](https://docs.godotengine.org/en/stable/classes/class_viewport.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IViewport: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    impl Viewport {
        pub fn set_world_2d(&mut self, world_2d: Gd < crate::engine::World2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::World2D >);
            let args = (world_2d,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_2d(&self,) -> Option < Gd < crate::engine::World2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World2D > >;
            type CallSig = (Option < Gd < crate::engine::World2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_world_2d(&self,) -> Option < Gd < crate::engine::World2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World2D > >;
            type CallSig = (Option < Gd < crate::engine::World2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_transform(&mut self, xform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_canvas_transform(&mut self, xform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_canvas_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_final_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_final_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visible_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparent_background(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transparent_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_transparent_background(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_transparent_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hdr_2d(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_hdr_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_hdr_2d(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_hdr_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msaa_2d(&mut self, msaa: crate::engine::viewport::MSAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::MSAA);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msaa_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msaa_2d(&self,) -> crate::engine::viewport::MSAA {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::MSAA >;
            type CallSig = (crate::engine::viewport::MSAA,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msaa_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msaa_3d(&mut self, msaa: crate::engine::viewport::MSAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::MSAA);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::engine::viewport::MSAA {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::MSAA >;
            type CallSig = (crate::engine::viewport::MSAA,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_screen_space_aa(&mut self, screen_space_aa: crate::engine::viewport::ScreenSpaceAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::ScreenSpaceAA);
            let args = (screen_space_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::engine::viewport::ScreenSpaceAA {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::ScreenSpaceAA >;
            type CallSig = (crate::engine::viewport::ScreenSpaceAA,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_taa(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_taa(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_debanding(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_debanding(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_occlusion_culling(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_occlusion_culling(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_draw(&mut self, debug_draw: crate::engine::viewport::DebugDraw,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::DebugDraw);
            let args = (debug_draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_debug_draw(&self,) -> crate::engine::viewport::DebugDraw {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::DebugDraw >;
            type CallSig = (crate::engine::viewport::DebugDraw,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_debug_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_info(&mut self, type_: crate::engine::viewport::RenderInfoType, info: crate::engine::viewport::RenderInfo,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::viewport::RenderInfoType, crate::engine::viewport::RenderInfo);
            let args = (type_, info,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::engine::ViewportTexture > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ViewportTexture > >;
            type CallSig = (Option < Gd < crate::engine::ViewportTexture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_object_picking(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_object_picking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_object_picking(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_object_picking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_object_picking_sort(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_object_picking_sort", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_object_picking_sort(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_object_picking_sort", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport_rid(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_viewport_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_text_input(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_text_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_input_full(&mut self, event: Gd < crate::engine::InputEvent >, in_local_coords: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::InputEvent >, bool);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            self.push_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_input_ex(&mut self, event: Gd < crate::engine::InputEvent >,) -> ExPushInput < '_ > {
            ExPushInput::new(self, event,)
        }
        pub(crate) fn push_unhandled_input_full(&mut self, event: Gd < crate::engine::InputEvent >, in_local_coords: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::InputEvent >, bool);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "push_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn push_unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            self.push_unhandled_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_unhandled_input_ex(&mut self, event: Gd < crate::engine::InputEvent >,) -> ExPushUnhandledInput < '_ > {
            ExPushUnhandledInput::new(self, event,)
        }
        pub fn get_camera_2d(&self,) -> Option < Gd < crate::engine::Camera2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Camera2D > >;
            type CallSig = (Option < Gd < crate::engine::Camera2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_audio_listener_2d(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_audio_listener_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_audio_listener_2d(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_audio_listener_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mouse_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_mouse_cursor_state(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_mouse_cursor_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_get_drag_data(&self,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gui_get_drag_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_is_dragging(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gui_is_dragging", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_is_drag_successful(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gui_is_drag_successful", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_release_focus(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gui_release_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_get_focus_owner(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gui_get_focus_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_input(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_input_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_positional_shadow_atlas_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_positional_shadow_atlas_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_16_bits(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_positional_shadow_atlas_16_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_16_bits(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_positional_shadow_atlas_16_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_controls_to_pixels(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_snap_controls_to_pixels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_controls_to_pixels_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_snap_controls_to_pixels_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_2d_transforms_to_pixel(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_snap_2d_transforms_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_2d_transforms_to_pixel_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_snap_2d_transforms_to_pixel_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_2d_vertices_to_pixel(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_snap_2d_vertices_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_2d_vertices_to_pixel_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_snap_2d_vertices_to_pixel_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_quadrant_subdiv(&mut self, quadrant: i32, subdiv: crate::engine::viewport::PositionalShadowAtlasQuadrantSubdiv,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::viewport::PositionalShadowAtlasQuadrantSubdiv);
            let args = (quadrant, subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_positional_shadow_atlas_quadrant_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_quadrant_subdiv(&self, quadrant: i32,) -> crate::engine::viewport::PositionalShadowAtlasQuadrantSubdiv {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::PositionalShadowAtlasQuadrantSubdiv >;
            type CallSig = (crate::engine::viewport::PositionalShadowAtlasQuadrantSubdiv, i32);
            let args = (quadrant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_positional_shadow_atlas_quadrant_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_as_handled(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_as_handled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_handled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_input_handled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_input_locally(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handle_input_locally", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_handling_input_locally(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_handling_input_locally", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_canvas_item_texture_filter(&mut self, mode: crate::engine::viewport::DefaultCanvasItemTextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::DefaultCanvasItemTextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_canvas_item_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_canvas_item_texture_filter(&self,) -> crate::engine::viewport::DefaultCanvasItemTextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::DefaultCanvasItemTextureFilter >;
            type CallSig = (crate::engine::viewport::DefaultCanvasItemTextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_canvas_item_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_embedding_subwindows(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_embedding_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_embedding_subwindows(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_embedding_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_embedded_subwindows(&self,) -> Array < Gd < crate::engine::Window > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Window > > >;
            type CallSig = (Array < Gd < crate::engine::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_embedded_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_cull_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_canvas_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_cull_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_cull_mask_bit(&mut self, layer: u32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, bool);
            let args = (layer, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_canvas_cull_mask_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_cull_mask_bit(&self, layer: u32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_cull_mask_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_canvas_item_texture_repeat(&mut self, mode: crate::engine::viewport::DefaultCanvasItemTextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::DefaultCanvasItemTextureRepeat);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_canvas_item_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_canvas_item_texture_repeat(&self,) -> crate::engine::viewport::DefaultCanvasItemTextureRepeat {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::DefaultCanvasItemTextureRepeat >;
            type CallSig = (crate::engine::viewport::DefaultCanvasItemTextureRepeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_canvas_item_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdf_oversize(&mut self, oversize: crate::engine::viewport::SDFOversize,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::SDFOversize);
            let args = (oversize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdf_oversize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdf_oversize(&self,) -> crate::engine::viewport::SDFOversize {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::SDFOversize >;
            type CallSig = (crate::engine::viewport::SDFOversize,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdf_oversize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdf_scale(&mut self, scale: crate::engine::viewport::SDFScale,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::SDFScale);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdf_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdf_scale(&self,) -> crate::engine::viewport::SDFScale {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::SDFScale >;
            type CallSig = (crate::engine::viewport::SDFScale,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdf_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_lod_threshold(&mut self, pixels: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pixels,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh_lod_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_lod_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh_lod_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_3d(&mut self, world_3d: Gd < crate::engine::World3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::World3D >);
            let args = (world_3d,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::engine::World3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World3D > >;
            type CallSig = (Option < Gd < crate::engine::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_world_3d(&self,) -> Option < Gd < crate::engine::World3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World3D > >;
            type CallSig = (Option < Gd < crate::engine::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_own_world_3d(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_own_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_own_world_3d(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_own_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_3d(&self,) -> Option < Gd < crate::engine::Camera3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Camera3D > >;
            type CallSig = (Option < Gd < crate::engine::Camera3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_audio_listener_3d(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_audio_listener_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_audio_listener_3d(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_audio_listener_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_3d(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_disable_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_3d_disabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_3d_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_xr(&mut self, use_: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_xr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_xr(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_xr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scaling_3d_mode(&mut self, scaling_3d_mode: crate::engine::viewport::Scaling3DMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::Scaling3DMode);
            let args = (scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::engine::viewport::Scaling3DMode {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::Scaling3DMode >;
            type CallSig = (crate::engine::viewport::Scaling3DMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scaling_3d_scale(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scaling_3d_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scaling_3d_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scaling_3d_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fsr_sharpness(&mut self, fsr_sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (fsr_sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_mipmap_bias(&mut self, texture_mipmap_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (texture_mipmap_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_mipmap_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_mipmap_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_mipmap_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_mode(&mut self, mode: crate::engine::viewport::VRSMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::viewport::VRSMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vrs_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_mode(&self,) -> crate::engine::viewport::VRSMode {
            type RetMarshal = PtrcallReturnT < crate::engine::viewport::VRSMode >;
            type CallSig = (crate::engine::viewport::VRSMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vrs_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_texture(&mut self, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vrs_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vrs_texture", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Viewport {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Viewport\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Viewport {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Viewport {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for Viewport {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Viewport {
        
    }
    impl crate::obj::ExportableObject for Viewport {
        
    }
    impl std::ops::Deref for Viewport {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Viewport {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Viewport {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Viewport > for $Class {
                
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
#[doc = "Default-param extender for [`Viewport::push_input_ex`][super::Viewport::push_input_ex]."]
#[must_use]
pub struct ExPushInput < 'a > {
    surround_object: &'a mut re_export::Viewport, event: Gd < crate::engine::InputEvent >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: Gd < crate::engine::InputEvent >,) -> Self {
        Self {
            surround_object, event, in_local_coords: false,
        }
    }
    #[inline]
    pub fn in_local_coords(self, value: bool) -> Self {
        Self {
            in_local_coords: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Viewport::push_input_full(self.surround_object, self.event, self.in_local_coords,)
    }
}
#[doc = "Default-param extender for [`Viewport::push_unhandled_input_ex`][super::Viewport::push_unhandled_input_ex]."]
#[must_use]
pub struct ExPushUnhandledInput < 'a > {
    surround_object: &'a mut re_export::Viewport, event: Gd < crate::engine::InputEvent >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushUnhandledInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: Gd < crate::engine::InputEvent >,) -> Self {
        Self {
            surround_object, event, in_local_coords: false,
        }
    }
    #[inline]
    pub fn in_local_coords(self, value: bool) -> Self {
        Self {
            in_local_coords: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Viewport::push_unhandled_input_full(self.surround_object, self.event, self.in_local_coords,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PositionalShadowAtlasQuadrantSubdiv {
    ord: i32
}
impl PositionalShadowAtlasQuadrantSubdiv {
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_1: Self = Self {
        ord: 1i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_4: Self = Self {
        ord: 2i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_16: Self = Self {
        ord: 3i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_64: Self = Self {
        ord: 4i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_256: Self = Self {
        ord: 5i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_1024: Self = Self {
        ord: 6i32
    };
    pub const SHADOW_ATLAS_QUADRANT_SUBDIV_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for PositionalShadowAtlasQuadrantSubdiv {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for PositionalShadowAtlasQuadrantSubdiv {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PositionalShadowAtlasQuadrantSubdiv {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Scaling3DMode {
    ord: i32
}
impl Scaling3DMode {
    pub const SCALING_3D_MODE_BILINEAR: Self = Self {
        ord: 0i32
    };
    pub const SCALING_3D_MODE_FSR: Self = Self {
        ord: 1i32
    };
    pub const SCALING_3D_MODE_FSR2: Self = Self {
        ord: 2i32
    };
    pub const SCALING_3D_MODE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for Scaling3DMode {
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
impl crate::obj::IndexEnum for Scaling3DMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for Scaling3DMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Scaling3DMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Scaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MSAA {
    ord: i32
}
impl MSAA {
    pub const MSAA_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const MSAA_2X: Self = Self {
        ord: 1i32
    };
    pub const MSAA_4X: Self = Self {
        ord: 2i32
    };
    pub const MSAA_8X: Self = Self {
        ord: 3i32
    };
    pub const MSAA_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for MSAA {
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
impl crate::obj::IndexEnum for MSAA {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for MSAA {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MSAA {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MSAA {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScreenSpaceAA {
    ord: i32
}
impl ScreenSpaceAA {
    pub const SCREEN_SPACE_AA_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SCREEN_SPACE_AA_FXAA: Self = Self {
        ord: 1i32
    };
    pub const SCREEN_SPACE_AA_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ScreenSpaceAA {
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
impl crate::obj::IndexEnum for ScreenSpaceAA {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for ScreenSpaceAA {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ScreenSpaceAA {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ScreenSpaceAA {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderInfo {
    ord: i32
}
impl RenderInfo {
    pub const RENDER_INFO_OBJECTS_IN_FRAME: Self = Self {
        ord: 0i32
    };
    pub const RENDER_INFO_PRIMITIVES_IN_FRAME: Self = Self {
        ord: 1i32
    };
    pub const RENDER_INFO_DRAW_CALLS_IN_FRAME: Self = Self {
        ord: 2i32
    };
    pub const RENDER_INFO_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for RenderInfo {
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
impl crate::obj::IndexEnum for RenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for RenderInfo {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RenderInfo {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderInfoType {
    ord: i32
}
impl RenderInfoType {
    pub const RENDER_INFO_TYPE_VISIBLE: Self = Self {
        ord: 0i32
    };
    pub const RENDER_INFO_TYPE_SHADOW: Self = Self {
        ord: 1i32
    };
    pub const RENDER_INFO_TYPE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for RenderInfoType {
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
impl crate::obj::IndexEnum for RenderInfoType {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for RenderInfoType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RenderInfoType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DebugDraw {
    ord: i32
}
impl DebugDraw {
    pub const DEBUG_DRAW_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DEBUG_DRAW_UNSHADED: Self = Self {
        ord: 1i32
    };
    pub const DEBUG_DRAW_LIGHTING: Self = Self {
        ord: 2i32
    };
    pub const DEBUG_DRAW_OVERDRAW: Self = Self {
        ord: 3i32
    };
    pub const DEBUG_DRAW_WIREFRAME: Self = Self {
        ord: 4i32
    };
    pub const DEBUG_DRAW_NORMAL_BUFFER: Self = Self {
        ord: 5i32
    };
    pub const DEBUG_DRAW_VOXEL_GI_ALBEDO: Self = Self {
        ord: 6i32
    };
    pub const DEBUG_DRAW_VOXEL_GI_LIGHTING: Self = Self {
        ord: 7i32
    };
    pub const DEBUG_DRAW_VOXEL_GI_EMISSION: Self = Self {
        ord: 8i32
    };
    pub const DEBUG_DRAW_SHADOW_ATLAS: Self = Self {
        ord: 9i32
    };
    pub const DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS: Self = Self {
        ord: 10i32
    };
    pub const DEBUG_DRAW_SCENE_LUMINANCE: Self = Self {
        ord: 11i32
    };
    pub const DEBUG_DRAW_SSAO: Self = Self {
        ord: 12i32
    };
    pub const DEBUG_DRAW_SSIL: Self = Self {
        ord: 13i32
    };
    pub const DEBUG_DRAW_PSSM_SPLITS: Self = Self {
        ord: 14i32
    };
    pub const DEBUG_DRAW_DECAL_ATLAS: Self = Self {
        ord: 15i32
    };
    pub const DEBUG_DRAW_SDFGI: Self = Self {
        ord: 16i32
    };
    pub const DEBUG_DRAW_SDFGI_PROBES: Self = Self {
        ord: 17i32
    };
    pub const DEBUG_DRAW_GI_BUFFER: Self = Self {
        ord: 18i32
    };
    pub const DEBUG_DRAW_DISABLE_LOD: Self = Self {
        ord: 19i32
    };
    pub const DEBUG_DRAW_CLUSTER_OMNI_LIGHTS: Self = Self {
        ord: 20i32
    };
    pub const DEBUG_DRAW_CLUSTER_SPOT_LIGHTS: Self = Self {
        ord: 21i32
    };
    pub const DEBUG_DRAW_CLUSTER_DECALS: Self = Self {
        ord: 22i32
    };
    pub const DEBUG_DRAW_CLUSTER_REFLECTION_PROBES: Self = Self {
        ord: 23i32
    };
    pub const DEBUG_DRAW_OCCLUDERS: Self = Self {
        ord: 24i32
    };
    pub const DEBUG_DRAW_MOTION_VECTORS: Self = Self {
        ord: 25i32
    };
    pub const DEBUG_DRAW_INTERNAL_BUFFER: Self = Self {
        ord: 26i32
    };
    
}
impl crate::obj::EngineEnum for DebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for DebugDraw {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DebugDraw {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DefaultCanvasItemTextureFilter {
    ord: i32
}
impl DefaultCanvasItemTextureFilter {
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self {
        ord: 2i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureFilter {
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
impl crate::obj::IndexEnum for DefaultCanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for DefaultCanvasItemTextureFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DefaultCanvasItemTextureFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DefaultCanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DefaultCanvasItemTextureRepeat {
    ord: i32
}
impl DefaultCanvasItemTextureRepeat {
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR: Self = Self {
        ord: 2i32
    };
    pub const DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureRepeat {
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
impl crate::obj::IndexEnum for DefaultCanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for DefaultCanvasItemTextureRepeat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DefaultCanvasItemTextureRepeat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DefaultCanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SDFOversize {
    ord: i32
}
impl SDFOversize {
    pub const SDF_OVERSIZE_100_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const SDF_OVERSIZE_120_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const SDF_OVERSIZE_150_PERCENT: Self = Self {
        ord: 2i32
    };
    pub const SDF_OVERSIZE_200_PERCENT: Self = Self {
        ord: 3i32
    };
    pub const SDF_OVERSIZE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SDFOversize {
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
impl crate::obj::IndexEnum for SDFOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for SDFOversize {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SDFOversize {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SDFOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SDFScale {
    ord: i32
}
impl SDFScale {
    pub const SDF_SCALE_100_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const SDF_SCALE_50_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const SDF_SCALE_25_PERCENT: Self = Self {
        ord: 2i32
    };
    pub const SDF_SCALE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for SDFScale {
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
impl crate::obj::IndexEnum for SDFScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for SDFScale {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SDFScale {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SDFScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRSMode {
    ord: i32
}
impl VRSMode {
    pub const VRS_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VRS_TEXTURE: Self = Self {
        ord: 1i32
    };
    pub const VRS_XR: Self = Self {
        ord: 2i32
    };
    pub const VRS_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for VRSMode {
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
impl crate::obj::IndexEnum for VRSMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for VRSMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VRSMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VRSMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}