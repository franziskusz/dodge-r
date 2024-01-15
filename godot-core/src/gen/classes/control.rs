#![doc = "Sidecar module for class [`Control`][crate::engine::Control].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Control` enums](https://docs.godotengine.org/en/stable/classes/class_control.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Control.`\n\nInherits [`CanvasItem`][crate::engine::CanvasItem].\n\nRelated symbols:\n\n* [`control`][crate::engine::control]: sidecar module with related enum/flag types\n* [`IControl`][crate::engine::IControl]: virtual methods\n* [`ControlNotification`][crate::engine::notify::ControlNotification]: notification type\n\n\nSee also [Godot docs for `Control`](https://docs.godotengine.org/en/stable/classes/class_control.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Control {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Control`][crate::engine::Control].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Control` methods](https://docs.godotengine.org/en/stable/classes/class_control.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IControl: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::engine::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    #[doc = "Notification type for class [`Control`][crate::engine::Control]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum ControlNotification {
        Resized = 40i32, MouseEnter = 41i32, MouseExit = 42i32, MouseEnterSelf = 60i32, MouseExitSelf = 61i32, FocusEnter = 43i32, FocusExit = 44i32, ThemeChanged = 45i32, ScrollBegin = 47i32, ScrollEnd = 48i32, LayoutDirectionChanged = 49i32, TransformChanged = 2000i32, LocalTransformChanged = 35i32, DrawOrNodeRecacheRequested = 30i32, VisibilityChangedOrNodeRecacheRequested = 31i32, EnterCanvas = 32i32, ExitCanvas = 33i32, World2dChanged = 36i32, EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for ControlNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                40i32 => Self::Resized, 41i32 => Self::MouseEnter, 42i32 => Self::MouseExit, 60i32 => Self::MouseEnterSelf, 61i32 => Self::MouseExitSelf, 43i32 => Self::FocusEnter, 44i32 => Self::FocusExit, 45i32 => Self::ThemeChanged, 47i32 => Self::ScrollBegin, 48i32 => Self::ScrollEnd, 49i32 => Self::LayoutDirectionChanged, 2000i32 => Self::TransformChanged, 35i32 => Self::LocalTransformChanged, 30i32 => Self::DrawOrNodeRecacheRequested, 31i32 => Self::VisibilityChangedOrNodeRecacheRequested, 32i32 => Self::EnterCanvas, 33i32 => Self::ExitCanvas, 36i32 => Self::World2dChanged, 10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ControlNotification > for i32 {
        fn from(notification: ControlNotification) -> i32 {
            match notification {
                ControlNotification::Resized => 40i32, ControlNotification::MouseEnter => 41i32, ControlNotification::MouseExit => 42i32, ControlNotification::MouseEnterSelf => 60i32, ControlNotification::MouseExitSelf => 61i32, ControlNotification::FocusEnter => 43i32, ControlNotification::FocusExit => 44i32, ControlNotification::ThemeChanged => 45i32, ControlNotification::ScrollBegin => 47i32, ControlNotification::ScrollEnd => 48i32, ControlNotification::LayoutDirectionChanged => 49i32, ControlNotification::TransformChanged => 2000i32, ControlNotification::LocalTransformChanged => 35i32, ControlNotification::DrawOrNodeRecacheRequested => 30i32, ControlNotification::VisibilityChangedOrNodeRecacheRequested => 31i32, ControlNotification::EnterCanvas => 32i32, ControlNotification::ExitCanvas => 33i32, ControlNotification::World2dChanged => 36i32, ControlNotification::EnterTree => 10i32, ControlNotification::ExitTree => 11i32, ControlNotification::MovedInParent => 12i32, ControlNotification::Ready => 13i32, ControlNotification::Paused => 14i32, ControlNotification::Unpaused => 15i32, ControlNotification::PhysicsProcess => 16i32, ControlNotification::Process => 17i32, ControlNotification::Parented => 18i32, ControlNotification::Unparented => 19i32, ControlNotification::SceneInstantiated => 20i32, ControlNotification::DragBegin => 21i32, ControlNotification::DragEnd => 22i32, ControlNotification::PathRenamed => 23i32, ControlNotification::ChildOrderChanged => 24i32, ControlNotification::InternalProcess => 25i32, ControlNotification::InternalPhysicsProcess => 26i32, ControlNotification::PostEnterTree => 27i32, ControlNotification::Disabled => 28i32, ControlNotification::Enabled => 29i32, ControlNotification::EditorPreSave => 9001i32, ControlNotification::EditorPostSave => 9002i32, ControlNotification::WmMouseEnter => 1002i32, ControlNotification::WmMouseExit => 1003i32, ControlNotification::WmWindowFocusIn => 1004i32, ControlNotification::WmWindowFocusOut => 1005i32, ControlNotification::WmCloseRequest => 1006i32, ControlNotification::WmGoBackRequest => 1007i32, ControlNotification::WmSizeChanged => 1008i32, ControlNotification::WmDpiChange => 1009i32, ControlNotification::VpMouseEnter => 1010i32, ControlNotification::VpMouseExit => 1011i32, ControlNotification::OsMemoryWarning => 2009i32, ControlNotification::TranslationChanged => 2010i32, ControlNotification::WmAbout => 2011i32, ControlNotification::Crash => 2012i32, ControlNotification::OsImeUpdate => 2013i32, ControlNotification::ApplicationResumed => 2014i32, ControlNotification::ApplicationPaused => 2015i32, ControlNotification::ApplicationFocusIn => 2016i32, ControlNotification::ApplicationFocusOut => 2017i32, ControlNotification::TextServerChanged => 2018i32, ControlNotification::Postinitialize => 0i32, ControlNotification::Predelete => 1i32, ControlNotification::Unknown(int) => int,
            }
        }
    }
    impl Control {
        pub fn accept_event(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "accept_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimum_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_combined_minimum_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_combined_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_anchors_preset_full(&mut self, preset: crate::engine::control::LayoutPreset, keep_offsets: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::LayoutPreset, bool);
            let args = (preset, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anchors_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_anchors_preset(&mut self, preset: crate::engine::control::LayoutPreset,) {
            self.set_anchors_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_anchors_preset_ex(&mut self, preset: crate::engine::control::LayoutPreset,) -> ExSetAnchorsPreset < '_ > {
            ExSetAnchorsPreset::new(self, preset,)
        }
        pub(crate) fn set_offsets_preset_full(&mut self, preset: crate::engine::control::LayoutPreset, resize_mode: crate::engine::control::LayoutPresetMode, margin: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::LayoutPreset, crate::engine::control::LayoutPresetMode, i32);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offsets_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_offsets_preset(&mut self, preset: crate::engine::control::LayoutPreset,) {
            self.set_offsets_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_offsets_preset_ex(&mut self, preset: crate::engine::control::LayoutPreset,) -> ExSetOffsetsPreset < '_ > {
            ExSetOffsetsPreset::new(self, preset,)
        }
        pub(crate) fn set_anchors_and_offsets_preset_full(&mut self, preset: crate::engine::control::LayoutPreset, resize_mode: crate::engine::control::LayoutPresetMode, margin: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::LayoutPreset, crate::engine::control::LayoutPresetMode, i32);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anchors_and_offsets_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_anchors_and_offsets_preset(&mut self, preset: crate::engine::control::LayoutPreset,) {
            self.set_anchors_and_offsets_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_anchors_and_offsets_preset_ex(&mut self, preset: crate::engine::control::LayoutPreset,) -> ExSetAnchorsAndOffsetsPreset < '_ > {
            ExSetAnchorsAndOffsetsPreset::new(self, preset,)
        }
        pub(crate) fn set_anchor_full(&mut self, side: crate::engine::global::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, f32, bool, bool);
            let args = (side, anchor, keep_offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anchor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_anchor(&mut self, side: crate::engine::global::Side, anchor: f32,) {
            self.set_anchor_ex(side, anchor,) . done()
        }
        #[inline]
        pub fn set_anchor_ex(&mut self, side: crate::engine::global::Side, anchor: f32,) -> ExSetAnchor < '_ > {
            ExSetAnchor::new(self, side, anchor,)
        }
        pub fn get_anchor(&self, side: crate::engine::global::Side,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_anchor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, side: crate::engine::global::Side, offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, f32);
            let args = (side, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self, offset: crate::engine::global::Side,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::global::Side);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_anchor_and_offset_full(&mut self, side: crate::engine::global::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, f32, f32, bool);
            let args = (side, anchor, offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anchor_and_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_anchor_and_offset(&mut self, side: crate::engine::global::Side, anchor: f32, offset: f32,) {
            self.set_anchor_and_offset_ex(side, anchor, offset,) . done()
        }
        #[inline]
        pub fn set_anchor_and_offset_ex(&mut self, side: crate::engine::global::Side, anchor: f32, offset: f32,) -> ExSetAnchorAndOffset < '_ > {
            ExSetAnchorAndOffset::new(self, side, anchor, offset,)
        }
        pub fn set_begin(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_end(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, bool);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_position(&mut self, position: Vector2,) {
            self.set_position_ex(position,) . done()
        }
        #[inline]
        pub fn set_position_ex(&mut self, position: Vector2,) -> ExSetPosition < '_ > {
            ExSetPosition::new(self, position,)
        }
        pub(crate) fn set_size_full(&mut self, size: Vector2, keep_offsets: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, bool);
            let args = (size, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_size(&mut self, size: Vector2,) {
            self.set_size_ex(size,) . done()
        }
        #[inline]
        pub fn set_size_ex(&mut self, size: Vector2,) -> ExSetSize < '_ > {
            ExSetSize::new(self, size,)
        }
        pub fn reset_size(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reset_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_minimum_size(&mut self, size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_global_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, bool);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_global_position(&mut self, position: Vector2,) {
            self.set_global_position_ex(position,) . done()
        }
        #[inline]
        pub fn set_global_position_ex(&mut self, position: Vector2,) -> ExSetGlobalPosition < '_ > {
            ExSetGlobalPosition::new(self, position,)
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pivot_offset(&mut self, pivot_offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (pivot_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pivot_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_begin(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_end(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pivot_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pivot_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_minimum_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_area_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent_area_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_mode(&mut self, mode: crate::engine::control::FocusMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::FocusMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_mode(&self,) -> crate::engine::control::FocusMode {
            type RetMarshal = PtrcallReturnT < crate::engine::control::FocusMode >;
            type CallSig = (crate::engine::control::FocusMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_focus(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grab_focus(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "grab_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn release_focus(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "release_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_prev_valid_focus(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_prev_valid_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_next_valid_focus(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_next_valid_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_valid_focus_neighbor(&self, side: crate::engine::global::Side,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >, crate::engine::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_valid_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_size_flags(&mut self, flags: crate::engine::control::SizeFlags,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::SizeFlags);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_size_flags(&self,) -> crate::engine::control::SizeFlags {
            type RetMarshal = PtrcallReturnT < crate::engine::control::SizeFlags >;
            type CallSig = (crate::engine::control::SizeFlags,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stretch_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stretch_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_size_flags(&mut self, flags: crate::engine::control::SizeFlags,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::SizeFlags);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_size_flags(&self,) -> crate::engine::control::SizeFlags {
            type RetMarshal = PtrcallReturnT < crate::engine::control::SizeFlags >;
            type CallSig = (crate::engine::control::SizeFlags,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme(&mut self, theme: Gd < crate::engine::Theme >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Theme >);
            let args = (theme,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme(&self,) -> Option < Gd < crate::engine::Theme > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Theme > >;
            type CallSig = (Option < Gd < crate::engine::Theme > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_type_variation(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin_bulk_theme_override(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "begin_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_bulk_theme_override(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "end_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_icon_override(&mut self, name: StringName, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Texture2D >);
            let args = (name, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_stylebox_override(&mut self, name: StringName, stylebox: Gd < crate::engine::StyleBox >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::StyleBox >);
            let args = (name, stylebox,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_override(&mut self, name: StringName, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Font >);
            let args = (name, font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_size_override(&mut self, name: StringName, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (name, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_color_override(&mut self, name: StringName, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Color);
            let args = (name, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_constant_override(&mut self, name: StringName, constant: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (name, constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_icon_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_stylebox_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_size_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_color_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_constant_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_theme_icon_full(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_icon(&self, name: StringName,) -> Option < Gd < crate::engine::Texture2D > > {
            self.get_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_icon_ex(&self, name: StringName,) -> ExGetThemeIcon < '_ > {
            ExGetThemeIcon::new(self, name,)
        }
        pub(crate) fn get_theme_stylebox_full(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::StyleBox > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::StyleBox > >;
            type CallSig = (Option < Gd < crate::engine::StyleBox > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_stylebox(&self, name: StringName,) -> Option < Gd < crate::engine::StyleBox > > {
            self.get_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_stylebox_ex(&self, name: StringName,) -> ExGetThemeStylebox < '_ > {
            ExGetThemeStylebox::new(self, name,)
        }
        pub(crate) fn get_theme_font_full(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_font(&self, name: StringName,) -> Option < Gd < crate::engine::Font > > {
            self.get_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_ex(&self, name: StringName,) -> ExGetThemeFont < '_ > {
            ExGetThemeFont::new(self, name,)
        }
        pub(crate) fn get_theme_font_size_full(&self, name: StringName, theme_type: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_font_size(&self, name: StringName,) -> i32 {
            self.get_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_size_ex(&self, name: StringName,) -> ExGetThemeFontSize < '_ > {
            ExGetThemeFontSize::new(self, name,)
        }
        pub(crate) fn get_theme_color_full(&self, name: StringName, theme_type: StringName,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_color(&self, name: StringName,) -> Color {
            self.get_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_color_ex(&self, name: StringName,) -> ExGetThemeColor < '_ > {
            ExGetThemeColor::new(self, name,)
        }
        pub(crate) fn get_theme_constant_full(&self, name: StringName, theme_type: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_theme_constant(&self, name: StringName,) -> i32 {
            self.get_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_constant_ex(&self, name: StringName,) -> ExGetThemeConstant < '_ > {
            ExGetThemeConstant::new(self, name,)
        }
        pub fn has_theme_icon_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_stylebox_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_size_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_color_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_constant_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn has_theme_icon_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_icon(&self, name: StringName,) -> bool {
            self.has_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_icon_ex(&self, name: StringName,) -> ExHasThemeIcon < '_ > {
            ExHasThemeIcon::new(self, name,)
        }
        pub(crate) fn has_theme_stylebox_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_stylebox(&self, name: StringName,) -> bool {
            self.has_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_stylebox_ex(&self, name: StringName,) -> ExHasThemeStylebox < '_ > {
            ExHasThemeStylebox::new(self, name,)
        }
        pub(crate) fn has_theme_font_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_font(&self, name: StringName,) -> bool {
            self.has_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_ex(&self, name: StringName,) -> ExHasThemeFont < '_ > {
            ExHasThemeFont::new(self, name,)
        }
        pub(crate) fn has_theme_font_size_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_font_size(&self, name: StringName,) -> bool {
            self.has_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_size_ex(&self, name: StringName,) -> ExHasThemeFontSize < '_ > {
            ExHasThemeFontSize::new(self, name,)
        }
        pub(crate) fn has_theme_color_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_color(&self, name: StringName,) -> bool {
            self.has_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_color_ex(&self, name: StringName,) -> ExHasThemeColor < '_ > {
            ExHasThemeColor::new(self, name,)
        }
        pub(crate) fn has_theme_constant_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn has_theme_constant(&self, name: StringName,) -> bool {
            self.has_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_constant_ex(&self, name: StringName,) -> ExHasThemeConstant < '_ > {
            ExHasThemeConstant::new(self, name,)
        }
        pub fn get_theme_default_base_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font(&self,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_control(&self,) -> Option < Gd < crate::engine::Control > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Control > >;
            type CallSig = (Option < Gd < crate::engine::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_grow_direction(&mut self, direction: crate::engine::control::GrowDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::GrowDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_h_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_grow_direction(&self,) -> crate::engine::control::GrowDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::GrowDirection >;
            type CallSig = (crate::engine::control::GrowDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_h_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_grow_direction(&mut self, direction: crate::engine::control::GrowDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::GrowDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_v_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_grow_direction(&self,) -> crate::engine::control::GrowDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::GrowDirection >;
            type CallSig = (crate::engine::control::GrowDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_v_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, hint: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tooltip_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_tooltip_full(&self, at_position: Vector2,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Vector2);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_tooltip(&self,) -> GString {
            self.get_tooltip_ex() . done()
        }
        #[inline]
        pub fn get_tooltip_ex(&self,) -> ExGetTooltip < '_ > {
            ExGetTooltip::new(self,)
        }
        pub fn set_default_cursor_shape(&mut self, shape: crate::engine::control::CursorShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_cursor_shape(&self,) -> crate::engine::control::CursorShape {
            type RetMarshal = PtrcallReturnT < crate::engine::control::CursorShape >;
            type CallSig = (crate::engine::control::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_cursor_shape_full(&self, position: Vector2,) -> crate::engine::control::CursorShape {
            type RetMarshal = PtrcallReturnT < crate::engine::control::CursorShape >;
            type CallSig = (crate::engine::control::CursorShape, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_cursor_shape(&self,) -> crate::engine::control::CursorShape {
            self.get_cursor_shape_ex() . done()
        }
        #[inline]
        pub fn get_cursor_shape_ex(&self,) -> ExGetCursorShape < '_ > {
            ExGetCursorShape::new(self,)
        }
        pub fn set_focus_neighbor(&mut self, side: crate::engine::global::Side, neighbor: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Side, NodePath);
            let args = (side, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_neighbor(&self, side: crate::engine::global::Side,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath, crate::engine::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_next(&mut self, next: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focus_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_next(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focus_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_previous(&mut self, previous: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (previous,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_focus_previous", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_previous(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_focus_previous", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_drag(&mut self, data: Variant, preview: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant, Gd < crate::engine::Control >);
            let args = (data, preview,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_drag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mouse_filter(&mut self, filter: crate::engine::control::MouseFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::MouseFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mouse_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_filter(&self,) -> crate::engine::control::MouseFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::control::MouseFilter >;
            type CallSig = (crate::engine::control::MouseFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mouse_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_pass_scroll_events(&mut self, force_pass_scroll_events: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (force_pass_scroll_events,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_force_pass_scroll_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_force_pass_scroll_events(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_force_pass_scroll_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_contents(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clip_contents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_clipping_contents(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_clipping_contents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grab_click_focus(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "grab_click_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_forwarding(&mut self, drag_func: Callable, can_drop_func: Callable, drop_func: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable, Callable, Callable);
            let args = (drag_func, can_drop_func, drop_func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_forwarding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_preview(&mut self, control: Gd < crate::engine::Control >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Control >);
            let args = (control,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_drag_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_successful(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_drag_successful", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_context(&mut self, node: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shortcut_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shortcut_context(&self,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shortcut_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_minimum_size(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layout_direction(&mut self, direction: crate::engine::control::LayoutDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::control::LayoutDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layout_direction(&self,) -> crate::engine::control::LayoutDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::control::LayoutDirection >;
            type CallSig = (crate::engine::control::LayoutDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layout_rtl(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_layout_rtl", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_translating(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_auto_translating", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_localize_numeral_system(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_localize_numeral_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_localizing_numeral_system(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_localizing_numeral_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ControlNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ControlNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_RESIZED: i32 = 40i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER: i32 = 41i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT: i32 = 42i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER_SELF: i32 = 60i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT_SELF: i32 = 61i32;
        pub(crate) const NOTIFICATION_FOCUS_ENTER: i32 = 43i32;
        pub(crate) const NOTIFICATION_FOCUS_EXIT: i32 = 44i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 45i32;
        pub(crate) const NOTIFICATION_SCROLL_BEGIN: i32 = 47i32;
        pub(crate) const NOTIFICATION_SCROLL_END: i32 = 48i32;
        pub(crate) const NOTIFICATION_LAYOUT_DIRECTION_CHANGED: i32 = 49i32;
        
    }
    impl crate::obj::GodotClass for Control {
        type Base = crate::engine::CanvasItem;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Control\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Control {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Control {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Control {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Control {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Control {
        
    }
    impl crate::obj::ExportableObject for Control {
        
    }
    impl crate::obj::cap::GodotDefault for Control {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Control {
        type Target = crate::engine::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Control {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Control {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Control > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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
#[doc = "Default-param extender for [`Control::set_anchors_preset_ex`][super::Control::set_anchors_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsPreset < 'a > {
    surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset,) -> Self {
        Self {
            surround_object, preset, keep_offsets: false,
        }
    }
    #[inline]
    pub fn keep_offsets(self, value: bool) -> Self {
        Self {
            keep_offsets: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_anchors_preset_full(self.surround_object, self.preset, self.keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_offsets_preset_ex`][super::Control::set_offsets_preset_ex]."]
#[must_use]
pub struct ExSetOffsetsPreset < 'a > {
    surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset, resize_mode: crate::engine::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetOffsetsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset,) -> Self {
        Self {
            surround_object, preset, resize_mode: crate::obj::EngineEnum::from_ord(0), margin: 0i32,
        }
    }
    #[inline]
    pub fn resize_mode(self, value: crate::engine::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: value, .. self
        }
    }
    #[inline]
    pub fn margin(self, value: i32) -> Self {
        Self {
            margin: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_offsets_preset_full(self.surround_object, self.preset, self.resize_mode, self.margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchors_and_offsets_preset_ex`][super::Control::set_anchors_and_offsets_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsAndOffsetsPreset < 'a > {
    surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset, resize_mode: crate::engine::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorsAndOffsetsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::engine::control::LayoutPreset,) -> Self {
        Self {
            surround_object, preset, resize_mode: crate::obj::EngineEnum::from_ord(0), margin: 0i32,
        }
    }
    #[inline]
    pub fn resize_mode(self, value: crate::engine::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: value, .. self
        }
    }
    #[inline]
    pub fn margin(self, value: i32) -> Self {
        Self {
            margin: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_anchors_and_offsets_preset_full(self.surround_object, self.preset, self.resize_mode, self.margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_ex`][super::Control::set_anchor_ex]."]
#[must_use]
pub struct ExSetAnchor < 'a > {
    surround_object: &'a mut re_export::Control, side: crate::engine::global::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchor < 'a > {
    fn new(surround_object: &'a mut re_export::Control, side: crate::engine::global::Side, anchor: f32,) -> Self {
        Self {
            surround_object, side, anchor, keep_offset: false, push_opposite_anchor: true,
        }
    }
    #[inline]
    pub fn keep_offset(self, value: bool) -> Self {
        Self {
            keep_offset: value, .. self
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, value: bool) -> Self {
        Self {
            push_opposite_anchor: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_anchor_full(self.surround_object, self.side, self.anchor, self.keep_offset, self.push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_and_offset_ex`][super::Control::set_anchor_and_offset_ex]."]
#[must_use]
pub struct ExSetAnchorAndOffset < 'a > {
    surround_object: &'a mut re_export::Control, side: crate::engine::global::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorAndOffset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, side: crate::engine::global::Side, anchor: f32, offset: f32,) -> Self {
        Self {
            surround_object, side, anchor, offset, push_opposite_anchor: false,
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, value: bool) -> Self {
        Self {
            push_opposite_anchor: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_anchor_and_offset_full(self.surround_object, self.side, self.anchor, self.offset, self.push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_position_ex`][super::Control::set_position_ex]."]
#[must_use]
pub struct ExSetPosition < 'a > {
    surround_object: &'a mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Control, position: Vector2,) -> Self {
        Self {
            surround_object, position, keep_offsets: false,
        }
    }
    #[inline]
    pub fn keep_offsets(self, value: bool) -> Self {
        Self {
            keep_offsets: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_position_full(self.surround_object, self.position, self.keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_size_ex`][super::Control::set_size_ex]."]
#[must_use]
pub struct ExSetSize < 'a > {
    surround_object: &'a mut re_export::Control, size: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetSize < 'a > {
    fn new(surround_object: &'a mut re_export::Control, size: Vector2,) -> Self {
        Self {
            surround_object, size, keep_offsets: false,
        }
    }
    #[inline]
    pub fn keep_offsets(self, value: bool) -> Self {
        Self {
            keep_offsets: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_size_full(self.surround_object, self.size, self.keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_global_position_ex`][super::Control::set_global_position_ex]."]
#[must_use]
pub struct ExSetGlobalPosition < 'a > {
    surround_object: &'a mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetGlobalPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Control, position: Vector2,) -> Self {
        Self {
            surround_object, position, keep_offsets: false,
        }
    }
    #[inline]
    pub fn keep_offsets(self, value: bool) -> Self {
        Self {
            keep_offsets: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Control::set_global_position_full(self.surround_object, self.position, self.keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_icon_ex`][super::Control::get_theme_icon_ex]."]
#[must_use]
pub struct ExGetThemeIcon < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Texture2D > > {
        re_export::Control::get_theme_icon_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_stylebox_ex`][super::Control::get_theme_stylebox_ex]."]
#[must_use]
pub struct ExGetThemeStylebox < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::StyleBox > > {
        re_export::Control::get_theme_stylebox_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_ex`][super::Control::get_theme_font_ex]."]
#[must_use]
pub struct ExGetThemeFont < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Font > > {
        re_export::Control::get_theme_font_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_size_ex`][super::Control::get_theme_font_size_ex]."]
#[must_use]
pub struct ExGetThemeFontSize < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Control::get_theme_font_size_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_color_ex`][super::Control::get_theme_color_ex]."]
#[must_use]
pub struct ExGetThemeColor < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Color {
        re_export::Control::get_theme_color_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_constant_ex`][super::Control::get_theme_constant_ex]."]
#[must_use]
pub struct ExGetThemeConstant < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Control::get_theme_constant_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_icon_ex`][super::Control::has_theme_icon_ex]."]
#[must_use]
pub struct ExHasThemeIcon < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_icon_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_stylebox_ex`][super::Control::has_theme_stylebox_ex]."]
#[must_use]
pub struct ExHasThemeStylebox < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_stylebox_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_ex`][super::Control::has_theme_font_ex]."]
#[must_use]
pub struct ExHasThemeFont < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_font_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_size_ex`][super::Control::has_theme_font_size_ex]."]
#[must_use]
pub struct ExHasThemeFontSize < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_font_size_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_color_ex`][super::Control::has_theme_color_ex]."]
#[must_use]
pub struct ExHasThemeColor < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_color_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_constant_ex`][super::Control::has_theme_constant_ex]."]
#[must_use]
pub struct ExHasThemeConstant < 'a > {
    surround_object: &'a re_export::Control, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Control, name: StringName,) -> Self {
        Self {
            surround_object, name, theme_type: StringName::from(""),
        }
    }
    #[inline]
    pub fn theme_type(self, value: StringName) -> Self {
        Self {
            theme_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Control::has_theme_constant_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_tooltip_ex`][super::Control::get_tooltip_ex]."]
#[must_use]
pub struct ExGetTooltip < 'a > {
    surround_object: &'a re_export::Control, at_position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTooltip < 'a > {
    fn new(surround_object: &'a re_export::Control,) -> Self {
        Self {
            surround_object, at_position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn at_position(self, value: Vector2) -> Self {
        Self {
            at_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Control::get_tooltip_full(self.surround_object, self.at_position,)
    }
}
#[doc = "Default-param extender for [`Control::get_cursor_shape_ex`][super::Control::get_cursor_shape_ex]."]
#[must_use]
pub struct ExGetCursorShape < 'a > {
    surround_object: &'a re_export::Control, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCursorShape < 'a > {
    fn new(surround_object: &'a re_export::Control,) -> Self {
        Self {
            surround_object, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::control::CursorShape {
        re_export::Control::get_cursor_shape_full(self.surround_object, self.position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FocusMode {
    ord: i32
}
impl FocusMode {
    pub const FOCUS_NONE: Self = Self {
        ord: 0i32
    };
    pub const FOCUS_CLICK: Self = Self {
        ord: 1i32
    };
    pub const FOCUS_ALL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for FocusMode {
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
impl crate::builtin::meta::GodotConvert for FocusMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FocusMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FocusMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    pub const CURSOR_ARROW: Self = Self {
        ord: 0i32
    };
    pub const CURSOR_IBEAM: Self = Self {
        ord: 1i32
    };
    pub const CURSOR_POINTING_HAND: Self = Self {
        ord: 2i32
    };
    pub const CURSOR_CROSS: Self = Self {
        ord: 3i32
    };
    pub const CURSOR_WAIT: Self = Self {
        ord: 4i32
    };
    pub const CURSOR_BUSY: Self = Self {
        ord: 5i32
    };
    pub const CURSOR_DRAG: Self = Self {
        ord: 6i32
    };
    pub const CURSOR_CAN_DROP: Self = Self {
        ord: 7i32
    };
    pub const CURSOR_FORBIDDEN: Self = Self {
        ord: 8i32
    };
    pub const CURSOR_VSIZE: Self = Self {
        ord: 9i32
    };
    pub const CURSOR_HSIZE: Self = Self {
        ord: 10i32
    };
    pub const CURSOR_BDIAGSIZE: Self = Self {
        ord: 11i32
    };
    pub const CURSOR_FDIAGSIZE: Self = Self {
        ord: 12i32
    };
    pub const CURSOR_MOVE: Self = Self {
        ord: 13i32
    };
    pub const CURSOR_VSPLIT: Self = Self {
        ord: 14i32
    };
    pub const CURSOR_HSPLIT: Self = Self {
        ord: 15i32
    };
    pub const CURSOR_HELP: Self = Self {
        ord: 16i32
    };
    
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CursorShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LayoutPreset {
    ord: i32
}
impl LayoutPreset {
    pub const PRESET_TOP_LEFT: Self = Self {
        ord: 0i32
    };
    pub const PRESET_TOP_RIGHT: Self = Self {
        ord: 1i32
    };
    pub const PRESET_BOTTOM_LEFT: Self = Self {
        ord: 2i32
    };
    pub const PRESET_BOTTOM_RIGHT: Self = Self {
        ord: 3i32
    };
    pub const PRESET_CENTER_LEFT: Self = Self {
        ord: 4i32
    };
    pub const PRESET_CENTER_TOP: Self = Self {
        ord: 5i32
    };
    pub const PRESET_CENTER_RIGHT: Self = Self {
        ord: 6i32
    };
    pub const PRESET_CENTER_BOTTOM: Self = Self {
        ord: 7i32
    };
    pub const PRESET_CENTER: Self = Self {
        ord: 8i32
    };
    pub const PRESET_LEFT_WIDE: Self = Self {
        ord: 9i32
    };
    pub const PRESET_TOP_WIDE: Self = Self {
        ord: 10i32
    };
    pub const PRESET_RIGHT_WIDE: Self = Self {
        ord: 11i32
    };
    pub const PRESET_BOTTOM_WIDE: Self = Self {
        ord: 12i32
    };
    pub const PRESET_VCENTER_WIDE: Self = Self {
        ord: 13i32
    };
    pub const PRESET_HCENTER_WIDE: Self = Self {
        ord: 14i32
    };
    pub const PRESET_FULL_RECT: Self = Self {
        ord: 15i32
    };
    
}
impl crate::obj::EngineEnum for LayoutPreset {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for LayoutPreset {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LayoutPreset {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LayoutPreset {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LayoutPresetMode {
    ord: i32
}
impl LayoutPresetMode {
    pub const PRESET_MODE_MINSIZE: Self = Self {
        ord: 0i32
    };
    pub const PRESET_MODE_KEEP_WIDTH: Self = Self {
        ord: 1i32
    };
    pub const PRESET_MODE_KEEP_HEIGHT: Self = Self {
        ord: 2i32
    };
    pub const PRESET_MODE_KEEP_SIZE: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for LayoutPresetMode {
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
impl crate::builtin::meta::GodotConvert for LayoutPresetMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LayoutPresetMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LayoutPresetMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct SizeFlags {
    ord: u64
}
impl SizeFlags {
    pub const SIZE_SHRINK_BEGIN: Self = Self {
        ord: 0u64
    };
    pub const SIZE_FILL: Self = Self {
        ord: 1u64
    };
    pub const SIZE_EXPAND: Self = Self {
        ord: 2u64
    };
    pub const SIZE_EXPAND_FILL: Self = Self {
        ord: 3u64
    };
    pub const SIZE_SHRINK_CENTER: Self = Self {
        ord: 4u64
    };
    pub const SIZE_SHRINK_END: Self = Self {
        ord: 8u64
    };
    
}
impl crate::obj::EngineBitfield for SizeFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for SizeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for SizeFlags {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for SizeFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SizeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseFilter {
    ord: i32
}
impl MouseFilter {
    pub const MOUSE_FILTER_STOP: Self = Self {
        ord: 0i32
    };
    pub const MOUSE_FILTER_PASS: Self = Self {
        ord: 1i32
    };
    pub const MOUSE_FILTER_IGNORE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for MouseFilter {
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
impl crate::builtin::meta::GodotConvert for MouseFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MouseFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MouseFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GrowDirection {
    ord: i32
}
impl GrowDirection {
    pub const GROW_DIRECTION_BEGIN: Self = Self {
        ord: 0i32
    };
    pub const GROW_DIRECTION_END: Self = Self {
        ord: 1i32
    };
    pub const GROW_DIRECTION_BOTH: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for GrowDirection {
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
impl crate::builtin::meta::GodotConvert for GrowDirection {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GrowDirection {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GrowDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Anchor {
    ord: i32
}
impl Anchor {
    pub const ANCHOR_BEGIN: Self = Self {
        ord: 0i32
    };
    pub const ANCHOR_END: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for Anchor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Anchor {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Anchor {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Anchor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LayoutDirection {
    ord: i32
}
impl LayoutDirection {
    pub const LAYOUT_DIRECTION_INHERITED: Self = Self {
        ord: 0i32
    };
    pub const LAYOUT_DIRECTION_LOCALE: Self = Self {
        ord: 1i32
    };
    pub const LAYOUT_DIRECTION_LTR: Self = Self {
        ord: 2i32
    };
    pub const LAYOUT_DIRECTION_RTL: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for LayoutDirection {
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
impl crate::builtin::meta::GodotConvert for LayoutDirection {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LayoutDirection {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LayoutDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextDirection {
    ord: i32
}
impl TextDirection {
    pub const TEXT_DIRECTION_INHERITED: Self = Self {
        ord: 3i32
    };
    pub const TEXT_DIRECTION_AUTO: Self = Self {
        ord: 0i32
    };
    pub const TEXT_DIRECTION_LTR: Self = Self {
        ord: 1i32
    };
    pub const TEXT_DIRECTION_RTL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TextDirection {
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
impl crate::builtin::meta::GodotConvert for TextDirection {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextDirection {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}