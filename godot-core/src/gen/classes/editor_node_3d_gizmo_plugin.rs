#![doc = "Sidecar module for class [`EditorNode3DGizmoPlugin`][crate::engine::EditorNode3DGizmoPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorNode3DGizmoPlugin.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`editor_node_3d_gizmo_plugin`][crate::engine::editor_node_3d_gizmo_plugin]: sidecar module with related enum/flag types\n* [`IEditorNode3DGizmoPlugin`][crate::engine::IEditorNode3DGizmoPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorNode3DGizmoPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorNode3DGizmoPlugin`][crate::engine::EditorNode3DGizmoPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorNode3DGizmoPlugin: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn has_gizmo(&self, for_node_3d: Gd < crate::engine::Node3D >,) -> bool {
            unimplemented !()
        }
        fn create_gizmo(&self, for_node_3d: Gd < crate::engine::Node3D >,) -> Option < Gd < crate::engine::EditorNode3DGizmo > > {
            unimplemented !()
        }
        fn get_gizmo_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_priority(&self,) -> i32 {
            unimplemented !()
        }
        fn can_be_hidden(&self,) -> bool {
            unimplemented !()
        }
        fn is_selectable_when_hidden(&self,) -> bool {
            unimplemented !()
        }
        fn redraw(&mut self, gizmo: Gd < crate::engine::EditorNode3DGizmo >,) {
            unimplemented !()
        }
        fn get_handle_name(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, handle_id: i32, secondary: bool,) -> GString {
            unimplemented !()
        }
        fn is_handle_highlighted(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, handle_id: i32, secondary: bool,) -> bool {
            unimplemented !()
        }
        fn get_handle_value(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, handle_id: i32, secondary: bool,) -> Variant {
            unimplemented !()
        }
        fn set_handle(&mut self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, handle_id: i32, secondary: bool, camera: Gd < crate::engine::Camera3D >, screen_pos: Vector2,) {
            unimplemented !()
        }
        fn commit_handle(&mut self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, handle_id: i32, secondary: bool, restore: Variant, cancel: bool,) {
            unimplemented !()
        }
        fn subgizmos_intersect_ray(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, camera: Gd < crate::engine::Camera3D >, screen_pos: Vector2,) -> i32 {
            unimplemented !()
        }
        fn subgizmos_intersect_frustum(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, camera: Gd < crate::engine::Camera3D >, frustum_planes: Array < Plane >,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_subgizmo_transform(&self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, subgizmo_id: i32,) -> Transform3D {
            unimplemented !()
        }
        fn set_subgizmo_transform(&mut self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, subgizmo_id: i32, transform: Transform3D,) {
            unimplemented !()
        }
        fn commit_subgizmos(&mut self, gizmo: Gd < crate::engine::EditorNode3DGizmo >, ids: PackedInt32Array, restores: Array < Transform3D >, cancel: bool,) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl EditorNode3DGizmoPlugin {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_material_full(&mut self, name: GString, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Color, bool, bool, bool);
            let args = (name, color, billboard, on_top, use_vertex_color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_material(&mut self, name: GString, color: Color,) {
            self.create_material_ex(name, color,) . done()
        }
        #[inline]
        pub fn create_material_ex(&mut self, name: GString, color: Color,) -> ExCreateMaterial < '_ > {
            ExCreateMaterial::new(self, name, color,)
        }
        pub(crate) fn create_icon_material_full(&mut self, name: GString, texture: Gd < crate::engine::Texture2D >, on_top: bool, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::Texture2D >, bool, Color);
            let args = (name, texture, on_top, color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_icon_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_icon_material(&mut self, name: GString, texture: Gd < crate::engine::Texture2D >,) {
            self.create_icon_material_ex(name, texture,) . done()
        }
        #[inline]
        pub fn create_icon_material_ex(&mut self, name: GString, texture: Gd < crate::engine::Texture2D >,) -> ExCreateIconMaterial < '_ > {
            ExCreateIconMaterial::new(self, name, texture,)
        }
        pub(crate) fn create_handle_material_full(&mut self, name: GString, billboard: bool, texture: Gd < crate::engine::Texture2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool, Gd < crate::engine::Texture2D >);
            let args = (name, billboard, texture,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_handle_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_handle_material(&mut self, name: GString,) {
            self.create_handle_material_ex(name,) . done()
        }
        #[inline]
        pub fn create_handle_material_ex(&mut self, name: GString,) -> ExCreateHandleMaterial < '_ > {
            ExCreateHandleMaterial::new(self, name,)
        }
        pub fn add_material(&mut self, name: GString, material: Gd < crate::engine::StandardMaterial3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Gd < crate::engine::StandardMaterial3D >);
            let args = (name, material,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_material_full(&mut self, name: GString, gizmo: Gd < crate::engine::EditorNode3DGizmo >,) -> Option < Gd < crate::engine::StandardMaterial3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::StandardMaterial3D > >;
            type CallSig = (Option < Gd < crate::engine::StandardMaterial3D > >, GString, Gd < crate::engine::EditorNode3DGizmo >);
            let args = (name, gizmo,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_material(&mut self, name: GString,) -> Option < Gd < crate::engine::StandardMaterial3D > > {
            self.get_material_ex(name,) . done()
        }
        #[inline]
        pub fn get_material_ex(&mut self, name: GString,) -> ExGetMaterial < '_ > {
            ExGetMaterial::new(self, name,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for EditorNode3DGizmoPlugin {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorNode3DGizmoPlugin\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorNode3DGizmoPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorNode3DGizmoPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::ExportableObject for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorNode3DGizmoPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorNode3DGizmoPlugin {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmoPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorNode3DGizmoPlugin {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorNode3DGizmoPlugin > for $Class {
                
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
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_material_ex`][super::EditorNode3DGizmoPlugin::create_material_ex]."]
#[must_use]
pub struct ExCreateMaterial < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, color: Color,) -> Self {
        Self {
            surround_object, name, color, billboard: false, on_top: false, use_vertex_color: false,
        }
    }
    #[inline]
    pub fn billboard(self, value: bool) -> Self {
        Self {
            billboard: value, .. self
        }
    }
    #[inline]
    pub fn on_top(self, value: bool) -> Self {
        Self {
            on_top: value, .. self
        }
    }
    #[inline]
    pub fn use_vertex_color(self, value: bool) -> Self {
        Self {
            use_vertex_color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmoPlugin::create_material_full(self.surround_object, self.name, self.color, self.billboard, self.on_top, self.use_vertex_color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_icon_material_ex`][super::EditorNode3DGizmoPlugin::create_icon_material_ex]."]
#[must_use]
pub struct ExCreateIconMaterial < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, texture: Gd < crate::engine::Texture2D >, on_top: bool, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateIconMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, texture: Gd < crate::engine::Texture2D >,) -> Self {
        Self {
            surround_object, name, texture, on_top: false, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn on_top(self, value: bool) -> Self {
        Self {
            on_top: value, .. self
        }
    }
    #[inline]
    pub fn color(self, value: Color) -> Self {
        Self {
            color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmoPlugin::create_icon_material_full(self.surround_object, self.name, self.texture, self.on_top, self.color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_handle_material_ex`][super::EditorNode3DGizmoPlugin::create_handle_material_ex]."]
#[must_use]
pub struct ExCreateHandleMaterial < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, billboard: bool, texture: Gd < crate::engine::Texture2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHandleMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString,) -> Self {
        Self {
            surround_object, name, billboard: false, texture: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn billboard(self, value: bool) -> Self {
        Self {
            billboard: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Gd < crate::engine::Texture2D >) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::EditorNode3DGizmoPlugin::create_handle_material_full(self.surround_object, self.name, self.billboard, self.texture,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::get_material_ex`][super::EditorNode3DGizmoPlugin::get_material_ex]."]
#[must_use]
pub struct ExGetMaterial < 'a > {
    surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString, gizmo: Gd < crate::engine::EditorNode3DGizmo >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: GString,) -> Self {
        Self {
            surround_object, name, gizmo: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn gizmo(self, value: Gd < crate::engine::EditorNode3DGizmo >) -> Self {
        Self {
            gizmo: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::StandardMaterial3D > > {
        re_export::EditorNode3DGizmoPlugin::get_material_full(self.surround_object, self.name, self.gizmo,)
    }
}