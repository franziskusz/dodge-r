#![doc = "Sidecar module for class [`Image`][crate::engine::Image].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Image` enums](https://docs.godotengine.org/en/stable/classes/class_image.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Image.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`image`][crate::engine::image]: sidecar module with related enum/flag types\n* [`IImage`][crate::engine::IImage]: virtual methods\n\n\nSee also [Godot docs for `Image`](https://docs.godotengine.org/en/stable/classes/class_image.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Image {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Image`][crate::engine::Image].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Image` methods](https://docs.godotengine.org/en/stable/classes/class_image.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImage: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Image {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_width(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_mipmaps(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_format(&self,) -> crate::engine::image::Format {
            type RetMarshal = PtrcallReturnT < crate::engine::image::Format >;
            type CallSig = (crate::engine::image::Format,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert(&mut self, format: crate::engine::image::Format,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::image::Format);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mipmap_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mipmap_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mipmap_offset(&self, mipmap: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mipmap_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn resize_to_po2_full(&mut self, square: bool, interpolation: crate::engine::image::Interpolation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, crate::engine::image::Interpolation);
            let args = (square, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resize_to_po2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resize_to_po2(&mut self,) {
            self.resize_to_po2_ex() . done()
        }
        #[inline]
        pub fn resize_to_po2_ex(&mut self,) -> ExResizeToPo2 < '_ > {
            ExResizeToPo2::new(self,)
        }
        pub(crate) fn resize_full(&mut self, width: i32, height: i32, interpolation: crate::engine::image::Interpolation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, crate::engine::image::Interpolation);
            let args = (width, height, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn resize(&mut self, width: i32, height: i32,) {
            self.resize_ex(width, height,) . done()
        }
        #[inline]
        pub fn resize_ex(&mut self, width: i32, height: i32,) -> ExResize < '_ > {
            ExResize::new(self, width, height,)
        }
        pub fn shrink_x2(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shrink_x2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn crop(&mut self, width: i32, height: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (width, height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "crop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn flip_x(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flip_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn flip_y(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flip_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_mipmaps_full(&mut self, renormalize: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, bool);
            let args = (renormalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn generate_mipmaps(&mut self,) -> crate::engine::global::Error {
            self.generate_mipmaps_ex() . done()
        }
        #[inline]
        pub fn generate_mipmaps_ex(&mut self,) -> ExGenerateMipmaps < '_ > {
            ExGenerateMipmaps::new(self,)
        }
        pub fn clear_mipmaps(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create(width: i32, height: i32, use_mipmaps: bool, format: crate::engine::image::Format,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32, i32, bool, crate::engine::image::Format);
            let args = (width, height, use_mipmaps, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn create_from_data(width: i32, height: i32, use_mipmaps: bool, format: crate::engine::image::Format, data: PackedByteArray,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32, i32, bool, crate::engine::image::Format, PackedByteArray);
            let args = (width, height, use_mipmaps, format, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_data", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_data(&mut self, width: i32, height: i32, use_mipmaps: bool, format: crate::engine::image::Format, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool, crate::engine::image::Format, PackedByteArray);
            let args = (width, height, use_mipmaps, format, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_empty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_from_file(path: GString,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_from_file", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn save_png(&self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_png", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_png_to_buffer(&self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_png_to_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn save_jpg_full(&self, path: GString, quality: f32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, f32);
            let args = (path, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_jpg", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_jpg(&self, path: GString,) -> crate::engine::global::Error {
            self.save_jpg_ex(path,) . done()
        }
        #[inline]
        pub fn save_jpg_ex(&self, path: GString,) -> ExSaveJpg < '_ > {
            ExSaveJpg::new(self, path,)
        }
        pub(crate) fn save_jpg_to_buffer_full(&self, quality: f32,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, f32);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_jpg_to_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_jpg_to_buffer(&self,) -> PackedByteArray {
            self.save_jpg_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_jpg_to_buffer_ex(&self,) -> ExSaveJpgToBuffer < '_ > {
            ExSaveJpgToBuffer::new(self,)
        }
        pub(crate) fn save_exr_full(&self, path: GString, grayscale: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (path, grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_exr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_exr(&self, path: GString,) -> crate::engine::global::Error {
            self.save_exr_ex(path,) . done()
        }
        #[inline]
        pub fn save_exr_ex(&self, path: GString,) -> ExSaveExr < '_ > {
            ExSaveExr::new(self, path,)
        }
        pub(crate) fn save_exr_to_buffer_full(&self, grayscale: bool,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, bool);
            let args = (grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_exr_to_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_exr_to_buffer(&self,) -> PackedByteArray {
            self.save_exr_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_exr_to_buffer_ex(&self,) -> ExSaveExrToBuffer < '_ > {
            ExSaveExrToBuffer::new(self,)
        }
        pub(crate) fn save_webp_full(&self, path: GString, lossy: bool, quality: f32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool, f32);
            let args = (path, lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_webp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_webp(&self, path: GString,) -> crate::engine::global::Error {
            self.save_webp_ex(path,) . done()
        }
        #[inline]
        pub fn save_webp_ex(&self, path: GString,) -> ExSaveWebp < '_ > {
            ExSaveWebp::new(self, path,)
        }
        pub(crate) fn save_webp_to_buffer_full(&self, lossy: bool, quality: f32,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, bool, f32);
            let args = (lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_webp_to_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_webp_to_buffer(&self,) -> PackedByteArray {
            self.save_webp_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_webp_to_buffer_ex(&self,) -> ExSaveWebpToBuffer < '_ > {
            ExSaveWebpToBuffer::new(self,)
        }
        pub fn detect_alpha(&self,) -> crate::engine::image::AlphaMode {
            type RetMarshal = PtrcallReturnT < crate::engine::image::AlphaMode >;
            type CallSig = (crate::engine::image::AlphaMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "detect_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_invisible(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_invisible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn detect_used_channels_full(&self, source: crate::engine::image::CompressSource,) -> crate::engine::image::UsedChannels {
            type RetMarshal = PtrcallReturnT < crate::engine::image::UsedChannels >;
            type CallSig = (crate::engine::image::UsedChannels, crate::engine::image::CompressSource);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "detect_used_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn detect_used_channels(&self,) -> crate::engine::image::UsedChannels {
            self.detect_used_channels_ex() . done()
        }
        #[inline]
        pub fn detect_used_channels_ex(&self,) -> ExDetectUsedChannels < '_ > {
            ExDetectUsedChannels::new(self,)
        }
        pub(crate) fn compress_full(&mut self, mode: crate::engine::image::CompressMode, source: crate::engine::image::CompressSource, astc_format: crate::engine::image::ASTCFormat,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, crate::engine::image::CompressMode, crate::engine::image::CompressSource, crate::engine::image::ASTCFormat);
            let args = (mode, source, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compress(&mut self, mode: crate::engine::image::CompressMode,) -> crate::engine::global::Error {
            self.compress_ex(mode,) . done()
        }
        #[inline]
        pub fn compress_ex(&mut self, mode: crate::engine::image::CompressMode,) -> ExCompress < '_ > {
            ExCompress::new(self, mode,)
        }
        pub(crate) fn compress_from_channels_full(&mut self, mode: crate::engine::image::CompressMode, channels: crate::engine::image::UsedChannels, astc_format: crate::engine::image::ASTCFormat,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, crate::engine::image::CompressMode, crate::engine::image::UsedChannels, crate::engine::image::ASTCFormat);
            let args = (mode, channels, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compress_from_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn compress_from_channels(&mut self, mode: crate::engine::image::CompressMode, channels: crate::engine::image::UsedChannels,) -> crate::engine::global::Error {
            self.compress_from_channels_ex(mode, channels,) . done()
        }
        #[inline]
        pub fn compress_from_channels_ex(&mut self, mode: crate::engine::image::CompressMode, channels: crate::engine::image::UsedChannels,) -> ExCompressFromChannels < '_ > {
            ExCompressFromChannels::new(self, mode, channels,)
        }
        pub fn decompress(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decompress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_compressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_compressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_90(&mut self, direction: crate::engine::global::ClockDirection,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::ClockDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_90", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_180(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate_180", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fix_alpha_edges(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fix_alpha_edges", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn premultiply_alpha(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "premultiply_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn srgb_to_linear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "srgb_to_linear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn normal_map_to_xy(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "normal_map_to_xy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rgbe_to_srgb(&mut self,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rgbe_to_srgb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bump_map_to_normal_map_full(&mut self, bump_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bump_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bump_map_to_normal_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn bump_map_to_normal_map(&mut self,) {
            self.bump_map_to_normal_map_ex() . done()
        }
        #[inline]
        pub fn bump_map_to_normal_map_ex(&mut self,) -> ExBumpMapToNormalMap < '_ > {
            ExBumpMapToNormalMap::new(self,)
        }
        pub fn compute_image_metrics(&mut self, compared_image: Gd < crate::engine::Image >, use_luma: bool,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Gd < crate::engine::Image >, bool);
            let args = (compared_image, use_luma,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_image_metrics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blit_rect(&mut self, src: Gd < crate::engine::Image >, src_rect: Rect2i, dst: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, Rect2i, Vector2i);
            let args = (src, src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blit_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blit_rect_mask(&mut self, src: Gd < crate::engine::Image >, mask: Gd < crate::engine::Image >, src_rect: Rect2i, dst: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, Gd < crate::engine::Image >, Rect2i, Vector2i);
            let args = (src, mask, src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blit_rect_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blend_rect(&mut self, src: Gd < crate::engine::Image >, src_rect: Rect2i, dst: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, Rect2i, Vector2i);
            let args = (src, src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn blend_rect_mask(&mut self, src: Gd < crate::engine::Image >, mask: Gd < crate::engine::Image >, src_rect: Rect2i, dst: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, Gd < crate::engine::Image >, Rect2i, Vector2i);
            let args = (src, mask, src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "blend_rect_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fill(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fill_rect(&mut self, rect: Rect2i, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rect2i, Color);
            let args = (rect, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fill_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_rect(&self,) -> Rect2i {
            type RetMarshal = PtrcallReturnT < Rect2i >;
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_used_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region(&self, region: Rect2i,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rect2i);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn copy_from(&mut self, src: Gd < crate::engine::Image >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >);
            let args = (src,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "copy_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixelv(&self, point: Vector2i,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, Vector2i);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pixelv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixel(&self, x: i32, y: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32, i32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixelv(&mut self, point: Vector2i, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i, Color);
            let args = (point, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pixelv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixel(&mut self, x: i32, y: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Color);
            let args = (x, y, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn adjust_bcs(&mut self, brightness: f32, contrast: f32, saturation: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, f32, f32);
            let args = (brightness, contrast, saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "adjust_bcs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_png_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_png_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_jpg_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_jpg_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_webp_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_webp_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_tga_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_tga_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_bmp_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_bmp_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_ktx_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_ktx_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn load_svg_from_buffer_full(&mut self, buffer: PackedByteArray, scale: f32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray, f32);
            let args = (buffer, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_svg_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load_svg_from_buffer(&mut self, buffer: PackedByteArray,) -> crate::engine::global::Error {
            self.load_svg_from_buffer_ex(buffer,) . done()
        }
        #[inline]
        pub fn load_svg_from_buffer_ex(&mut self, buffer: PackedByteArray,) -> ExLoadSvgFromBuffer < '_ > {
            ExLoadSvgFromBuffer::new(self, buffer,)
        }
        pub(crate) fn load_svg_from_string_full(&mut self, svg_str: GString, scale: f32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, f32);
            let args = (svg_str, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_svg_from_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load_svg_from_string(&mut self, svg_str: GString,) -> crate::engine::global::Error {
            self.load_svg_from_string_ex(svg_str,) . done()
        }
        #[inline]
        pub fn load_svg_from_string_ex(&mut self, svg_str: GString,) -> ExLoadSvgFromString < '_ > {
            ExLoadSvgFromString::new(self, svg_str,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const MAX_WIDTH: i32 = 16777216i32;
        pub const MAX_HEIGHT: i32 = 16777216i32;
        
    }
    impl crate::obj::GodotClass for Image {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Image\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Image {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Image {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Image {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Image {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Image {
        
    }
    impl crate::obj::ExportableObject for Image {
        
    }
    impl crate::obj::cap::GodotDefault for Image {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Image {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Image {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Image {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Image > for $Class {
                
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
#[doc = "Default-param extender for [`Image::resize_to_po2_ex`][super::Image::resize_to_po2_ex]."]
#[must_use]
pub struct ExResizeToPo2 < 'a > {
    surround_object: &'a mut re_export::Image, square: bool, interpolation: crate::engine::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResizeToPo2 < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        Self {
            surround_object, square: false, interpolation: crate::obj::EngineEnum::from_ord(1),
        }
    }
    #[inline]
    pub fn square(self, value: bool) -> Self {
        Self {
            square: value, .. self
        }
    }
    #[inline]
    pub fn interpolation(self, value: crate::engine::image::Interpolation) -> Self {
        Self {
            interpolation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Image::resize_to_po2_full(self.surround_object, self.square, self.interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::resize_ex`][super::Image::resize_ex]."]
#[must_use]
pub struct ExResize < 'a > {
    surround_object: &'a mut re_export::Image, width: i32, height: i32, interpolation: crate::engine::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResize < 'a > {
    fn new(surround_object: &'a mut re_export::Image, width: i32, height: i32,) -> Self {
        Self {
            surround_object, width, height, interpolation: crate::obj::EngineEnum::from_ord(1),
        }
    }
    #[inline]
    pub fn interpolation(self, value: crate::engine::image::Interpolation) -> Self {
        Self {
            interpolation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Image::resize_full(self.surround_object, self.width, self.height, self.interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::generate_mipmaps_ex`][super::Image::generate_mipmaps_ex]."]
#[must_use]
pub struct ExGenerateMipmaps < 'a > {
    surround_object: &'a mut re_export::Image, renormalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateMipmaps < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        Self {
            surround_object, renormalize: false,
        }
    }
    #[inline]
    pub fn renormalize(self, value: bool) -> Self {
        Self {
            renormalize: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::generate_mipmaps_full(self.surround_object, self.renormalize,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_ex`][super::Image::save_jpg_ex]."]
#[must_use]
pub struct ExSaveJpg < 'a > {
    surround_object: &'a re_export::Image, path: GString, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveJpg < 'a > {
    fn new(surround_object: &'a re_export::Image, path: GString,) -> Self {
        Self {
            surround_object, path, quality: 0.75f32,
        }
    }
    #[inline]
    pub fn quality(self, value: f32) -> Self {
        Self {
            quality: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::save_jpg_full(self.surround_object, self.path, self.quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_to_buffer_ex`][super::Image::save_jpg_to_buffer_ex]."]
#[must_use]
pub struct ExSaveJpgToBuffer < 'a > {
    surround_object: &'a re_export::Image, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveJpgToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        Self {
            surround_object, quality: 0.75f32,
        }
    }
    #[inline]
    pub fn quality(self, value: f32) -> Self {
        Self {
            quality: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::Image::save_jpg_to_buffer_full(self.surround_object, self.quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_ex`][super::Image::save_exr_ex]."]
#[must_use]
pub struct ExSaveExr < 'a > {
    surround_object: &'a re_export::Image, path: GString, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveExr < 'a > {
    fn new(surround_object: &'a re_export::Image, path: GString,) -> Self {
        Self {
            surround_object, path, grayscale: false,
        }
    }
    #[inline]
    pub fn grayscale(self, value: bool) -> Self {
        Self {
            grayscale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::save_exr_full(self.surround_object, self.path, self.grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_to_buffer_ex`][super::Image::save_exr_to_buffer_ex]."]
#[must_use]
pub struct ExSaveExrToBuffer < 'a > {
    surround_object: &'a re_export::Image, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveExrToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        Self {
            surround_object, grayscale: false,
        }
    }
    #[inline]
    pub fn grayscale(self, value: bool) -> Self {
        Self {
            grayscale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::Image::save_exr_to_buffer_full(self.surround_object, self.grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_ex`][super::Image::save_webp_ex]."]
#[must_use]
pub struct ExSaveWebp < 'a > {
    surround_object: &'a re_export::Image, path: GString, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveWebp < 'a > {
    fn new(surround_object: &'a re_export::Image, path: GString,) -> Self {
        Self {
            surround_object, path, lossy: false, quality: 0.75f32,
        }
    }
    #[inline]
    pub fn lossy(self, value: bool) -> Self {
        Self {
            lossy: value, .. self
        }
    }
    #[inline]
    pub fn quality(self, value: f32) -> Self {
        Self {
            quality: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::save_webp_full(self.surround_object, self.path, self.lossy, self.quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_to_buffer_ex`][super::Image::save_webp_to_buffer_ex]."]
#[must_use]
pub struct ExSaveWebpToBuffer < 'a > {
    surround_object: &'a re_export::Image, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveWebpToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        Self {
            surround_object, lossy: false, quality: 0.75f32,
        }
    }
    #[inline]
    pub fn lossy(self, value: bool) -> Self {
        Self {
            lossy: value, .. self
        }
    }
    #[inline]
    pub fn quality(self, value: f32) -> Self {
        Self {
            quality: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::Image::save_webp_to_buffer_full(self.surround_object, self.lossy, self.quality,)
    }
}
#[doc = "Default-param extender for [`Image::detect_used_channels_ex`][super::Image::detect_used_channels_ex]."]
#[must_use]
pub struct ExDetectUsedChannels < 'a > {
    surround_object: &'a re_export::Image, source: crate::engine::image::CompressSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDetectUsedChannels < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        Self {
            surround_object, source: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn source(self, value: crate::engine::image::CompressSource) -> Self {
        Self {
            source: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::image::UsedChannels {
        re_export::Image::detect_used_channels_full(self.surround_object, self.source,)
    }
}
#[doc = "Default-param extender for [`Image::compress_ex`][super::Image::compress_ex]."]
#[must_use]
pub struct ExCompress < 'a > {
    surround_object: &'a mut re_export::Image, mode: crate::engine::image::CompressMode, source: crate::engine::image::CompressSource, astc_format: crate::engine::image::ASTCFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompress < 'a > {
    fn new(surround_object: &'a mut re_export::Image, mode: crate::engine::image::CompressMode,) -> Self {
        Self {
            surround_object, mode, source: crate::obj::EngineEnum::from_ord(0), astc_format: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn source(self, value: crate::engine::image::CompressSource) -> Self {
        Self {
            source: value, .. self
        }
    }
    #[inline]
    pub fn astc_format(self, value: crate::engine::image::ASTCFormat) -> Self {
        Self {
            astc_format: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::compress_full(self.surround_object, self.mode, self.source, self.astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::compress_from_channels_ex`][super::Image::compress_from_channels_ex]."]
#[must_use]
pub struct ExCompressFromChannels < 'a > {
    surround_object: &'a mut re_export::Image, mode: crate::engine::image::CompressMode, channels: crate::engine::image::UsedChannels, astc_format: crate::engine::image::ASTCFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompressFromChannels < 'a > {
    fn new(surround_object: &'a mut re_export::Image, mode: crate::engine::image::CompressMode, channels: crate::engine::image::UsedChannels,) -> Self {
        Self {
            surround_object, mode, channels, astc_format: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn astc_format(self, value: crate::engine::image::ASTCFormat) -> Self {
        Self {
            astc_format: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::compress_from_channels_full(self.surround_object, self.mode, self.channels, self.astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::bump_map_to_normal_map_ex`][super::Image::bump_map_to_normal_map_ex]."]
#[must_use]
pub struct ExBumpMapToNormalMap < 'a > {
    surround_object: &'a mut re_export::Image, bump_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBumpMapToNormalMap < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        Self {
            surround_object, bump_scale: 1f32,
        }
    }
    #[inline]
    pub fn bump_scale(self, value: f32) -> Self {
        Self {
            bump_scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Image::bump_map_to_normal_map_full(self.surround_object, self.bump_scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_buffer_ex`][super::Image::load_svg_from_buffer_ex]."]
#[must_use]
pub struct ExLoadSvgFromBuffer < 'a > {
    surround_object: &'a mut re_export::Image, buffer: PackedByteArray, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadSvgFromBuffer < 'a > {
    fn new(surround_object: &'a mut re_export::Image, buffer: PackedByteArray,) -> Self {
        Self {
            surround_object, buffer, scale: 1f32,
        }
    }
    #[inline]
    pub fn scale(self, value: f32) -> Self {
        Self {
            scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::load_svg_from_buffer_full(self.surround_object, self.buffer, self.scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_string_ex`][super::Image::load_svg_from_string_ex]."]
#[must_use]
pub struct ExLoadSvgFromString < 'a > {
    surround_object: &'a mut re_export::Image, svg_str: GString, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadSvgFromString < 'a > {
    fn new(surround_object: &'a mut re_export::Image, svg_str: GString,) -> Self {
        Self {
            surround_object, svg_str, scale: 1f32,
        }
    }
    #[inline]
    pub fn scale(self, value: f32) -> Self {
        Self {
            scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Image::load_svg_from_string_full(self.surround_object, self.svg_str, self.scale,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Format {
    ord: i32
}
impl Format {
    pub const FORMAT_L8: Self = Self {
        ord: 0i32
    };
    pub const FORMAT_LA8: Self = Self {
        ord: 1i32
    };
    pub const FORMAT_R8: Self = Self {
        ord: 2i32
    };
    pub const FORMAT_RG8: Self = Self {
        ord: 3i32
    };
    pub const FORMAT_RGB8: Self = Self {
        ord: 4i32
    };
    pub const FORMAT_RGBA8: Self = Self {
        ord: 5i32
    };
    pub const FORMAT_RGBA4444: Self = Self {
        ord: 6i32
    };
    pub const FORMAT_RGB565: Self = Self {
        ord: 7i32
    };
    pub const FORMAT_RF: Self = Self {
        ord: 8i32
    };
    pub const FORMAT_RGF: Self = Self {
        ord: 9i32
    };
    pub const FORMAT_RGBF: Self = Self {
        ord: 10i32
    };
    pub const FORMAT_RGBAF: Self = Self {
        ord: 11i32
    };
    pub const FORMAT_RH: Self = Self {
        ord: 12i32
    };
    pub const FORMAT_RGH: Self = Self {
        ord: 13i32
    };
    pub const FORMAT_RGBH: Self = Self {
        ord: 14i32
    };
    pub const FORMAT_RGBAH: Self = Self {
        ord: 15i32
    };
    pub const FORMAT_RGBE9995: Self = Self {
        ord: 16i32
    };
    pub const FORMAT_DXT1: Self = Self {
        ord: 17i32
    };
    pub const FORMAT_DXT3: Self = Self {
        ord: 18i32
    };
    pub const FORMAT_DXT5: Self = Self {
        ord: 19i32
    };
    pub const FORMAT_RGTC_R: Self = Self {
        ord: 20i32
    };
    pub const FORMAT_RGTC_RG: Self = Self {
        ord: 21i32
    };
    pub const FORMAT_BPTC_RGBA: Self = Self {
        ord: 22i32
    };
    pub const FORMAT_BPTC_RGBF: Self = Self {
        ord: 23i32
    };
    pub const FORMAT_BPTC_RGBFU: Self = Self {
        ord: 24i32
    };
    pub const FORMAT_ETC: Self = Self {
        ord: 25i32
    };
    pub const FORMAT_ETC2_R11: Self = Self {
        ord: 26i32
    };
    pub const FORMAT_ETC2_R11S: Self = Self {
        ord: 27i32
    };
    pub const FORMAT_ETC2_RG11: Self = Self {
        ord: 28i32
    };
    pub const FORMAT_ETC2_RG11S: Self = Self {
        ord: 29i32
    };
    pub const FORMAT_ETC2_RGB8: Self = Self {
        ord: 30i32
    };
    pub const FORMAT_ETC2_RGBA8: Self = Self {
        ord: 31i32
    };
    pub const FORMAT_ETC2_RGB8A1: Self = Self {
        ord: 32i32
    };
    pub const FORMAT_ETC2_RA_AS_RG: Self = Self {
        ord: 33i32
    };
    pub const FORMAT_DXT5_RA_AS_RG: Self = Self {
        ord: 34i32
    };
    pub const FORMAT_ASTC_4x4: Self = Self {
        ord: 35i32
    };
    pub const FORMAT_ASTC_4x4_HDR: Self = Self {
        ord: 36i32
    };
    pub const FORMAT_ASTC_8x8: Self = Self {
        ord: 37i32
    };
    pub const FORMAT_ASTC_8x8_HDR: Self = Self {
        ord: 38i32
    };
    pub const FORMAT_MAX: Self = Self {
        ord: 39i32
    };
    
}
impl crate::obj::EngineEnum for Format {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Format {
    const ENUMERATOR_COUNT: usize = 39usize;
    
}
impl crate::builtin::meta::GodotConvert for Format {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Format {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Format {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Interpolation {
    ord: i32
}
impl Interpolation {
    pub const INTERPOLATE_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const INTERPOLATE_BILINEAR: Self = Self {
        ord: 1i32
    };
    pub const INTERPOLATE_CUBIC: Self = Self {
        ord: 2i32
    };
    pub const INTERPOLATE_TRILINEAR: Self = Self {
        ord: 3i32
    };
    pub const INTERPOLATE_LANCZOS: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for Interpolation {
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
impl crate::builtin::meta::GodotConvert for Interpolation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Interpolation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Interpolation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlphaMode {
    ord: i32
}
impl AlphaMode {
    pub const ALPHA_NONE: Self = Self {
        ord: 0i32
    };
    pub const ALPHA_BIT: Self = Self {
        ord: 1i32
    };
    pub const ALPHA_BLEND: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AlphaMode {
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
impl crate::builtin::meta::GodotConvert for AlphaMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AlphaMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AlphaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompressMode {
    ord: i32
}
impl CompressMode {
    pub const COMPRESS_S3TC: Self = Self {
        ord: 0i32
    };
    pub const COMPRESS_ETC: Self = Self {
        ord: 1i32
    };
    pub const COMPRESS_ETC2: Self = Self {
        ord: 2i32
    };
    pub const COMPRESS_BPTC: Self = Self {
        ord: 3i32
    };
    pub const COMPRESS_ASTC: Self = Self {
        ord: 4i32
    };
    pub const COMPRESS_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for CompressMode {
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
impl crate::obj::IndexEnum for CompressMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for CompressMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompressMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompressMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UsedChannels {
    ord: i32
}
impl UsedChannels {
    pub const USED_CHANNELS_L: Self = Self {
        ord: 0i32
    };
    pub const USED_CHANNELS_LA: Self = Self {
        ord: 1i32
    };
    pub const USED_CHANNELS_R: Self = Self {
        ord: 2i32
    };
    pub const USED_CHANNELS_RG: Self = Self {
        ord: 3i32
    };
    pub const USED_CHANNELS_RGB: Self = Self {
        ord: 4i32
    };
    pub const USED_CHANNELS_RGBA: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for UsedChannels {
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
impl crate::builtin::meta::GodotConvert for UsedChannels {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for UsedChannels {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UsedChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompressSource {
    ord: i32
}
impl CompressSource {
    pub const COMPRESS_SOURCE_GENERIC: Self = Self {
        ord: 0i32
    };
    pub const COMPRESS_SOURCE_SRGB: Self = Self {
        ord: 1i32
    };
    pub const COMPRESS_SOURCE_NORMAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CompressSource {
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
impl crate::builtin::meta::GodotConvert for CompressSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompressSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompressSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ASTCFormat {
    ord: i32
}
impl ASTCFormat {
    pub const ASTC_FORMAT_4x4: Self = Self {
        ord: 0i32
    };
    pub const ASTC_FORMAT_8x8: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for ASTCFormat {
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
impl crate::builtin::meta::GodotConvert for ASTCFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ASTCFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ASTCFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}