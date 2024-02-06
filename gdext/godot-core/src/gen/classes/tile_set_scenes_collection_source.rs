#![doc = "Sidecar module for class [`TileSetScenesCollectionSource`][crate::engine::TileSetScenesCollectionSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetScenesCollectionSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetScenesCollectionSource.`\n\nInherits [`TileSetSource`][crate::engine::TileSetSource].\n\nRelated symbols:\n\n* [`tile_set_scenes_collection_source`][crate::engine::tile_set_scenes_collection_source]: sidecar module with related enum/flag types\n* [`ITileSetScenesCollectionSource`][crate::engine::ITileSetScenesCollectionSource]: virtual methods\n\n\nSee also [Godot docs for `TileSetScenesCollectionSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetScenesCollectionSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileSetScenesCollectionSource`][crate::engine::TileSetScenesCollectionSource].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSetScenesCollectionSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSetScenesCollectionSource: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl TileSetScenesCollectionSource {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_scene_tiles_count(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_tiles_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_tile_id(&mut self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_scene_tile_id(&mut self, id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_scene_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_scene_tile_full(&mut self, packed_scene: Gd < crate::engine::PackedScene >, id_override: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::PackedScene >, i32);
            let args = (packed_scene, id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_scene_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_scene_tile(&mut self, packed_scene: Gd < crate::engine::PackedScene >,) -> i32 {
            self.create_scene_tile_ex(packed_scene,) . done()
        }
        #[inline]
        pub fn create_scene_tile_ex(&mut self, packed_scene: Gd < crate::engine::PackedScene >,) -> ExCreateSceneTile < '_ > {
            ExCreateSceneTile::new(self, packed_scene,)
        }
        pub fn set_scene_tile_id(&mut self, id: i32, new_id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (id, new_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_tile_scene(&mut self, id: i32, packed_scene: Gd < crate::engine::PackedScene >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::PackedScene >);
            let args = (id, packed_scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_tile_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_tile_scene(&self, id: i32,) -> Option < Gd < crate::engine::PackedScene > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PackedScene > >;
            type CallSig = (Option < Gd < crate::engine::PackedScene > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_tile_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_tile_display_placeholder(&mut self, id: i32, display_placeholder: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (id, display_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_tile_display_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_tile_display_placeholder(&self, id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_tile_display_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_scene_tile(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_scene_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_scene_tile_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next_scene_tile_id", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileSetScenesCollectionSource {
        type Base = crate::engine::TileSetSource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileSetScenesCollectionSource\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetScenesCollectionSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileSetScenesCollectionSource {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::TileSetSource > for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::ExportableObject for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::cap::GodotDefault for TileSetScenesCollectionSource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSetScenesCollectionSource {
        type Target = crate::engine::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetScenesCollectionSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileSetScenesCollectionSource {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileSetScenesCollectionSource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::TileSetSource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TileSetScenesCollectionSource::create_scene_tile_ex`][super::TileSetScenesCollectionSource::create_scene_tile_ex]."]
#[must_use]
pub struct ExCreateSceneTile < 'a > {
    surround_object: &'a mut re_export::TileSetScenesCollectionSource, packed_scene: Gd < crate::engine::PackedScene >, id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateSceneTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetScenesCollectionSource, packed_scene: Gd < crate::engine::PackedScene >,) -> Self {
        Self {
            surround_object, packed_scene, id_override: - 1i32,
        }
    }
    #[inline]
    pub fn id_override(self, value: i32) -> Self {
        Self {
            id_override: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::TileSetScenesCollectionSource::create_scene_tile_full(self.surround_object, self.packed_scene, self.id_override,)
    }
}