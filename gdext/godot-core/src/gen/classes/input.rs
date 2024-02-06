#![doc = "Sidecar module for class [`Input`][crate::engine::Input].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Input` enums](https://docs.godotengine.org/en/stable/classes/class_input.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Input.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`input`][crate::engine::input]: sidecar module with related enum/flag types\n* [`IInput`][crate::engine::IInput]: virtual methods\n\n\nSee also [Godot docs for `Input`](https://docs.godotengine.org/en/stable/classes/class_input.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Input {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Input`][crate::engine::Input].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Input` methods](https://docs.godotengine.org/en/stable/classes/class_input.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInput: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Input {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Input\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_anything_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_anything_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_key_pressed(&self, keycode: crate::engine::global::Key,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_key_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physical_key_pressed(&self, keycode: crate::engine::global::Key,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_physical_key_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_key_label_pressed(&self, keycode: crate::engine::global::Key,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_key_label_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_mouse_button_pressed(&self, button: crate::engine::global::MouseButton,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::global::MouseButton);
            let args = (button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_mouse_button_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_joy_button_pressed(&self, device: i32, button: crate::engine::global::JoyButton,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, crate::engine::global::JoyButton);
            let args = (device, button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_joy_button_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_action_pressed_full(&self, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action_pressed(&self, action: StringName,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_pressed_ex(&self, action: StringName,) -> ExIsActionPressed < '_ > {
            ExIsActionPressed::new(self, action,)
        }
        pub(crate) fn is_action_just_pressed_full(&self, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_just_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action_just_pressed(&self, action: StringName,) -> bool {
            self.is_action_just_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_pressed_ex(&self, action: StringName,) -> ExIsActionJustPressed < '_ > {
            ExIsActionJustPressed::new(self, action,)
        }
        pub(crate) fn is_action_just_released_full(&self, action: StringName, exact_match: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_just_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn is_action_just_released(&self, action: StringName,) -> bool {
            self.is_action_just_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_released_ex(&self, action: StringName,) -> ExIsActionJustReleased < '_ > {
            ExIsActionJustReleased::new(self, action,)
        }
        pub(crate) fn get_action_strength_full(&self, action: StringName, exact_match: bool,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_action_strength(&self, action: StringName,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_strength_ex(&self, action: StringName,) -> ExGetActionStrength < '_ > {
            ExGetActionStrength::new(self, action,)
        }
        pub(crate) fn get_action_raw_strength_full(&self, action: StringName, exact_match: bool,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_raw_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_action_raw_strength(&self, action: StringName,) -> f32 {
            self.get_action_raw_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_raw_strength_ex(&self, action: StringName,) -> ExGetActionRawStrength < '_ > {
            ExGetActionRawStrength::new(self, action,)
        }
        pub fn get_axis(&self, negative_action: StringName, positive_action: StringName,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, StringName, StringName);
            let args = (negative_action, positive_action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_vector_full(&self, negative_x: StringName, positive_x: StringName, negative_y: StringName, positive_y: StringName, deadzone: f32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, StringName, StringName, StringName, StringName, f32);
            let args = (negative_x, positive_x, negative_y, positive_y, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_vector(&self, negative_x: StringName, positive_x: StringName, negative_y: StringName, positive_y: StringName,) -> Vector2 {
            self.get_vector_ex(negative_x, positive_x, negative_y, positive_y,) . done()
        }
        #[inline]
        pub fn get_vector_ex(&self, negative_x: StringName, positive_x: StringName, negative_y: StringName, positive_y: StringName,) -> ExGetVector < '_ > {
            ExGetVector::new(self, negative_x, positive_x, negative_y, positive_y,)
        }
        pub(crate) fn add_joy_mapping_full(&mut self, mapping: GString, update_existing: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (mapping, update_existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_joy_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_joy_mapping(&mut self, mapping: GString,) {
            self.add_joy_mapping_ex(mapping,) . done()
        }
        #[inline]
        pub fn add_joy_mapping_ex(&mut self, mapping: GString,) -> ExAddJoyMapping < '_ > {
            ExAddJoyMapping::new(self, mapping,)
        }
        pub fn remove_joy_mapping(&mut self, guid: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (guid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_joy_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_joy_known(&mut self, device: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_joy_known", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_axis(&self, device: i32, axis: crate::engine::global::JoyAxis,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, crate::engine::global::JoyAxis);
            let args = (device, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_name(&mut self, device: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_guid(&self, device: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_guid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_info(&self, device: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn should_ignore_device(&self, vendor_id: i32, product_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (vendor_id, product_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "should_ignore_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connected_joypads(&mut self,) -> Array < i64 > {
            type RetMarshal = PtrcallReturnT < Array < i64 > >;
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_connected_joypads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_vibration_strength(&mut self, device: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_vibration_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_vibration_duration(&mut self, device: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joy_vibration_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn start_joy_vibration_full(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32, f32, f32);
            let args = (device, weak_magnitude, strong_magnitude, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start_joy_vibration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn start_joy_vibration(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) {
            self.start_joy_vibration_ex(device, weak_magnitude, strong_magnitude,) . done()
        }
        #[inline]
        pub fn start_joy_vibration_ex(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> ExStartJoyVibration < '_ > {
            ExStartJoyVibration::new(self, device, weak_magnitude, strong_magnitude,)
        }
        pub fn stop_joy_vibration(&mut self, device: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop_joy_vibration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vibrate_handheld_full(&mut self, duration_ms: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (duration_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "vibrate_handheld", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn vibrate_handheld(&mut self,) {
            self.vibrate_handheld_ex() . done()
        }
        #[inline]
        pub fn vibrate_handheld_ex(&mut self,) -> ExVibrateHandheld < '_ > {
            ExVibrateHandheld::new(self,)
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accelerometer(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_accelerometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_magnetometer(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_magnetometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gyroscope(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gyroscope", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, value: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accelerometer(&mut self, value: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_accelerometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_magnetometer(&mut self, value: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_magnetometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gyroscope(&mut self, value: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gyroscope", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_mouse_velocity(&mut self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_last_mouse_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_button_mask(&self,) -> crate::engine::global::MouseButtonMask {
            type RetMarshal = PtrcallReturnT < crate::engine::global::MouseButtonMask >;
            type CallSig = (crate::engine::global::MouseButtonMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mouse_button_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mouse_mode(&mut self, mode: crate::engine::input::MouseMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::input::MouseMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mouse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_mode(&self,) -> crate::engine::input::MouseMode {
            type RetMarshal = PtrcallReturnT < crate::engine::input::MouseMode >;
            type CallSig = (crate::engine::input::MouseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mouse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn action_press_full(&mut self, action: StringName, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f32);
            let args = (action, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_press", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn action_press(&mut self, action: StringName,) {
            self.action_press_ex(action,) . done()
        }
        #[inline]
        pub fn action_press_ex(&mut self, action: StringName,) -> ExActionPress < '_ > {
            ExActionPress::new(self, action,)
        }
        pub fn action_release(&mut self, action: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "action_release", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_default_cursor_shape_full(&mut self, shape: crate::engine::input::CursorShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::input::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_default_cursor_shape(&mut self,) {
            self.set_default_cursor_shape_ex() . done()
        }
        #[inline]
        pub fn set_default_cursor_shape_ex(&mut self,) -> ExSetDefaultCursorShape < '_ > {
            ExSetDefaultCursorShape::new(self,)
        }
        pub fn get_current_cursor_shape(&self,) -> crate::engine::input::CursorShape {
            type RetMarshal = PtrcallReturnT < crate::engine::input::CursorShape >;
            type CallSig = (crate::engine::input::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_custom_mouse_cursor_full(&mut self, image: Gd < crate::engine::Resource >, shape: crate::engine::input::CursorShape, hotspot: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Resource >, crate::engine::input::CursorShape, Vector2);
            let args = (image, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_mouse_cursor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_custom_mouse_cursor(&mut self, image: Gd < crate::engine::Resource >,) {
            self.set_custom_mouse_cursor_ex(image,) . done()
        }
        #[inline]
        pub fn set_custom_mouse_cursor_ex(&mut self, image: Gd < crate::engine::Resource >,) -> ExSetCustomMouseCursor < '_ > {
            ExSetCustomMouseCursor::new(self, image,)
        }
        pub fn parse_input_event(&mut self, event: Gd < crate::engine::InputEvent >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::InputEvent >);
            let args = (event,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_input_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_accumulated_input(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_accumulated_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_accumulated_input(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_accumulated_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn flush_buffered_events(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flush_buffered_events", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Input {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Input\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Input {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Input {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Input {
        
    }
    impl std::ops::Deref for Input {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Input {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Input {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Input > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Input::is_action_pressed_ex`][super::Input::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    surround_object: &'a re_export::Input, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Input::is_action_pressed_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_ex`][super::Input::is_action_just_pressed_ex]."]
#[must_use]
pub struct ExIsActionJustPressed < 'a > {
    surround_object: &'a re_export::Input, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Input::is_action_just_pressed_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_ex`][super::Input::is_action_just_released_ex]."]
#[must_use]
pub struct ExIsActionJustReleased < 'a > {
    surround_object: &'a re_export::Input, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustReleased < 'a > {
    fn new(surround_object: &'a re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::Input::is_action_just_released_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_strength_ex`][super::Input::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    surround_object: &'a re_export::Input, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Input::get_action_strength_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_raw_strength_ex`][super::Input::get_action_raw_strength_ex]."]
#[must_use]
pub struct ExGetActionRawStrength < 'a > {
    surround_object: &'a re_export::Input, action: StringName, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionRawStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, exact_match: false,
        }
    }
    #[inline]
    pub fn exact_match(self, value: bool) -> Self {
        Self {
            exact_match: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::Input::get_action_raw_strength_full(self.surround_object, self.action, self.exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_vector_ex`][super::Input::get_vector_ex]."]
#[must_use]
pub struct ExGetVector < 'a > {
    surround_object: &'a re_export::Input, negative_x: StringName, positive_x: StringName, negative_y: StringName, positive_y: StringName, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVector < 'a > {
    fn new(surround_object: &'a re_export::Input, negative_x: StringName, positive_x: StringName, negative_y: StringName, positive_y: StringName,) -> Self {
        Self {
            surround_object, negative_x, positive_x, negative_y, positive_y, deadzone: - 1f32,
        }
    }
    #[inline]
    pub fn deadzone(self, value: f32) -> Self {
        Self {
            deadzone: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        re_export::Input::get_vector_full(self.surround_object, self.negative_x, self.positive_x, self.negative_y, self.positive_y, self.deadzone,)
    }
}
#[doc = "Default-param extender for [`Input::add_joy_mapping_ex`][super::Input::add_joy_mapping_ex]."]
#[must_use]
pub struct ExAddJoyMapping < 'a > {
    surround_object: &'a mut re_export::Input, mapping: GString, update_existing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddJoyMapping < 'a > {
    fn new(surround_object: &'a mut re_export::Input, mapping: GString,) -> Self {
        Self {
            surround_object, mapping, update_existing: false,
        }
    }
    #[inline]
    pub fn update_existing(self, value: bool) -> Self {
        Self {
            update_existing: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::add_joy_mapping_full(self.surround_object, self.mapping, self.update_existing,)
    }
}
#[doc = "Default-param extender for [`Input::start_joy_vibration_ex`][super::Input::start_joy_vibration_ex]."]
#[must_use]
pub struct ExStartJoyVibration < 'a > {
    surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStartJoyVibration < 'a > {
    fn new(surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> Self {
        Self {
            surround_object, device, weak_magnitude, strong_magnitude, duration: 0f32,
        }
    }
    #[inline]
    pub fn duration(self, value: f32) -> Self {
        Self {
            duration: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::start_joy_vibration_full(self.surround_object, self.device, self.weak_magnitude, self.strong_magnitude, self.duration,)
    }
}
#[doc = "Default-param extender for [`Input::vibrate_handheld_ex`][super::Input::vibrate_handheld_ex]."]
#[must_use]
pub struct ExVibrateHandheld < 'a > {
    surround_object: &'a mut re_export::Input, duration_ms: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVibrateHandheld < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        Self {
            surround_object, duration_ms: 500i32,
        }
    }
    #[inline]
    pub fn duration_ms(self, value: i32) -> Self {
        Self {
            duration_ms: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::vibrate_handheld_full(self.surround_object, self.duration_ms,)
    }
}
#[doc = "Default-param extender for [`Input::action_press_ex`][super::Input::action_press_ex]."]
#[must_use]
pub struct ExActionPress < 'a > {
    surround_object: &'a mut re_export::Input, action: StringName, strength: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExActionPress < 'a > {
    fn new(surround_object: &'a mut re_export::Input, action: StringName,) -> Self {
        Self {
            surround_object, action, strength: 1f32,
        }
    }
    #[inline]
    pub fn strength(self, value: f32) -> Self {
        Self {
            strength: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::action_press_full(self.surround_object, self.action, self.strength,)
    }
}
#[doc = "Default-param extender for [`Input::set_default_cursor_shape_ex`][super::Input::set_default_cursor_shape_ex]."]
#[must_use]
pub struct ExSetDefaultCursorShape < 'a > {
    surround_object: &'a mut re_export::Input, shape: crate::engine::input::CursorShape,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetDefaultCursorShape < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        Self {
            surround_object, shape: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn shape(self, value: crate::engine::input::CursorShape) -> Self {
        Self {
            shape: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::set_default_cursor_shape_full(self.surround_object, self.shape,)
    }
}
#[doc = "Default-param extender for [`Input::set_custom_mouse_cursor_ex`][super::Input::set_custom_mouse_cursor_ex]."]
#[must_use]
pub struct ExSetCustomMouseCursor < 'a > {
    surround_object: &'a mut re_export::Input, image: Gd < crate::engine::Resource >, shape: crate::engine::input::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomMouseCursor < 'a > {
    fn new(surround_object: &'a mut re_export::Input, image: Gd < crate::engine::Resource >,) -> Self {
        Self {
            surround_object, image, shape: crate::obj::EngineEnum::from_ord(0), hotspot: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn shape(self, value: crate::engine::input::CursorShape) -> Self {
        Self {
            shape: value, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, value: Vector2) -> Self {
        Self {
            hotspot: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Input::set_custom_mouse_cursor_full(self.surround_object, self.image, self.shape, self.hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    pub const MOUSE_MODE_VISIBLE: Self = Self {
        ord: 0i32
    };
    pub const MOUSE_MODE_HIDDEN: Self = Self {
        ord: 1i32
    };
    pub const MOUSE_MODE_CAPTURED: Self = Self {
        ord: 2i32
    };
    pub const MOUSE_MODE_CONFINED: Self = Self {
        ord: 3i32
    };
    pub const MOUSE_MODE_CONFINED_HIDDEN: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for MouseMode {
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
impl crate::builtin::meta::GodotConvert for MouseMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MouseMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MouseMode {
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