#![doc = "Sidecar module for class [`ResourceLoader`][crate::engine::ResourceLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceLoader` enums](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceLoader.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`resource_loader`][crate::engine::resource_loader]: sidecar module with related enum/flag types\n* [`IResourceLoader`][crate::engine::IResourceLoader]: virtual methods\n\n\nSee also [Godot docs for `ResourceLoader`](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ResourceLoader`][crate::engine::ResourceLoader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ResourceLoader` methods](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResourceLoader: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ResourceLoader {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"ResourceLoader\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn load_full(&mut self, path: GString, type_hint: GString, cache_mode: crate::engine::resource_loader::CacheMode,) -> Option < Gd < crate::engine::Resource > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Resource > >;
            type CallSig = (Option < Gd < crate::engine::Resource > >, GString, GString, crate::engine::resource_loader::CacheMode);
            let args = (path, type_hint, cache_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load(&mut self, path: GString,) -> Option < Gd < crate::engine::Resource > > {
            self.load_ex(path,) . done()
        }
        #[inline]
        pub fn load_ex(&mut self, path: GString,) -> ExLoad < '_ > {
            ExLoad::new(self, path,)
        }
        pub fn get_recognized_extensions_for_type(&mut self, type_: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_recognized_extensions_for_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_resource_format_loader_full(&mut self, format_loader: Gd < crate::engine::ResourceFormatLoader >, at_front: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ResourceFormatLoader >, bool);
            let args = (format_loader, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_resource_format_loader", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_resource_format_loader(&mut self, format_loader: Gd < crate::engine::ResourceFormatLoader >,) {
            self.add_resource_format_loader_ex(format_loader,) . done()
        }
        #[inline]
        pub fn add_resource_format_loader_ex(&mut self, format_loader: Gd < crate::engine::ResourceFormatLoader >,) -> ExAddResourceFormatLoader < '_ > {
            ExAddResourceFormatLoader::new(self, format_loader,)
        }
        pub fn remove_resource_format_loader(&mut self, format_loader: Gd < crate::engine::ResourceFormatLoader >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ResourceFormatLoader >);
            let args = (format_loader,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_resource_format_loader", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_abort_on_missing_resources(&mut self, abort: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (abort,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_abort_on_missing_resources", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dependencies(&mut self, path: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_dependencies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_cached(&mut self, path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_cached", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn exists_full(&mut self, path: GString, type_hint: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, GString);
            let args = (path, type_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn exists(&mut self, path: GString,) -> bool {
            self.exists_ex(path,) . done()
        }
        #[inline]
        pub fn exists_ex(&mut self, path: GString,) -> ExExists < '_ > {
            ExExists::new(self, path,)
        }
        pub fn get_resource_uid(&mut self, path: GString,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resource_uid", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ResourceLoader {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ResourceLoader\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceLoader {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ResourceLoader {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for ResourceLoader {
        
    }
    impl std::ops::Deref for ResourceLoader {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ResourceLoader {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ResourceLoader > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ResourceLoader::load_ex`][super::ResourceLoader::load_ex]."]
#[must_use]
pub struct ExLoad < 'a > {
    surround_object: &'a mut re_export::ResourceLoader, path: GString, type_hint: GString, cache_mode: crate::engine::resource_loader::CacheMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoad < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: GString,) -> Self {
        Self {
            surround_object, path, type_hint: GString::from(""), cache_mode: crate::obj::EngineEnum::from_ord(1),
        }
    }
    #[inline]
    pub fn type_hint(self, value: GString) -> Self {
        Self {
            type_hint: value, .. self
        }
    }
    #[inline]
    pub fn cache_mode(self, value: crate::engine::resource_loader::CacheMode) -> Self {
        Self {
            cache_mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Resource > > {
        re_export::ResourceLoader::load_full(self.surround_object, self.path, self.type_hint, self.cache_mode,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::add_resource_format_loader_ex`][super::ResourceLoader::add_resource_format_loader_ex]."]
#[must_use]
pub struct ExAddResourceFormatLoader < 'a > {
    surround_object: &'a mut re_export::ResourceLoader, format_loader: Gd < crate::engine::ResourceFormatLoader >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatLoader < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, format_loader: Gd < crate::engine::ResourceFormatLoader >,) -> Self {
        Self {
            surround_object, format_loader, at_front: false,
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
        re_export::ResourceLoader::add_resource_format_loader_full(self.surround_object, self.format_loader, self.at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::exists_ex`][super::ResourceLoader::exists_ex]."]
#[must_use]
pub struct ExExists < 'a > {
    surround_object: &'a mut re_export::ResourceLoader, path: GString, type_hint: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExists < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: GString,) -> Self {
        Self {
            surround_object, path, type_hint: GString::from(""),
        }
    }
    #[inline]
    pub fn type_hint(self, value: GString) -> Self {
        Self {
            type_hint: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::ResourceLoader::exists_full(self.surround_object, self.path, self.type_hint,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ThreadLoadStatus {
    ord: i32
}
impl ThreadLoadStatus {
    pub const THREAD_LOAD_INVALID_RESOURCE: Self = Self {
        ord: 0i32
    };
    pub const THREAD_LOAD_IN_PROGRESS: Self = Self {
        ord: 1i32
    };
    pub const THREAD_LOAD_FAILED: Self = Self {
        ord: 2i32
    };
    pub const THREAD_LOAD_LOADED: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ThreadLoadStatus {
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
impl crate::builtin::meta::GodotConvert for ThreadLoadStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ThreadLoadStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ThreadLoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CacheMode {
    ord: i32
}
impl CacheMode {
    pub const CACHE_MODE_IGNORE: Self = Self {
        ord: 0i32
    };
    pub const CACHE_MODE_REUSE: Self = Self {
        ord: 1i32
    };
    pub const CACHE_MODE_REPLACE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CacheMode {
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
impl crate::builtin::meta::GodotConvert for CacheMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CacheMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CacheMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}