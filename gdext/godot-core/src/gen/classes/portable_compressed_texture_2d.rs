#![doc = "Sidecar module for class [`PortableCompressedTexture2D`][crate::engine::PortableCompressedTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PortableCompressedTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PortableCompressedTexture2D.`\n\nInherits [`Texture2D`][crate::engine::Texture2D].\n\nRelated symbols:\n\n* [`portable_compressed_texture_2d`][crate::engine::portable_compressed_texture_2d]: sidecar module with related enum/flag types\n* [`IPortableCompressedTexture2D`][crate::engine::IPortableCompressedTexture2D]: virtual methods\n\n\nSee also [Godot docs for `PortableCompressedTexture2D`](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PortableCompressedTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PortableCompressedTexture2D`][crate::engine::PortableCompressedTexture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PortableCompressedTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPortableCompressedTexture2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PortableCompressedTexture2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_from_image_full(&mut self, image: Gd < crate::engine::Image >, compression_mode: crate::engine::portable_compressed_texture_2d::CompressionMode, normal_map: bool, lossy_quality: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, crate::engine::portable_compressed_texture_2d::CompressionMode, bool, f32);
            let args = (image, compression_mode, normal_map, lossy_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_from_image(&mut self, image: Gd < crate::engine::Image >, compression_mode: crate::engine::portable_compressed_texture_2d::CompressionMode,) {
            self.create_from_image_ex(image, compression_mode,) . done()
        }
        #[inline]
        pub fn create_from_image_ex(&mut self, image: Gd < crate::engine::Image >, compression_mode: crate::engine::portable_compressed_texture_2d::CompressionMode,) -> ExCreateFromImage < '_ > {
            ExCreateFromImage::new(self, image, compression_mode,)
        }
        pub fn get_format(&self,) -> crate::engine::image::Format {
            type RetMarshal = PtrcallReturnT < crate::engine::image::Format >;
            type CallSig = (crate::engine::image::Format,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_compression_mode(&self,) -> crate::engine::portable_compressed_texture_2d::CompressionMode {
            type RetMarshal = PtrcallReturnT < crate::engine::portable_compressed_texture_2d::CompressionMode >;
            type CallSig = (crate::engine::portable_compressed_texture_2d::CompressionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_compression_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size_override(&mut self, size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size_override(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keep_compressed_buffer(&mut self, keep: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keep_compressed_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keeping_compressed_buffer(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_keeping_compressed_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keep_all_compressed_buffers(keep: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keep_all_compressed_buffers", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn is_keeping_all_compressed_buffers() -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_keeping_all_compressed_buffers", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for PortableCompressedTexture2D {
        type Base = crate::engine::Texture2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PortableCompressedTexture2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PortableCompressedTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PortableCompressedTexture2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Texture2D > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Texture > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::ExportableObject for PortableCompressedTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for PortableCompressedTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PortableCompressedTexture2D {
        type Target = crate::engine::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PortableCompressedTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PortableCompressedTexture2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PortableCompressedTexture2D > for $Class {
                
            }
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
#[doc = "Default-param extender for [`PortableCompressedTexture2D::create_from_image_ex`][super::PortableCompressedTexture2D::create_from_image_ex]."]
#[must_use]
pub struct ExCreateFromImage < 'a > {
    surround_object: &'a mut re_export::PortableCompressedTexture2D, image: Gd < crate::engine::Image >, compression_mode: crate::engine::portable_compressed_texture_2d::CompressionMode, normal_map: bool, lossy_quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromImage < 'a > {
    fn new(surround_object: &'a mut re_export::PortableCompressedTexture2D, image: Gd < crate::engine::Image >, compression_mode: crate::engine::portable_compressed_texture_2d::CompressionMode,) -> Self {
        Self {
            surround_object, image, compression_mode, normal_map: false, lossy_quality: 0.8f32,
        }
    }
    #[inline]
    pub fn normal_map(self, value: bool) -> Self {
        Self {
            normal_map: value, .. self
        }
    }
    #[inline]
    pub fn lossy_quality(self, value: f32) -> Self {
        Self {
            lossy_quality: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PortableCompressedTexture2D::create_from_image_full(self.surround_object, self.image, self.compression_mode, self.normal_map, self.lossy_quality,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    pub const COMPRESSION_MODE_LOSSLESS: Self = Self {
        ord: 0i32
    };
    pub const COMPRESSION_MODE_LOSSY: Self = Self {
        ord: 1i32
    };
    pub const COMPRESSION_MODE_BASIS_UNIVERSAL: Self = Self {
        ord: 2i32
    };
    pub const COMPRESSION_MODE_S3TC: Self = Self {
        ord: 3i32
    };
    pub const COMPRESSION_MODE_ETC2: Self = Self {
        ord: 4i32
    };
    pub const COMPRESSION_MODE_BPTC: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for CompressionMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompressionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}