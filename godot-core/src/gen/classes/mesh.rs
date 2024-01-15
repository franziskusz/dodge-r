#![doc = "Sidecar module for class [`Mesh`][crate::engine::Mesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Mesh` enums](https://docs.godotengine.org/en/stable/classes/class_mesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Mesh.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`mesh`][crate::engine::mesh]: sidecar module with related enum/flag types\n* [`IMesh`][crate::engine::IMesh]: virtual methods\n\n\nSee also [Godot docs for `Mesh`](https://docs.godotengine.org/en/stable/classes/class_mesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Mesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Mesh`][crate::engine::Mesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Mesh` methods](https://docs.godotengine.org/en/stable/classes/class_mesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_surface_count(&self,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_index_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_arrays(&self, index: i32,) -> VariantArray {
            unimplemented !()
        }
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray > {
            unimplemented !()
        }
        fn surface_get_lods(&self, index: i32,) -> Dictionary {
            unimplemented !()
        }
        fn surface_get_format(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_get_primitive_type(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_set_material(&mut self, index: i32, material: Gd < crate::engine::Material >,) {
            unimplemented !()
        }
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::engine::Material > > {
            unimplemented !()
        }
        fn get_blend_shape_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_blend_shape_name(&self, index: i32,) -> StringName {
            unimplemented !()
        }
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Mesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_faces(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_arrays(&self, surf_idx: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_blend_shape_arrays(&self, surf_idx: i32,) -> Array < VariantArray > {
            type RetMarshal = PtrcallReturnT < Array < VariantArray > >;
            type CallSig = (Array < VariantArray >, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_blend_shape_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_material(&mut self, surf_idx: i32, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Material >);
            let args = (surf_idx, material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_material(&self, surf_idx: i32,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_placeholder(&self,) -> Option < Gd < crate::engine::Resource > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Resource > >;
            type CallSig = (Option < Gd < crate::engine::Resource > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_trimesh_shape(&self,) -> Option < Gd < crate::engine::ConcavePolygonShape3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ConcavePolygonShape3D > >;
            type CallSig = (Option < Gd < crate::engine::ConcavePolygonShape3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_trimesh_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_convex_shape_full(&self, clean: bool, simplify: bool,) -> Option < Gd < crate::engine::ConvexPolygonShape3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ConvexPolygonShape3D > >;
            type CallSig = (Option < Gd < crate::engine::ConvexPolygonShape3D > >, bool, bool);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_convex_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_convex_shape(&self,) -> Option < Gd < crate::engine::ConvexPolygonShape3D > > {
            self.create_convex_shape_ex() . done()
        }
        #[inline]
        pub fn create_convex_shape_ex(&self,) -> ExCreateConvexShape < '_ > {
            ExCreateConvexShape::new(self,)
        }
        pub fn create_outline(&self, margin: f32,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >, f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::engine::TriangleMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::TriangleMesh > >;
            type CallSig = (Option < Gd < crate::engine::TriangleMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_triangle_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Mesh {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Mesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Mesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Mesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Mesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Mesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Mesh {
        
    }
    impl crate::obj::ExportableObject for Mesh {
        
    }
    impl crate::obj::cap::GodotDefault for Mesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Mesh {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Mesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Mesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Mesh > for $Class {
                
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
#[doc = "Default-param extender for [`Mesh::create_convex_shape_ex`][super::Mesh::create_convex_shape_ex]."]
#[must_use]
pub struct ExCreateConvexShape < 'a > {
    surround_object: &'a re_export::Mesh, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateConvexShape < 'a > {
    fn new(surround_object: &'a re_export::Mesh,) -> Self {
        Self {
            surround_object, clean: true, simplify: false,
        }
    }
    #[inline]
    pub fn clean(self, value: bool) -> Self {
        Self {
            clean: value, .. self
        }
    }
    #[inline]
    pub fn simplify(self, value: bool) -> Self {
        Self {
            simplify: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::ConvexPolygonShape3D > > {
        re_export::Mesh::create_convex_shape_full(self.surround_object, self.clean, self.simplify,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PrimitiveType {
    ord: i32
}
impl PrimitiveType {
    pub const PRIMITIVE_POINTS: Self = Self {
        ord: 0i32
    };
    pub const PRIMITIVE_LINES: Self = Self {
        ord: 1i32
    };
    pub const PRIMITIVE_LINE_STRIP: Self = Self {
        ord: 2i32
    };
    pub const PRIMITIVE_TRIANGLES: Self = Self {
        ord: 3i32
    };
    pub const PRIMITIVE_TRIANGLE_STRIP: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for PrimitiveType {
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
impl crate::builtin::meta::GodotConvert for PrimitiveType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PrimitiveType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PrimitiveType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ArrayType {
    ord: i32
}
impl ArrayType {
    pub const ARRAY_VERTEX: Self = Self {
        ord: 0i32
    };
    pub const ARRAY_NORMAL: Self = Self {
        ord: 1i32
    };
    pub const ARRAY_TANGENT: Self = Self {
        ord: 2i32
    };
    pub const ARRAY_COLOR: Self = Self {
        ord: 3i32
    };
    pub const ARRAY_TEX_UV: Self = Self {
        ord: 4i32
    };
    pub const ARRAY_TEX_UV2: Self = Self {
        ord: 5i32
    };
    pub const ARRAY_CUSTOM0: Self = Self {
        ord: 6i32
    };
    pub const ARRAY_CUSTOM1: Self = Self {
        ord: 7i32
    };
    pub const ARRAY_CUSTOM2: Self = Self {
        ord: 8i32
    };
    pub const ARRAY_CUSTOM3: Self = Self {
        ord: 9i32
    };
    pub const ARRAY_BONES: Self = Self {
        ord: 10i32
    };
    pub const ARRAY_WEIGHTS: Self = Self {
        ord: 11i32
    };
    pub const ARRAY_INDEX: Self = Self {
        ord: 12i32
    };
    pub const ARRAY_MAX: Self = Self {
        ord: 13i32
    };
    
}
impl crate::obj::EngineEnum for ArrayType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for ArrayType {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::builtin::meta::GodotConvert for ArrayType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ArrayType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ArrayType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ArrayCustomFormat {
    ord: i32
}
impl ArrayCustomFormat {
    pub const ARRAY_CUSTOM_RGBA8_UNORM: Self = Self {
        ord: 0i32
    };
    pub const ARRAY_CUSTOM_RGBA8_SNORM: Self = Self {
        ord: 1i32
    };
    pub const ARRAY_CUSTOM_RG_HALF: Self = Self {
        ord: 2i32
    };
    pub const ARRAY_CUSTOM_RGBA_HALF: Self = Self {
        ord: 3i32
    };
    pub const ARRAY_CUSTOM_R_FLOAT: Self = Self {
        ord: 4i32
    };
    pub const ARRAY_CUSTOM_RG_FLOAT: Self = Self {
        ord: 5i32
    };
    pub const ARRAY_CUSTOM_RGB_FLOAT: Self = Self {
        ord: 6i32
    };
    pub const ARRAY_CUSTOM_RGBA_FLOAT: Self = Self {
        ord: 7i32
    };
    pub const ARRAY_CUSTOM_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
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
impl crate::obj::IndexEnum for ArrayCustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for ArrayCustomFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ArrayCustomFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ArrayCustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ArrayFormat {
    ord: u64
}
impl ArrayFormat {
    pub const ARRAY_FORMAT_VERTEX: Self = Self {
        ord: 1u64
    };
    pub const ARRAY_FORMAT_NORMAL: Self = Self {
        ord: 2u64
    };
    pub const ARRAY_FORMAT_TANGENT: Self = Self {
        ord: 4u64
    };
    pub const ARRAY_FORMAT_COLOR: Self = Self {
        ord: 8u64
    };
    pub const ARRAY_FORMAT_TEX_UV: Self = Self {
        ord: 16u64
    };
    pub const ARRAY_FORMAT_TEX_UV2: Self = Self {
        ord: 32u64
    };
    pub const ARRAY_FORMAT_CUSTOM0: Self = Self {
        ord: 64u64
    };
    pub const ARRAY_FORMAT_CUSTOM1: Self = Self {
        ord: 128u64
    };
    pub const ARRAY_FORMAT_CUSTOM2: Self = Self {
        ord: 256u64
    };
    pub const ARRAY_FORMAT_CUSTOM3: Self = Self {
        ord: 512u64
    };
    pub const ARRAY_FORMAT_BONES: Self = Self {
        ord: 1024u64
    };
    pub const ARRAY_FORMAT_WEIGHTS: Self = Self {
        ord: 2048u64
    };
    pub const ARRAY_FORMAT_INDEX: Self = Self {
        ord: 4096u64
    };
    pub const ARRAY_FORMAT_BLEND_SHAPE_MASK: Self = Self {
        ord: 7u64
    };
    pub const ARRAY_FORMAT_CUSTOM_BASE: Self = Self {
        ord: 13u64
    };
    pub const ARRAY_FORMAT_CUSTOM_BITS: Self = Self {
        ord: 3u64
    };
    pub const ARRAY_FORMAT_CUSTOM0_SHIFT: Self = Self {
        ord: 13u64
    };
    pub const ARRAY_FORMAT_CUSTOM1_SHIFT: Self = Self {
        ord: 16u64
    };
    pub const ARRAY_FORMAT_CUSTOM2_SHIFT: Self = Self {
        ord: 19u64
    };
    pub const ARRAY_FORMAT_CUSTOM3_SHIFT: Self = Self {
        ord: 22u64
    };
    pub const ARRAY_FORMAT_CUSTOM_MASK: Self = Self {
        ord: 7u64
    };
    pub const ARRAY_COMPRESS_FLAGS_BASE: Self = Self {
        ord: 25u64
    };
    pub const ARRAY_FLAG_USE_2D_VERTICES: Self = Self {
        ord: 33554432u64
    };
    pub const ARRAY_FLAG_USE_DYNAMIC_UPDATE: Self = Self {
        ord: 67108864u64
    };
    pub const ARRAY_FLAG_USE_8_BONE_WEIGHTS: Self = Self {
        ord: 134217728u64
    };
    pub const ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY: Self = Self {
        ord: 268435456u64
    };
    pub const ARRAY_FLAG_COMPRESS_ATTRIBUTES: Self = Self {
        ord: 536870912u64
    };
    
}
impl crate::obj::EngineBitfield for ArrayFormat {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for ArrayFormat {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for ArrayFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ArrayFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlendShapeMode {
    ord: i32
}
impl BlendShapeMode {
    pub const BLEND_SHAPE_MODE_NORMALIZED: Self = Self {
        ord: 0i32
    };
    pub const BLEND_SHAPE_MODE_RELATIVE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for BlendShapeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for BlendShapeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BlendShapeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BlendShapeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}