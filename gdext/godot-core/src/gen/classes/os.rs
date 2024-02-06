#![doc = "Sidecar module for class [`Os`][crate::engine::Os].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OS` enums](https://docs.godotengine.org/en/stable/classes/class_os.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OS.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`os`][crate::engine::os]: sidecar module with related enum/flag types\n* [`IOs`][crate::engine::IOs]: virtual methods\n\n\nSee also [Godot docs for `OS`](https://docs.godotengine.org/en/stable/classes/class_os.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Os {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Os`][crate::engine::Os].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OS` methods](https://docs.godotengine.org/en/stable/classes/class_os.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOs: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Os {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"OS\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_connected_midi_inputs(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connected_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn open_midi_inputs(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close_midi_inputs(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn alert_full(&mut self, text: GString, title: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (text, title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "alert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn alert(&mut self, text: GString,) {
            self.alert_ex(text,) . done()
        }
        #[inline]
        pub fn alert_ex(&mut self, text: GString,) -> ExAlert < '_ > {
            ExAlert::new(self, text,)
        }
        pub fn crash(&mut self, message: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (message,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "crash", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_low_processor_usage_mode(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_low_processor_usage_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_low_processor_usage_mode(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_in_low_processor_usage_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_low_processor_usage_mode_sleep_usec(&mut self, usec: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_low_processor_usage_mode_sleep_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_low_processor_usage_mode_sleep_usec(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_low_processor_usage_mode_sleep_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_delta_smoothing(&mut self, delta_smoothing_enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (delta_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_delta_smoothing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_delta_smoothing_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_delta_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_processor_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_processor_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_processor_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_processor_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_fonts(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_system_fonts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_system_font_path_full(&self, font_name: GString, weight: i32, stretch: i32, italic: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, i32, i32, bool);
            let args = (font_name, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_system_font_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_system_font_path(&self, font_name: GString,) -> GString {
            self.get_system_font_path_ex(font_name,) . done()
        }
        #[inline]
        pub fn get_system_font_path_ex(&self, font_name: GString,) -> ExGetSystemFontPath < '_ > {
            ExGetSystemFontPath::new(self, font_name,)
        }
        pub(crate) fn get_system_font_path_for_text_full(&self, font_name: GString, text: GString, locale: GString, script: GString, weight: i32, stretch: i32, italic: bool,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString, GString, GString, GString, i32, i32, bool);
            let args = (font_name, text, locale, script, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_system_font_path_for_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_system_font_path_for_text(&self, font_name: GString, text: GString,) -> PackedStringArray {
            self.get_system_font_path_for_text_ex(font_name, text,) . done()
        }
        #[inline]
        pub fn get_system_font_path_for_text_ex(&self, font_name: GString, text: GString,) -> ExGetSystemFontPathForText < '_ > {
            ExGetSystemFontPathForText::new(self, font_name, text,)
        }
        pub fn get_executable_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_executable_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn read_string_from_stdin(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "read_string_from_stdin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn execute_full(&mut self, path: GString, arguments: PackedStringArray, output: VariantArray, read_stderr: bool, open_console: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, PackedStringArray, VariantArray, bool, bool);
            let args = (path, arguments, output, read_stderr, open_console,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "execute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn execute(&mut self, path: GString, arguments: PackedStringArray,) -> i32 {
            self.execute_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn execute_ex(&mut self, path: GString, arguments: PackedStringArray,) -> ExExecute < '_ > {
            ExExecute::new(self, path, arguments,)
        }
        pub(crate) fn create_process_full(&mut self, path: GString, arguments: PackedStringArray, open_console: bool,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, PackedStringArray, bool);
            let args = (path, arguments, open_console,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_process(&mut self, path: GString, arguments: PackedStringArray,) -> i32 {
            self.create_process_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn create_process_ex(&mut self, path: GString, arguments: PackedStringArray,) -> ExCreateProcess < '_ > {
            ExCreateProcess::new(self, path, arguments,)
        }
        pub fn create_instance(&mut self, arguments: PackedStringArray,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, PackedStringArray);
            let args = (arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn kill(&mut self, pid: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, i32);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "kill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shell_open(&mut self, uri: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (uri,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shell_open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shell_show_in_file_manager_full(&mut self, file_or_dir_path: GString, open_folder: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (file_or_dir_path, open_folder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shell_show_in_file_manager", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shell_show_in_file_manager(&mut self, file_or_dir_path: GString,) -> crate::engine::global::Error {
            self.shell_show_in_file_manager_ex(file_or_dir_path,) . done()
        }
        #[inline]
        pub fn shell_show_in_file_manager_ex(&mut self, file_or_dir_path: GString,) -> ExShellShowInFileManager < '_ > {
            ExShellShowInFileManager::new(self, file_or_dir_path,)
        }
        pub fn is_process_running(&self, pid: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_process_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_environment(&self, variable: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (variable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment(&self, variable: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (variable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment(&self, variable: GString, value: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (variable, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unset_environment(&self, variable: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (variable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unset_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distribution_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_distribution_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cmdline_args(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cmdline_args", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cmdline_user_args(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cmdline_user_args", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_driver_info(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_video_adapter_driver_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_restart_on_exit_full(&mut self, restart: bool, arguments: PackedStringArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, PackedStringArray);
            let args = (restart, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_restart_on_exit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_restart_on_exit(&mut self, restart: bool,) {
            self.set_restart_on_exit_ex(restart,) . done()
        }
        #[inline]
        pub fn set_restart_on_exit_ex(&mut self, restart: bool,) -> ExSetRestartOnExit < '_ > {
            ExSetRestartOnExit::new(self, restart,)
        }
        pub fn is_restart_on_exit_set(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_restart_on_exit_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_restart_on_exit_arguments(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_restart_on_exit_arguments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delay_usec(&self, usec: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "delay_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delay_msec(&self, msec: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (msec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "delay_msec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale_language(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_locale_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_model_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_model_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_userfs_persistent(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_userfs_persistent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stdout_verbose(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_stdout_verbose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debug_build(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_debug_build", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_static_memory_usage(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_static_memory_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_static_memory_peak_usage(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_static_memory_peak_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_memory_info(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_memory_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_to_trash(&self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_to_trash", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_user_data_dir(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_user_data_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_system_dir_full(&self, dir: crate::engine::os::SystemDir, shared_storage: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, crate::engine::os::SystemDir, bool);
            let args = (dir, shared_storage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_system_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_system_dir(&self, dir: crate::engine::os::SystemDir,) -> GString {
            self.get_system_dir_ex(dir,) . done()
        }
        #[inline]
        pub fn get_system_dir_ex(&self, dir: crate::engine::os::SystemDir,) -> ExGetSystemDir < '_ > {
            ExGetSystemDir::new(self, dir,)
        }
        pub fn get_config_dir(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_config_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data_dir(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_dir(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keycode_string(&self, code: crate::engine::global::Key,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, crate::engine::global::Key);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keycode_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keycode_unicode(&self, code: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_keycode_unicode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_keycode_from_string(&self, string: GString,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key, GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_keycode_from_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_file_access_save_and_swap(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_file_access_save_and_swap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_thread_name(&mut self, name: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_thread_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_thread_caller_id(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_thread_caller_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_main_thread_id(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_main_thread_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_feature(&self, tag_name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (tag_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sandboxed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sandboxed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_permission(&mut self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_permission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_permissions(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_permissions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_granted_permissions(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_granted_permissions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn revoke_granted_permissions(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "revoke_granted_permissions", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Os {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OS\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Os {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Os {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Os {
        
    }
    impl std::ops::Deref for Os {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Os {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Os {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Os > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Os::alert_ex`][super::Os::alert_ex]."]
#[must_use]
pub struct ExAlert < 'a > {
    surround_object: &'a mut re_export::Os, text: GString, title: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAlert < 'a > {
    fn new(surround_object: &'a mut re_export::Os, text: GString,) -> Self {
        Self {
            surround_object, text, title: GString::from("Alert!"),
        }
    }
    #[inline]
    pub fn title(self, value: GString) -> Self {
        Self {
            title: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Os::alert_full(self.surround_object, self.text, self.title,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_ex`][super::Os::get_system_font_path_ex]."]
#[must_use]
pub struct ExGetSystemFontPath < 'a > {
    surround_object: &'a re_export::Os, font_name: GString, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPath < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: GString,) -> Self {
        Self {
            surround_object, font_name, weight: 400i32, stretch: 100i32, italic: false,
        }
    }
    #[inline]
    pub fn weight(self, value: i32) -> Self {
        Self {
            weight: value, .. self
        }
    }
    #[inline]
    pub fn stretch(self, value: i32) -> Self {
        Self {
            stretch: value, .. self
        }
    }
    #[inline]
    pub fn italic(self, value: bool) -> Self {
        Self {
            italic: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Os::get_system_font_path_full(self.surround_object, self.font_name, self.weight, self.stretch, self.italic,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_for_text_ex`][super::Os::get_system_font_path_for_text_ex]."]
#[must_use]
pub struct ExGetSystemFontPathForText < 'a > {
    surround_object: &'a re_export::Os, font_name: GString, text: GString, locale: GString, script: GString, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPathForText < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: GString, text: GString,) -> Self {
        Self {
            surround_object, font_name, text, locale: GString::from(""), script: GString::from(""), weight: 400i32, stretch: 100i32, italic: false,
        }
    }
    #[inline]
    pub fn locale(self, value: GString) -> Self {
        Self {
            locale: value, .. self
        }
    }
    #[inline]
    pub fn script(self, value: GString) -> Self {
        Self {
            script: value, .. self
        }
    }
    #[inline]
    pub fn weight(self, value: i32) -> Self {
        Self {
            weight: value, .. self
        }
    }
    #[inline]
    pub fn stretch(self, value: i32) -> Self {
        Self {
            stretch: value, .. self
        }
    }
    #[inline]
    pub fn italic(self, value: bool) -> Self {
        Self {
            italic: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::Os::get_system_font_path_for_text_full(self.surround_object, self.font_name, self.text, self.locale, self.script, self.weight, self.stretch, self.italic,)
    }
}
#[doc = "Default-param extender for [`Os::execute_ex`][super::Os::execute_ex]."]
#[must_use]
pub struct ExExecute < 'a > {
    surround_object: &'a mut re_export::Os, path: GString, arguments: PackedStringArray, output: VariantArray, read_stderr: bool, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExecute < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: GString, arguments: PackedStringArray,) -> Self {
        Self {
            surround_object, path, arguments, output: Array::new(), read_stderr: false, open_console: false,
        }
    }
    #[inline]
    pub fn output(self, value: VariantArray) -> Self {
        Self {
            output: value, .. self
        }
    }
    #[inline]
    pub fn read_stderr(self, value: bool) -> Self {
        Self {
            read_stderr: value, .. self
        }
    }
    #[inline]
    pub fn open_console(self, value: bool) -> Self {
        Self {
            open_console: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Os::execute_full(self.surround_object, self.path, self.arguments, self.output, self.read_stderr, self.open_console,)
    }
}
#[doc = "Default-param extender for [`Os::create_process_ex`][super::Os::create_process_ex]."]
#[must_use]
pub struct ExCreateProcess < 'a > {
    surround_object: &'a mut re_export::Os, path: GString, arguments: PackedStringArray, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateProcess < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: GString, arguments: PackedStringArray,) -> Self {
        Self {
            surround_object, path, arguments, open_console: false,
        }
    }
    #[inline]
    pub fn open_console(self, value: bool) -> Self {
        Self {
            open_console: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::Os::create_process_full(self.surround_object, self.path, self.arguments, self.open_console,)
    }
}
#[doc = "Default-param extender for [`Os::shell_show_in_file_manager_ex`][super::Os::shell_show_in_file_manager_ex]."]
#[must_use]
pub struct ExShellShowInFileManager < 'a > {
    surround_object: &'a mut re_export::Os, file_or_dir_path: GString, open_folder: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShellShowInFileManager < 'a > {
    fn new(surround_object: &'a mut re_export::Os, file_or_dir_path: GString,) -> Self {
        Self {
            surround_object, file_or_dir_path, open_folder: true,
        }
    }
    #[inline]
    pub fn open_folder(self, value: bool) -> Self {
        Self {
            open_folder: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Os::shell_show_in_file_manager_full(self.surround_object, self.file_or_dir_path, self.open_folder,)
    }
}
#[doc = "Default-param extender for [`Os::set_restart_on_exit_ex`][super::Os::set_restart_on_exit_ex]."]
#[must_use]
pub struct ExSetRestartOnExit < 'a > {
    surround_object: &'a mut re_export::Os, restart: bool, arguments: PackedStringArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRestartOnExit < 'a > {
    fn new(surround_object: &'a mut re_export::Os, restart: bool,) -> Self {
        Self {
            surround_object, restart, arguments: PackedStringArray::new(),
        }
    }
    #[inline]
    pub fn arguments(self, value: PackedStringArray) -> Self {
        Self {
            arguments: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Os::set_restart_on_exit_full(self.surround_object, self.restart, self.arguments,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_dir_ex`][super::Os::get_system_dir_ex]."]
#[must_use]
pub struct ExGetSystemDir < 'a > {
    surround_object: &'a re_export::Os, dir: crate::engine::os::SystemDir, shared_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemDir < 'a > {
    fn new(surround_object: &'a re_export::Os, dir: crate::engine::os::SystemDir,) -> Self {
        Self {
            surround_object, dir, shared_storage: true,
        }
    }
    #[inline]
    pub fn shared_storage(self, value: bool) -> Self {
        Self {
            shared_storage: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Os::get_system_dir_full(self.surround_object, self.dir, self.shared_storage,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderingDriver {
    ord: i32
}
impl RenderingDriver {
    pub const RENDERING_DRIVER_VULKAN: Self = Self {
        ord: 0i32
    };
    pub const RENDERING_DRIVER_OPENGL3: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for RenderingDriver {
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
impl crate::builtin::meta::GodotConvert for RenderingDriver {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RenderingDriver {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RenderingDriver {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SystemDir {
    ord: i32
}
impl SystemDir {
    pub const SYSTEM_DIR_DESKTOP: Self = Self {
        ord: 0i32
    };
    pub const SYSTEM_DIR_DCIM: Self = Self {
        ord: 1i32
    };
    pub const SYSTEM_DIR_DOCUMENTS: Self = Self {
        ord: 2i32
    };
    pub const SYSTEM_DIR_DOWNLOADS: Self = Self {
        ord: 3i32
    };
    pub const SYSTEM_DIR_MOVIES: Self = Self {
        ord: 4i32
    };
    pub const SYSTEM_DIR_MUSIC: Self = Self {
        ord: 5i32
    };
    pub const SYSTEM_DIR_PICTURES: Self = Self {
        ord: 6i32
    };
    pub const SYSTEM_DIR_RINGTONES: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for SystemDir {
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
impl crate::builtin::meta::GodotConvert for SystemDir {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SystemDir {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SystemDir {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}