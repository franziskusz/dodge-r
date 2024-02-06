#![doc = "Sidecar module for class [`Texture2D`][crate::engine::Texture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Texture2D` enums](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Texture2D.`\n\nInherits [`Texture`][crate::engine::Texture].\n\nRelated symbols:\n\n* [`texture_2d`][crate::engine::texture_2d]: sidecar module with related enum/flag types\n* [`ITexture2D`][crate::engine::ITexture2D]: virtual methods\n\n\nSee also [Godot docs for `Texture2D`](https://docs.godotengine.org/en/stable/classes/class_texture2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Texture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Texture2D`][crate::engine::Texture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Texture2D` methods](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITexture2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32 {
            unimplemented !()
        }
        fn get_height(&self,) -> i32 {
            unimplemented !()
        }
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Texture2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_alpha(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_full(&self, canvas_item: Rid, position: Vector2, modulate: Color, transpose: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Color, bool);
            let args = (canvas_item, position, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw(&self, canvas_item: Rid, position: Vector2,) {
            self.draw_ex(canvas_item, position,) . done()
        }
        #[inline]
        pub fn draw_ex(&self, canvas_item: Rid, position: Vector2,) -> ExDraw < '_ > {
            ExDraw::new(self, canvas_item, position,)
        }
        pub(crate) fn draw_rect_full(&self, canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, bool, Color, bool);
            let args = (canvas_item, rect, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_rect(&self, canvas_item: Rid, rect: Rect2, tile: bool,) {
            self.draw_rect_ex(canvas_item, rect, tile,) . done()
        }
        #[inline]
        pub fn draw_rect_ex(&self, canvas_item: Rid, rect: Rect2, tile: bool,) -> ExDrawRect < '_ > {
            ExDrawRect::new(self, canvas_item, rect, tile,)
        }
        pub(crate) fn draw_rect_region_full(&self, canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rect2, Color, bool, bool);
            let args = (canvas_item, rect, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_rect_region(&self, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) {
            self.draw_rect_region_ex(canvas_item, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_rect_region_ex(&self, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) -> ExDrawRectRegion < '_ > {
            ExDrawRectRegion::new(self, canvas_item, rect, src_rect,)
        }
        pub fn get_image(&self,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_placeholder(&self,) -> Option < Gd < crate::engine::Resource > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Resource > >;
            type CallSig = (Option < Gd < crate::engine::Resource > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_placeholder", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Texture2D {
        type Base = crate::engine::Texture;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Texture2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Texture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Texture2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Texture > for Texture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Texture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Texture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Texture2D {
        
    }
    impl crate::obj::ExportableObject for Texture2D {
        
    }
    impl crate::obj::cap::GodotDefault for Texture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Texture2D {
        type Target = crate::engine::Texture;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Texture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Texture2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Texture2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Texture > for $Class {
                
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
#[doc = "Default-param extender for [`Texture2D::draw_ex`][super::Texture2D::draw_ex]."]
#[must_use]
pub struct ExDraw < 'a > {
    surround_object: &'a re_export::Texture2D, canvas_item: Rid, position: Vector2, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDraw < 'a > {
    fn new(surround_object: &'a re_export::Texture2D, canvas_item: Rid, position: Vector2,) -> Self {
        Self {
            surround_object, canvas_item, position, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn transpose(self, value: bool) -> Self {
        Self {
            transpose: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Texture2D::draw_full(self.surround_object, self.canvas_item, self.position, self.modulate, self.transpose,)
    }
}
#[doc = "Default-param extender for [`Texture2D::draw_rect_ex`][super::Texture2D::draw_rect_ex]."]
#[must_use]
pub struct ExDrawRect < 'a > {
    surround_object: &'a re_export::Texture2D, canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawRect < 'a > {
    fn new(surround_object: &'a re_export::Texture2D, canvas_item: Rid, rect: Rect2, tile: bool,) -> Self {
        Self {
            surround_object, canvas_item, rect, tile, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn transpose(self, value: bool) -> Self {
        Self {
            transpose: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Texture2D::draw_rect_full(self.surround_object, self.canvas_item, self.rect, self.tile, self.modulate, self.transpose,)
    }
}
#[doc = "Default-param extender for [`Texture2D::draw_rect_region_ex`][super::Texture2D::draw_rect_region_ex]."]
#[must_use]
pub struct ExDrawRectRegion < 'a > {
    surround_object: &'a re_export::Texture2D, canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawRectRegion < 'a > {
    fn new(surround_object: &'a re_export::Texture2D, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) -> Self {
        Self {
            surround_object, canvas_item, rect, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false, clip_uv: true,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn transpose(self, value: bool) -> Self {
        Self {
            transpose: value, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, value: bool) -> Self {
        Self {
            clip_uv: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Texture2D::draw_rect_region_full(self.surround_object, self.canvas_item, self.rect, self.src_rect, self.modulate, self.transpose, self.clip_uv,)
    }
}