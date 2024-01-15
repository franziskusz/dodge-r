#![doc = "Sidecar module for class [`Window`][crate::engine::Window].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Window` enums](https://docs.godotengine.org/en/stable/classes/class_window.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Window.`\n\nInherits [`Viewport`][crate::engine::Viewport].\n\nRelated symbols:\n\n* [`window`][crate::engine::window]: sidecar module with related enum/flag types\n* [`IWindow`][crate::engine::IWindow]: virtual methods\n* [`WindowNotification`][crate::engine::notify::WindowNotification]: notification type\n\n\nSee also [Godot docs for `Window`](https://docs.godotengine.org/en/stable/classes/class_window.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Window {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Window`][crate::engine::Window].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Window` methods](https://docs.godotengine.org/en/stable/classes/class_window.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWindow: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
            unimplemented !()
        }
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    #[doc = "Notification type for class [`Window`][crate::engine::Window]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum WindowNotification {
        VisibilityChangedOrNodeRecacheRequested = 30i32, ThemeChanged = 32i32, EnterTree = 10i32, ExitTree = 11i32, MovedInParent = 12i32, Ready = 13i32, Paused = 14i32, Unpaused = 15i32, PhysicsProcess = 16i32, Process = 17i32, Parented = 18i32, Unparented = 19i32, SceneInstantiated = 20i32, DragBegin = 21i32, DragEnd = 22i32, PathRenamed = 23i32, ChildOrderChanged = 24i32, InternalProcess = 25i32, InternalPhysicsProcess = 26i32, PostEnterTree = 27i32, Disabled = 28i32, Enabled = 29i32, EditorPreSave = 9001i32, EditorPostSave = 9002i32, WmMouseEnter = 1002i32, WmMouseExit = 1003i32, WmWindowFocusIn = 1004i32, WmWindowFocusOut = 1005i32, WmCloseRequest = 1006i32, WmGoBackRequest = 1007i32, WmSizeChanged = 1008i32, WmDpiChange = 1009i32, VpMouseEnter = 1010i32, VpMouseExit = 1011i32, OsMemoryWarning = 2009i32, TranslationChanged = 2010i32, WmAbout = 2011i32, Crash = 2012i32, OsImeUpdate = 2013i32, ApplicationResumed = 2014i32, ApplicationPaused = 2015i32, ApplicationFocusIn = 2016i32, ApplicationFocusOut = 2017i32, TextServerChanged = 2018i32, Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for WindowNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                30i32 => Self::VisibilityChangedOrNodeRecacheRequested, 32i32 => Self::ThemeChanged, 10i32 => Self::EnterTree, 11i32 => Self::ExitTree, 12i32 => Self::MovedInParent, 13i32 => Self::Ready, 14i32 => Self::Paused, 15i32 => Self::Unpaused, 16i32 => Self::PhysicsProcess, 17i32 => Self::Process, 18i32 => Self::Parented, 19i32 => Self::Unparented, 20i32 => Self::SceneInstantiated, 21i32 => Self::DragBegin, 22i32 => Self::DragEnd, 23i32 => Self::PathRenamed, 24i32 => Self::ChildOrderChanged, 25i32 => Self::InternalProcess, 26i32 => Self::InternalPhysicsProcess, 27i32 => Self::PostEnterTree, 28i32 => Self::Disabled, 29i32 => Self::Enabled, 9001i32 => Self::EditorPreSave, 9002i32 => Self::EditorPostSave, 1002i32 => Self::WmMouseEnter, 1003i32 => Self::WmMouseExit, 1004i32 => Self::WmWindowFocusIn, 1005i32 => Self::WmWindowFocusOut, 1006i32 => Self::WmCloseRequest, 1007i32 => Self::WmGoBackRequest, 1008i32 => Self::WmSizeChanged, 1009i32 => Self::WmDpiChange, 1010i32 => Self::VpMouseEnter, 1011i32 => Self::VpMouseExit, 2009i32 => Self::OsMemoryWarning, 2010i32 => Self::TranslationChanged, 2011i32 => Self::WmAbout, 2012i32 => Self::Crash, 2013i32 => Self::OsImeUpdate, 2014i32 => Self::ApplicationResumed, 2015i32 => Self::ApplicationPaused, 2016i32 => Self::ApplicationFocusIn, 2017i32 => Self::ApplicationFocusOut, 2018i32 => Self::TextServerChanged, 0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < WindowNotification > for i32 {
        fn from(notification: WindowNotification) -> i32 {
            match notification {
                WindowNotification::VisibilityChangedOrNodeRecacheRequested => 30i32, WindowNotification::ThemeChanged => 32i32, WindowNotification::EnterTree => 10i32, WindowNotification::ExitTree => 11i32, WindowNotification::MovedInParent => 12i32, WindowNotification::Ready => 13i32, WindowNotification::Paused => 14i32, WindowNotification::Unpaused => 15i32, WindowNotification::PhysicsProcess => 16i32, WindowNotification::Process => 17i32, WindowNotification::Parented => 18i32, WindowNotification::Unparented => 19i32, WindowNotification::SceneInstantiated => 20i32, WindowNotification::DragBegin => 21i32, WindowNotification::DragEnd => 22i32, WindowNotification::PathRenamed => 23i32, WindowNotification::ChildOrderChanged => 24i32, WindowNotification::InternalProcess => 25i32, WindowNotification::InternalPhysicsProcess => 26i32, WindowNotification::PostEnterTree => 27i32, WindowNotification::Disabled => 28i32, WindowNotification::Enabled => 29i32, WindowNotification::EditorPreSave => 9001i32, WindowNotification::EditorPostSave => 9002i32, WindowNotification::WmMouseEnter => 1002i32, WindowNotification::WmMouseExit => 1003i32, WindowNotification::WmWindowFocusIn => 1004i32, WindowNotification::WmWindowFocusOut => 1005i32, WindowNotification::WmCloseRequest => 1006i32, WindowNotification::WmGoBackRequest => 1007i32, WindowNotification::WmSizeChanged => 1008i32, WindowNotification::WmDpiChange => 1009i32, WindowNotification::VpMouseEnter => 1010i32, WindowNotification::VpMouseExit => 1011i32, WindowNotification::OsMemoryWarning => 2009i32, WindowNotification::TranslationChanged => 2010i32, WindowNotification::WmAbout => 2011i32, WindowNotification::Crash => 2012i32, WindowNotification::OsImeUpdate => 2013i32, WindowNotification::ApplicationResumed => 2014i32, WindowNotification::ApplicationPaused => 2015i32, WindowNotification::ApplicationFocusIn => 2016i32, WindowNotification::ApplicationFocusOut => 2017i32, WindowNotification::TextServerChanged => 2018i32, WindowNotification::Postinitialize => 0i32, WindowNotification::Predelete => 1i32, WindowNotification::Unknown(int) => int,
            }
        }
    }
    impl Window {
        pub fn set_title(&mut self, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_title(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_window_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_initial_position(&mut self, initial_position: crate::engine::window::WindowInitialPosition,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::WindowInitialPosition);
            let args = (initial_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_initial_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_initial_position(&self,) -> crate::engine::window::WindowInitialPosition {
            type RetMarshal = PtrcallReturnT < crate::engine::window::WindowInitialPosition >;
            type CallSig = (crate::engine::window::WindowInitialPosition,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_initial_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_screen(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_screen(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_to_center(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_to_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_size(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reset_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position_with_decorations(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size_with_decorations(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_size(&mut self, max_size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_size(&mut self, min_size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (min_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::engine::window::Mode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::engine::window::Mode {
            type RetMarshal = PtrcallReturnT < crate::engine::window::Mode >;
            type CallSig = (crate::engine::window::Mode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::engine::window::Flags, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::Flags, bool);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::engine::window::Flags,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::window::Flags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_maximize_allowed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_maximize_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_attention(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_attention", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_to_foreground(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_to_foreground", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transient(&mut self, transient: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (transient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transient(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_transient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclusive(&mut self, exclusive: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (exclusive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclusive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_exclusive(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_exclusive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unparent_when_invisible(&mut self, unparent: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (unparent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unparent_when_invisible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_draw(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_focus(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grab_focus(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "grab_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ime_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ime_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ime_position(&mut self, position: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ime_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_embedded(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_embedded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contents_minimum_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contents_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_content_scale_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_content_scale_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_scale_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_scale_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_content_scale_mode(&mut self, mode: crate::engine::window::ContentScaleMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::ContentScaleMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_content_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_scale_mode(&self,) -> crate::engine::window::ContentScaleMode {
            type RetMarshal = PtrcallReturnT < crate::engine::window::ContentScaleMode >;
            type CallSig = (crate::engine::window::ContentScaleMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_content_scale_aspect(&mut self, aspect: crate::engine::window::ContentScaleAspect,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::ContentScaleAspect);
            let args = (aspect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_content_scale_aspect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_scale_aspect(&self,) -> crate::engine::window::ContentScaleAspect {
            type RetMarshal = PtrcallReturnT < crate::engine::window::ContentScaleAspect >;
            type CallSig = (crate::engine::window::ContentScaleAspect,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_scale_aspect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_content_scale_stretch(&mut self, stretch: crate::engine::window::ContentScaleStretch,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::ContentScaleStretch);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_content_scale_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_scale_stretch(&self,) -> crate::engine::window::ContentScaleStretch {
            type RetMarshal = PtrcallReturnT < crate::engine::window::ContentScaleStretch >;
            type CallSig = (crate::engine::window::ContentScaleStretch,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_scale_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keep_title_visible(&mut self, title_visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (title_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keep_title_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keep_title_visible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keep_title_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_content_scale_factor(&mut self, factor: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (factor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_content_scale_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_scale_factor(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_content_scale_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_font_oversampling(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_font_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_font_oversampling(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_font_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mouse_passthrough_polygon(&mut self, polygon: PackedVector2Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mouse_passthrough_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_passthrough_polygon(&self,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mouse_passthrough_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wrap_controls(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_wrap_controls", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_wrapping_controls(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_wrapping_controls", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn child_controls_changed(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "child_controls_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme(&mut self, theme: Gd < crate::engine::Theme >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Theme >);
            let args = (theme,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme(&self,) -> Option < Gd < crate::engine::Theme > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Theme > >;
            type CallSig = (Option < Gd < crate::engine::Theme > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_type_variation(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin_bulk_theme_override(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "begin_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_bulk_theme_override(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "end_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_icon_override(&mut self, name: StringName, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Texture2D >);
            let args = (name, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_stylebox_override(&mut self, name: StringName, stylebox: Gd < crate::engine::StyleBox >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::StyleBox >);
            let args = (name, stylebox,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_override(&mut self, name: StringName, font: Gd < crate::engine::Font >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Gd < crate::engine::Font >);
            let args = (name, font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_size_override(&mut self, name: StringName, font_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (name, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_color_override(&mut self, name: StringName, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Color);
            let args = (name, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_constant_override(&mut self, name: StringName, constant: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, i32);
            let args = (name, constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_icon_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_stylebox_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_size_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_color_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_constant_override(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_theme_icon_full(&self, name: StringName, theme_type: StringName,) -> Option < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture2D > >;
            type CallSig = (Option < Gd < crate::engine::Texture2D > >, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9879usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9880usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9881usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9882usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9883usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9884usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_stylebox_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_size_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_color_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_constant_override(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn has_theme_icon_full(&self, name: StringName, theme_type: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9891usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9892usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9893usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9894usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9895usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9896usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(9897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font(&self,) -> Option < Gd < crate::engine::Font > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Font > >;
            type CallSig = (Option < Gd < crate::engine::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_theme_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layout_direction(&mut self, direction: crate::engine::window::LayoutDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::window::LayoutDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layout_direction(&self,) -> crate::engine::window::LayoutDirection {
            type RetMarshal = PtrcallReturnT < crate::engine::window::LayoutDirection >;
            type CallSig = (crate::engine::window::LayoutDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layout_rtl(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_layout_rtl", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_translating(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_auto_translating", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_full(&mut self, rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup(&mut self,) {
            self.popup_ex() . done()
        }
        #[inline]
        pub fn popup_ex(&mut self,) -> ExPopup < '_ > {
            ExPopup::new(self,)
        }
        pub fn popup_on_parent(&mut self, parent_rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i);
            let args = (parent_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_on_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_centered_full(&mut self, minsize: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (minsize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_centered(&mut self,) {
            self.popup_centered_ex() . done()
        }
        #[inline]
        pub fn popup_centered_ex(&mut self,) -> ExPopupCentered < '_ > {
            ExPopupCentered::new(self,)
        }
        pub(crate) fn popup_centered_ratio_full(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_centered_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_centered_ratio(&mut self,) {
            self.popup_centered_ratio_ex() . done()
        }
        #[inline]
        pub fn popup_centered_ratio_ex(&mut self,) -> ExPopupCenteredRatio < '_ > {
            ExPopupCenteredRatio::new(self,)
        }
        pub(crate) fn popup_centered_clamped_full(&mut self, minsize: Vector2i, fallback_ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, f32);
            let args = (minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_centered_clamped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_centered_clamped(&mut self,) {
            self.popup_centered_clamped_ex() . done()
        }
        #[inline]
        pub fn popup_centered_clamped_ex(&mut self,) -> ExPopupCenteredClamped < '_ > {
            ExPopupCenteredClamped::new(self,)
        }
        pub(crate) fn popup_exclusive_full(&mut self, from_node: Gd < crate::engine::Node >, rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, Rect2i);
            let args = (from_node, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_exclusive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_exclusive(&mut self, from_node: Gd < crate::engine::Node >,) {
            self.popup_exclusive_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_ex(&mut self, from_node: Gd < crate::engine::Node >,) -> ExPopupExclusive < '_ > {
            ExPopupExclusive::new(self, from_node,)
        }
        pub fn popup_exclusive_on_parent(&mut self, from_node: Gd < crate::engine::Node >, parent_rect: Rect2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, Rect2i);
            let args = (from_node, parent_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_exclusive_on_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_exclusive_centered_full(&mut self, from_node: Gd < crate::engine::Node >, minsize: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, Vector2i);
            let args = (from_node, minsize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_exclusive_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_exclusive_centered(&mut self, from_node: Gd < crate::engine::Node >,) {
            self.popup_exclusive_centered_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_ex(&mut self, from_node: Gd < crate::engine::Node >,) -> ExPopupExclusiveCentered < '_ > {
            ExPopupExclusiveCentered::new(self, from_node,)
        }
        pub(crate) fn popup_exclusive_centered_ratio_full(&mut self, from_node: Gd < crate::engine::Node >, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, f32);
            let args = (from_node, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_exclusive_centered_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_exclusive_centered_ratio(&mut self, from_node: Gd < crate::engine::Node >,) {
            self.popup_exclusive_centered_ratio_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_ratio_ex(&mut self, from_node: Gd < crate::engine::Node >,) -> ExPopupExclusiveCenteredRatio < '_ > {
            ExPopupExclusiveCenteredRatio::new(self, from_node,)
        }
        pub(crate) fn popup_exclusive_centered_clamped_full(&mut self, from_node: Gd < crate::engine::Node >, minsize: Vector2i, fallback_ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >, Vector2i, f32);
            let args = (from_node, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "popup_exclusive_centered_clamped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn popup_exclusive_centered_clamped(&mut self, from_node: Gd < crate::engine::Node >,) {
            self.popup_exclusive_centered_clamped_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_clamped_ex(&mut self, from_node: Gd < crate::engine::Node >,) -> ExPopupExclusiveCenteredClamped < '_ > {
            ExPopupExclusiveCenteredClamped::new(self, from_node,)
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
        pub fn notify(&mut self, what: WindowNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: WindowNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 30i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 32i32;
        
    }
    impl crate::obj::GodotClass for Window {
        type Base = crate::engine::Viewport;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Window\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Window {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Window {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Viewport > for Window {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Window {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Window {
        
    }
    impl crate::obj::ExportableObject for Window {
        
    }
    impl crate::obj::cap::GodotDefault for Window {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Window {
        type Target = crate::engine::Viewport;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Window {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Window {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Window > for $Class {
                
            }
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
#[doc = "Default-param extender for [`Window::get_theme_icon_ex`][super::Window::get_theme_icon_ex]."]
#[must_use]
pub struct ExGetThemeIcon < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_icon_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_stylebox_ex`][super::Window::get_theme_stylebox_ex]."]
#[must_use]
pub struct ExGetThemeStylebox < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_stylebox_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_font_ex`][super::Window::get_theme_font_ex]."]
#[must_use]
pub struct ExGetThemeFont < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_font_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_font_size_ex`][super::Window::get_theme_font_size_ex]."]
#[must_use]
pub struct ExGetThemeFontSize < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_font_size_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_color_ex`][super::Window::get_theme_color_ex]."]
#[must_use]
pub struct ExGetThemeColor < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_color_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_constant_ex`][super::Window::get_theme_constant_ex]."]
#[must_use]
pub struct ExGetThemeConstant < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::get_theme_constant_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_icon_ex`][super::Window::has_theme_icon_ex]."]
#[must_use]
pub struct ExHasThemeIcon < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_icon_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_stylebox_ex`][super::Window::has_theme_stylebox_ex]."]
#[must_use]
pub struct ExHasThemeStylebox < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_stylebox_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_font_ex`][super::Window::has_theme_font_ex]."]
#[must_use]
pub struct ExHasThemeFont < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_font_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_font_size_ex`][super::Window::has_theme_font_size_ex]."]
#[must_use]
pub struct ExHasThemeFontSize < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_font_size_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_color_ex`][super::Window::has_theme_color_ex]."]
#[must_use]
pub struct ExHasThemeColor < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_color_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_constant_ex`][super::Window::has_theme_constant_ex]."]
#[must_use]
pub struct ExHasThemeConstant < 'a > {
    surround_object: &'a re_export::Window, name: StringName, theme_type: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Window, name: StringName,) -> Self {
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
        re_export::Window::has_theme_constant_full(self.surround_object, self.name, self.theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::popup_ex`][super::Window::popup_ex]."]
#[must_use]
pub struct ExPopup < 'a > {
    surround_object: &'a mut re_export::Window, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopup < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        Self {
            surround_object, rect: Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn rect(self, value: Rect2i) -> Self {
        Self {
            rect: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_full(self.surround_object, self.rect,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_ex`][super::Window::popup_centered_ex]."]
#[must_use]
pub struct ExPopupCentered < 'a > {
    surround_object: &'a mut re_export::Window, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCentered < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        Self {
            surround_object, minsize: Vector2i::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_centered_full(self.surround_object, self.minsize,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_ratio_ex`][super::Window::popup_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupCenteredRatio < 'a > {
    surround_object: &'a mut re_export::Window, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        Self {
            surround_object, ratio: 0.8f32,
        }
    }
    #[inline]
    pub fn ratio(self, value: f32) -> Self {
        Self {
            ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_centered_ratio_full(self.surround_object, self.ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_clamped_ex`][super::Window::popup_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupCenteredClamped < 'a > {
    surround_object: &'a mut re_export::Window, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        Self {
            surround_object, minsize: Vector2i::new(0 as _, 0 as _), fallback_ratio: 0.75f32,
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, value: f32) -> Self {
        Self {
            fallback_ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_centered_clamped_full(self.surround_object, self.minsize, self.fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_ex`][super::Window::popup_exclusive_ex]."]
#[must_use]
pub struct ExPopupExclusive < 'a > {
    surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusive < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, from_node, rect: Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn rect(self, value: Rect2i) -> Self {
        Self {
            rect: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_exclusive_full(self.surround_object, self.from_node, self.rect,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_ex`][super::Window::popup_exclusive_centered_ex]."]
#[must_use]
pub struct ExPopupExclusiveCentered < 'a > {
    surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCentered < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, from_node, minsize: Vector2i::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_exclusive_centered_full(self.surround_object, self.from_node, self.minsize,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_ratio_ex`][super::Window::popup_exclusive_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupExclusiveCenteredRatio < 'a > {
    surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, from_node, ratio: 0.8f32,
        }
    }
    #[inline]
    pub fn ratio(self, value: f32) -> Self {
        Self {
            ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_exclusive_centered_ratio_full(self.surround_object, self.from_node, self.ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_clamped_ex`][super::Window::popup_exclusive_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupExclusiveCenteredClamped < 'a > {
    surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: Gd < crate::engine::Node >,) -> Self {
        Self {
            surround_object, from_node, minsize: Vector2i::new(0 as _, 0 as _), fallback_ratio: 0.75f32,
        }
    }
    #[inline]
    pub fn minsize(self, value: Vector2i) -> Self {
        Self {
            minsize: value, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, value: f32) -> Self {
        Self {
            fallback_ratio: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Window::popup_exclusive_centered_clamped_full(self.surround_object, self.from_node, self.minsize, self.fallback_ratio,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mode {
    ord: i32
}
impl Mode {
    pub const MODE_WINDOWED: Self = Self {
        ord: 0i32
    };
    pub const MODE_MINIMIZED: Self = Self {
        ord: 1i32
    };
    pub const MODE_MAXIMIZED: Self = Self {
        ord: 2i32
    };
    pub const MODE_FULLSCREEN: Self = Self {
        ord: 3i32
    };
    pub const MODE_EXCLUSIVE_FULLSCREEN: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Mode {
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
impl crate::builtin::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Mode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Flags {
    ord: i32
}
impl Flags {
    pub const FLAG_RESIZE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const FLAG_BORDERLESS: Self = Self {
        ord: 1i32
    };
    pub const FLAG_ALWAYS_ON_TOP: Self = Self {
        ord: 2i32
    };
    pub const FLAG_TRANSPARENT: Self = Self {
        ord: 3i32
    };
    pub const FLAG_NO_FOCUS: Self = Self {
        ord: 4i32
    };
    pub const FLAG_POPUP: Self = Self {
        ord: 5i32
    };
    pub const FLAG_EXTEND_TO_TITLE: Self = Self {
        ord: 6i32
    };
    pub const FLAG_MOUSE_PASSTHROUGH: Self = Self {
        ord: 7i32
    };
    pub const FLAG_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for Flags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Flags {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for Flags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Flags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContentScaleMode {
    ord: i32
}
impl ContentScaleMode {
    pub const CONTENT_SCALE_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CONTENT_SCALE_MODE_CANVAS_ITEMS: Self = Self {
        ord: 1i32
    };
    pub const CONTENT_SCALE_MODE_VIEWPORT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ContentScaleMode {
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
impl crate::builtin::meta::GodotConvert for ContentScaleMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ContentScaleMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ContentScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContentScaleAspect {
    ord: i32
}
impl ContentScaleAspect {
    pub const CONTENT_SCALE_ASPECT_IGNORE: Self = Self {
        ord: 0i32
    };
    pub const CONTENT_SCALE_ASPECT_KEEP: Self = Self {
        ord: 1i32
    };
    pub const CONTENT_SCALE_ASPECT_KEEP_WIDTH: Self = Self {
        ord: 2i32
    };
    pub const CONTENT_SCALE_ASPECT_KEEP_HEIGHT: Self = Self {
        ord: 3i32
    };
    pub const CONTENT_SCALE_ASPECT_EXPAND: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ContentScaleAspect {
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
impl crate::builtin::meta::GodotConvert for ContentScaleAspect {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ContentScaleAspect {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ContentScaleAspect {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContentScaleStretch {
    ord: i32
}
impl ContentScaleStretch {
    pub const CONTENT_SCALE_STRETCH_FRACTIONAL: Self = Self {
        ord: 0i32
    };
    pub const CONTENT_SCALE_STRETCH_INTEGER: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for ContentScaleStretch {
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
impl crate::builtin::meta::GodotConvert for ContentScaleStretch {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ContentScaleStretch {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ContentScaleStretch {
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
pub struct WindowInitialPosition {
    ord: i32
}
impl WindowInitialPosition {
    pub const WINDOW_INITIAL_POSITION_ABSOLUTE: Self = Self {
        ord: 0i32
    };
    pub const WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN: Self = Self {
        ord: 1i32
    };
    pub const WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN: Self = Self {
        ord: 2i32
    };
    pub const WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN: Self = Self {
        ord: 3i32
    };
    pub const WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS: Self = Self {
        ord: 4i32
    };
    pub const WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for WindowInitialPosition {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for WindowInitialPosition {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for WindowInitialPosition {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for WindowInitialPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}