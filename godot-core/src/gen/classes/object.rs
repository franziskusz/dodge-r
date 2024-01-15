#![doc = "Sidecar module for class [`Object`][crate::engine::Object].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Object` enums](https://docs.godotengine.org/en/stable/classes/class_object.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Object.`\n\nThis is the base class for all other classes at the root of the hierarchy. Every instance of `Object` can be stored in a [`Gd`][crate::obj::Gd] smart pointer.\n\nRelated symbols:\n\n* [`object`][crate::engine::object]: sidecar module with related enum/flag types\n* [`IObject`][crate::engine::IObject]: virtual methods\n* [`ObjectNotification`][crate::engine::notify::ObjectNotification]: notification type\n\n\nSee also [Godot docs for `Object`](https://docs.godotengine.org/en/stable/classes/class_object.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Object {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Object`][crate::engine::Object].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Object` methods](https://docs.godotengine.org/en/stable/classes/class_object.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IObject: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Object`][crate::engine::Object]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum ObjectNotification {
        Postinitialize = 0i32, Predelete = 1i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        Unknown(i32),
    }
    impl From < i32 > for ObjectNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                0i32 => Self::Postinitialize, 1i32 => Self::Predelete, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ObjectNotification > for i32 {
        fn from(notification: ObjectNotification) -> i32 {
            match notification {
                ObjectNotification::Postinitialize => 0i32, ObjectNotification::Predelete => 1i32, ObjectNotification::Unknown(int) => int,
            }
        }
    }
    impl Object {
        pub fn get_class(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_class(&self, class: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set(&mut self, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get(&self, property: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (property,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indexed(&mut self, property_path: NodePath, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath, Variant);
            let args = (property_path, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_indexed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_indexed(&self, property_path: NodePath,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, NodePath);
            let args = (property_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_indexed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_property_list(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_property_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_method_list(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_method_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_can_revert(&self, property: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (property,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_can_revert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_revert(&self, property: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (property,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "property_get_revert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn notification(&mut self, what: i32, reversed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (what, reversed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn to_string(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_script(&mut self, script: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script(&self,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_meta(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_meta_full(&self, name: StringName, default: Variant,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName, Variant);
            let args = (name, default,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_meta(&self, name: StringName,) -> Variant {
            self.get_meta_ex(name,) . done()
        }
        #[inline]
        pub fn get_meta_ex(&self, name: StringName,) -> ExGetMeta < '_ > {
            ExGetMeta::new(self, name,)
        }
        pub fn has_meta(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meta_list(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_meta_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_user_signal_full(&mut self, signal: GString, arguments: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, VariantArray);
            let args = (signal, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_user_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_user_signal(&mut self, signal: GString,) {
            self.add_user_signal_ex(signal,) . done()
        }
        #[inline]
        pub fn add_user_signal_ex(&mut self, signal: GString,) -> ExAddUserSignal < '_ > {
            ExAddUserSignal::new(self, signal,)
        }
        pub fn has_user_signal(&self, signal: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_user_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn emit_signal(&mut self, signal: StringName, varargs: &[Variant]) -> crate::engine::global::Error {
            type CallSig = (crate::engine::global::Error, StringName);
            let args = (signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5226usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "emit_signal", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn call(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            type CallSig = (Variant, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5227usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn call_deferred(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            type CallSig = (Variant, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5228usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "call_deferred", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_deferred(&mut self, property: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (property, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deferred", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn callv(&mut self, method: StringName, arg_array: VariantArray,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName, VariantArray);
            let args = (method, arg_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "callv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_method(&self, method: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_signal(&self, signal: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signal_list(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_signal_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signal_connection_list(&self, signal: StringName,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, StringName);
            let args = (signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_signal_connection_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_incoming_connections(&self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_incoming_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_full(&mut self, signal: StringName, callable: Callable, flags: u32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, StringName, Callable, u32);
            let args = (signal, callable, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect(&mut self, signal: StringName, callable: Callable,) -> crate::engine::global::Error {
            self.connect_ex(signal, callable,) . done()
        }
        #[inline]
        pub fn connect_ex(&mut self, signal: StringName, callable: Callable,) -> ExConnect < '_ > {
            ExConnect::new(self, signal, callable,)
        }
        pub fn disconnect(&mut self, signal: StringName, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Callable);
            let args = (signal, callable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_connected(&self, signal: StringName, callable: Callable,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, Callable);
            let args = (signal, callable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_block_signals(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_block_signals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_blocking_signals(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_blocking_signals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_property_list_changed(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "notify_property_list_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_message_translation(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_message_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_translate_messages(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_translate_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn tr_full(&self, message: StringName, context: StringName,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, StringName, StringName);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn tr(&self, message: StringName,) -> GString {
            self.tr_ex(message,) . done()
        }
        #[inline]
        pub fn tr_ex(&self, message: StringName,) -> ExTr < '_ > {
            ExTr::new(self, message,)
        }
        pub(crate) fn tr_n_full(&self, message: StringName, plural_message: StringName, n: i32, context: StringName,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, StringName, StringName, i32, StringName);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tr_n", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn tr_n(&self, message: StringName, plural_message: StringName, n: i32,) -> GString {
            self.tr_n_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn tr_n_ex(&self, message: StringName, plural_message: StringName, n: i32,) -> ExTrN < '_ > {
            ExTrN::new(self, message, plural_message, n,)
        }
        pub fn is_queued_for_deletion(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_queued_for_deletion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cancel_free(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cancel_free", self.object_ptr, self.__checked_id(), args,)
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
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub(crate) const NOTIFICATION_POSTINITIALIZE: i32 = 0i32;
        pub(crate) const NOTIFICATION_PREDELETE: i32 = 1i32;
        
    }
    impl crate::obj::GodotClass for Object {
        type Base = ();
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Object\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Object {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemDynamic;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Object {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::cap::GodotDefault for Object {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Object {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Object::get_meta_ex`][super::Object::get_meta_ex]."]
#[must_use]
pub struct ExGetMeta < 'a > {
    surround_object: &'a re_export::Object, name: StringName, default: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMeta < 'a > {
    fn new(surround_object: &'a re_export::Object, name: StringName,) -> Self {
        Self {
            surround_object, name, default: Variant::nil(),
        }
    }
    #[inline]
    pub fn default(self, value: Variant) -> Self {
        Self {
            default: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::Object::get_meta_full(self.surround_object, self.name, self.default,)
    }
}
#[doc = "Default-param extender for [`Object::add_user_signal_ex`][super::Object::add_user_signal_ex]."]
#[must_use]
pub struct ExAddUserSignal < 'a > {
    surround_object: &'a mut re_export::Object, signal: GString, arguments: VariantArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddUserSignal < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: GString,) -> Self {
        Self {
            surround_object, signal, arguments: Array::new(),
        }
    }
    #[inline]
    pub fn arguments(self, value: VariantArray) -> Self {
        Self {
            arguments: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Object::add_user_signal_full(self.surround_object, self.signal, self.arguments,)
    }
}
#[doc = "Default-param extender for [`Object::connect_ex`][super::Object::connect_ex]."]
#[must_use]
pub struct ExConnect < 'a > {
    surround_object: &'a mut re_export::Object, signal: StringName, callable: Callable, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnect < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: StringName, callable: Callable,) -> Self {
        Self {
            surround_object, signal, callable, flags: 0u32,
        }
    }
    #[inline]
    pub fn flags(self, value: u32) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Object::connect_full(self.surround_object, self.signal, self.callable, self.flags,)
    }
}
#[doc = "Default-param extender for [`Object::tr_ex`][super::Object::tr_ex]."]
#[must_use]
pub struct ExTr < 'a > {
    surround_object: &'a re_export::Object, message: StringName, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTr < 'a > {
    fn new(surround_object: &'a re_export::Object, message: StringName,) -> Self {
        Self {
            surround_object, message, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Object::tr_full(self.surround_object, self.message, self.context,)
    }
}
#[doc = "Default-param extender for [`Object::tr_n_ex`][super::Object::tr_n_ex]."]
#[must_use]
pub struct ExTrN < 'a > {
    surround_object: &'a re_export::Object, message: StringName, plural_message: StringName, n: i32, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrN < 'a > {
    fn new(surround_object: &'a re_export::Object, message: StringName, plural_message: StringName, n: i32,) -> Self {
        Self {
            surround_object, message, plural_message, n, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Object::tr_n_full(self.surround_object, self.message, self.plural_message, self.n, self.context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ConnectFlags {
    ord: i32
}
impl ConnectFlags {
    pub const CONNECT_DEFERRED: Self = Self {
        ord: 1i32
    };
    pub const CONNECT_PERSIST: Self = Self {
        ord: 2i32
    };
    pub const CONNECT_ONE_SHOT: Self = Self {
        ord: 4i32
    };
    pub const CONNECT_REFERENCE_COUNTED: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for ConnectFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ConnectFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ConnectFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ConnectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}