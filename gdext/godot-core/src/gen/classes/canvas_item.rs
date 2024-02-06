#![doc = "Sidecar module for class [`CanvasItem`][crate::engine::CanvasItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItem` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CanvasItem.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`canvas_item`][crate::engine::canvas_item]: sidecar module with related enum/flag types\n* [`ICanvasItem`][crate::engine::ICanvasItem]: virtual methods\n* [`CanvasItemNotification`][crate::engine::notify::CanvasItemNotification]: notification type\n\n\nSee also [Godot docs for `CanvasItem`](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CanvasItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CanvasItem`][crate::engine::CanvasItem].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CanvasItem` methods](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICanvasItem: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    #[doc = "Notification type for class [`CanvasItem`][crate::engine::CanvasItem]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum CanvasItemNotification {
        TransformChanged = 2000i32, LocalTransformChanged = 35i32, DrawOrNodeRecacheRequested = 30i32, VisibilityChangedOrNodeRecacheRequested = 31i32, EnterCanvas = 32i32, ExitCanvas = 33i32, World2dChanged = 36i32, EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for CanvasItemNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TransformChanged, 35i32 => Self::LocalTransformChanged, 30i32 => Self::DrawOrNodeRecacheRequested, 31i32 => Self::VisibilityChangedOrNodeRecacheRequested, 32i32 => Self::EnterCanvas, 33i32 => Self::ExitCanvas, 36i32 => Self::World2dChanged, 10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < CanvasItemNotification > for i32 {
        fn from(notification: CanvasItemNotification) -> i32 {
            match notification {
                CanvasItemNotification::TransformChanged => 2000i32, CanvasItemNotification::LocalTransformChanged => 35i32, CanvasItemNotification::DrawOrNodeRecacheRequested => 30i32, CanvasItemNotification::VisibilityChangedOrNodeRecacheRequested => 31i32, CanvasItemNotification::EnterCanvas => 32i32, CanvasItemNotification::ExitCanvas => 33i32, CanvasItemNotification::World2dChanged => 36i32, CanvasItemNotification::EnterTree => 10i32, CanvasItemNotification::ExitTree => 11i32, CanvasItemNotification::MovedInParent => 12i32, CanvasItemNotification::Ready => 13i32, CanvasItemNotification::Paused => 14i32, CanvasItemNotification::Unpaused => 15i32, CanvasItemNotification::PhysicsProcess => 16i32, CanvasItemNotification::Process => 17i32, CanvasItemNotification::Parented => 18i32, CanvasItemNotification::Unparented => 19i32, CanvasItemNotification::SceneInstantiated => 20i32, CanvasItemNotification::DragBegin => 21i32, CanvasItemNotification::DragEnd => 22i32, CanvasItemNotification::PathRenamed => 23i32, CanvasItemNotification::ChildOrderChanged => 24i32, CanvasItemNotification::InternalProcess => 25i32, CanvasItemNotification::InternalPhysicsProcess => 26i32, CanvasItemNotification::PostEnterTree => 27i32, CanvasItemNotification::Disabled => 28i32, CanvasItemNotification::Enabled => 29i32, CanvasItemNotification::EditorPreSave => 9001i32, CanvasItemNotification::EditorPostSave => 9002i32, CanvasItemNotification::WmMouseEnter => 1002i32, CanvasItemNotification::WmMouseExit => 1003i32, CanvasItemNotification::WmWindowFocusIn => 1004i32, CanvasItemNotification::WmWindowFocusOut => 1005i32, CanvasItemNotification::WmCloseRequest => 1006i32, CanvasItemNotification::WmGoBackRequest => 1007i32, CanvasItemNotification::WmSizeChanged => 1008i32, CanvasItemNotification::WmDpiChange => 1009i32, CanvasItemNotification::VpMouseEnter => 1010i32, CanvasItemNotification::VpMouseExit => 1011i32, CanvasItemNotification::OsMemoryWarning => 2009i32, CanvasItemNotification::TranslationChanged => 2010i32, CanvasItemNotification::WmAbout => 2011i32, CanvasItemNotification::Crash => 2012i32, CanvasItemNotification::OsImeUpdate => 2013i32, CanvasItemNotification::ApplicationResumed => 2014i32, CanvasItemNotification::ApplicationPaused => 2015i32, CanvasItemNotification::ApplicationFocusIn => 2016i32, CanvasItemNotification::ApplicationFocusOut => 2017i32, CanvasItemNotification::TextServerChanged => 2018i32, CanvasItemNotification::Postinitialize => 0i32, CanvasItemNotification::Predelete => 1i32, CanvasItemNotification::Unknown(int) => int,
            }
        }
    }
    impl CanvasItem {
        pub fn get_canvas_item(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue_redraw(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "queue_redraw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_to_front(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_to_front", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_light_mask(&mut self, light_mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_light_mask(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_self_modulate(&mut self, self_modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (self_modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_self_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_self_modulate(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_self_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_z_index(&mut self, z_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_z_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_z_as_relative(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_z_as_relative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_z_relative(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_z_relative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_y_sort_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_y_sort_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_y_sort_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_y_sort_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_behind_parent(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_draw_behind_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_behind_parent_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_draw_behind_parent_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2, Color, f32, bool);
            let args = (from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_line_ex(from, to, color,) . done()
        }
        #[inline]
        pub fn draw_line_ex(&mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawLine < '_ > {
            ExDrawLine::new(self, from, to, color,)
        }
        pub(crate) fn draw_dashed_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2, Color, f32, f32, bool);
            let args = (from, to, color, width, dash, aligned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_dashed_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_dashed_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_dashed_line_ex(from, to, color,) . done()
        }
        #[inline]
        pub fn draw_dashed_line_ex(&mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawDashedLine < '_ > {
            ExDrawDashedLine::new(self, from, to, color,)
        }
        pub(crate) fn draw_polyline_full(&mut self, points: PackedVector2Array, color: Color, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, Color, f32, bool);
            let args = (points, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_polyline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_polyline(&mut self, points: PackedVector2Array, color: Color,) {
            self.draw_polyline_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_polyline_ex(&mut self, points: PackedVector2Array, color: Color,) -> ExDrawPolyline < '_ > {
            ExDrawPolyline::new(self, points, color,)
        }
        pub(crate) fn draw_polyline_colors_full(&mut self, points: PackedVector2Array, colors: PackedColorArray, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, PackedColorArray, f32, bool);
            let args = (points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_polyline_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_polyline_colors(&mut self, points: PackedVector2Array, colors: PackedColorArray,) {
            self.draw_polyline_colors_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_polyline_colors_ex(&mut self, points: PackedVector2Array, colors: PackedColorArray,) -> ExDrawPolylineColors < '_ > {
            ExDrawPolylineColors::new(self, points, colors,)
        }
        pub(crate) fn draw_arc_full(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, f32, f32, f32, i32, Color, f32, bool);
            let args = (center, radius, start_angle, end_angle, point_count, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_arc", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_arc(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) {
            self.draw_arc_ex(center, radius, start_angle, end_angle, point_count, color,) . done()
        }
        #[inline]
        pub fn draw_arc_ex(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> ExDrawArc < '_ > {
            ExDrawArc::new(self, center, radius, start_angle, end_angle, point_count, color,)
        }
        pub(crate) fn draw_multiline_full(&mut self, points: PackedVector2Array, color: Color, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, Color, f32);
            let args = (points, color, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline(&mut self, points: PackedVector2Array, color: Color,) {
            self.draw_multiline_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_multiline_ex(&mut self, points: PackedVector2Array, color: Color,) -> ExDrawMultiline < '_ > {
            ExDrawMultiline::new(self, points, color,)
        }
        pub(crate) fn draw_multiline_colors_full(&mut self, points: PackedVector2Array, colors: PackedColorArray, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, PackedColorArray, f32);
            let args = (points, colors, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline_colors(&mut self, points: PackedVector2Array, colors: PackedColorArray,) {
            self.draw_multiline_colors_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_multiline_colors_ex(&mut self, points: PackedVector2Array, colors: PackedColorArray,) -> ExDrawMultilineColors < '_ > {
            ExDrawMultilineColors::new(self, points, colors,)
        }
        pub(crate) fn draw_rect_full(&mut self, rect: Rect2, color: Color, filled: bool, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2, Color, bool, f32);
            let args = (rect, color, filled, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_rect(&mut self, rect: Rect2, color: Color,) {
            self.draw_rect_ex(rect, color,) . done()
        }
        #[inline]
        pub fn draw_rect_ex(&mut self, rect: Rect2, color: Color,) -> ExDrawRect < '_ > {
            ExDrawRect::new(self, rect, color,)
        }
        pub fn draw_circle(&mut self, position: Vector2, radius: f32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, f32, Color);
            let args = (position, radius, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_texture_full(&mut self, texture: Gd < crate::engine::Texture2D >, position: Vector2, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Vector2, Color);
            let args = (texture, position, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_texture(&mut self, texture: Gd < crate::engine::Texture2D >, position: Vector2,) {
            self.draw_texture_ex(texture, position,) . done()
        }
        #[inline]
        pub fn draw_texture_ex(&mut self, texture: Gd < crate::engine::Texture2D >, position: Vector2,) -> ExDrawTexture < '_ > {
            ExDrawTexture::new(self, texture, position,)
        }
        pub(crate) fn draw_texture_rect_full(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Rect2, bool, Color, bool);
            let args = (texture, rect, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_texture_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_texture_rect(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, tile: bool,) {
            self.draw_texture_rect_ex(texture, rect, tile,) . done()
        }
        #[inline]
        pub fn draw_texture_rect_ex(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, tile: bool,) -> ExDrawTextureRect < '_ > {
            ExDrawTextureRect::new(self, texture, rect, tile,)
        }
        pub(crate) fn draw_texture_rect_region_full(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Rect2, Rect2, Color, bool, bool);
            let args = (texture, rect, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_texture_rect_region(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) {
            self.draw_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_texture_rect_region_ex(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> ExDrawTextureRectRegion < '_ > {
            ExDrawTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub(crate) fn draw_msdf_texture_rect_region_full(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Rect2, Rect2, Color, f64, f64, f64);
            let args = (texture, rect, src_rect, modulate, outline, pixel_range, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_msdf_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_msdf_texture_rect_region(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) {
            self.draw_msdf_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_msdf_texture_rect_region_ex(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> ExDrawMsdfTextureRectRegion < '_ > {
            ExDrawMsdfTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub(crate) fn draw_lcd_texture_rect_region_full(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture2D >, Rect2, Rect2, Color);
            let args = (texture, rect, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_lcd_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_lcd_texture_rect_region(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) {
            self.draw_lcd_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_lcd_texture_rect_region_ex(&mut self, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> ExDrawLcdTextureRectRegion < '_ > {
            ExDrawLcdTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub fn draw_style_box(&mut self, style_box: Gd < crate::engine::StyleBox >, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::StyleBox >, Rect2);
            let args = (style_box, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_style_box", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_primitive_full(&mut self, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, PackedColorArray, PackedVector2Array, Gd < crate::engine::Texture2D >);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_primitive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_primitive(&mut self, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array,) {
            self.draw_primitive_ex(points, colors, uvs,) . done()
        }
        #[inline]
        pub fn draw_primitive_ex(&mut self, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array,) -> ExDrawPrimitive < '_ > {
            ExDrawPrimitive::new(self, points, colors, uvs,)
        }
        pub(crate) fn draw_polygon_full(&mut self, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, PackedColorArray, PackedVector2Array, Gd < crate::engine::Texture2D >);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_polygon(&mut self, points: PackedVector2Array, colors: PackedColorArray,) {
            self.draw_polygon_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_polygon_ex(&mut self, points: PackedVector2Array, colors: PackedColorArray,) -> ExDrawPolygon < '_ > {
            ExDrawPolygon::new(self, points, colors,)
        }
        pub(crate) fn draw_colored_polygon_full(&mut self, points: PackedVector2Array, color: Color, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array, Color, PackedVector2Array, Gd < crate::engine::Texture2D >);
            let args = (points, color, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_colored_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_colored_polygon(&mut self, points: PackedVector2Array, color: Color,) {
            self.draw_colored_polygon_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_colored_polygon_ex(&mut self, points: PackedVector2Array, color: Color,) -> ExDrawColoredPolygon < '_ > {
            ExDrawColoredPolygon::new(self, points, color,)
        }
        pub(crate) fn draw_string_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, Color, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_string(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) {
            self.draw_string_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> ExDrawString < '_ > {
            ExDrawString::new(self, font, pos, text,)
        }
        pub(crate) fn draw_multiline_string_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, Color, crate::engine::text_server::LineBreakFlag, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline_string(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) {
            self.draw_multiline_string_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> ExDrawMultilineString < '_ > {
            ExDrawMultilineString::new(self, font, pos, text,)
        }
        pub(crate) fn draw_string_outline_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, Color, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_string_outline(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) {
            self.draw_string_outline_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_outline_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> ExDrawStringOutline < '_ > {
            ExDrawStringOutline::new(self, font, pos, text,)
        }
        pub(crate) fn draw_multiline_string_outline_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, crate::engine::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::engine::text_server::LineBreakFlag, crate::engine::text_server::JustificationFlag, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multiline_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_multiline_string_outline(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) {
            self.draw_multiline_string_outline_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_outline_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> ExDrawMultilineStringOutline < '_ > {
            ExDrawMultilineStringOutline::new(self, font, pos, text,)
        }
        pub(crate) fn draw_char_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString, font_size: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, i32, Color);
            let args = (font, pos, char, font_size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_char(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) {
            self.draw_char_ex(font, pos, char,) . done()
        }
        #[inline]
        pub fn draw_char_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) -> ExDrawChar < '_ > {
            ExDrawChar::new(self, font, pos, char,)
        }
        pub(crate) fn draw_char_outline_full(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString, font_size: i32, size: i32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Font >, Vector2, GString, i32, i32, Color);
            let args = (font, pos, char, font_size, size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_char_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_char_outline(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) {
            self.draw_char_outline_ex(font, pos, char,) . done()
        }
        #[inline]
        pub fn draw_char_outline_ex(&self, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) -> ExDrawCharOutline < '_ > {
            ExDrawCharOutline::new(self, font, pos, char,)
        }
        pub(crate) fn draw_mesh_full(&mut self, mesh: Gd < crate::engine::Mesh >, texture: Gd < crate::engine::Texture2D >, transform: Transform2D, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >, Gd < crate::engine::Texture2D >, Transform2D, Color);
            let args = (mesh, texture, transform, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_mesh(&mut self, mesh: Gd < crate::engine::Mesh >, texture: Gd < crate::engine::Texture2D >,) {
            self.draw_mesh_ex(mesh, texture,) . done()
        }
        #[inline]
        pub fn draw_mesh_ex(&mut self, mesh: Gd < crate::engine::Mesh >, texture: Gd < crate::engine::Texture2D >,) -> ExDrawMesh < '_ > {
            ExDrawMesh::new(self, mesh, texture,)
        }
        pub fn draw_multimesh(&mut self, multimesh: Gd < crate::engine::MultiMesh >, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::MultiMesh >, Gd < crate::engine::Texture2D >);
            let args = (multimesh, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_multimesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_set_transform_full(&mut self, position: Vector2, rotation: f32, scale: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, f32, Vector2);
            let args = (position, rotation, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_set_transform(&mut self, position: Vector2,) {
            self.draw_set_transform_ex(position,) . done()
        }
        #[inline]
        pub fn draw_set_transform_ex(&mut self, position: Vector2,) -> ExDrawSetTransform < '_ > {
            ExDrawSetTransform::new(self, position,)
        }
        pub fn draw_set_transform_matrix(&mut self, xform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_set_transform_matrix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_animation_slice_full(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64, f64, f64, f64);
            let args = (animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_animation_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_animation_slice(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.draw_animation_slice_ex(animation_length, slice_begin, slice_end,) . done()
        }
        #[inline]
        pub fn draw_animation_slice_ex(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExDrawAnimationSlice < '_ > {
            ExDrawAnimationSlice::new(self, animation_length, slice_begin, slice_end,)
        }
        pub fn draw_end_animation(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_end_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_transform_with_canvas(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_transform_with_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_viewport_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport_rect(&self,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_viewport_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_screen_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_mouse_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_local_mouse_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_mouse_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_mouse_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_2d(&self,) -> Option < Gd < crate::engine::World2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::World2D > >;
            type CallSig = (Option < Gd < crate::engine::World2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_parent_material(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_parent_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_parent_material(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_parent_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_notify_local_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_local_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_notify_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_transform(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_update_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_canvas_position_local(&self, screen_point: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_canvas_position_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_input_local(&self, event: Gd < crate::engine::InputEvent >,) -> Option < Gd < crate::engine::InputEvent > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::InputEvent > >;
            type CallSig = (Option < Gd < crate::engine::InputEvent > >, Gd < crate::engine::InputEvent >);
            let args = (event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_input_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_layer(&mut self, layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_layer(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_layer_bit(&mut self, layer: u32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32, bool);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visibility_layer_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_layer_bit(&self, layer: u32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_layer_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::engine::canvas_item::TextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::canvas_item::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::engine::canvas_item::TextureFilter {
            type RetMarshal = PtrcallReturnT < crate::engine::canvas_item::TextureFilter >;
            type CallSig = (crate::engine::canvas_item::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, mode: crate::engine::canvas_item::TextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::canvas_item::TextureRepeat);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::engine::canvas_item::TextureRepeat {
            type RetMarshal = PtrcallReturnT < crate::engine::canvas_item::TextureRepeat >;
            type CallSig = (crate::engine::canvas_item::TextureRepeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_children_mode(&mut self, mode: crate::engine::canvas_item::ClipChildrenMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::canvas_item::ClipChildrenMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_clip_children_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_children_mode(&self,) -> crate::engine::canvas_item::ClipChildrenMode {
            type RetMarshal = PtrcallReturnT < crate::engine::canvas_item::ClipChildrenMode >;
            type CallSig = (crate::engine::canvas_item::ClipChildrenMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_clip_children_mode", self.object_ptr, self.__checked_id(), args,)
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
        pub fn notify(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 35i32;
        pub(crate) const NOTIFICATION_DRAW: i32 = 30i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 31i32;
        pub(crate) const NOTIFICATION_ENTER_CANVAS: i32 = 32i32;
        pub(crate) const NOTIFICATION_EXIT_CANVAS: i32 = 33i32;
        pub(crate) const NOTIFICATION_WORLD_2D_CHANGED: i32 = 36i32;
        
    }
    impl crate::obj::GodotClass for CanvasItem {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CanvasItem\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CanvasItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CanvasItem {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for CanvasItem {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CanvasItem {
        
    }
    impl crate::obj::ExportableObject for CanvasItem {
        
    }
    impl std::ops::Deref for CanvasItem {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CanvasItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CanvasItem {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`CanvasItem::draw_line_ex`][super::CanvasItem::draw_line_ex]."]
#[must_use]
pub struct ExDrawLine < 'a > {
    surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawLine < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        Self {
            surround_object, from, to, color, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_line_full(self.surround_object, self.from, self.to, self.color, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_dashed_line_ex`][super::CanvasItem::draw_dashed_line_ex]."]
#[must_use]
pub struct ExDrawDashedLine < 'a > {
    surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawDashedLine < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        Self {
            surround_object, from, to, color, width: - 1f32, dash: 2f32, aligned: true,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn dash(self, value: f32) -> Self {
        Self {
            dash: value, .. self
        }
    }
    #[inline]
    pub fn aligned(self, value: bool) -> Self {
        Self {
            aligned: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_dashed_line_full(self.surround_object, self.from, self.to, self.color, self.width, self.dash, self.aligned,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_ex`][super::CanvasItem::draw_polyline_ex]."]
#[must_use]
pub struct ExDrawPolyline < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color,) -> Self {
        Self {
            surround_object, points, color, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_polyline_full(self.surround_object, self.points, self.color, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_colors_ex`][super::CanvasItem::draw_polyline_colors_ex]."]
#[must_use]
pub struct ExDrawPolylineColors < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolylineColors < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, points, colors, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_polyline_colors_full(self.surround_object, self.points, self.colors, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_arc_ex`][super::CanvasItem::draw_arc_ex]."]
#[must_use]
pub struct ExDrawArc < 'a > {
    surround_object: &'a mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawArc < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> Self {
        Self {
            surround_object, center, radius, start_angle, end_angle, point_count, color, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_arc_full(self.surround_object, self.center, self.radius, self.start_angle, self.end_angle, self.point_count, self.color, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_ex`][super::CanvasItem::draw_multiline_ex]."]
#[must_use]
pub struct ExDrawMultiline < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color, width: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultiline < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color,) -> Self {
        Self {
            surround_object, points, color, width: - 1f32,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_multiline_full(self.surround_object, self.points, self.color, self.width,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_colors_ex`][super::CanvasItem::draw_multiline_colors_ex]."]
#[must_use]
pub struct ExDrawMultilineColors < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray, width: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineColors < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, points, colors, width: - 1f32,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_multiline_colors_full(self.surround_object, self.points, self.colors, self.width,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_rect_ex`][super::CanvasItem::draw_rect_ex]."]
#[must_use]
pub struct ExDrawRect < 'a > {
    surround_object: &'a mut re_export::CanvasItem, rect: Rect2, color: Color, filled: bool, width: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawRect < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, rect: Rect2, color: Color,) -> Self {
        Self {
            surround_object, rect, color, filled: true, width: - 1f32,
        }
    }
    #[inline]
    pub fn filled(self, value: bool) -> Self {
        Self {
            filled: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_rect_full(self.surround_object, self.rect, self.color, self.filled, self.width,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_ex`][super::CanvasItem::draw_texture_ex]."]
#[must_use]
pub struct ExDrawTexture < 'a > {
    surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, position: Vector2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTexture < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, position: Vector2,) -> Self {
        Self {
            surround_object, texture, position, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_texture_full(self.surround_object, self.texture, self.position, self.modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_ex`][super::CanvasItem::draw_texture_rect_ex]."]
#[must_use]
pub struct ExDrawTextureRect < 'a > {
    surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTextureRect < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, tile: bool,) -> Self {
        Self {
            surround_object, texture, rect, tile, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn transpose(self, value: bool) -> Self {
        Self {
            transpose: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_texture_rect_full(self.surround_object, self.texture, self.rect, self.tile, self.modulate, self.transpose,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_region_ex`][super::CanvasItem::draw_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawTextureRectRegion < 'a > {
    surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> Self {
        Self {
            surround_object, texture, rect, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false, clip_uv: true,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn transpose(self, value: bool) -> Self {
        Self {
            transpose: value, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, value: bool) -> Self {
        Self {
            clip_uv: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_texture_rect_region_full(self.surround_object, self.texture, self.rect, self.src_rect, self.modulate, self.transpose, self.clip_uv,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_msdf_texture_rect_region_ex`][super::CanvasItem::draw_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawMsdfTextureRectRegion < 'a > {
    surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMsdfTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> Self {
        Self {
            surround_object, texture, rect, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), outline: 0f64, pixel_range: 4f64, scale: 1f64,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn outline(self, value: f64) -> Self {
        Self {
            outline: value, .. self
        }
    }
    #[inline]
    pub fn pixel_range(self, value: f64) -> Self {
        Self {
            pixel_range: value, .. self
        }
    }
    #[inline]
    pub fn scale(self, value: f64) -> Self {
        Self {
            scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_msdf_texture_rect_region_full(self.surround_object, self.texture, self.rect, self.src_rect, self.modulate, self.outline, self.pixel_range, self.scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_lcd_texture_rect_region_ex`][super::CanvasItem::draw_lcd_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawLcdTextureRectRegion < 'a > {
    surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawLcdTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: Gd < crate::engine::Texture2D >, rect: Rect2, src_rect: Rect2,) -> Self {
        Self {
            surround_object, texture, rect, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_lcd_texture_rect_region_full(self.surround_object, self.texture, self.rect, self.src_rect, self.modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_primitive_ex`][super::CanvasItem::draw_primitive_ex]."]
#[must_use]
pub struct ExDrawPrimitive < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPrimitive < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array,) -> Self {
        Self {
            surround_object, points, colors, uvs, texture: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn texture(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_primitive_full(self.surround_object, self.points, self.colors, self.uvs, self.texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polygon_ex`][super::CanvasItem::draw_polygon_ex]."]
#[must_use]
pub struct ExDrawPolygon < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, points, colors, uvs: PackedVector2Array::new(), texture: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn uvs(self, value: PackedVector2Array) -> Self {
        Self {
            uvs: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_polygon_full(self.surround_object, self.points, self.colors, self.uvs, self.texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_colored_polygon_ex`][super::CanvasItem::draw_colored_polygon_ex]."]
#[must_use]
pub struct ExDrawColoredPolygon < 'a > {
    surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color, uvs: PackedVector2Array, texture: Gd < crate::engine::Texture2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawColoredPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: PackedVector2Array, color: Color,) -> Self {
        Self {
            surround_object, points, color, uvs: PackedVector2Array::new(), texture: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn uvs(self, value: PackedVector2Array) -> Self {
        Self {
            uvs: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_colored_polygon_full(self.surround_object, self.points, self.color, self.uvs, self.texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_ex`][super::CanvasItem::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawString < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, font, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_string_full(self.surround_object, self.font, self.pos, self.text, self.alignment, self.width, self.font_size, self.modulate, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_ex`][super::CanvasItem::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineString < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, font, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, max_lines: - 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), brk_flags: crate::obj::EngineBitfield::from_ord(3), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, value: i32) -> Self {
        Self {
            max_lines: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_multiline_string_full(self.surround_object, self.font, self.pos, self.text, self.alignment, self.width, self.font_size, self.max_lines, self.modulate, self.brk_flags, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_outline_ex`][super::CanvasItem::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawStringOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, font, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, size: 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_string_outline_full(self.surround_object, self.font, self.pos, self.text, self.alignment, self.width, self.font_size, self.size, self.modulate, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_outline_ex`][super::CanvasItem::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString, alignment: crate::engine::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::engine::text_server::LineBreakFlag, justification_flags: crate::engine::text_server::JustificationFlag, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineStringOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, text: GString,) -> Self {
        Self {
            surround_object, font, pos, text, alignment: crate::obj::EngineEnum::from_ord(0), width: - 1f32, font_size: 16i32, max_lines: - 1i32, size: 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), brk_flags: crate::obj::EngineBitfield::from_ord(3), justification_flags: crate::obj::EngineBitfield::from_ord(3), direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn alignment(self, value: crate::engine::global::HorizontalAlignment) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, value: i32) -> Self {
        Self {
            max_lines: value, .. self
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: value, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_multiline_string_outline_full(self.surround_object, self.font, self.pos, self.text, self.alignment, self.width, self.font_size, self.max_lines, self.size, self.modulate, self.brk_flags, self.justification_flags, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_ex`][super::CanvasItem::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, char: GString, font_size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawChar < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) -> Self {
        Self {
            surround_object, font, pos, char, font_size: 16i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_char_full(self.surround_object, self.font, self.pos, self.char, self.font_size, self.modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_outline_ex`][super::CanvasItem::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'a > {
    surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, char: GString, font_size: i32, size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawCharOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: Gd < crate::engine::Font >, pos: Vector2, char: GString,) -> Self {
        Self {
            surround_object, font, pos, char, font_size: 16i32, size: - 1i32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn font_size(self, value: i32) -> Self {
        Self {
            font_size: value, .. self
        }
    }
    #[inline]
    pub fn size(self, value: i32) -> Self {
        Self {
            size: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_char_outline_full(self.surround_object, self.font, self.pos, self.char, self.font_size, self.size, self.modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_mesh_ex`][super::CanvasItem::draw_mesh_ex]."]
#[must_use]
pub struct ExDrawMesh < 'a > {
    surround_object: &'a mut re_export::CanvasItem, mesh: Gd < crate::engine::Mesh >, texture: Gd < crate::engine::Texture2D >, transform: Transform2D, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMesh < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, mesh: Gd < crate::engine::Mesh >, texture: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, mesh, texture, transform: Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _), modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn transform(self, value: Transform2D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_mesh_full(self.surround_object, self.mesh, self.texture, self.transform, self.modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_set_transform_ex`][super::CanvasItem::draw_set_transform_ex]."]
#[must_use]
pub struct ExDrawSetTransform < 'a > {
    surround_object: &'a mut re_export::CanvasItem, position: Vector2, rotation: f32, scale: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawSetTransform < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, position: Vector2,) -> Self {
        Self {
            surround_object, position, rotation: 0f32, scale: Vector2::new(1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn rotation(self, value: f32) -> Self {
        Self {
            rotation: value, .. self
        }
    }
    #[inline]
    pub fn scale(self, value: Vector2) -> Self {
        Self {
            scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_set_transform_full(self.surround_object, self.position, self.rotation, self.scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_animation_slice_ex`][super::CanvasItem::draw_animation_slice_ex]."]
#[must_use]
pub struct ExDrawAnimationSlice < 'a > {
    surround_object: &'a mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawAnimationSlice < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        Self {
            surround_object, animation_length, slice_begin, slice_end, offset: 0f64,
        }
    }
    #[inline]
    pub fn offset(self, value: f64) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CanvasItem::draw_animation_slice_full(self.surround_object, self.animation_length, self.slice_begin, self.slice_end, self.offset,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    pub const TEXTURE_FILTER_PARENT_NODE: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_FILTER_NEAREST: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_FILTER_LINEAR: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 6i32
    };
    pub const TEXTURE_FILTER_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for TextureFilter {
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
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    pub const TEXTURE_REPEAT_PARENT_NODE: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_REPEAT_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_REPEAT_ENABLED: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_REPEAT_MIRROR: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_REPEAT_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TextureRepeat {
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
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureRepeat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ClipChildrenMode {
    ord: i32
}
impl ClipChildrenMode {
    pub const CLIP_CHILDREN_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CLIP_CHILDREN_ONLY: Self = Self {
        ord: 1i32
    };
    pub const CLIP_CHILDREN_AND_DRAW: Self = Self {
        ord: 2i32
    };
    pub const CLIP_CHILDREN_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ClipChildrenMode {
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
impl crate::obj::IndexEnum for ClipChildrenMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ClipChildrenMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ClipChildrenMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ClipChildrenMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}