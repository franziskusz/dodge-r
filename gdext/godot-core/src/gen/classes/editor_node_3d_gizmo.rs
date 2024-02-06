#![doc = "Sidecar module for class [`EditorNode3DGizmo`][crate::engine::EditorNode3DGizmo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmo` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorNode3DGizmo.`\n\nInherits [`Node3DGizmo`][crate::engine::Node3DGizmo].\n\nRelated symbols:\n\n* [`editor_node_3d_gizmo`][crate::engine::editor_node_3d_gizmo]: sidecar module with related enum/flag types\n* [`IEditorNode3DGizmo`][crate::engine::IEditorNode3DGizmo]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmo`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorNode3DGizmo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorNode3DGizmo`][crate::engine::EditorNode3DGizmo].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorNode3DGizmo` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorNode3DGizmo: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn redraw(&mut self,) {
            unimplemented !()
        }
        fn get_handle_name(&self, id: i32, secondary: bool,) -> GString {
            unimplemented !()
        }
        fn is_handle_highlighted(&self, id: i32, secondary: bool,) -> bool {
            unimplemented !()
        }
        fn get_handle_value(&self, id: i32, secondary: bool,) -> Variant {
            unimplemented !()
        }
        fn set_handle(&mut self, id: i32, secondary: bool, camera: Gd < crate::engine::Camera3D >, point: Vector2,) {
            unimplemented !()
        }
        fn commit_handle(&mut self, id: i32, secondary: bool, restore: Variant, cancel: bool,) {
            unimplemented !()
        }
        fn subgizmos_intersect_ray(&self, camera: Gd < crate::engine::Camera3D >, point: Vector2,) -> i32 {
            unimplemented !()
        }
        fn subgizmos_intersect_frustum(&self, camera: Gd < crate::engine::Camera3D >, frustum: Array < Plane >,) -> PackedInt32Array {
            unimplemented !()
        }
        fn set_subgizmo_transform(&mut self, id: i32, transform: Transform3D,) {
            unimplemented !()
        }
        fn get_subgizmo_transform(&self, id: i32,) -> Transform3D {
            unimplemented !()
        }
        fn commit_subgizmos(&mut self, ids: PackedInt32Array, restores: Array < Transform3D >, cancel: bool,) {
            unimplemented !()
        }
    }
    impl EditorNode3DGizmo {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_lines_full(&mut self, lines: PackedVector3Array, material: Gd < crate::engine::Material >, billboard: bool, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array, Gd < crate::engine::Material >, bool, Color);
            let args = (lines, material, billboard, modulate,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_lines(&mut self, lines: PackedVector3Array, material: Gd < crate::engine::Material >,) {
            self.add_lines_ex(lines, material,) . done()
        }
        #[inline]
        pub fn add_lines_ex(&mut self, lines: PackedVector3Array, material: Gd < crate::engine::Material >,) -> ExAddLines < '_ > {
            ExAddLines::new(self, lines, material,)
        }
        pub(crate) fn add_mesh_full(&mut self, mesh: Gd < crate::engine::Mesh >, material: Gd < crate::engine::Material >, transform: Transform3D, skeleton: Gd < crate::engine::SkinReference >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >, Gd < crate::engine::Material >, Transform3D, Gd < crate::engine::SkinReference >);
            let args = (mesh, material, transform, skeleton,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_mesh(&mut self, mesh: Gd < crate::engine::Mesh >,) {
            self.add_mesh_ex(mesh,) . done()
        }
        #[inline]
        pub fn add_mesh_ex(&mut self, mesh: Gd < crate::engine::Mesh >,) -> ExAddMesh < '_ > {
            ExAddMesh::new(self, mesh,)
        }
        pub fn add_collision_segments(&mut self, segments: PackedVector3Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array);
            let args = (segments,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_collision_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_triangles(&mut self, triangles: Gd < crate::engine::TriangleMesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::TriangleMesh >);
            let args = (triangles,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_collision_triangles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_unscaled_billboard_full(&mut self, material: Gd < crate::engine::Material >, default_scale: f32, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >, f32, Color);
            let args = (material, default_scale, modulate,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_unscaled_billboard", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_unscaled_billboard(&mut self, material: Gd < crate::engine::Material >,) {
            self.add_unscaled_billboard_ex(material,) . done()
        }
        #[inline]
        pub fn add_unscaled_billboard_ex(&mut self, material: Gd < crate::engine::Material >,) -> ExAddUnscaledBillboard < '_ > {
            ExAddUnscaledBillboard::new(self, material,)
        }
        pub(crate) fn add_handles_full(&mut self, handles: PackedVector3Array, material: Gd < crate::engine::Material >, ids: PackedInt32Array, billboard: bool, secondary: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array, Gd < crate::engine::Material >, PackedInt32Array, bool, bool);
            let args = (handles, material, ids, billboard, secondary,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_handles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_handles(&mut self, handles: PackedVector3Array, material: Gd < crate::engine::Material >, ids: PackedInt32Array,) {
            self.add_handles_ex(handles, material, ids,) . done()
        }
        #[inline]
        pub fn add_handles_ex(&mut self, handles: PackedVector3Array, material: Gd < crate::engine::Material >, ids: PackedInt32Array,) -> ExAddHandles < '_ > {
            ExAddHandles::new(self, handles, material, ids,)
        }
        pub fn set_node_3d(&mut self, node: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_node_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_3d(&self,) -> Option < Gd < crate::engine::Node3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node3D > >;
            type CallSig = (Option < Gd < crate::engine::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_plugin(&self,) -> Option < Gd < crate::engine::EditorNode3DGizmoPlugin > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorNode3DGizmoPlugin > >;
            type CallSig = (Option < Gd < crate::engine::EditorNode3DGizmoPlugin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hidden(&mut self, hidden: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (hidden,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_subgizmo_selected(&self, id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_subgizmo_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subgizmo_selection(&self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subgizmo_selection", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorNode3DGizmo {
        type Base = crate::engine::Node3DGizmo;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorNode3DGizmo\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorNode3DGizmo {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorNode3DGizmo {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3DGizmo > for EditorNode3DGizmo {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorNode3DGizmo {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorNode3DGizmo {
        
    }
    impl crate::obj::cap::GodotDefault for EditorNode3DGizmo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorNode3DGizmo {
        type Target = crate::engine::Node3DGizmo;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorNode3DGizmo {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorNode3DGizmo > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3DGizmo > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_lines_ex`][super::EditorNode3DGizmo::add_lines_ex]."]
#[must_use]
pub struct ExAddLines < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmo, lines: PackedVector3Array, material: Gd < crate::engine::Material >, billboard: bool, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddLines < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, lines: PackedVector3Array, material: Gd < crate::engine::Material >,) -> Self {
        Self {
            surround_object, lines, material, billboard: false, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn billboard(self, value: bool) -> Self {
        Self {
            billboard: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmo::add_lines_full(self.surround_object, self.lines, self.material, self.billboard, self.modulate,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_mesh_ex`][super::EditorNode3DGizmo::add_mesh_ex]."]
#[must_use]
pub struct ExAddMesh < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmo, mesh: Gd < crate::engine::Mesh >, material: Gd < crate::engine::Material >, transform: Transform3D, skeleton: Gd < crate::engine::SkinReference >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMesh < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, mesh: Gd < crate::engine::Mesh >,) -> Self {
        Self {
            surround_object, mesh, material: unimplemented !("see #156"), transform: Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _), skeleton: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn material(self, value: Gd < crate::engine::Material >) -> Self {
        Self {
            material: value, .. self
        }
    }
    #[inline]
    pub fn transform(self, value: Transform3D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn skeleton(self, value: Gd < crate::engine::SkinReference >) -> Self {
        Self {
            skeleton: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmo::add_mesh_full(self.surround_object, self.mesh, self.material, self.transform, self.skeleton,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_unscaled_billboard_ex`][super::EditorNode3DGizmo::add_unscaled_billboard_ex]."]
#[must_use]
pub struct ExAddUnscaledBillboard < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmo, material: Gd < crate::engine::Material >, default_scale: f32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddUnscaledBillboard < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, material: Gd < crate::engine::Material >,) -> Self {
        Self {
            surround_object, material, default_scale: 1f32, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn default_scale(self, value: f32) -> Self {
        Self {
            default_scale: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmo::add_unscaled_billboard_full(self.surround_object, self.material, self.default_scale, self.modulate,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_handles_ex`][super::EditorNode3DGizmo::add_handles_ex]."]
#[must_use]
pub struct ExAddHandles < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmo, handles: PackedVector3Array, material: Gd < crate::engine::Material >, ids: PackedInt32Array, billboard: bool, secondary: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddHandles < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, handles: PackedVector3Array, material: Gd < crate::engine::Material >, ids: PackedInt32Array,) -> Self {
        Self {
            surround_object, handles, material, ids, billboard: false, secondary: false,
        }
    }
    #[inline]
    pub fn billboard(self, value: bool) -> Self {
        Self {
            billboard: value, .. self
        }
    }
    #[inline]
    pub fn secondary(self, value: bool) -> Self {
        Self {
            secondary: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmo::add_handles_full(self.surround_object, self.handles, self.material, self.ids, self.billboard, self.secondary,)
    }
}