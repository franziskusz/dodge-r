#![doc = "Sidecar module for class [`VisualShader`][crate::engine::VisualShader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShader` enums](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShader.`\n\nInherits [`Shader`][crate::engine::Shader].\n\nRelated symbols:\n\n* [`visual_shader`][crate::engine::visual_shader]: sidecar module with related enum/flag types\n* [`IVisualShader`][crate::engine::IVisualShader]: virtual methods\n\n\nSee also [Godot docs for `VisualShader`](https://docs.godotengine.org/en/stable/classes/class_visualshader.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShader`][crate::engine::VisualShader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShader` methods](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShader: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShader {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_mode(&mut self, mode: crate::engine::shader::Mode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::shader::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_node(&mut self, type_: crate::engine::visual_shader::Type, node: Gd < crate::engine::VisualShaderNode >, position: Vector2, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, Gd < crate::engine::VisualShaderNode >, Vector2, i32);
            let args = (type_, node, position, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node(&self, type_: crate::engine::visual_shader::Type, id: i32,) -> Option < Gd < crate::engine::VisualShaderNode > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::VisualShaderNode > >;
            type CallSig = (Option < Gd < crate::engine::VisualShaderNode > >, crate::engine::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_node_position(&mut self, type_: crate::engine::visual_shader::Type, id: i32, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, i32, Vector2);
            let args = (type_, id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_position(&self, type_: crate::engine::visual_shader::Type, id: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, crate::engine::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_list(&self, type_: crate::engine::visual_shader::Type,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, crate::engine::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_valid_node_id(&self, type_: crate::engine::visual_shader::Type,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_valid_node_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node(&mut self, type_: crate::engine::visual_shader::Type, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn replace_node(&mut self, type_: crate::engine::visual_shader::Type, id: i32, new_class: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, i32, StringName);
            let args = (type_, id, new_class,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "replace_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_node_connection(&self, type_: crate::engine::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_node_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_connect_nodes(&self, type_: crate::engine::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "can_connect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_nodes(&mut self, type_: crate::engine::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, crate::engine::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_nodes(&mut self, type_: crate::engine::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_nodes_forced(&mut self, type_: crate::engine::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_nodes_forced", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_connections(&self, type_: crate::engine::visual_shader::Type,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, crate::engine::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_varying(&mut self, name: GString, mode: crate::engine::visual_shader::VaryingMode, type_: crate::engine::visual_shader::VaryingType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, crate::engine::visual_shader::VaryingMode, crate::engine::visual_shader::VaryingType);
            let args = (name, mode, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_varying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_varying(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_varying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_varying(&self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_varying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const NODE_ID_INVALID: i32 = - 1i32;
        pub const NODE_ID_OUTPUT: i32 = 0i32;
        
    }
    impl crate::obj::GodotClass for VisualShader {
        type Base = crate::engine::Shader;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShader\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShader {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Shader > for VisualShader {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShader {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShader {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShader {
        
    }
    impl crate::obj::ExportableObject for VisualShader {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShader {
        type Target = crate::engine::Shader;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShader {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShader > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Shader > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Type {
    ord: i32
}
impl Type {
    pub const TYPE_VERTEX: Self = Self {
        ord: 0i32
    };
    pub const TYPE_FRAGMENT: Self = Self {
        ord: 1i32
    };
    pub const TYPE_LIGHT: Self = Self {
        ord: 2i32
    };
    pub const TYPE_START: Self = Self {
        ord: 3i32
    };
    pub const TYPE_PROCESS: Self = Self {
        ord: 4i32
    };
    pub const TYPE_COLLIDE: Self = Self {
        ord: 5i32
    };
    pub const TYPE_START_CUSTOM: Self = Self {
        ord: 6i32
    };
    pub const TYPE_PROCESS_CUSTOM: Self = Self {
        ord: 7i32
    };
    pub const TYPE_SKY: Self = Self {
        ord: 8i32
    };
    pub const TYPE_FOG: Self = Self {
        ord: 9i32
    };
    pub const TYPE_MAX: Self = Self {
        ord: 10i32
    };
    
}
impl crate::obj::EngineEnum for Type {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Type {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::builtin::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Type {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VaryingMode {
    ord: i32
}
impl VaryingMode {
    pub const VARYING_MODE_VERTEX_TO_FRAG_LIGHT: Self = Self {
        ord: 0i32
    };
    pub const VARYING_MODE_FRAG_TO_LIGHT: Self = Self {
        ord: 1i32
    };
    pub const VARYING_MODE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for VaryingMode {
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
impl crate::obj::IndexEnum for VaryingMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for VaryingMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VaryingMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VaryingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VaryingType {
    ord: i32
}
impl VaryingType {
    pub const VARYING_TYPE_FLOAT: Self = Self {
        ord: 0i32
    };
    pub const VARYING_TYPE_INT: Self = Self {
        ord: 1i32
    };
    pub const VARYING_TYPE_UINT: Self = Self {
        ord: 2i32
    };
    pub const VARYING_TYPE_VECTOR_2D: Self = Self {
        ord: 3i32
    };
    pub const VARYING_TYPE_VECTOR_3D: Self = Self {
        ord: 4i32
    };
    pub const VARYING_TYPE_VECTOR_4D: Self = Self {
        ord: 5i32
    };
    pub const VARYING_TYPE_BOOLEAN: Self = Self {
        ord: 6i32
    };
    pub const VARYING_TYPE_TRANSFORM: Self = Self {
        ord: 7i32
    };
    pub const VARYING_TYPE_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for VaryingType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for VaryingType {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for VaryingType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VaryingType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VaryingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}