#![doc = "Sidecar module for class [`TileMapPattern`][crate::engine::TileMapPattern].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMapPattern` enums](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMapPattern.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`tile_map_pattern`][crate::engine::tile_map_pattern]: sidecar module with related enum/flag types\n* [`ITileMapPattern`][crate::engine::ITileMapPattern]: virtual methods\n\n\nSee also [Godot docs for `TileMapPattern`](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMapPattern {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileMapPattern`][crate::engine::TileMapPattern].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileMapPattern` methods](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMapPattern: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileMapPattern {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn set_cell_full(&mut self, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, i32, Vector2i, i32);
            let args = (coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_cell(&mut self, coords: Vector2i,) {
            self.set_cell_ex(coords,) . done()
        }
        #[inline]
        pub fn set_cell_ex(&mut self, coords: Vector2i,) -> ExSetCell < '_ > {
            ExSetCell::new(self, coords,)
        }
        pub fn has_cell(&self, coords: Vector2i,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_cell(&mut self, coords: Vector2i, update_size: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, bool);
            let args = (coords, update_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_source_id(&self, coords: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_atlas_coords(&self, coords: Vector2i,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_atlas_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_alternative_tile(&self, coords: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cell_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_empty", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileMapPattern {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TileMapPattern\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMapPattern {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TileMapPattern {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TileMapPattern {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TileMapPattern {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TileMapPattern {
        
    }
    impl crate::obj::ExportableObject for TileMapPattern {
        
    }
    impl crate::obj::cap::GodotDefault for TileMapPattern {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMapPattern {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMapPattern {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TileMapPattern {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TileMapPattern > for $Class {
                
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
#[doc = "Default-param extender for [`TileMapPattern::set_cell_ex`][super::TileMapPattern::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'a > {
    surround_object: &'a mut re_export::TileMapPattern, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCell < 'a > {
    fn new(surround_object: &'a mut re_export::TileMapPattern, coords: Vector2i,) -> Self {
        Self {
            surround_object, coords, source_id: - 1i32, atlas_coords: Vector2i::new(- 1 as _, - 1 as _), alternative_tile: - 1i32,
        }
    }
    #[inline]
    pub fn source_id(self, value: i32) -> Self {
        Self {
            source_id: value, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, value: Vector2i) -> Self {
        Self {
            atlas_coords: value, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, value: i32) -> Self {
        Self {
            alternative_tile: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TileMapPattern::set_cell_full(self.surround_object, self.coords, self.source_id, self.atlas_coords, self.alternative_tile,)
    }
}