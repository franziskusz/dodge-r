#![doc = "Sidecar module for class [`ResourceSaver`][crate::engine::ResourceSaver].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceSaver` enums](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceSaver.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`resource_saver`][crate::engine::resource_saver]: sidecar module with related enum/flag types\n* [`IResourceSaver`][crate::engine::IResourceSaver]: virtual methods\n\n\nSee also [Godot docs for `ResourceSaver`](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceSaver {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ResourceSaver`][crate::engine::ResourceSaver].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ResourceSaver` methods](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResourceSaver: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ResourceSaver {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"ResourceSaver\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn save_full(&mut self, resource: Gd < crate::engine::Resource >, path: GString, flags: crate::engine::resource_saver::SaverFlags,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Resource >, GString, crate::engine::resource_saver::SaverFlags);
            let args = (resource, path, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save(&mut self, resource: Gd < crate::engine::Resource >,) -> crate::engine::global::Error {
            self.save_ex(resource,) . done()
        }
        #[inline]
        pub fn save_ex(&mut self, resource: Gd < crate::engine::Resource >,) -> ExSave < '_ > {
            ExSave::new(self, resource,)
        }
        pub fn get_recognized_extensions(&mut self, type_: Gd < crate::engine::Resource >,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, Gd < crate::engine::Resource >);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_recognized_extensions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_resource_format_saver_full(&mut self, format_saver: Gd < crate::engine::ResourceFormatSaver >, at_front: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ResourceFormatSaver >, bool);
            let args = (format_saver, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_resource_format_saver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_resource_format_saver(&mut self, format_saver: Gd < crate::engine::ResourceFormatSaver >,) {
            self.add_resource_format_saver_ex(format_saver,) . done()
        }
        #[inline]
        pub fn add_resource_format_saver_ex(&mut self, format_saver: Gd < crate::engine::ResourceFormatSaver >,) -> ExAddResourceFormatSaver < '_ > {
            ExAddResourceFormatSaver::new(self, format_saver,)
        }
        pub fn remove_resource_format_saver(&mut self, format_saver: Gd < crate::engine::ResourceFormatSaver >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ResourceFormatSaver >);
            let args = (format_saver,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_resource_format_saver", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ResourceSaver {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ResourceSaver\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceSaver {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ResourceSaver {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for ResourceSaver {
        
    }
    impl std::ops::Deref for ResourceSaver {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceSaver {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ResourceSaver {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ResourceSaver > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ResourceSaver::save_ex`][super::ResourceSaver::save_ex]."]
#[must_use]
pub struct ExSave < 'a > {
    surround_object: &'a mut re_export::ResourceSaver, resource: Gd < crate::engine::Resource >, path: GString, flags: crate::engine::resource_saver::SaverFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSave < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, resource: Gd < crate::engine::Resource >,) -> Self {
        Self {
            surround_object, resource, path: GString::from(""), flags: crate::obj::EngineBitfield::from_ord(0),
        }
    }
    #[inline]
    pub fn path(self, value: GString) -> Self {
        Self {
            path: value, .. self
        }
    }
    #[inline]
    pub fn flags(self, value: crate::engine::resource_saver::SaverFlags) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::ResourceSaver::save_full(self.surround_object, self.resource, self.path, self.flags,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::add_resource_format_saver_ex`][super::ResourceSaver::add_resource_format_saver_ex]."]
#[must_use]
pub struct ExAddResourceFormatSaver < 'a > {
    surround_object: &'a mut re_export::ResourceSaver, format_saver: Gd < crate::engine::ResourceFormatSaver >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatSaver < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, format_saver: Gd < crate::engine::ResourceFormatSaver >,) -> Self {
        Self {
            surround_object, format_saver, at_front: false,
        }
    }
    #[inline]
    pub fn at_front(self, value: bool) -> Self {
        Self {
            at_front: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ResourceSaver::add_resource_format_saver_full(self.surround_object, self.format_saver, self.at_front,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct SaverFlags {
    ord: u64
}
impl SaverFlags {
    pub const FLAG_NONE: Self = Self {
        ord: 0u64
    };
    pub const FLAG_RELATIVE_PATHS: Self = Self {
        ord: 1u64
    };
    pub const FLAG_BUNDLE_RESOURCES: Self = Self {
        ord: 2u64
    };
    pub const FLAG_CHANGE_PATH: Self = Self {
        ord: 4u64
    };
    pub const FLAG_OMIT_EDITOR_PROPERTIES: Self = Self {
        ord: 8u64
    };
    pub const FLAG_SAVE_BIG_ENDIAN: Self = Self {
        ord: 16u64
    };
    pub const FLAG_COMPRESS: Self = Self {
        ord: 32u64
    };
    pub const FLAG_REPLACE_SUBRESOURCE_PATHS: Self = Self {
        ord: 64u64
    };
    
}
impl crate::obj::EngineBitfield for SaverFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for SaverFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for SaverFlags {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for SaverFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SaverFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}