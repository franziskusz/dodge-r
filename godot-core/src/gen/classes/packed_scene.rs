#![doc = "Sidecar module for class [`PackedScene`][crate::engine::PackedScene].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PackedScene` enums](https://docs.godotengine.org/en/stable/classes/class_packedscene.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PackedScene.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`packed_scene`][crate::engine::packed_scene]: sidecar module with related enum/flag types\n* [`IPackedScene`][crate::engine::IPackedScene]: virtual methods\n\n\nSee also [Godot docs for `PackedScene`](https://docs.godotengine.org/en/stable/classes/class_packedscene.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PackedScene {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PackedScene`][crate::engine::PackedScene].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PackedScene` methods](https://docs.godotengine.org/en/stable/classes/class_packedscene.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPackedScene: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PackedScene {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn pack(&mut self, path: Gd < crate::engine::Node >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::Node >);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pack", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn instantiate_full(&self, edit_state: crate::engine::packed_scene::GenEditState,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, crate::engine::packed_scene::GenEditState);
            let args = (edit_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn instantiate(&self,) -> Option < Gd < crate::engine::Node > > {
            self.instantiate_ex() . done()
        }
        #[inline]
        pub fn instantiate_ex(&self,) -> ExInstantiate < '_ > {
            ExInstantiate::new(self,)
        }
        pub fn can_instantiate(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state(&self,) -> Option < Gd < crate::engine::SceneState > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::SceneState > >;
            type CallSig = (Option < Gd < crate::engine::SceneState > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_state", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PackedScene {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PackedScene\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PackedScene {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PackedScene {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for PackedScene {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PackedScene {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PackedScene {
        
    }
    impl crate::obj::ExportableObject for PackedScene {
        
    }
    impl crate::obj::cap::GodotDefault for PackedScene {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PackedScene {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PackedScene {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PackedScene {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PackedScene > for $Class {
                
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
#[doc = "Default-param extender for [`PackedScene::instantiate_ex`][super::PackedScene::instantiate_ex]."]
#[must_use]
pub struct ExInstantiate < 'a > {
    surround_object: &'a re_export::PackedScene, edit_state: crate::engine::packed_scene::GenEditState,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstantiate < 'a > {
    fn new(surround_object: &'a re_export::PackedScene,) -> Self {
        Self {
            surround_object, edit_state: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn edit_state(self, value: crate::engine::packed_scene::GenEditState) -> Self {
        Self {
            edit_state: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Node > > {
        re_export::PackedScene::instantiate_full(self.surround_object, self.edit_state,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenEditState {
    ord: i32
}
impl GenEditState {
    pub const GEN_EDIT_STATE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const GEN_EDIT_STATE_INSTANCE: Self = Self {
        ord: 1i32
    };
    pub const GEN_EDIT_STATE_MAIN: Self = Self {
        ord: 2i32
    };
    pub const GEN_EDIT_STATE_MAIN_INHERITED: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for GenEditState {
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
impl crate::builtin::meta::GodotConvert for GenEditState {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GenEditState {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GenEditState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}