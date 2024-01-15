#![doc = "Sidecar module for class [`ProjectSettings`][crate::engine::ProjectSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProjectSettings` enums](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProjectSettings.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`project_settings`][crate::engine::project_settings]: sidecar module with related enum/flag types\n* [`IProjectSettings`][crate::engine::IProjectSettings]: virtual methods\n\n\nSee also [Godot docs for `ProjectSettings`](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProjectSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ProjectSettings`][crate::engine::ProjectSettings].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ProjectSettings` methods](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IProjectSettings: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ProjectSettings {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"ProjectSettings\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_setting(&self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_setting(&mut self, name: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_setting_full(&self, name: GString, default_value: Variant,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString, Variant);
            let args = (name, default_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_setting(&self, name: GString,) -> Variant {
            self.get_setting_ex(name,) . done()
        }
        #[inline]
        pub fn get_setting_ex(&self, name: GString,) -> ExGetSetting < '_ > {
            ExGetSetting::new(self, name,)
        }
        pub fn get_setting_with_override(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_setting_with_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_class_list(&mut self,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_class_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_order(&mut self, name: GString, position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, i32);
            let args = (name, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_order(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_initial_value(&mut self, name: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_initial_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_basic(&mut self, name: GString, basic: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (name, basic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_basic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_internal(&mut self, name: GString, internal: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (name, internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_property_info(&mut self, hint: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_property_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_restart_if_changed(&mut self, name: GString, restart: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (name, restart,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_restart_if_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn localize_path(&self, path: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "localize_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn globalize_path(&self, path: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "globalize_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn load_resource_pack_full(&mut self, pack: GString, replace_files: bool, offset: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, bool, i32);
            let args = (pack, replace_files, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_resource_pack", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load_resource_pack(&mut self, pack: GString,) -> bool {
            self.load_resource_pack_ex(pack,) . done()
        }
        #[inline]
        pub fn load_resource_pack_ex(&mut self, pack: GString,) -> ExLoadResourcePack < '_ > {
            ExLoadResourcePack::new(self, pack,)
        }
        pub fn save_custom(&mut self, file: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_custom", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ProjectSettings {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ProjectSettings\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ProjectSettings {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ProjectSettings {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for ProjectSettings {
        
    }
    impl std::ops::Deref for ProjectSettings {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProjectSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ProjectSettings {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ProjectSettings > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ProjectSettings::get_setting_ex`][super::ProjectSettings::get_setting_ex]."]
#[must_use]
pub struct ExGetSetting < 'a > {
    surround_object: &'a re_export::ProjectSettings, name: GString, default_value: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSetting < 'a > {
    fn new(surround_object: &'a re_export::ProjectSettings, name: GString,) -> Self {
        Self {
            surround_object, name, default_value: Variant::nil(),
        }
    }
    #[inline]
    pub fn default_value(self, value: Variant) -> Self {
        Self {
            default_value: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::ProjectSettings::get_setting_full(self.surround_object, self.name, self.default_value,)
    }
}
#[doc = "Default-param extender for [`ProjectSettings::load_resource_pack_ex`][super::ProjectSettings::load_resource_pack_ex]."]
#[must_use]
pub struct ExLoadResourcePack < 'a > {
    surround_object: &'a mut re_export::ProjectSettings, pack: GString, replace_files: bool, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadResourcePack < 'a > {
    fn new(surround_object: &'a mut re_export::ProjectSettings, pack: GString,) -> Self {
        Self {
            surround_object, pack, replace_files: true, offset: 0i32,
        }
    }
    #[inline]
    pub fn replace_files(self, value: bool) -> Self {
        Self {
            replace_files: value, .. self
        }
    }
    #[inline]
    pub fn offset(self, value: i32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::ProjectSettings::load_resource_pack_full(self.surround_object, self.pack, self.replace_files, self.offset,)
    }
}