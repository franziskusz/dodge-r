#![doc = "Sidecar module for class [`RenderingDevice`][crate::engine::RenderingDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingDevice` enums](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingDevice.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`rendering_device`][crate::engine::rendering_device]: sidecar module with related enum/flag types\n* [`IRenderingDevice`][crate::engine::IRenderingDevice]: virtual methods\n\n\nSee also [Godot docs for `RenderingDevice`](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingDevice {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RenderingDevice`][crate::engine::RenderingDevice].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderingDevice` methods](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderingDevice: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderingDevice {
        pub(crate) fn texture_create_full(&mut self, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >, data: Array < PackedByteArray >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::RdTextureFormat >, Gd < crate::engine::RdTextureView >, Array < PackedByteArray >);
            let args = (format, view, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_create(&mut self, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >,) -> Rid {
            self.texture_create_ex(format, view,) . done()
        }
        #[inline]
        pub fn texture_create_ex(&mut self, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >,) -> ExTextureCreate < '_ > {
            ExTextureCreate::new(self, format, view,)
        }
        pub fn texture_create_shared(&mut self, view: Gd < crate::engine::RdTextureView >, with_texture: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::RdTextureView >, Rid);
            let args = (view, with_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_create_shared", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn texture_create_shared_from_slice_full(&mut self, view: Gd < crate::engine::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::engine::rendering_device::TextureSliceType,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::RdTextureView >, Rid, u32, u32, u32, crate::engine::rendering_device::TextureSliceType);
            let args = (view, with_texture, layer, mipmap, mipmaps, slice_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_create_shared_from_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_create_shared_from_slice(&mut self, view: Gd < crate::engine::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> Rid {
            self.texture_create_shared_from_slice_ex(view, with_texture, layer, mipmap,) . done()
        }
        #[inline]
        pub fn texture_create_shared_from_slice_ex(&mut self, view: Gd < crate::engine::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> ExTextureCreateSharedFromSlice < '_ > {
            ExTextureCreateSharedFromSlice::new(self, view, with_texture, layer, mipmap,)
        }
        pub fn texture_create_from_extension(&mut self, type_: crate::engine::rendering_device::TextureType, format: crate::engine::rendering_device::DataFormat, samples: crate::engine::rendering_device::TextureSamples, usage_flags: crate::engine::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, crate::engine::rendering_device::TextureType, crate::engine::rendering_device::DataFormat, crate::engine::rendering_device::TextureSamples, crate::engine::rendering_device::TextureUsageBits, u64, u64, u64, u64, u64);
            let args = (type_, format, samples, usage_flags, image, width, height, depth, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_create_from_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn texture_update_full(&mut self, texture: Rid, layer: u32, data: PackedByteArray, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, u32, PackedByteArray, crate::engine::rendering_device::BarrierMask);
            let args = (texture, layer, data, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_update(&mut self, texture: Rid, layer: u32, data: PackedByteArray,) -> crate::engine::global::Error {
            self.texture_update_ex(texture, layer, data,) . done()
        }
        #[inline]
        pub fn texture_update_ex(&mut self, texture: Rid, layer: u32, data: PackedByteArray,) -> ExTextureUpdate < '_ > {
            ExTextureUpdate::new(self, texture, layer, data,)
        }
        pub fn texture_get_data(&mut self, texture: Rid, layer: u32,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Rid, u32);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_format_supported_for_usage(&self, format: crate::engine::rendering_device::DataFormat, usage_flags: crate::engine::rendering_device::TextureUsageBits,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::rendering_device::DataFormat, crate::engine::rendering_device::TextureUsageBits);
            let args = (format, usage_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_is_format_supported_for_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_shared(&mut self, texture: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_is_shared", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_valid(&mut self, texture: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn texture_copy_full(&mut self, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, Rid, Vector3, Vector3, Vector3, u32, u32, u32, u32, crate::engine::rendering_device::BarrierMask);
            let args = (from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_copy(&mut self, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32,) -> crate::engine::global::Error {
            self.texture_copy_ex(from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer,) . done()
        }
        #[inline]
        pub fn texture_copy_ex(&mut self, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32,) -> ExTextureCopy < '_ > {
            ExTextureCopy::new(self, from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer,)
        }
        pub(crate) fn texture_clear_full(&mut self, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, Color, u32, u32, u32, u32, crate::engine::rendering_device::BarrierMask);
            let args = (texture, color, base_mipmap, mipmap_count, base_layer, layer_count, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_clear(&mut self, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32,) -> crate::engine::global::Error {
            self.texture_clear_ex(texture, color, base_mipmap, mipmap_count, base_layer, layer_count,) . done()
        }
        #[inline]
        pub fn texture_clear_ex(&mut self, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32,) -> ExTextureClear < '_ > {
            ExTextureClear::new(self, texture, color, base_mipmap, mipmap_count, base_layer, layer_count,)
        }
        pub(crate) fn texture_resolve_multisample_full(&mut self, from_texture: Rid, to_texture: Rid, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, Rid, crate::engine::rendering_device::BarrierMask);
            let args = (from_texture, to_texture, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_resolve_multisample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_resolve_multisample(&mut self, from_texture: Rid, to_texture: Rid,) -> crate::engine::global::Error {
            self.texture_resolve_multisample_ex(from_texture, to_texture,) . done()
        }
        #[inline]
        pub fn texture_resolve_multisample_ex(&mut self, from_texture: Rid, to_texture: Rid,) -> ExTextureResolveMultisample < '_ > {
            ExTextureResolveMultisample::new(self, from_texture, to_texture,)
        }
        pub fn texture_get_format(&mut self, texture: Rid,) -> Option < Gd < crate::engine::RdTextureFormat > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RdTextureFormat > >;
            type CallSig = (Option < Gd < crate::engine::RdTextureFormat > >, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_native_handle(&mut self, texture: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_native_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn framebuffer_format_create_full(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, view_count: u32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Array < Gd < crate::engine::RdAttachmentFormat > >, u32);
            let args = (attachments, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_format_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_format_create(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >,) -> i64 {
            self.framebuffer_format_create_ex(attachments,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_ex(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >,) -> ExFramebufferFormatCreate < '_ > {
            ExFramebufferFormatCreate::new(self, attachments,)
        }
        pub(crate) fn framebuffer_format_create_multipass_full(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, passes: Array < Gd < crate::engine::RdFramebufferPass > >, view_count: u32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Array < Gd < crate::engine::RdAttachmentFormat > >, Array < Gd < crate::engine::RdFramebufferPass > >, u32);
            let args = (attachments, passes, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_format_create_multipass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_format_create_multipass(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> i64 {
            self.framebuffer_format_create_multipass_ex(attachments, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_multipass_ex(&mut self, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> ExFramebufferFormatCreateMultipass < '_ > {
            ExFramebufferFormatCreateMultipass::new(self, attachments, passes,)
        }
        pub(crate) fn framebuffer_format_create_empty_full(&mut self, samples: crate::engine::rendering_device::TextureSamples,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, crate::engine::rendering_device::TextureSamples);
            let args = (samples,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_format_create_empty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_format_create_empty(&mut self,) -> i64 {
            self.framebuffer_format_create_empty_ex() . done()
        }
        #[inline]
        pub fn framebuffer_format_create_empty_ex(&mut self,) -> ExFramebufferFormatCreateEmpty < '_ > {
            ExFramebufferFormatCreateEmpty::new(self,)
        }
        pub(crate) fn framebuffer_format_get_texture_samples_full(&mut self, format: i64, render_pass: u32,) -> crate::engine::rendering_device::TextureSamples {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::TextureSamples >;
            type CallSig = (crate::engine::rendering_device::TextureSamples, i64, u32);
            let args = (format, render_pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_format_get_texture_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_format_get_texture_samples(&mut self, format: i64,) -> crate::engine::rendering_device::TextureSamples {
            self.framebuffer_format_get_texture_samples_ex(format,) . done()
        }
        #[inline]
        pub fn framebuffer_format_get_texture_samples_ex(&mut self, format: i64,) -> ExFramebufferFormatGetTextureSamples < '_ > {
            ExFramebufferFormatGetTextureSamples::new(self, format,)
        }
        pub(crate) fn framebuffer_create_full(&mut self, textures: Array < Rid >, validate_with_format: i64, view_count: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Array < Rid >, i64, u32);
            let args = (textures, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_create(&mut self, textures: Array < Rid >,) -> Rid {
            self.framebuffer_create_ex(textures,) . done()
        }
        #[inline]
        pub fn framebuffer_create_ex(&mut self, textures: Array < Rid >,) -> ExFramebufferCreate < '_ > {
            ExFramebufferCreate::new(self, textures,)
        }
        pub(crate) fn framebuffer_create_multipass_full(&mut self, textures: Array < Rid >, passes: Array < Gd < crate::engine::RdFramebufferPass > >, validate_with_format: i64, view_count: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Array < Rid >, Array < Gd < crate::engine::RdFramebufferPass > >, i64, u32);
            let args = (textures, passes, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_create_multipass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_create_multipass(&mut self, textures: Array < Rid >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> Rid {
            self.framebuffer_create_multipass_ex(textures, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_create_multipass_ex(&mut self, textures: Array < Rid >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> ExFramebufferCreateMultipass < '_ > {
            ExFramebufferCreateMultipass::new(self, textures, passes,)
        }
        pub(crate) fn framebuffer_create_empty_full(&mut self, size: Vector2i, samples: crate::engine::rendering_device::TextureSamples, validate_with_format: i64,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Vector2i, crate::engine::rendering_device::TextureSamples, i64);
            let args = (size, samples, validate_with_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_create_empty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn framebuffer_create_empty(&mut self, size: Vector2i,) -> Rid {
            self.framebuffer_create_empty_ex(size,) . done()
        }
        #[inline]
        pub fn framebuffer_create_empty_ex(&mut self, size: Vector2i,) -> ExFramebufferCreateEmpty < '_ > {
            ExFramebufferCreateEmpty::new(self, size,)
        }
        pub fn framebuffer_get_format(&mut self, framebuffer: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn framebuffer_is_valid(&self, framebuffer: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "framebuffer_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sampler_create(&mut self, state: Gd < crate::engine::RdSamplerState >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::RdSamplerState >);
            let args = (state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sampler_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sampler_is_format_supported_for_filter(&self, format: crate::engine::rendering_device::DataFormat, sampler_filter: crate::engine::rendering_device::SamplerFilter,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::rendering_device::DataFormat, crate::engine::rendering_device::SamplerFilter);
            let args = (format, sampler_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sampler_is_format_supported_for_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vertex_buffer_create_full(&mut self, size_bytes: u32, data: PackedByteArray, use_as_storage: bool,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, PackedByteArray, bool);
            let args = (size_bytes, data, use_as_storage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "vertex_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn vertex_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.vertex_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn vertex_buffer_create_ex(&mut self, size_bytes: u32,) -> ExVertexBufferCreate < '_ > {
            ExVertexBufferCreate::new(self, size_bytes,)
        }
        pub fn vertex_format_create(&mut self, vertex_descriptions: Array < Gd < crate::engine::RdVertexAttribute > >,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Array < Gd < crate::engine::RdVertexAttribute > >);
            let args = (vertex_descriptions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "vertex_format_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vertex_array_create_full(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: Array < Rid >, offsets: PackedInt64Array,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, i64, Array < Rid >, PackedInt64Array);
            let args = (vertex_count, vertex_format, src_buffers, offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "vertex_array_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn vertex_array_create(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: Array < Rid >,) -> Rid {
            self.vertex_array_create_ex(vertex_count, vertex_format, src_buffers,) . done()
        }
        #[inline]
        pub fn vertex_array_create_ex(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: Array < Rid >,) -> ExVertexArrayCreate < '_ > {
            ExVertexArrayCreate::new(self, vertex_count, vertex_format, src_buffers,)
        }
        pub(crate) fn index_buffer_create_full(&mut self, size_indices: u32, format: crate::engine::rendering_device::IndexBufferFormat, data: PackedByteArray, use_restart_indices: bool,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, crate::engine::rendering_device::IndexBufferFormat, PackedByteArray, bool);
            let args = (size_indices, format, data, use_restart_indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "index_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn index_buffer_create(&mut self, size_indices: u32, format: crate::engine::rendering_device::IndexBufferFormat,) -> Rid {
            self.index_buffer_create_ex(size_indices, format,) . done()
        }
        #[inline]
        pub fn index_buffer_create_ex(&mut self, size_indices: u32, format: crate::engine::rendering_device::IndexBufferFormat,) -> ExIndexBufferCreate < '_ > {
            ExIndexBufferCreate::new(self, size_indices, format,)
        }
        pub fn index_array_create(&mut self, index_buffer: Rid, index_offset: u32, index_count: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, u32, u32);
            let args = (index_buffer, index_offset, index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "index_array_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shader_compile_spirv_from_source_full(&mut self, shader_source: Gd < crate::engine::RdShaderSource >, allow_cache: bool,) -> Option < Gd < crate::engine::RdShaderSpirv > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RdShaderSpirv > >;
            type CallSig = (Option < Gd < crate::engine::RdShaderSpirv > >, Gd < crate::engine::RdShaderSource >, bool);
            let args = (shader_source, allow_cache,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_compile_spirv_from_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_compile_spirv_from_source(&mut self, shader_source: Gd < crate::engine::RdShaderSource >,) -> Option < Gd < crate::engine::RdShaderSpirv > > {
            self.shader_compile_spirv_from_source_ex(shader_source,) . done()
        }
        #[inline]
        pub fn shader_compile_spirv_from_source_ex(&mut self, shader_source: Gd < crate::engine::RdShaderSource >,) -> ExShaderCompileSpirvFromSource < '_ > {
            ExShaderCompileSpirvFromSource::new(self, shader_source,)
        }
        pub(crate) fn shader_compile_binary_from_spirv_full(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >, name: GString,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Gd < crate::engine::RdShaderSpirv >, GString);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_compile_binary_from_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_compile_binary_from_spirv(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> PackedByteArray {
            self.shader_compile_binary_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_compile_binary_from_spirv_ex(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> ExShaderCompileBinaryFromSpirv < '_ > {
            ExShaderCompileBinaryFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_spirv_full(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >, name: GString,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::RdShaderSpirv >, GString);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_create_from_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_create_from_spirv(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> Rid {
            self.shader_create_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_spirv_ex(&mut self, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> ExShaderCreateFromSpirv < '_ > {
            ExShaderCreateFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_bytecode_full(&mut self, binary_data: PackedByteArray, placeholder_rid: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, PackedByteArray, Rid);
            let args = (binary_data, placeholder_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_create_from_bytecode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_create_from_bytecode(&mut self, binary_data: PackedByteArray,) -> Rid {
            self.shader_create_from_bytecode_ex(binary_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_bytecode_ex(&mut self, binary_data: PackedByteArray,) -> ExShaderCreateFromBytecode < '_ > {
            ExShaderCreateFromBytecode::new(self, binary_data,)
        }
        pub fn shader_create_placeholder(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_create_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_get_vertex_input_attribute_mask(&mut self, shader: Rid,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_get_vertex_input_attribute_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn uniform_buffer_create_full(&mut self, size_bytes: u32, data: PackedByteArray,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, PackedByteArray);
            let args = (size_bytes, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "uniform_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn uniform_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.uniform_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn uniform_buffer_create_ex(&mut self, size_bytes: u32,) -> ExUniformBufferCreate < '_ > {
            ExUniformBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn storage_buffer_create_full(&mut self, size_bytes: u32, data: PackedByteArray, usage: crate::engine::rendering_device::StorageBufferUsage,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, PackedByteArray, crate::engine::rendering_device::StorageBufferUsage);
            let args = (size_bytes, data, usage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "storage_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn storage_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.storage_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn storage_buffer_create_ex(&mut self, size_bytes: u32,) -> ExStorageBufferCreate < '_ > {
            ExStorageBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn texture_buffer_create_full(&mut self, size_bytes: u32, format: crate::engine::rendering_device::DataFormat, data: PackedByteArray,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32, crate::engine::rendering_device::DataFormat, PackedByteArray);
            let args = (size_bytes, format, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_buffer_create(&mut self, size_bytes: u32, format: crate::engine::rendering_device::DataFormat,) -> Rid {
            self.texture_buffer_create_ex(size_bytes, format,) . done()
        }
        #[inline]
        pub fn texture_buffer_create_ex(&mut self, size_bytes: u32, format: crate::engine::rendering_device::DataFormat,) -> ExTextureBufferCreate < '_ > {
            ExTextureBufferCreate::new(self, size_bytes, format,)
        }
        pub fn uniform_set_create(&mut self, uniforms: Array < Gd < crate::engine::RdUniform > >, shader: Rid, shader_set: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Array < Gd < crate::engine::RdUniform > >, Rid, u32);
            let args = (uniforms, shader, shader_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "uniform_set_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uniform_set_is_valid(&mut self, uniform_set: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (uniform_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "uniform_set_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn buffer_update_full(&mut self, buffer: Rid, offset: u32, size_bytes: u32, data: PackedByteArray, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, u32, u32, PackedByteArray, crate::engine::rendering_device::BarrierMask);
            let args = (buffer, offset, size_bytes, data, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "buffer_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn buffer_update(&mut self, buffer: Rid, offset: u32, size_bytes: u32, data: PackedByteArray,) -> crate::engine::global::Error {
            self.buffer_update_ex(buffer, offset, size_bytes, data,) . done()
        }
        #[inline]
        pub fn buffer_update_ex(&mut self, buffer: Rid, offset: u32, size_bytes: u32, data: PackedByteArray,) -> ExBufferUpdate < '_ > {
            ExBufferUpdate::new(self, buffer, offset, size_bytes, data,)
        }
        pub(crate) fn buffer_clear_full(&mut self, buffer: Rid, offset: u32, size_bytes: u32, post_barrier: crate::engine::rendering_device::BarrierMask,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Rid, u32, u32, crate::engine::rendering_device::BarrierMask);
            let args = (buffer, offset, size_bytes, post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "buffer_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn buffer_clear(&mut self, buffer: Rid, offset: u32, size_bytes: u32,) -> crate::engine::global::Error {
            self.buffer_clear_ex(buffer, offset, size_bytes,) . done()
        }
        #[inline]
        pub fn buffer_clear_ex(&mut self, buffer: Rid, offset: u32, size_bytes: u32,) -> ExBufferClear < '_ > {
            ExBufferClear::new(self, buffer, offset, size_bytes,)
        }
        pub(crate) fn buffer_get_data_full(&mut self, buffer: Rid, offset_bytes: u32, size_bytes: u32,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Rid, u32, u32);
            let args = (buffer, offset_bytes, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "buffer_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn buffer_get_data(&mut self, buffer: Rid,) -> PackedByteArray {
            self.buffer_get_data_ex(buffer,) . done()
        }
        #[inline]
        pub fn buffer_get_data_ex(&mut self, buffer: Rid,) -> ExBufferGetData < '_ > {
            ExBufferGetData::new(self, buffer,)
        }
        pub(crate) fn render_pipeline_create_full(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::engine::rendering_device::RenderPrimitive, rasterization_state: Gd < crate::engine::RdPipelineRasterizationState >, multisample_state: Gd < crate::engine::RdPipelineMultisampleState >, stencil_state: Gd < crate::engine::RdPipelineDepthStencilState >, color_blend_state: Gd < crate::engine::RdPipelineColorBlendState >, dynamic_state_flags: crate::engine::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i64, i64, crate::engine::rendering_device::RenderPrimitive, Gd < crate::engine::RdPipelineRasterizationState >, Gd < crate::engine::RdPipelineMultisampleState >, Gd < crate::engine::RdPipelineDepthStencilState >, Gd < crate::engine::RdPipelineColorBlendState >, crate::engine::rendering_device::PipelineDynamicStateFlags, u32, Array < Gd < crate::engine::RdPipelineSpecializationConstant > >);
            let args = (shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "render_pipeline_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn render_pipeline_create(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::engine::rendering_device::RenderPrimitive, rasterization_state: Gd < crate::engine::RdPipelineRasterizationState >, multisample_state: Gd < crate::engine::RdPipelineMultisampleState >, stencil_state: Gd < crate::engine::RdPipelineDepthStencilState >, color_blend_state: Gd < crate::engine::RdPipelineColorBlendState >,) -> Rid {
            self.render_pipeline_create_ex(shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,) . done()
        }
        #[inline]
        pub fn render_pipeline_create_ex(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::engine::rendering_device::RenderPrimitive, rasterization_state: Gd < crate::engine::RdPipelineRasterizationState >, multisample_state: Gd < crate::engine::RdPipelineMultisampleState >, stencil_state: Gd < crate::engine::RdPipelineDepthStencilState >, color_blend_state: Gd < crate::engine::RdPipelineColorBlendState >,) -> ExRenderPipelineCreate < '_ > {
            ExRenderPipelineCreate::new(self, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,)
        }
        pub fn render_pipeline_is_valid(&mut self, render_pipeline: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "render_pipeline_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn compute_pipeline_create_full(&mut self, shader: Rid, specialization_constants: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, Array < Gd < crate::engine::RdPipelineSpecializationConstant > >);
            let args = (shader, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_pipeline_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compute_pipeline_create(&mut self, shader: Rid,) -> Rid {
            self.compute_pipeline_create_ex(shader,) . done()
        }
        #[inline]
        pub fn compute_pipeline_create_ex(&mut self, shader: Rid,) -> ExComputePipelineCreate < '_ > {
            ExComputePipelineCreate::new(self, shader,)
        }
        pub fn compute_pipeline_is_valid(&mut self, compute_pipeline: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_pipeline_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_width_full(&self, screen: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_width(&self,) -> i32 {
            self.screen_get_width_ex() . done()
        }
        #[inline]
        pub fn screen_get_width_ex(&self,) -> ExScreenGetWidth < '_ > {
            ExScreenGetWidth::new(self,)
        }
        pub(crate) fn screen_get_height_full(&self, screen: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn screen_get_height(&self,) -> i32 {
            self.screen_get_height_ex() . done()
        }
        #[inline]
        pub fn screen_get_height_ex(&self,) -> ExScreenGetHeight < '_ > {
            ExScreenGetHeight::new(self,)
        }
        pub fn screen_get_framebuffer_format(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_get_framebuffer_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_list_begin_for_screen_full(&mut self, screen: i32, clear_color: Color,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, i32, Color);
            let args = (screen, clear_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_begin_for_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_begin_for_screen(&mut self,) -> i64 {
            self.draw_list_begin_for_screen_ex() . done()
        }
        #[inline]
        pub fn draw_list_begin_for_screen_ex(&mut self,) -> ExDrawListBeginForScreen < '_ > {
            ExDrawListBeginForScreen::new(self,)
        }
        pub(crate) fn draw_list_begin_full(&mut self, framebuffer: Rid, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction, clear_color_values: PackedColorArray, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: Array < Rid >,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, crate::engine::rendering_device::InitialAction, crate::engine::rendering_device::FinalAction, crate::engine::rendering_device::InitialAction, crate::engine::rendering_device::FinalAction, PackedColorArray, f32, u32, Rect2, Array < Rid >);
            let args = (framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_begin(&mut self, framebuffer: Rid, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> i64 {
            self.draw_list_begin_ex(framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action,) . done()
        }
        #[inline]
        pub fn draw_list_begin_ex(&mut self, framebuffer: Rid, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> ExDrawListBegin < '_ > {
            ExDrawListBegin::new(self, framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action,)
        }
        pub(crate) fn draw_list_begin_split_full(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction, clear_color_values: PackedColorArray, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: Array < Rid >,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, Rid, u32, crate::engine::rendering_device::InitialAction, crate::engine::rendering_device::FinalAction, crate::engine::rendering_device::InitialAction, crate::engine::rendering_device::FinalAction, PackedColorArray, f32, u32, Rect2, Array < Rid >);
            let args = (framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_begin_split", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_begin_split(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> PackedInt64Array {
            self.draw_list_begin_split_ex(framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,) . done()
        }
        #[inline]
        pub fn draw_list_begin_split_ex(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> ExDrawListBeginSplit < '_ > {
            ExDrawListBeginSplit::new(self, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,)
        }
        pub fn draw_list_set_blend_constants(&mut self, draw_list: i64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Color);
            let args = (draw_list, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_set_blend_constants", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_render_pipeline(&mut self, draw_list: i64, render_pipeline: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid);
            let args = (draw_list, render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_bind_render_pipeline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_uniform_set(&mut self, draw_list: i64, uniform_set: Rid, set_index: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid, u32);
            let args = (draw_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_bind_uniform_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_vertex_array(&mut self, draw_list: i64, vertex_array: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid);
            let args = (draw_list, vertex_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_bind_vertex_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_index_array(&mut self, draw_list: i64, index_array: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid);
            let args = (draw_list, index_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_bind_index_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_set_push_constant(&mut self, draw_list: i64, buffer: PackedByteArray, size_bytes: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, PackedByteArray, u32);
            let args = (draw_list, buffer, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_set_push_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_list_draw_full(&mut self, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, bool, u32, u32);
            let args = (draw_list, use_indices, instances, procedural_vertex_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_draw(&mut self, draw_list: i64, use_indices: bool, instances: u32,) {
            self.draw_list_draw_ex(draw_list, use_indices, instances,) . done()
        }
        #[inline]
        pub fn draw_list_draw_ex(&mut self, draw_list: i64, use_indices: bool, instances: u32,) -> ExDrawListDraw < '_ > {
            ExDrawListDraw::new(self, draw_list, use_indices, instances,)
        }
        pub(crate) fn draw_list_enable_scissor_full(&mut self, draw_list: i64, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rect2);
            let args = (draw_list, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_enable_scissor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_enable_scissor(&mut self, draw_list: i64,) {
            self.draw_list_enable_scissor_ex(draw_list,) . done()
        }
        #[inline]
        pub fn draw_list_enable_scissor_ex(&mut self, draw_list: i64,) -> ExDrawListEnableScissor < '_ > {
            ExDrawListEnableScissor::new(self, draw_list,)
        }
        pub fn draw_list_disable_scissor(&mut self, draw_list: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (draw_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_disable_scissor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass(&mut self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_switch_to_next_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass_split(&mut self, splits: u32,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, u32);
            let args = (splits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_switch_to_next_pass_split", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_list_end_full(&mut self, post_barrier: crate::engine::rendering_device::BarrierMask,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BarrierMask);
            let args = (post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_list_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn draw_list_end(&mut self,) {
            self.draw_list_end_ex() . done()
        }
        #[inline]
        pub fn draw_list_end_ex(&mut self,) -> ExDrawListEnd < '_ > {
            ExDrawListEnd::new(self,)
        }
        pub(crate) fn compute_list_begin_full(&mut self, allow_draw_overlap: bool,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, bool);
            let args = (allow_draw_overlap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compute_list_begin(&mut self,) -> i64 {
            self.compute_list_begin_ex() . done()
        }
        #[inline]
        pub fn compute_list_begin_ex(&mut self,) -> ExComputeListBegin < '_ > {
            ExComputeListBegin::new(self,)
        }
        pub fn compute_list_bind_compute_pipeline(&mut self, compute_list: i64, compute_pipeline: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid);
            let args = (compute_list, compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_bind_compute_pipeline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_set_push_constant(&mut self, compute_list: i64, buffer: PackedByteArray, size_bytes: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, PackedByteArray, u32);
            let args = (compute_list, buffer, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_set_push_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_bind_uniform_set(&mut self, compute_list: i64, uniform_set: Rid, set_index: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Rid, u32);
            let args = (compute_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_bind_uniform_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_dispatch(&mut self, compute_list: i64, x_groups: u32, y_groups: u32, z_groups: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, u32, u32, u32);
            let args = (compute_list, x_groups, y_groups, z_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_dispatch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_add_barrier(&mut self, compute_list: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (compute_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_add_barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn compute_list_end_full(&mut self, post_barrier: crate::engine::rendering_device::BarrierMask,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BarrierMask);
            let args = (post_barrier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_list_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compute_list_end(&mut self,) {
            self.compute_list_end_ex() . done()
        }
        #[inline]
        pub fn compute_list_end_ex(&mut self,) -> ExComputeListEnd < '_ > {
            ExComputeListEnd::new(self,)
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_timestamp(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "capture_timestamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamps_count(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_captured_timestamps_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamps_frame(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_captured_timestamps_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_gpu_time(&self, index: u32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_captured_timestamp_gpu_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_cpu_time(&self, index: u32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_captured_timestamp_cpu_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_name(&self, index: u32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_captured_timestamp_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn limit_get(&self, limit: crate::engine::rendering_device::Limit,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, crate::engine::rendering_device::Limit);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "limit_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_delay(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn submit(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "submit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sync(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn barrier_full(&mut self, from: crate::engine::rendering_device::BarrierMask, to: crate::engine::rendering_device::BarrierMask,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BarrierMask, crate::engine::rendering_device::BarrierMask);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn barrier(&mut self,) {
            self.barrier_ex() . done()
        }
        #[inline]
        pub fn barrier_ex(&mut self,) -> ExBarrier < '_ > {
            ExBarrier::new(self,)
        }
        pub fn full_barrier(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "full_barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_local_device(&mut self,) -> Option < Gd < crate::engine::RenderingDevice > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RenderingDevice > >;
            type CallSig = (Option < Gd < crate::engine::RenderingDevice > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_local_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_resource_name(&mut self, id: Rid, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (id, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_resource_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_begin_label(&mut self, name: GString, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Color);
            let args = (name, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_command_begin_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_insert_label(&mut self, name: GString, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Color);
            let args = (name, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_command_insert_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_end_label(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_command_end_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_vendor_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device_vendor_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_pipeline_cache_uuid(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_device_pipeline_cache_uuid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_memory_usage(&self, type_: crate::engine::rendering_device::MemoryType,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, crate::engine::rendering_device::MemoryType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_memory_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_driver_resource(&mut self, resource: crate::engine::rendering_device::DriverResource, rid: Rid, index: u64,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, crate::engine::rendering_device::DriverResource, Rid, u64);
            let args = (resource, rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_driver_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const INVALID_ID: i32 = - 1i32;
        pub const INVALID_FORMAT_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for RenderingDevice {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RenderingDevice\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderingDevice {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RenderingDevice {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for RenderingDevice {
        
    }
    impl std::ops::Deref for RenderingDevice {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RenderingDevice {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RenderingDevice > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_ex`][super::RenderingDevice::texture_create_ex]."]
#[must_use]
pub struct ExTextureCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >, data: Array < PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >,) -> Self {
        Self {
            surround_object, format, view, data: Array::new(),
        }
    }
    #[inline]
    pub fn data(self, value: Array < PackedByteArray >) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::texture_create_full(self.surround_object, self.format, self.view, self.data,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_shared_from_slice_ex`][super::RenderingDevice::texture_create_shared_from_slice_ex]."]
#[must_use]
pub struct ExTextureCreateSharedFromSlice < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, view: Gd < crate::engine::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::engine::rendering_device::TextureSliceType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreateSharedFromSlice < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, view: Gd < crate::engine::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> Self {
        Self {
            surround_object, view, with_texture, layer, mipmap, mipmaps: 1u32, slice_type: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn mipmaps(self, value: u32) -> Self {
        Self {
            mipmaps: value, .. self
        }
    }
    #[inline]
    pub fn slice_type(self, value: crate::engine::rendering_device::TextureSliceType) -> Self {
        Self {
            slice_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::texture_create_shared_from_slice_full(self.surround_object, self.view, self.with_texture, self.layer, self.mipmap, self.mipmaps, self.slice_type,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_update_ex`][super::RenderingDevice::texture_update_ex]."]
#[must_use]
pub struct ExTextureUpdate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, texture: Rid, layer: u32, data: PackedByteArray, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, texture: Rid, layer: u32, data: PackedByteArray,) -> Self {
        Self {
            surround_object, texture, layer, data, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::texture_update_full(self.surround_object, self.texture, self.layer, self.data, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_copy_ex`][super::RenderingDevice::texture_copy_ex]."]
#[must_use]
pub struct ExTextureCopy < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCopy < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32,) -> Self {
        Self {
            surround_object, from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::texture_copy_full(self.surround_object, self.from_texture, self.to_texture, self.from_pos, self.to_pos, self.size, self.src_mipmap, self.dst_mipmap, self.src_layer, self.dst_layer, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_clear_ex`][super::RenderingDevice::texture_clear_ex]."]
#[must_use]
pub struct ExTextureClear < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureClear < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32,) -> Self {
        Self {
            surround_object, texture, color, base_mipmap, mipmap_count, base_layer, layer_count, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::texture_clear_full(self.surround_object, self.texture, self.color, self.base_mipmap, self.mipmap_count, self.base_layer, self.layer_count, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_resolve_multisample_ex`][super::RenderingDevice::texture_resolve_multisample_ex]."]
#[must_use]
pub struct ExTextureResolveMultisample < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, from_texture: Rid, to_texture: Rid, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureResolveMultisample < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, from_texture: Rid, to_texture: Rid,) -> Self {
        Self {
            surround_object, from_texture, to_texture, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::texture_resolve_multisample_full(self.surround_object, self.from_texture, self.to_texture, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_ex`][super::RenderingDevice::framebuffer_format_create_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >,) -> Self {
        Self {
            surround_object, attachments, view_count: 1u32,
        }
    }
    #[inline]
    pub fn view_count(self, value: u32) -> Self {
        Self {
            view_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::framebuffer_format_create_full(self.surround_object, self.attachments, self.view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_multipass_ex`][super::RenderingDevice::framebuffer_format_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateMultipass < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, passes: Array < Gd < crate::engine::RdFramebufferPass > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: Array < Gd < crate::engine::RdAttachmentFormat > >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> Self {
        Self {
            surround_object, attachments, passes, view_count: 1u32,
        }
    }
    #[inline]
    pub fn view_count(self, value: u32) -> Self {
        Self {
            view_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::framebuffer_format_create_multipass_full(self.surround_object, self.attachments, self.passes, self.view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_empty_ex`][super::RenderingDevice::framebuffer_format_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateEmpty < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, samples: crate::engine::rendering_device::TextureSamples,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, samples: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn samples(self, value: crate::engine::rendering_device::TextureSamples) -> Self {
        Self {
            samples: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::framebuffer_format_create_empty_full(self.surround_object, self.samples,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_get_texture_samples_ex`][super::RenderingDevice::framebuffer_format_get_texture_samples_ex]."]
#[must_use]
pub struct ExFramebufferFormatGetTextureSamples < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, format: i64, render_pass: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatGetTextureSamples < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: i64,) -> Self {
        Self {
            surround_object, format, render_pass: 0u32,
        }
    }
    #[inline]
    pub fn render_pass(self, value: u32) -> Self {
        Self {
            render_pass: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::rendering_device::TextureSamples {
        re_export::RenderingDevice::framebuffer_format_get_texture_samples_full(self.surround_object, self.format, self.render_pass,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_ex`][super::RenderingDevice::framebuffer_create_ex]."]
#[must_use]
pub struct ExFramebufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, textures: Array < Rid >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: Array < Rid >,) -> Self {
        Self {
            surround_object, textures, validate_with_format: - 1i64, view_count: 1u32,
        }
    }
    #[inline]
    pub fn validate_with_format(self, value: i64) -> Self {
        Self {
            validate_with_format: value, .. self
        }
    }
    #[inline]
    pub fn view_count(self, value: u32) -> Self {
        Self {
            view_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::framebuffer_create_full(self.surround_object, self.textures, self.validate_with_format, self.view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_multipass_ex`][super::RenderingDevice::framebuffer_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferCreateMultipass < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, textures: Array < Rid >, passes: Array < Gd < crate::engine::RdFramebufferPass > >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: Array < Rid >, passes: Array < Gd < crate::engine::RdFramebufferPass > >,) -> Self {
        Self {
            surround_object, textures, passes, validate_with_format: - 1i64, view_count: 1u32,
        }
    }
    #[inline]
    pub fn validate_with_format(self, value: i64) -> Self {
        Self {
            validate_with_format: value, .. self
        }
    }
    #[inline]
    pub fn view_count(self, value: u32) -> Self {
        Self {
            view_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::framebuffer_create_multipass_full(self.surround_object, self.textures, self.passes, self.validate_with_format, self.view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_empty_ex`][super::RenderingDevice::framebuffer_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferCreateEmpty < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size: Vector2i, samples: crate::engine::rendering_device::TextureSamples, validate_with_format: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size: Vector2i,) -> Self {
        Self {
            surround_object, size, samples: crate::obj::EngineEnum::from_ord(0), validate_with_format: - 1i64,
        }
    }
    #[inline]
    pub fn samples(self, value: crate::engine::rendering_device::TextureSamples) -> Self {
        Self {
            samples: value, .. self
        }
    }
    #[inline]
    pub fn validate_with_format(self, value: i64) -> Self {
        Self {
            validate_with_format: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::framebuffer_create_empty_full(self.surround_object, self.size, self.samples, self.validate_with_format,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_buffer_create_ex`][super::RenderingDevice::vertex_buffer_create_ex]."]
#[must_use]
pub struct ExVertexBufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: PackedByteArray, use_as_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        Self {
            surround_object, size_bytes, data: PackedByteArray::new(), use_as_storage: false,
        }
    }
    #[inline]
    pub fn data(self, value: PackedByteArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn use_as_storage(self, value: bool) -> Self {
        Self {
            use_as_storage: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::vertex_buffer_create_full(self.surround_object, self.size_bytes, self.data, self.use_as_storage,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_array_create_ex`][super::RenderingDevice::vertex_array_create_ex]."]
#[must_use]
pub struct ExVertexArrayCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: Array < Rid >, offsets: PackedInt64Array,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexArrayCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: Array < Rid >,) -> Self {
        Self {
            surround_object, vertex_count, vertex_format, src_buffers, offsets: PackedInt64Array::new(),
        }
    }
    #[inline]
    pub fn offsets(self, value: PackedInt64Array) -> Self {
        Self {
            offsets: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::vertex_array_create_full(self.surround_object, self.vertex_count, self.vertex_format, self.src_buffers, self.offsets,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::index_buffer_create_ex`][super::RenderingDevice::index_buffer_create_ex]."]
#[must_use]
pub struct ExIndexBufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::engine::rendering_device::IndexBufferFormat, data: PackedByteArray, use_restart_indices: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIndexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::engine::rendering_device::IndexBufferFormat,) -> Self {
        Self {
            surround_object, size_indices, format, data: PackedByteArray::new(), use_restart_indices: false,
        }
    }
    #[inline]
    pub fn data(self, value: PackedByteArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn use_restart_indices(self, value: bool) -> Self {
        Self {
            use_restart_indices: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::index_buffer_create_full(self.surround_object, self.size_indices, self.format, self.data, self.use_restart_indices,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_spirv_from_source_ex`][super::RenderingDevice::shader_compile_spirv_from_source_ex]."]
#[must_use]
pub struct ExShaderCompileSpirvFromSource < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, shader_source: Gd < crate::engine::RdShaderSource >, allow_cache: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileSpirvFromSource < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader_source: Gd < crate::engine::RdShaderSource >,) -> Self {
        Self {
            surround_object, shader_source, allow_cache: true,
        }
    }
    #[inline]
    pub fn allow_cache(self, value: bool) -> Self {
        Self {
            allow_cache: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::RdShaderSpirv > > {
        re_export::RenderingDevice::shader_compile_spirv_from_source_full(self.surround_object, self.shader_source, self.allow_cache,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_binary_from_spirv_ex`][super::RenderingDevice::shader_compile_binary_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCompileBinaryFromSpirv < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, spirv_data: Gd < crate::engine::RdShaderSpirv >, name: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileBinaryFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> Self {
        Self {
            surround_object, spirv_data, name: GString::from(""),
        }
    }
    #[inline]
    pub fn name(self, value: GString) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::RenderingDevice::shader_compile_binary_from_spirv_full(self.surround_object, self.spirv_data, self.name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_spirv_ex`][super::RenderingDevice::shader_create_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCreateFromSpirv < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, spirv_data: Gd < crate::engine::RdShaderSpirv >, name: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: Gd < crate::engine::RdShaderSpirv >,) -> Self {
        Self {
            surround_object, spirv_data, name: GString::from(""),
        }
    }
    #[inline]
    pub fn name(self, value: GString) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::shader_create_from_spirv_full(self.surround_object, self.spirv_data, self.name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_bytecode_ex`][super::RenderingDevice::shader_create_from_bytecode_ex]."]
#[must_use]
pub struct ExShaderCreateFromBytecode < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, binary_data: PackedByteArray, placeholder_rid: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromBytecode < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, binary_data: PackedByteArray,) -> Self {
        Self {
            surround_object, binary_data, placeholder_rid: Rid::Invalid,
        }
    }
    #[inline]
    pub fn placeholder_rid(self, value: Rid) -> Self {
        Self {
            placeholder_rid: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::shader_create_from_bytecode_full(self.surround_object, self.binary_data, self.placeholder_rid,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::uniform_buffer_create_ex`][super::RenderingDevice::uniform_buffer_create_ex]."]
#[must_use]
pub struct ExUniformBufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: PackedByteArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUniformBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        Self {
            surround_object, size_bytes, data: PackedByteArray::new(),
        }
    }
    #[inline]
    pub fn data(self, value: PackedByteArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::uniform_buffer_create_full(self.surround_object, self.size_bytes, self.data,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::storage_buffer_create_ex`][super::RenderingDevice::storage_buffer_create_ex]."]
#[must_use]
pub struct ExStorageBufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: PackedByteArray, usage: crate::engine::rendering_device::StorageBufferUsage,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStorageBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        Self {
            surround_object, size_bytes, data: PackedByteArray::new(), usage: crate::obj::EngineBitfield::from_ord(0),
        }
    }
    #[inline]
    pub fn data(self, value: PackedByteArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn usage(self, value: crate::engine::rendering_device::StorageBufferUsage) -> Self {
        Self {
            usage: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::storage_buffer_create_full(self.surround_object, self.size_bytes, self.data, self.usage,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_buffer_create_ex`][super::RenderingDevice::texture_buffer_create_ex]."]
#[must_use]
pub struct ExTextureBufferCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::engine::rendering_device::DataFormat, data: PackedByteArray,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::engine::rendering_device::DataFormat,) -> Self {
        Self {
            surround_object, size_bytes, format, data: PackedByteArray::new(),
        }
    }
    #[inline]
    pub fn data(self, value: PackedByteArray) -> Self {
        Self {
            data: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::texture_buffer_create_full(self.surround_object, self.size_bytes, self.format, self.data,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_update_ex`][super::RenderingDevice::buffer_update_ex]."]
#[must_use]
pub struct ExBufferUpdate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset: u32, size_bytes: u32, data: PackedByteArray, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset: u32, size_bytes: u32, data: PackedByteArray,) -> Self {
        Self {
            surround_object, buffer, offset, size_bytes, data, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::buffer_update_full(self.surround_object, self.buffer, self.offset, self.size_bytes, self.data, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_clear_ex`][super::RenderingDevice::buffer_clear_ex]."]
#[must_use]
pub struct ExBufferClear < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset: u32, size_bytes: u32, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferClear < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset: u32, size_bytes: u32,) -> Self {
        Self {
            surround_object, buffer, offset, size_bytes, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::RenderingDevice::buffer_clear_full(self.surround_object, self.buffer, self.offset, self.size_bytes, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_get_data_ex`][super::RenderingDevice::buffer_get_data_ex]."]
#[must_use]
pub struct ExBufferGetData < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset_bytes: u32, size_bytes: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferGetData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid,) -> Self {
        Self {
            surround_object, buffer, offset_bytes: 0u32, size_bytes: 0u32,
        }
    }
    #[inline]
    pub fn offset_bytes(self, value: u32) -> Self {
        Self {
            offset_bytes: value, .. self
        }
    }
    #[inline]
    pub fn size_bytes(self, value: u32) -> Self {
        Self {
            size_bytes: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::RenderingDevice::buffer_get_data_full(self.surround_object, self.buffer, self.offset_bytes, self.size_bytes,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::render_pipeline_create_ex`][super::RenderingDevice::render_pipeline_create_ex]."]
#[must_use]
pub struct ExRenderPipelineCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::engine::rendering_device::RenderPrimitive, rasterization_state: Gd < crate::engine::RdPipelineRasterizationState >, multisample_state: Gd < crate::engine::RdPipelineMultisampleState >, stencil_state: Gd < crate::engine::RdPipelineDepthStencilState >, color_blend_state: Gd < crate::engine::RdPipelineColorBlendState >, dynamic_state_flags: crate::engine::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRenderPipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::engine::rendering_device::RenderPrimitive, rasterization_state: Gd < crate::engine::RdPipelineRasterizationState >, multisample_state: Gd < crate::engine::RdPipelineMultisampleState >, stencil_state: Gd < crate::engine::RdPipelineDepthStencilState >, color_blend_state: Gd < crate::engine::RdPipelineColorBlendState >,) -> Self {
        Self {
            surround_object, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags: crate::obj::EngineBitfield::from_ord(0), for_render_pass: 0u32, specialization_constants: Array::new(),
        }
    }
    #[inline]
    pub fn dynamic_state_flags(self, value: crate::engine::rendering_device::PipelineDynamicStateFlags) -> Self {
        Self {
            dynamic_state_flags: value, .. self
        }
    }
    #[inline]
    pub fn for_render_pass(self, value: u32) -> Self {
        Self {
            for_render_pass: value, .. self
        }
    }
    #[inline]
    pub fn specialization_constants(self, value: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::render_pipeline_create_full(self.surround_object, self.shader, self.framebuffer_format, self.vertex_format, self.primitive, self.rasterization_state, self.multisample_state, self.stencil_state, self.color_blend_state, self.dynamic_state_flags, self.for_render_pass, self.specialization_constants,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::compute_pipeline_create_ex`][super::RenderingDevice::compute_pipeline_create_ex]."]
#[must_use]
pub struct ExComputePipelineCreate < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, shader: Rid, specialization_constants: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExComputePipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid,) -> Self {
        Self {
            surround_object, shader, specialization_constants: Array::new(),
        }
    }
    #[inline]
    pub fn specialization_constants(self, value: Array < Gd < crate::engine::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingDevice::compute_pipeline_create_full(self.surround_object, self.shader, self.specialization_constants,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_width_ex`][super::RenderingDevice::screen_get_width_ex]."]
#[must_use]
pub struct ExScreenGetWidth < 'a > {
    surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetWidth < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, screen: 0i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::RenderingDevice::screen_get_width_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_height_ex`][super::RenderingDevice::screen_get_height_ex]."]
#[must_use]
pub struct ExScreenGetHeight < 'a > {
    surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetHeight < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, screen: 0i32,
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::RenderingDevice::screen_get_height_full(self.surround_object, self.screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_for_screen_ex`][super::RenderingDevice::draw_list_begin_for_screen_ex]."]
#[must_use]
pub struct ExDrawListBeginForScreen < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, screen: i32, clear_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginForScreen < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, screen: 0i32, clear_color: Color::from_rgba(0 as _, 0 as _, 0 as _, 1 as _),
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn clear_color(self, value: Color) -> Self {
        Self {
            clear_color: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::draw_list_begin_for_screen_full(self.surround_object, self.screen, self.clear_color,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_ex`][super::RenderingDevice::draw_list_begin_ex]."]
#[must_use]
pub struct ExDrawListBegin < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction, clear_color_values: PackedColorArray, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: Array < Rid >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBegin < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> Self {
        Self {
            surround_object, framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values: PackedColorArray::new(), clear_depth: 1f32, clear_stencil: 0u32, region: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), storage_textures: Array::new(),
        }
    }
    #[inline]
    pub fn clear_color_values(self, value: PackedColorArray) -> Self {
        Self {
            clear_color_values: value, .. self
        }
    }
    #[inline]
    pub fn clear_depth(self, value: f32) -> Self {
        Self {
            clear_depth: value, .. self
        }
    }
    #[inline]
    pub fn clear_stencil(self, value: u32) -> Self {
        Self {
            clear_stencil: value, .. self
        }
    }
    #[inline]
    pub fn region(self, value: Rect2) -> Self {
        Self {
            region: value, .. self
        }
    }
    #[inline]
    pub fn storage_textures(self, value: Array < Rid >) -> Self {
        Self {
            storage_textures: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::draw_list_begin_full(self.surround_object, self.framebuffer, self.initial_color_action, self.final_color_action, self.initial_depth_action, self.final_depth_action, self.clear_color_values, self.clear_depth, self.clear_stencil, self.region, self.storage_textures,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_split_ex`][super::RenderingDevice::draw_list_begin_split_ex]."]
#[must_use]
pub struct ExDrawListBeginSplit < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction, clear_color_values: PackedColorArray, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: Array < Rid >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginSplit < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::engine::rendering_device::InitialAction, final_color_action: crate::engine::rendering_device::FinalAction, initial_depth_action: crate::engine::rendering_device::InitialAction, final_depth_action: crate::engine::rendering_device::FinalAction,) -> Self {
        Self {
            surround_object, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values: PackedColorArray::new(), clear_depth: 1f32, clear_stencil: 0u32, region: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), storage_textures: Array::new(),
        }
    }
    #[inline]
    pub fn clear_color_values(self, value: PackedColorArray) -> Self {
        Self {
            clear_color_values: value, .. self
        }
    }
    #[inline]
    pub fn clear_depth(self, value: f32) -> Self {
        Self {
            clear_depth: value, .. self
        }
    }
    #[inline]
    pub fn clear_stencil(self, value: u32) -> Self {
        Self {
            clear_stencil: value, .. self
        }
    }
    #[inline]
    pub fn region(self, value: Rect2) -> Self {
        Self {
            region: value, .. self
        }
    }
    #[inline]
    pub fn storage_textures(self, value: Array < Rid >) -> Self {
        Self {
            storage_textures: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        re_export::RenderingDevice::draw_list_begin_split_full(self.surround_object, self.framebuffer, self.splits, self.initial_color_action, self.final_color_action, self.initial_depth_action, self.final_depth_action, self.clear_color_values, self.clear_depth, self.clear_stencil, self.region, self.storage_textures,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_draw_ex`][super::RenderingDevice::draw_list_draw_ex]."]
#[must_use]
pub struct ExDrawListDraw < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListDraw < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32,) -> Self {
        Self {
            surround_object, draw_list, use_indices, instances, procedural_vertex_count: 0u32,
        }
    }
    #[inline]
    pub fn procedural_vertex_count(self, value: u32) -> Self {
        Self {
            procedural_vertex_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingDevice::draw_list_draw_full(self.surround_object, self.draw_list, self.use_indices, self.instances, self.procedural_vertex_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_enable_scissor_ex`][super::RenderingDevice::draw_list_enable_scissor_ex]."]
#[must_use]
pub struct ExDrawListEnableScissor < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListEnableScissor < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64,) -> Self {
        Self {
            surround_object, draw_list, rect: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn rect(self, value: Rect2) -> Self {
        Self {
            rect: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingDevice::draw_list_enable_scissor_full(self.surround_object, self.draw_list, self.rect,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_end_ex`][super::RenderingDevice::draw_list_end_ex]."]
#[must_use]
pub struct ExDrawListEnd < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListEnd < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingDevice::draw_list_end_full(self.surround_object, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::compute_list_begin_ex`][super::RenderingDevice::compute_list_begin_ex]."]
#[must_use]
pub struct ExComputeListBegin < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, allow_draw_overlap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExComputeListBegin < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, allow_draw_overlap: false,
        }
    }
    #[inline]
    pub fn allow_draw_overlap(self, value: bool) -> Self {
        Self {
            allow_draw_overlap: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::RenderingDevice::compute_list_begin_full(self.surround_object, self.allow_draw_overlap,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::compute_list_end_ex`][super::RenderingDevice::compute_list_end_ex]."]
#[must_use]
pub struct ExComputeListEnd < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, post_barrier: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExComputeListEnd < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, post_barrier: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn post_barrier(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            post_barrier: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingDevice::compute_list_end_full(self.surround_object, self.post_barrier,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::barrier_ex`][super::RenderingDevice::barrier_ex]."]
#[must_use]
pub struct ExBarrier < 'a > {
    surround_object: &'a mut re_export::RenderingDevice, from: crate::engine::rendering_device::BarrierMask, to: crate::engine::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBarrier < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        Self {
            surround_object, from: crate::obj::EngineBitfield::from_ord(32767), to: crate::obj::EngineBitfield::from_ord(32767),
        }
    }
    #[inline]
    pub fn from(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            from: value, .. self
        }
    }
    #[inline]
    pub fn to(self, value: crate::engine::rendering_device::BarrierMask) -> Self {
        Self {
            to: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingDevice::barrier_full(self.surround_object, self.from, self.to,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DeviceType {
    ord: i32
}
impl DeviceType {
    pub const DEVICE_TYPE_OTHER: Self = Self {
        ord: 0i32
    };
    pub const DEVICE_TYPE_INTEGRATED_GPU: Self = Self {
        ord: 1i32
    };
    pub const DEVICE_TYPE_DISCRETE_GPU: Self = Self {
        ord: 2i32
    };
    pub const DEVICE_TYPE_VIRTUAL_GPU: Self = Self {
        ord: 3i32
    };
    pub const DEVICE_TYPE_CPU: Self = Self {
        ord: 4i32
    };
    pub const DEVICE_TYPE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for DeviceType {
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
impl crate::obj::IndexEnum for DeviceType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for DeviceType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DeviceType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DeviceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DriverResource {
    ord: i32
}
impl DriverResource {
    pub const DRIVER_RESOURCE_VULKAN_DEVICE: Self = Self {
        ord: 0i32
    };
    pub const DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE: Self = Self {
        ord: 1i32
    };
    pub const DRIVER_RESOURCE_VULKAN_INSTANCE: Self = Self {
        ord: 2i32
    };
    pub const DRIVER_RESOURCE_VULKAN_QUEUE: Self = Self {
        ord: 3i32
    };
    pub const DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX: Self = Self {
        ord: 4i32
    };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE: Self = Self {
        ord: 5i32
    };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE_VIEW: Self = Self {
        ord: 6i32
    };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT: Self = Self {
        ord: 7i32
    };
    pub const DRIVER_RESOURCE_VULKAN_SAMPLER: Self = Self {
        ord: 8i32
    };
    pub const DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET: Self = Self {
        ord: 9i32
    };
    pub const DRIVER_RESOURCE_VULKAN_BUFFER: Self = Self {
        ord: 10i32
    };
    pub const DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE: Self = Self {
        ord: 11i32
    };
    pub const DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE: Self = Self {
        ord: 12i32
    };
    
}
impl crate::obj::EngineEnum for DriverResource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for DriverResource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DriverResource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DriverResource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DataFormat {
    ord: i32
}
impl DataFormat {
    pub const DATA_FORMAT_R4G4_UNORM_PACK8: Self = Self {
        ord: 0i32
    };
    pub const DATA_FORMAT_R4G4B4A4_UNORM_PACK16: Self = Self {
        ord: 1i32
    };
    pub const DATA_FORMAT_B4G4R4A4_UNORM_PACK16: Self = Self {
        ord: 2i32
    };
    pub const DATA_FORMAT_R5G6B5_UNORM_PACK16: Self = Self {
        ord: 3i32
    };
    pub const DATA_FORMAT_B5G6R5_UNORM_PACK16: Self = Self {
        ord: 4i32
    };
    pub const DATA_FORMAT_R5G5B5A1_UNORM_PACK16: Self = Self {
        ord: 5i32
    };
    pub const DATA_FORMAT_B5G5R5A1_UNORM_PACK16: Self = Self {
        ord: 6i32
    };
    pub const DATA_FORMAT_A1R5G5B5_UNORM_PACK16: Self = Self {
        ord: 7i32
    };
    pub const DATA_FORMAT_R8_UNORM: Self = Self {
        ord: 8i32
    };
    pub const DATA_FORMAT_R8_SNORM: Self = Self {
        ord: 9i32
    };
    pub const DATA_FORMAT_R8_USCALED: Self = Self {
        ord: 10i32
    };
    pub const DATA_FORMAT_R8_SSCALED: Self = Self {
        ord: 11i32
    };
    pub const DATA_FORMAT_R8_UINT: Self = Self {
        ord: 12i32
    };
    pub const DATA_FORMAT_R8_SINT: Self = Self {
        ord: 13i32
    };
    pub const DATA_FORMAT_R8_SRGB: Self = Self {
        ord: 14i32
    };
    pub const DATA_FORMAT_R8G8_UNORM: Self = Self {
        ord: 15i32
    };
    pub const DATA_FORMAT_R8G8_SNORM: Self = Self {
        ord: 16i32
    };
    pub const DATA_FORMAT_R8G8_USCALED: Self = Self {
        ord: 17i32
    };
    pub const DATA_FORMAT_R8G8_SSCALED: Self = Self {
        ord: 18i32
    };
    pub const DATA_FORMAT_R8G8_UINT: Self = Self {
        ord: 19i32
    };
    pub const DATA_FORMAT_R8G8_SINT: Self = Self {
        ord: 20i32
    };
    pub const DATA_FORMAT_R8G8_SRGB: Self = Self {
        ord: 21i32
    };
    pub const DATA_FORMAT_R8G8B8_UNORM: Self = Self {
        ord: 22i32
    };
    pub const DATA_FORMAT_R8G8B8_SNORM: Self = Self {
        ord: 23i32
    };
    pub const DATA_FORMAT_R8G8B8_USCALED: Self = Self {
        ord: 24i32
    };
    pub const DATA_FORMAT_R8G8B8_SSCALED: Self = Self {
        ord: 25i32
    };
    pub const DATA_FORMAT_R8G8B8_UINT: Self = Self {
        ord: 26i32
    };
    pub const DATA_FORMAT_R8G8B8_SINT: Self = Self {
        ord: 27i32
    };
    pub const DATA_FORMAT_R8G8B8_SRGB: Self = Self {
        ord: 28i32
    };
    pub const DATA_FORMAT_B8G8R8_UNORM: Self = Self {
        ord: 29i32
    };
    pub const DATA_FORMAT_B8G8R8_SNORM: Self = Self {
        ord: 30i32
    };
    pub const DATA_FORMAT_B8G8R8_USCALED: Self = Self {
        ord: 31i32
    };
    pub const DATA_FORMAT_B8G8R8_SSCALED: Self = Self {
        ord: 32i32
    };
    pub const DATA_FORMAT_B8G8R8_UINT: Self = Self {
        ord: 33i32
    };
    pub const DATA_FORMAT_B8G8R8_SINT: Self = Self {
        ord: 34i32
    };
    pub const DATA_FORMAT_B8G8R8_SRGB: Self = Self {
        ord: 35i32
    };
    pub const DATA_FORMAT_R8G8B8A8_UNORM: Self = Self {
        ord: 36i32
    };
    pub const DATA_FORMAT_R8G8B8A8_SNORM: Self = Self {
        ord: 37i32
    };
    pub const DATA_FORMAT_R8G8B8A8_USCALED: Self = Self {
        ord: 38i32
    };
    pub const DATA_FORMAT_R8G8B8A8_SSCALED: Self = Self {
        ord: 39i32
    };
    pub const DATA_FORMAT_R8G8B8A8_UINT: Self = Self {
        ord: 40i32
    };
    pub const DATA_FORMAT_R8G8B8A8_SINT: Self = Self {
        ord: 41i32
    };
    pub const DATA_FORMAT_R8G8B8A8_SRGB: Self = Self {
        ord: 42i32
    };
    pub const DATA_FORMAT_B8G8R8A8_UNORM: Self = Self {
        ord: 43i32
    };
    pub const DATA_FORMAT_B8G8R8A8_SNORM: Self = Self {
        ord: 44i32
    };
    pub const DATA_FORMAT_B8G8R8A8_USCALED: Self = Self {
        ord: 45i32
    };
    pub const DATA_FORMAT_B8G8R8A8_SSCALED: Self = Self {
        ord: 46i32
    };
    pub const DATA_FORMAT_B8G8R8A8_UINT: Self = Self {
        ord: 47i32
    };
    pub const DATA_FORMAT_B8G8R8A8_SINT: Self = Self {
        ord: 48i32
    };
    pub const DATA_FORMAT_B8G8R8A8_SRGB: Self = Self {
        ord: 49i32
    };
    pub const DATA_FORMAT_A8B8G8R8_UNORM_PACK32: Self = Self {
        ord: 50i32
    };
    pub const DATA_FORMAT_A8B8G8R8_SNORM_PACK32: Self = Self {
        ord: 51i32
    };
    pub const DATA_FORMAT_A8B8G8R8_USCALED_PACK32: Self = Self {
        ord: 52i32
    };
    pub const DATA_FORMAT_A8B8G8R8_SSCALED_PACK32: Self = Self {
        ord: 53i32
    };
    pub const DATA_FORMAT_A8B8G8R8_UINT_PACK32: Self = Self {
        ord: 54i32
    };
    pub const DATA_FORMAT_A8B8G8R8_SINT_PACK32: Self = Self {
        ord: 55i32
    };
    pub const DATA_FORMAT_A8B8G8R8_SRGB_PACK32: Self = Self {
        ord: 56i32
    };
    pub const DATA_FORMAT_A2R10G10B10_UNORM_PACK32: Self = Self {
        ord: 57i32
    };
    pub const DATA_FORMAT_A2R10G10B10_SNORM_PACK32: Self = Self {
        ord: 58i32
    };
    pub const DATA_FORMAT_A2R10G10B10_USCALED_PACK32: Self = Self {
        ord: 59i32
    };
    pub const DATA_FORMAT_A2R10G10B10_SSCALED_PACK32: Self = Self {
        ord: 60i32
    };
    pub const DATA_FORMAT_A2R10G10B10_UINT_PACK32: Self = Self {
        ord: 61i32
    };
    pub const DATA_FORMAT_A2R10G10B10_SINT_PACK32: Self = Self {
        ord: 62i32
    };
    pub const DATA_FORMAT_A2B10G10R10_UNORM_PACK32: Self = Self {
        ord: 63i32
    };
    pub const DATA_FORMAT_A2B10G10R10_SNORM_PACK32: Self = Self {
        ord: 64i32
    };
    pub const DATA_FORMAT_A2B10G10R10_USCALED_PACK32: Self = Self {
        ord: 65i32
    };
    pub const DATA_FORMAT_A2B10G10R10_SSCALED_PACK32: Self = Self {
        ord: 66i32
    };
    pub const DATA_FORMAT_A2B10G10R10_UINT_PACK32: Self = Self {
        ord: 67i32
    };
    pub const DATA_FORMAT_A2B10G10R10_SINT_PACK32: Self = Self {
        ord: 68i32
    };
    pub const DATA_FORMAT_R16_UNORM: Self = Self {
        ord: 69i32
    };
    pub const DATA_FORMAT_R16_SNORM: Self = Self {
        ord: 70i32
    };
    pub const DATA_FORMAT_R16_USCALED: Self = Self {
        ord: 71i32
    };
    pub const DATA_FORMAT_R16_SSCALED: Self = Self {
        ord: 72i32
    };
    pub const DATA_FORMAT_R16_UINT: Self = Self {
        ord: 73i32
    };
    pub const DATA_FORMAT_R16_SINT: Self = Self {
        ord: 74i32
    };
    pub const DATA_FORMAT_R16_SFLOAT: Self = Self {
        ord: 75i32
    };
    pub const DATA_FORMAT_R16G16_UNORM: Self = Self {
        ord: 76i32
    };
    pub const DATA_FORMAT_R16G16_SNORM: Self = Self {
        ord: 77i32
    };
    pub const DATA_FORMAT_R16G16_USCALED: Self = Self {
        ord: 78i32
    };
    pub const DATA_FORMAT_R16G16_SSCALED: Self = Self {
        ord: 79i32
    };
    pub const DATA_FORMAT_R16G16_UINT: Self = Self {
        ord: 80i32
    };
    pub const DATA_FORMAT_R16G16_SINT: Self = Self {
        ord: 81i32
    };
    pub const DATA_FORMAT_R16G16_SFLOAT: Self = Self {
        ord: 82i32
    };
    pub const DATA_FORMAT_R16G16B16_UNORM: Self = Self {
        ord: 83i32
    };
    pub const DATA_FORMAT_R16G16B16_SNORM: Self = Self {
        ord: 84i32
    };
    pub const DATA_FORMAT_R16G16B16_USCALED: Self = Self {
        ord: 85i32
    };
    pub const DATA_FORMAT_R16G16B16_SSCALED: Self = Self {
        ord: 86i32
    };
    pub const DATA_FORMAT_R16G16B16_UINT: Self = Self {
        ord: 87i32
    };
    pub const DATA_FORMAT_R16G16B16_SINT: Self = Self {
        ord: 88i32
    };
    pub const DATA_FORMAT_R16G16B16_SFLOAT: Self = Self {
        ord: 89i32
    };
    pub const DATA_FORMAT_R16G16B16A16_UNORM: Self = Self {
        ord: 90i32
    };
    pub const DATA_FORMAT_R16G16B16A16_SNORM: Self = Self {
        ord: 91i32
    };
    pub const DATA_FORMAT_R16G16B16A16_USCALED: Self = Self {
        ord: 92i32
    };
    pub const DATA_FORMAT_R16G16B16A16_SSCALED: Self = Self {
        ord: 93i32
    };
    pub const DATA_FORMAT_R16G16B16A16_UINT: Self = Self {
        ord: 94i32
    };
    pub const DATA_FORMAT_R16G16B16A16_SINT: Self = Self {
        ord: 95i32
    };
    pub const DATA_FORMAT_R16G16B16A16_SFLOAT: Self = Self {
        ord: 96i32
    };
    pub const DATA_FORMAT_R32_UINT: Self = Self {
        ord: 97i32
    };
    pub const DATA_FORMAT_R32_SINT: Self = Self {
        ord: 98i32
    };
    pub const DATA_FORMAT_R32_SFLOAT: Self = Self {
        ord: 99i32
    };
    pub const DATA_FORMAT_R32G32_UINT: Self = Self {
        ord: 100i32
    };
    pub const DATA_FORMAT_R32G32_SINT: Self = Self {
        ord: 101i32
    };
    pub const DATA_FORMAT_R32G32_SFLOAT: Self = Self {
        ord: 102i32
    };
    pub const DATA_FORMAT_R32G32B32_UINT: Self = Self {
        ord: 103i32
    };
    pub const DATA_FORMAT_R32G32B32_SINT: Self = Self {
        ord: 104i32
    };
    pub const DATA_FORMAT_R32G32B32_SFLOAT: Self = Self {
        ord: 105i32
    };
    pub const DATA_FORMAT_R32G32B32A32_UINT: Self = Self {
        ord: 106i32
    };
    pub const DATA_FORMAT_R32G32B32A32_SINT: Self = Self {
        ord: 107i32
    };
    pub const DATA_FORMAT_R32G32B32A32_SFLOAT: Self = Self {
        ord: 108i32
    };
    pub const DATA_FORMAT_R64_UINT: Self = Self {
        ord: 109i32
    };
    pub const DATA_FORMAT_R64_SINT: Self = Self {
        ord: 110i32
    };
    pub const DATA_FORMAT_R64_SFLOAT: Self = Self {
        ord: 111i32
    };
    pub const DATA_FORMAT_R64G64_UINT: Self = Self {
        ord: 112i32
    };
    pub const DATA_FORMAT_R64G64_SINT: Self = Self {
        ord: 113i32
    };
    pub const DATA_FORMAT_R64G64_SFLOAT: Self = Self {
        ord: 114i32
    };
    pub const DATA_FORMAT_R64G64B64_UINT: Self = Self {
        ord: 115i32
    };
    pub const DATA_FORMAT_R64G64B64_SINT: Self = Self {
        ord: 116i32
    };
    pub const DATA_FORMAT_R64G64B64_SFLOAT: Self = Self {
        ord: 117i32
    };
    pub const DATA_FORMAT_R64G64B64A64_UINT: Self = Self {
        ord: 118i32
    };
    pub const DATA_FORMAT_R64G64B64A64_SINT: Self = Self {
        ord: 119i32
    };
    pub const DATA_FORMAT_R64G64B64A64_SFLOAT: Self = Self {
        ord: 120i32
    };
    pub const DATA_FORMAT_B10G11R11_UFLOAT_PACK32: Self = Self {
        ord: 121i32
    };
    pub const DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32: Self = Self {
        ord: 122i32
    };
    pub const DATA_FORMAT_D16_UNORM: Self = Self {
        ord: 123i32
    };
    pub const DATA_FORMAT_X8_D24_UNORM_PACK32: Self = Self {
        ord: 124i32
    };
    pub const DATA_FORMAT_D32_SFLOAT: Self = Self {
        ord: 125i32
    };
    pub const DATA_FORMAT_S8_UINT: Self = Self {
        ord: 126i32
    };
    pub const DATA_FORMAT_D16_UNORM_S8_UINT: Self = Self {
        ord: 127i32
    };
    pub const DATA_FORMAT_D24_UNORM_S8_UINT: Self = Self {
        ord: 128i32
    };
    pub const DATA_FORMAT_D32_SFLOAT_S8_UINT: Self = Self {
        ord: 129i32
    };
    pub const DATA_FORMAT_BC1_RGB_UNORM_BLOCK: Self = Self {
        ord: 130i32
    };
    pub const DATA_FORMAT_BC1_RGB_SRGB_BLOCK: Self = Self {
        ord: 131i32
    };
    pub const DATA_FORMAT_BC1_RGBA_UNORM_BLOCK: Self = Self {
        ord: 132i32
    };
    pub const DATA_FORMAT_BC1_RGBA_SRGB_BLOCK: Self = Self {
        ord: 133i32
    };
    pub const DATA_FORMAT_BC2_UNORM_BLOCK: Self = Self {
        ord: 134i32
    };
    pub const DATA_FORMAT_BC2_SRGB_BLOCK: Self = Self {
        ord: 135i32
    };
    pub const DATA_FORMAT_BC3_UNORM_BLOCK: Self = Self {
        ord: 136i32
    };
    pub const DATA_FORMAT_BC3_SRGB_BLOCK: Self = Self {
        ord: 137i32
    };
    pub const DATA_FORMAT_BC4_UNORM_BLOCK: Self = Self {
        ord: 138i32
    };
    pub const DATA_FORMAT_BC4_SNORM_BLOCK: Self = Self {
        ord: 139i32
    };
    pub const DATA_FORMAT_BC5_UNORM_BLOCK: Self = Self {
        ord: 140i32
    };
    pub const DATA_FORMAT_BC5_SNORM_BLOCK: Self = Self {
        ord: 141i32
    };
    pub const DATA_FORMAT_BC6H_UFLOAT_BLOCK: Self = Self {
        ord: 142i32
    };
    pub const DATA_FORMAT_BC6H_SFLOAT_BLOCK: Self = Self {
        ord: 143i32
    };
    pub const DATA_FORMAT_BC7_UNORM_BLOCK: Self = Self {
        ord: 144i32
    };
    pub const DATA_FORMAT_BC7_SRGB_BLOCK: Self = Self {
        ord: 145i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: Self = Self {
        ord: 146i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: Self = Self {
        ord: 147i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self {
        ord: 148i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self {
        ord: 149i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self {
        ord: 150i32
    };
    pub const DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self {
        ord: 151i32
    };
    pub const DATA_FORMAT_EAC_R11_UNORM_BLOCK: Self = Self {
        ord: 152i32
    };
    pub const DATA_FORMAT_EAC_R11_SNORM_BLOCK: Self = Self {
        ord: 153i32
    };
    pub const DATA_FORMAT_EAC_R11G11_UNORM_BLOCK: Self = Self {
        ord: 154i32
    };
    pub const DATA_FORMAT_EAC_R11G11_SNORM_BLOCK: Self = Self {
        ord: 155i32
    };
    pub const DATA_FORMAT_ASTC_4x4_UNORM_BLOCK: Self = Self {
        ord: 156i32
    };
    pub const DATA_FORMAT_ASTC_4x4_SRGB_BLOCK: Self = Self {
        ord: 157i32
    };
    pub const DATA_FORMAT_ASTC_5x4_UNORM_BLOCK: Self = Self {
        ord: 158i32
    };
    pub const DATA_FORMAT_ASTC_5x4_SRGB_BLOCK: Self = Self {
        ord: 159i32
    };
    pub const DATA_FORMAT_ASTC_5x5_UNORM_BLOCK: Self = Self {
        ord: 160i32
    };
    pub const DATA_FORMAT_ASTC_5x5_SRGB_BLOCK: Self = Self {
        ord: 161i32
    };
    pub const DATA_FORMAT_ASTC_6x5_UNORM_BLOCK: Self = Self {
        ord: 162i32
    };
    pub const DATA_FORMAT_ASTC_6x5_SRGB_BLOCK: Self = Self {
        ord: 163i32
    };
    pub const DATA_FORMAT_ASTC_6x6_UNORM_BLOCK: Self = Self {
        ord: 164i32
    };
    pub const DATA_FORMAT_ASTC_6x6_SRGB_BLOCK: Self = Self {
        ord: 165i32
    };
    pub const DATA_FORMAT_ASTC_8x5_UNORM_BLOCK: Self = Self {
        ord: 166i32
    };
    pub const DATA_FORMAT_ASTC_8x5_SRGB_BLOCK: Self = Self {
        ord: 167i32
    };
    pub const DATA_FORMAT_ASTC_8x6_UNORM_BLOCK: Self = Self {
        ord: 168i32
    };
    pub const DATA_FORMAT_ASTC_8x6_SRGB_BLOCK: Self = Self {
        ord: 169i32
    };
    pub const DATA_FORMAT_ASTC_8x8_UNORM_BLOCK: Self = Self {
        ord: 170i32
    };
    pub const DATA_FORMAT_ASTC_8x8_SRGB_BLOCK: Self = Self {
        ord: 171i32
    };
    pub const DATA_FORMAT_ASTC_10x5_UNORM_BLOCK: Self = Self {
        ord: 172i32
    };
    pub const DATA_FORMAT_ASTC_10x5_SRGB_BLOCK: Self = Self {
        ord: 173i32
    };
    pub const DATA_FORMAT_ASTC_10x6_UNORM_BLOCK: Self = Self {
        ord: 174i32
    };
    pub const DATA_FORMAT_ASTC_10x6_SRGB_BLOCK: Self = Self {
        ord: 175i32
    };
    pub const DATA_FORMAT_ASTC_10x8_UNORM_BLOCK: Self = Self {
        ord: 176i32
    };
    pub const DATA_FORMAT_ASTC_10x8_SRGB_BLOCK: Self = Self {
        ord: 177i32
    };
    pub const DATA_FORMAT_ASTC_10x10_UNORM_BLOCK: Self = Self {
        ord: 178i32
    };
    pub const DATA_FORMAT_ASTC_10x10_SRGB_BLOCK: Self = Self {
        ord: 179i32
    };
    pub const DATA_FORMAT_ASTC_12x10_UNORM_BLOCK: Self = Self {
        ord: 180i32
    };
    pub const DATA_FORMAT_ASTC_12x10_SRGB_BLOCK: Self = Self {
        ord: 181i32
    };
    pub const DATA_FORMAT_ASTC_12x12_UNORM_BLOCK: Self = Self {
        ord: 182i32
    };
    pub const DATA_FORMAT_ASTC_12x12_SRGB_BLOCK: Self = Self {
        ord: 183i32
    };
    pub const DATA_FORMAT_G8B8G8R8_422_UNORM: Self = Self {
        ord: 184i32
    };
    pub const DATA_FORMAT_B8G8R8G8_422_UNORM: Self = Self {
        ord: 185i32
    };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM: Self = Self {
        ord: 186i32
    };
    pub const DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM: Self = Self {
        ord: 187i32
    };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM: Self = Self {
        ord: 188i32
    };
    pub const DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM: Self = Self {
        ord: 189i32
    };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM: Self = Self {
        ord: 190i32
    };
    pub const DATA_FORMAT_R10X6_UNORM_PACK16: Self = Self {
        ord: 191i32
    };
    pub const DATA_FORMAT_R10X6G10X6_UNORM_2PACK16: Self = Self {
        ord: 192i32
    };
    pub const DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self {
        ord: 193i32
    };
    pub const DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self {
        ord: 194i32
    };
    pub const DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self {
        ord: 195i32
    };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self {
        ord: 196i32
    };
    pub const DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self {
        ord: 197i32
    };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self {
        ord: 198i32
    };
    pub const DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self {
        ord: 199i32
    };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self {
        ord: 200i32
    };
    pub const DATA_FORMAT_R12X4_UNORM_PACK16: Self = Self {
        ord: 201i32
    };
    pub const DATA_FORMAT_R12X4G12X4_UNORM_2PACK16: Self = Self {
        ord: 202i32
    };
    pub const DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self {
        ord: 203i32
    };
    pub const DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self {
        ord: 204i32
    };
    pub const DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self {
        ord: 205i32
    };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self {
        ord: 206i32
    };
    pub const DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self {
        ord: 207i32
    };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self {
        ord: 208i32
    };
    pub const DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self {
        ord: 209i32
    };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self {
        ord: 210i32
    };
    pub const DATA_FORMAT_G16B16G16R16_422_UNORM: Self = Self {
        ord: 211i32
    };
    pub const DATA_FORMAT_B16G16R16G16_422_UNORM: Self = Self {
        ord: 212i32
    };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM: Self = Self {
        ord: 213i32
    };
    pub const DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM: Self = Self {
        ord: 214i32
    };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM: Self = Self {
        ord: 215i32
    };
    pub const DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM: Self = Self {
        ord: 216i32
    };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM: Self = Self {
        ord: 217i32
    };
    pub const DATA_FORMAT_MAX: Self = Self {
        ord: 218i32
    };
    
}
impl crate::obj::EngineEnum for DataFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 | ord @ 60i32 | ord @ 61i32 | ord @ 62i32 | ord @ 63i32 | ord @ 64i32 | ord @ 65i32 | ord @ 66i32 | ord @ 67i32 | ord @ 68i32 | ord @ 69i32 | ord @ 70i32 | ord @ 71i32 | ord @ 72i32 | ord @ 73i32 | ord @ 74i32 | ord @ 75i32 | ord @ 76i32 | ord @ 77i32 | ord @ 78i32 | ord @ 79i32 | ord @ 80i32 | ord @ 81i32 | ord @ 82i32 | ord @ 83i32 | ord @ 84i32 | ord @ 85i32 | ord @ 86i32 | ord @ 87i32 | ord @ 88i32 | ord @ 89i32 | ord @ 90i32 | ord @ 91i32 | ord @ 92i32 | ord @ 93i32 | ord @ 94i32 | ord @ 95i32 | ord @ 96i32 | ord @ 97i32 | ord @ 98i32 | ord @ 99i32 | ord @ 100i32 | ord @ 101i32 | ord @ 102i32 | ord @ 103i32 | ord @ 104i32 | ord @ 105i32 | ord @ 106i32 | ord @ 107i32 | ord @ 108i32 | ord @ 109i32 | ord @ 110i32 | ord @ 111i32 | ord @ 112i32 | ord @ 113i32 | ord @ 114i32 | ord @ 115i32 | ord @ 116i32 | ord @ 117i32 | ord @ 118i32 | ord @ 119i32 | ord @ 120i32 | ord @ 121i32 | ord @ 122i32 | ord @ 123i32 | ord @ 124i32 | ord @ 125i32 | ord @ 126i32 | ord @ 127i32 | ord @ 128i32 | ord @ 129i32 | ord @ 130i32 | ord @ 131i32 | ord @ 132i32 | ord @ 133i32 | ord @ 134i32 | ord @ 135i32 | ord @ 136i32 | ord @ 137i32 | ord @ 138i32 | ord @ 139i32 | ord @ 140i32 | ord @ 141i32 | ord @ 142i32 | ord @ 143i32 | ord @ 144i32 | ord @ 145i32 | ord @ 146i32 | ord @ 147i32 | ord @ 148i32 | ord @ 149i32 | ord @ 150i32 | ord @ 151i32 | ord @ 152i32 | ord @ 153i32 | ord @ 154i32 | ord @ 155i32 | ord @ 156i32 | ord @ 157i32 | ord @ 158i32 | ord @ 159i32 | ord @ 160i32 | ord @ 161i32 | ord @ 162i32 | ord @ 163i32 | ord @ 164i32 | ord @ 165i32 | ord @ 166i32 | ord @ 167i32 | ord @ 168i32 | ord @ 169i32 | ord @ 170i32 | ord @ 171i32 | ord @ 172i32 | ord @ 173i32 | ord @ 174i32 | ord @ 175i32 | ord @ 176i32 | ord @ 177i32 | ord @ 178i32 | ord @ 179i32 | ord @ 180i32 | ord @ 181i32 | ord @ 182i32 | ord @ 183i32 | ord @ 184i32 | ord @ 185i32 | ord @ 186i32 | ord @ 187i32 | ord @ 188i32 | ord @ 189i32 | ord @ 190i32 | ord @ 191i32 | ord @ 192i32 | ord @ 193i32 | ord @ 194i32 | ord @ 195i32 | ord @ 196i32 | ord @ 197i32 | ord @ 198i32 | ord @ 199i32 | ord @ 200i32 | ord @ 201i32 | ord @ 202i32 | ord @ 203i32 | ord @ 204i32 | ord @ 205i32 | ord @ 206i32 | ord @ 207i32 | ord @ 208i32 | ord @ 209i32 | ord @ 210i32 | ord @ 211i32 | ord @ 212i32 | ord @ 213i32 | ord @ 214i32 | ord @ 215i32 | ord @ 216i32 | ord @ 217i32 | ord @ 218i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for DataFormat {
    const ENUMERATOR_COUNT: usize = 218usize;
    
}
impl crate::builtin::meta::GodotConvert for DataFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DataFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DataFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct BarrierMask {
    ord: u64
}
impl BarrierMask {
    pub const BARRIER_MASK_VERTEX: Self = Self {
        ord: 1u64
    };
    pub const BARRIER_MASK_FRAGMENT: Self = Self {
        ord: 8u64
    };
    pub const BARRIER_MASK_COMPUTE: Self = Self {
        ord: 2u64
    };
    pub const BARRIER_MASK_TRANSFER: Self = Self {
        ord: 4u64
    };
    pub const BARRIER_MASK_RASTER: Self = Self {
        ord: 9u64
    };
    pub const BARRIER_MASK_ALL_BARRIERS: Self = Self {
        ord: 32767u64
    };
    pub const BARRIER_MASK_NO_BARRIER: Self = Self {
        ord: 32768u64
    };
    
}
impl crate::obj::EngineBitfield for BarrierMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for BarrierMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for BarrierMask {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for BarrierMask {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BarrierMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    pub const TEXTURE_TYPE_1D: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_TYPE_2D: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_TYPE_3D: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_TYPE_CUBE: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_TYPE_1D_ARRAY: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_TYPE_2D_ARRAY: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_TYPE_CUBE_ARRAY: Self = Self {
        ord: 6i32
    };
    pub const TEXTURE_TYPE_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for TextureType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureSamples {
    ord: i32
}
impl TextureSamples {
    pub const TEXTURE_SAMPLES_1: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_SAMPLES_2: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_SAMPLES_4: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_SAMPLES_8: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_SAMPLES_16: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_SAMPLES_32: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_SAMPLES_64: Self = Self {
        ord: 6i32
    };
    pub const TEXTURE_SAMPLES_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for TextureSamples {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for TextureSamples {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureSamples {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureSamples {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureSamples {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TextureUsageBits {
    ord: u64
}
impl TextureUsageBits {
    pub const TEXTURE_USAGE_SAMPLING_BIT: Self = Self {
        ord: 1u64
    };
    pub const TEXTURE_USAGE_COLOR_ATTACHMENT_BIT: Self = Self {
        ord: 2u64
    };
    pub const TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self {
        ord: 4u64
    };
    pub const TEXTURE_USAGE_STORAGE_BIT: Self = Self {
        ord: 8u64
    };
    pub const TEXTURE_USAGE_STORAGE_ATOMIC_BIT: Self = Self {
        ord: 16u64
    };
    pub const TEXTURE_USAGE_CPU_READ_BIT: Self = Self {
        ord: 32u64
    };
    pub const TEXTURE_USAGE_CAN_UPDATE_BIT: Self = Self {
        ord: 64u64
    };
    pub const TEXTURE_USAGE_CAN_COPY_FROM_BIT: Self = Self {
        ord: 128u64
    };
    pub const TEXTURE_USAGE_CAN_COPY_TO_BIT: Self = Self {
        ord: 256u64
    };
    pub const TEXTURE_USAGE_INPUT_ATTACHMENT_BIT: Self = Self {
        ord: 512u64
    };
    
}
impl crate::obj::EngineBitfield for TextureUsageBits {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for TextureUsageBits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for TextureUsageBits {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for TextureUsageBits {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureUsageBits {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureSwizzle {
    ord: i32
}
impl TextureSwizzle {
    pub const TEXTURE_SWIZZLE_IDENTITY: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_SWIZZLE_ZERO: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_SWIZZLE_ONE: Self = Self {
        ord: 2i32
    };
    pub const TEXTURE_SWIZZLE_R: Self = Self {
        ord: 3i32
    };
    pub const TEXTURE_SWIZZLE_G: Self = Self {
        ord: 4i32
    };
    pub const TEXTURE_SWIZZLE_B: Self = Self {
        ord: 5i32
    };
    pub const TEXTURE_SWIZZLE_A: Self = Self {
        ord: 6i32
    };
    pub const TEXTURE_SWIZZLE_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for TextureSwizzle {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for TextureSwizzle {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for TextureSwizzle {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureSwizzle {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureSwizzle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureSliceType {
    ord: i32
}
impl TextureSliceType {
    pub const TEXTURE_SLICE_2D: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_SLICE_CUBEMAP: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_SLICE_3D: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TextureSliceType {
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
impl crate::builtin::meta::GodotConvert for TextureSliceType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureSliceType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureSliceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SamplerFilter {
    ord: i32
}
impl SamplerFilter {
    pub const SAMPLER_FILTER_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const SAMPLER_FILTER_LINEAR: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for SamplerFilter {
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
impl crate::builtin::meta::GodotConvert for SamplerFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SamplerFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SamplerFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SamplerRepeatMode {
    ord: i32
}
impl SamplerRepeatMode {
    pub const SAMPLER_REPEAT_MODE_REPEAT: Self = Self {
        ord: 0i32
    };
    pub const SAMPLER_REPEAT_MODE_MIRRORED_REPEAT: Self = Self {
        ord: 1i32
    };
    pub const SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE: Self = Self {
        ord: 2i32
    };
    pub const SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER: Self = Self {
        ord: 3i32
    };
    pub const SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE: Self = Self {
        ord: 4i32
    };
    pub const SAMPLER_REPEAT_MODE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for SamplerRepeatMode {
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
impl crate::obj::IndexEnum for SamplerRepeatMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for SamplerRepeatMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SamplerRepeatMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SamplerRepeatMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SamplerBorderColor {
    ord: i32
}
impl SamplerBorderColor {
    pub const SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: Self = Self {
        ord: 0i32
    };
    pub const SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK: Self = Self {
        ord: 1i32
    };
    pub const SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK: Self = Self {
        ord: 2i32
    };
    pub const SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK: Self = Self {
        ord: 3i32
    };
    pub const SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE: Self = Self {
        ord: 4i32
    };
    pub const SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE: Self = Self {
        ord: 5i32
    };
    pub const SAMPLER_BORDER_COLOR_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for SamplerBorderColor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for SamplerBorderColor {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for SamplerBorderColor {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SamplerBorderColor {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SamplerBorderColor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VertexFrequency {
    ord: i32
}
impl VertexFrequency {
    pub const VERTEX_FREQUENCY_VERTEX: Self = Self {
        ord: 0i32
    };
    pub const VERTEX_FREQUENCY_INSTANCE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for VertexFrequency {
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
impl crate::builtin::meta::GodotConvert for VertexFrequency {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VertexFrequency {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VertexFrequency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct IndexBufferFormat {
    ord: i32
}
impl IndexBufferFormat {
    pub const INDEX_BUFFER_FORMAT_UINT16: Self = Self {
        ord: 0i32
    };
    pub const INDEX_BUFFER_FORMAT_UINT32: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for IndexBufferFormat {
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
impl crate::builtin::meta::GodotConvert for IndexBufferFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for IndexBufferFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for IndexBufferFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct StorageBufferUsage {
    ord: u64
}
impl StorageBufferUsage {
    pub const STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT: Self = Self {
        ord: 1u64
    };
    
}
impl crate::obj::EngineBitfield for StorageBufferUsage {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for StorageBufferUsage {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for StorageBufferUsage {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for StorageBufferUsage {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for StorageBufferUsage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UniformType {
    ord: i32
}
impl UniformType {
    pub const UNIFORM_TYPE_SAMPLER: Self = Self {
        ord: 0i32
    };
    pub const UNIFORM_TYPE_SAMPLER_WITH_TEXTURE: Self = Self {
        ord: 1i32
    };
    pub const UNIFORM_TYPE_TEXTURE: Self = Self {
        ord: 2i32
    };
    pub const UNIFORM_TYPE_IMAGE: Self = Self {
        ord: 3i32
    };
    pub const UNIFORM_TYPE_TEXTURE_BUFFER: Self = Self {
        ord: 4i32
    };
    pub const UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER: Self = Self {
        ord: 5i32
    };
    pub const UNIFORM_TYPE_IMAGE_BUFFER: Self = Self {
        ord: 6i32
    };
    pub const UNIFORM_TYPE_UNIFORM_BUFFER: Self = Self {
        ord: 7i32
    };
    pub const UNIFORM_TYPE_STORAGE_BUFFER: Self = Self {
        ord: 8i32
    };
    pub const UNIFORM_TYPE_INPUT_ATTACHMENT: Self = Self {
        ord: 9i32
    };
    pub const UNIFORM_TYPE_MAX: Self = Self {
        ord: 10i32
    };
    
}
impl crate::obj::EngineEnum for UniformType {
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
impl crate::obj::IndexEnum for UniformType {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::builtin::meta::GodotConvert for UniformType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for UniformType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UniformType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderPrimitive {
    ord: i32
}
impl RenderPrimitive {
    pub const RENDER_PRIMITIVE_POINTS: Self = Self {
        ord: 0i32
    };
    pub const RENDER_PRIMITIVE_LINES: Self = Self {
        ord: 1i32
    };
    pub const RENDER_PRIMITIVE_LINES_WITH_ADJACENCY: Self = Self {
        ord: 2i32
    };
    pub const RENDER_PRIMITIVE_LINESTRIPS: Self = Self {
        ord: 3i32
    };
    pub const RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY: Self = Self {
        ord: 4i32
    };
    pub const RENDER_PRIMITIVE_TRIANGLES: Self = Self {
        ord: 5i32
    };
    pub const RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY: Self = Self {
        ord: 6i32
    };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS: Self = Self {
        ord: 7i32
    };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY: Self = Self {
        ord: 8i32
    };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX: Self = Self {
        ord: 9i32
    };
    pub const RENDER_PRIMITIVE_TESSELATION_PATCH: Self = Self {
        ord: 10i32
    };
    pub const RENDER_PRIMITIVE_MAX: Self = Self {
        ord: 11i32
    };
    
}
impl crate::obj::EngineEnum for RenderPrimitive {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for RenderPrimitive {
    const ENUMERATOR_COUNT: usize = 11usize;
    
}
impl crate::builtin::meta::GodotConvert for RenderPrimitive {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RenderPrimitive {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RenderPrimitive {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PolygonCullMode {
    ord: i32
}
impl PolygonCullMode {
    pub const POLYGON_CULL_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const POLYGON_CULL_FRONT: Self = Self {
        ord: 1i32
    };
    pub const POLYGON_CULL_BACK: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PolygonCullMode {
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
impl crate::builtin::meta::GodotConvert for PolygonCullMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PolygonCullMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PolygonFrontFace {
    ord: i32
}
impl PolygonFrontFace {
    pub const POLYGON_FRONT_FACE_CLOCKWISE: Self = Self {
        ord: 0i32
    };
    pub const POLYGON_FRONT_FACE_COUNTER_CLOCKWISE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for PolygonFrontFace {
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
impl crate::builtin::meta::GodotConvert for PolygonFrontFace {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PolygonFrontFace {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PolygonFrontFace {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StencilOperation {
    ord: i32
}
impl StencilOperation {
    pub const STENCIL_OP_KEEP: Self = Self {
        ord: 0i32
    };
    pub const STENCIL_OP_ZERO: Self = Self {
        ord: 1i32
    };
    pub const STENCIL_OP_REPLACE: Self = Self {
        ord: 2i32
    };
    pub const STENCIL_OP_INCREMENT_AND_CLAMP: Self = Self {
        ord: 3i32
    };
    pub const STENCIL_OP_DECREMENT_AND_CLAMP: Self = Self {
        ord: 4i32
    };
    pub const STENCIL_OP_INVERT: Self = Self {
        ord: 5i32
    };
    pub const STENCIL_OP_INCREMENT_AND_WRAP: Self = Self {
        ord: 6i32
    };
    pub const STENCIL_OP_DECREMENT_AND_WRAP: Self = Self {
        ord: 7i32
    };
    pub const STENCIL_OP_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for StencilOperation {
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
impl crate::obj::IndexEnum for StencilOperation {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for StencilOperation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for StencilOperation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for StencilOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompareOperator {
    ord: i32
}
impl CompareOperator {
    pub const COMPARE_OP_NEVER: Self = Self {
        ord: 0i32
    };
    pub const COMPARE_OP_LESS: Self = Self {
        ord: 1i32
    };
    pub const COMPARE_OP_EQUAL: Self = Self {
        ord: 2i32
    };
    pub const COMPARE_OP_LESS_OR_EQUAL: Self = Self {
        ord: 3i32
    };
    pub const COMPARE_OP_GREATER: Self = Self {
        ord: 4i32
    };
    pub const COMPARE_OP_NOT_EQUAL: Self = Self {
        ord: 5i32
    };
    pub const COMPARE_OP_GREATER_OR_EQUAL: Self = Self {
        ord: 6i32
    };
    pub const COMPARE_OP_ALWAYS: Self = Self {
        ord: 7i32
    };
    pub const COMPARE_OP_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for CompareOperator {
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
impl crate::obj::IndexEnum for CompareOperator {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for CompareOperator {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompareOperator {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompareOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LogicOperation {
    ord: i32
}
impl LogicOperation {
    pub const LOGIC_OP_CLEAR: Self = Self {
        ord: 0i32
    };
    pub const LOGIC_OP_AND: Self = Self {
        ord: 1i32
    };
    pub const LOGIC_OP_AND_REVERSE: Self = Self {
        ord: 2i32
    };
    pub const LOGIC_OP_COPY: Self = Self {
        ord: 3i32
    };
    pub const LOGIC_OP_AND_INVERTED: Self = Self {
        ord: 4i32
    };
    pub const LOGIC_OP_NO_OP: Self = Self {
        ord: 5i32
    };
    pub const LOGIC_OP_XOR: Self = Self {
        ord: 6i32
    };
    pub const LOGIC_OP_OR: Self = Self {
        ord: 7i32
    };
    pub const LOGIC_OP_NOR: Self = Self {
        ord: 8i32
    };
    pub const LOGIC_OP_EQUIVALENT: Self = Self {
        ord: 9i32
    };
    pub const LOGIC_OP_INVERT: Self = Self {
        ord: 10i32
    };
    pub const LOGIC_OP_OR_REVERSE: Self = Self {
        ord: 11i32
    };
    pub const LOGIC_OP_COPY_INVERTED: Self = Self {
        ord: 12i32
    };
    pub const LOGIC_OP_OR_INVERTED: Self = Self {
        ord: 13i32
    };
    pub const LOGIC_OP_NAND: Self = Self {
        ord: 14i32
    };
    pub const LOGIC_OP_SET: Self = Self {
        ord: 15i32
    };
    pub const LOGIC_OP_MAX: Self = Self {
        ord: 16i32
    };
    
}
impl crate::obj::EngineEnum for LogicOperation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for LogicOperation {
    const ENUMERATOR_COUNT: usize = 16usize;
    
}
impl crate::builtin::meta::GodotConvert for LogicOperation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LogicOperation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LogicOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlendFactor {
    ord: i32
}
impl BlendFactor {
    pub const BLEND_FACTOR_ZERO: Self = Self {
        ord: 0i32
    };
    pub const BLEND_FACTOR_ONE: Self = Self {
        ord: 1i32
    };
    pub const BLEND_FACTOR_SRC_COLOR: Self = Self {
        ord: 2i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_SRC_COLOR: Self = Self {
        ord: 3i32
    };
    pub const BLEND_FACTOR_DST_COLOR: Self = Self {
        ord: 4i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_DST_COLOR: Self = Self {
        ord: 5i32
    };
    pub const BLEND_FACTOR_SRC_ALPHA: Self = Self {
        ord: 6i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: Self = Self {
        ord: 7i32
    };
    pub const BLEND_FACTOR_DST_ALPHA: Self = Self {
        ord: 8i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_DST_ALPHA: Self = Self {
        ord: 9i32
    };
    pub const BLEND_FACTOR_CONSTANT_COLOR: Self = Self {
        ord: 10i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: Self = Self {
        ord: 11i32
    };
    pub const BLEND_FACTOR_CONSTANT_ALPHA: Self = Self {
        ord: 12i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: Self = Self {
        ord: 13i32
    };
    pub const BLEND_FACTOR_SRC_ALPHA_SATURATE: Self = Self {
        ord: 14i32
    };
    pub const BLEND_FACTOR_SRC1_COLOR: Self = Self {
        ord: 15i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: Self = Self {
        ord: 16i32
    };
    pub const BLEND_FACTOR_SRC1_ALPHA: Self = Self {
        ord: 17i32
    };
    pub const BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: Self = Self {
        ord: 18i32
    };
    pub const BLEND_FACTOR_MAX: Self = Self {
        ord: 19i32
    };
    
}
impl crate::obj::EngineEnum for BlendFactor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for BlendFactor {
    const ENUMERATOR_COUNT: usize = 19usize;
    
}
impl crate::builtin::meta::GodotConvert for BlendFactor {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BlendFactor {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BlendFactor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BlendOperation {
    ord: i32
}
impl BlendOperation {
    pub const BLEND_OP_ADD: Self = Self {
        ord: 0i32
    };
    pub const BLEND_OP_SUBTRACT: Self = Self {
        ord: 1i32
    };
    pub const BLEND_OP_REVERSE_SUBTRACT: Self = Self {
        ord: 2i32
    };
    pub const BLEND_OP_MINIMUM: Self = Self {
        ord: 3i32
    };
    pub const BLEND_OP_MAXIMUM: Self = Self {
        ord: 4i32
    };
    pub const BLEND_OP_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for BlendOperation {
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
impl crate::obj::IndexEnum for BlendOperation {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for BlendOperation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BlendOperation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BlendOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PipelineDynamicStateFlags {
    ord: u64
}
impl PipelineDynamicStateFlags {
    pub const DYNAMIC_STATE_LINE_WIDTH: Self = Self {
        ord: 1u64
    };
    pub const DYNAMIC_STATE_DEPTH_BIAS: Self = Self {
        ord: 2u64
    };
    pub const DYNAMIC_STATE_BLEND_CONSTANTS: Self = Self {
        ord: 4u64
    };
    pub const DYNAMIC_STATE_DEPTH_BOUNDS: Self = Self {
        ord: 8u64
    };
    pub const DYNAMIC_STATE_STENCIL_COMPARE_MASK: Self = Self {
        ord: 16u64
    };
    pub const DYNAMIC_STATE_STENCIL_WRITE_MASK: Self = Self {
        ord: 32u64
    };
    pub const DYNAMIC_STATE_STENCIL_REFERENCE: Self = Self {
        ord: 64u64
    };
    
}
impl crate::obj::EngineBitfield for PipelineDynamicStateFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for PipelineDynamicStateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for PipelineDynamicStateFlags {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for PipelineDynamicStateFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PipelineDynamicStateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InitialAction {
    ord: i32
}
impl InitialAction {
    pub const INITIAL_ACTION_CLEAR: Self = Self {
        ord: 0i32
    };
    pub const INITIAL_ACTION_CLEAR_REGION: Self = Self {
        ord: 1i32
    };
    pub const INITIAL_ACTION_CLEAR_REGION_CONTINUE: Self = Self {
        ord: 2i32
    };
    pub const INITIAL_ACTION_KEEP: Self = Self {
        ord: 3i32
    };
    pub const INITIAL_ACTION_DROP: Self = Self {
        ord: 4i32
    };
    pub const INITIAL_ACTION_CONTINUE: Self = Self {
        ord: 5i32
    };
    pub const INITIAL_ACTION_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for InitialAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for InitialAction {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for InitialAction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InitialAction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InitialAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FinalAction {
    ord: i32
}
impl FinalAction {
    pub const FINAL_ACTION_READ: Self = Self {
        ord: 0i32
    };
    pub const FINAL_ACTION_DISCARD: Self = Self {
        ord: 1i32
    };
    pub const FINAL_ACTION_CONTINUE: Self = Self {
        ord: 2i32
    };
    pub const FINAL_ACTION_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for FinalAction {
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
impl crate::obj::IndexEnum for FinalAction {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for FinalAction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FinalAction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FinalAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShaderStage {
    ord: i32
}
impl ShaderStage {
    pub const SHADER_STAGE_VERTEX: Self = Self {
        ord: 0i32
    };
    pub const SHADER_STAGE_FRAGMENT: Self = Self {
        ord: 1i32
    };
    pub const SHADER_STAGE_TESSELATION_CONTROL: Self = Self {
        ord: 2i32
    };
    pub const SHADER_STAGE_TESSELATION_EVALUATION: Self = Self {
        ord: 3i32
    };
    pub const SHADER_STAGE_COMPUTE: Self = Self {
        ord: 4i32
    };
    pub const SHADER_STAGE_MAX: Self = Self {
        ord: 5i32
    };
    pub const SHADER_STAGE_VERTEX_BIT: Self = Self {
        ord: 1i32
    };
    pub const SHADER_STAGE_FRAGMENT_BIT: Self = Self {
        ord: 2i32
    };
    pub const SHADER_STAGE_TESSELATION_CONTROL_BIT: Self = Self {
        ord: 4i32
    };
    pub const SHADER_STAGE_TESSELATION_EVALUATION_BIT: Self = Self {
        ord: 8i32
    };
    pub const SHADER_STAGE_COMPUTE_BIT: Self = Self {
        ord: 16i32
    };
    
}
impl crate::obj::EngineEnum for ShaderStage {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ShaderStage {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShaderStage {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShaderStage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShaderLanguage {
    ord: i32
}
impl ShaderLanguage {
    pub const SHADER_LANGUAGE_GLSL: Self = Self {
        ord: 0i32
    };
    pub const SHADER_LANGUAGE_HLSL: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for ShaderLanguage {
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
impl crate::builtin::meta::GodotConvert for ShaderLanguage {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShaderLanguage {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShaderLanguage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PipelineSpecializationConstantType {
    ord: i32
}
impl PipelineSpecializationConstantType {
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL: Self = Self {
        ord: 0i32
    };
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT: Self = Self {
        ord: 1i32
    };
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PipelineSpecializationConstantType {
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
impl crate::builtin::meta::GodotConvert for PipelineSpecializationConstantType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PipelineSpecializationConstantType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PipelineSpecializationConstantType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Limit {
    ord: i32
}
impl Limit {
    pub const LIMIT_MAX_BOUND_UNIFORM_SETS: Self = Self {
        ord: 0i32
    };
    pub const LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS: Self = Self {
        ord: 1i32
    };
    pub const LIMIT_MAX_TEXTURES_PER_UNIFORM_SET: Self = Self {
        ord: 2i32
    };
    pub const LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET: Self = Self {
        ord: 3i32
    };
    pub const LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET: Self = Self {
        ord: 4i32
    };
    pub const LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET: Self = Self {
        ord: 5i32
    };
    pub const LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET: Self = Self {
        ord: 6i32
    };
    pub const LIMIT_MAX_DRAW_INDEXED_INDEX: Self = Self {
        ord: 7i32
    };
    pub const LIMIT_MAX_FRAMEBUFFER_HEIGHT: Self = Self {
        ord: 8i32
    };
    pub const LIMIT_MAX_FRAMEBUFFER_WIDTH: Self = Self {
        ord: 9i32
    };
    pub const LIMIT_MAX_TEXTURE_ARRAY_LAYERS: Self = Self {
        ord: 10i32
    };
    pub const LIMIT_MAX_TEXTURE_SIZE_1D: Self = Self {
        ord: 11i32
    };
    pub const LIMIT_MAX_TEXTURE_SIZE_2D: Self = Self {
        ord: 12i32
    };
    pub const LIMIT_MAX_TEXTURE_SIZE_3D: Self = Self {
        ord: 13i32
    };
    pub const LIMIT_MAX_TEXTURE_SIZE_CUBE: Self = Self {
        ord: 14i32
    };
    pub const LIMIT_MAX_TEXTURES_PER_SHADER_STAGE: Self = Self {
        ord: 15i32
    };
    pub const LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE: Self = Self {
        ord: 16i32
    };
    pub const LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE: Self = Self {
        ord: 17i32
    };
    pub const LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE: Self = Self {
        ord: 18i32
    };
    pub const LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: Self = Self {
        ord: 19i32
    };
    pub const LIMIT_MAX_PUSH_CONSTANT_SIZE: Self = Self {
        ord: 20i32
    };
    pub const LIMIT_MAX_UNIFORM_BUFFER_SIZE: Self = Self {
        ord: 21i32
    };
    pub const LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET: Self = Self {
        ord: 22i32
    };
    pub const LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES: Self = Self {
        ord: 23i32
    };
    pub const LIMIT_MAX_VERTEX_INPUT_BINDINGS: Self = Self {
        ord: 24i32
    };
    pub const LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE: Self = Self {
        ord: 25i32
    };
    pub const LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT: Self = Self {
        ord: 26i32
    };
    pub const LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE: Self = Self {
        ord: 27i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X: Self = Self {
        ord: 28i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y: Self = Self {
        ord: 29i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z: Self = Self {
        ord: 30i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS: Self = Self {
        ord: 31i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X: Self = Self {
        ord: 32i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y: Self = Self {
        ord: 33i32
    };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z: Self = Self {
        ord: 34i32
    };
    pub const LIMIT_MAX_VIEWPORT_DIMENSIONS_X: Self = Self {
        ord: 35i32
    };
    pub const LIMIT_MAX_VIEWPORT_DIMENSIONS_Y: Self = Self {
        ord: 36i32
    };
    
}
impl crate::obj::EngineEnum for Limit {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Limit {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Limit {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Limit {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MemoryType {
    ord: i32
}
impl MemoryType {
    pub const MEMORY_TEXTURES: Self = Self {
        ord: 0i32
    };
    pub const MEMORY_BUFFERS: Self = Self {
        ord: 1i32
    };
    pub const MEMORY_TOTAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for MemoryType {
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
impl crate::builtin::meta::GodotConvert for MemoryType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MemoryType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MemoryType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}