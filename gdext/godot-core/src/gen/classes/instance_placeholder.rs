#![doc = "Sidecar module for class [`InstancePlaceholder`][crate::engine::InstancePlaceholder].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InstancePlaceholder` enums](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InstancePlaceholder.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`instance_placeholder`][crate::engine::instance_placeholder]: sidecar module with related enum/flag types\n* [`IInstancePlaceholder`][crate::engine::IInstancePlaceholder]: virtual methods\n\n\nSee also [Godot docs for `InstancePlaceholder`](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InstancePlaceholder {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InstancePlaceholder`][crate::engine::InstancePlaceholder].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InstancePlaceholder` methods](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInstancePlaceholder: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InstancePlaceholder {
        pub(crate) fn get_stored_values_full(&mut self, with_order: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, bool);
            let args = (with_order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stored_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_stored_values(&mut self,) -> Dictionary {
            self.get_stored_values_ex() . done()
        }
        #[inline]
        pub fn get_stored_values_ex(&mut self,) -> ExGetStoredValues < '_ > {
            ExGetStoredValues::new(self,)
        }
        pub(crate) fn create_instance_full(&mut self, replace: bool, custom_scene: Gd < crate::engine::PackedScene >,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, bool, Gd < crate::engine::PackedScene >);
            let args = (replace, custom_scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_instance(&mut self,) -> Option < Gd < crate::engine::Node > > {
            self.create_instance_ex() . done()
        }
        #[inline]
        pub fn create_instance_ex(&mut self,) -> ExCreateInstance < '_ > {
            ExCreateInstance::new(self,)
        }
        pub fn get_instance_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_instance_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InstancePlaceholder {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"InstancePlaceholder\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InstancePlaceholder {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for InstancePlaceholder {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for InstancePlaceholder {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for InstancePlaceholder {
        
    }
    impl crate::obj::ExportableObject for InstancePlaceholder {
        
    }
    impl std::ops::Deref for InstancePlaceholder {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InstancePlaceholder {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_InstancePlaceholder {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::InstancePlaceholder > for $Class {
                
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
#[doc = "Default-param extender for [`InstancePlaceholder::get_stored_values_ex`][super::InstancePlaceholder::get_stored_values_ex]."]
#[must_use]
pub struct ExGetStoredValues < 'a > {
    surround_object: &'a mut re_export::InstancePlaceholder, with_order: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStoredValues < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        Self {
            surround_object, with_order: false,
        }
    }
    #[inline]
    pub fn with_order(self, value: bool) -> Self {
        Self {
            with_order: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        re_export::InstancePlaceholder::get_stored_values_full(self.surround_object, self.with_order,)
    }
}
#[doc = "Default-param extender for [`InstancePlaceholder::create_instance_ex`][super::InstancePlaceholder::create_instance_ex]."]
#[must_use]
pub struct ExCreateInstance < 'a > {
    surround_object: &'a mut re_export::InstancePlaceholder, replace: bool, custom_scene: Gd < crate::engine::PackedScene >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateInstance < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        Self {
            surround_object, replace: false, custom_scene: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn replace(self, value: bool) -> Self {
        Self {
            replace: value, .. self
        }
    }
    #[inline]
    pub fn custom_scene(self, value: Gd < crate::engine::PackedScene >) -> Self {
        Self {
            custom_scene: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::InstancePlaceholder::create_instance_full(self.surround_object, self.replace, self.custom_scene,)
    }
}