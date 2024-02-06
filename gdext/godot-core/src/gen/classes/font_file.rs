#![doc = "Sidecar module for class [`FontFile`][crate::engine::FontFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontFile` enums](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FontFile.`\n\nInherits [`Font`][crate::engine::Font].\n\nRelated symbols:\n\n* [`IFontFile`][crate::engine::IFontFile]: virtual methods\n\n\nSee also [Godot docs for `FontFile`](https://docs.godotengine.org/en/stable/classes/class_fontfile.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FontFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FontFile`][crate::engine::FontFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FontFile` methods](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFontFile: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FontFile {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn load_bitmap_font(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_bitmap_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_dynamic_font(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_dynamic_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_data(&mut self, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedByteArray);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_name(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_style_name(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_style(&mut self, style: crate::engine::text_server::FontStyle,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::FontStyle);
            let args = (style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_font_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_antialiasing(&mut self, antialiasing: crate::engine::text_server::FontAntialiasing,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::FontAntialiasing);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::engine::text_server::FontAntialiasing {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FontAntialiasing >;
            type CallSig = (crate::engine::text_server::FontAntialiasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_size(&mut self, fixed_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_size(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_size_scale_mode(&mut self, fixed_size_scale_mode: crate::engine::text_server::FixedSizeScaleMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::FixedSizeScaleMode);
            let args = (fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_size_scale_mode(&self,) -> crate::engine::text_server::FixedSizeScaleMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FixedSizeScaleMode >;
            type CallSig = (crate::engine::text_server::FixedSizeScaleMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::engine::text_server::Hinting,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::Hinting);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::engine::text_server::Hinting {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Hinting >;
            type CallSig = (crate::engine::text_server::Hinting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::engine::text_server::SubpixelPositioning,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::text_server::SubpixelPositioning);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::engine::text_server::SubpixelPositioning {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::SubpixelPositioning >;
            type CallSig = (crate::engine::text_server::SubpixelPositioning,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_cache(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_cache(&mut self, cache_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size_cache_list(&self, cache_index: i32,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_size_cache_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_size_cache(&mut self, cache_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_size_cache(&mut self, cache_index: i32, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_coordinates(&mut self, cache_index: i32, variation_coordinates: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Dictionary);
            let args = (cache_index, variation_coordinates,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_coordinates(&self, cache_index: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_embolden(&mut self, cache_index: i32, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (cache_index, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_embolden(&self, cache_index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, cache_index: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Transform2D);
            let args = (cache_index, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self, cache_index: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_extra_spacing(&mut self, cache_index: i32, spacing: crate::engine::text_server::SpacingType, value: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::text_server::SpacingType, i64);
            let args = (cache_index, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_extra_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extra_spacing(&self, cache_index: i32, spacing: crate::engine::text_server::SpacingType,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, i32, crate::engine::text_server::SpacingType);
            let args = (cache_index, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_extra_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_face_index(&mut self, cache_index: i32, face_index: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i64);
            let args = (cache_index, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_index(&self, cache_index: i32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_ascent(&mut self, cache_index: i32, size: i32, ascent: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_ascent(&self, cache_index: i32, size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_descent(&mut self, cache_index: i32, size: i32, descent: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_descent(&self, cache_index: i32, size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_underline_position(&mut self, cache_index: i32, size: i32, underline_position: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_underline_position(&self, cache_index: i32, size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_underline_thickness(&mut self, cache_index: i32, size: i32, underline_thickness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_underline_thickness(&self, cache_index: i32, size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_scale(&mut self, cache_index: i32, size: i32, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cache_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_scale(&self, cache_index: i32, size: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cache_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_count(&self, cache_index: i32, size: Vector2i,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_textures(&mut self, cache_index: i32, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_texture(&mut self, cache_index: i32, size: Vector2i, texture_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_image(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, image: Gd < crate::engine::Image >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Gd < crate::engine::Image >);
            let args = (cache_index, size, texture_index, image,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_image(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_offsets(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, offset: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, PackedInt32Array);
            let args = (cache_index, size, texture_index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_offsets(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_list(&self, cache_index: i32, size: Vector2i,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_glyphs(&mut self, cache_index: i32, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_glyph(&mut self, cache_index: i32, size: Vector2i, glyph: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_advance(&mut self, cache_index: i32, size: i32, glyph: i32, advance: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32, Vector2);
            let args = (cache_index, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_advance(&self, cache_index: i32, size: i32, glyph: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_offset(&mut self, cache_index: i32, size: Vector2i, glyph: i32, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Vector2);
            let args = (cache_index, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_offset(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_size(&mut self, cache_index: i32, size: Vector2i, glyph: i32, gl_size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Vector2);
            let args = (cache_index, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_size(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_uv_rect(&mut self, cache_index: i32, size: Vector2i, glyph: i32, uv_rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, Rect2);
            let args = (cache_index, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_uv_rect(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_texture_idx(&mut self, cache_index: i32, size: Vector2i, glyph: i32, texture_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32, i32);
            let args = (cache_index, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_texture_idx(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_kerning_list(&self, cache_index: i32, size: i32,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_kerning_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_kerning_map(&mut self, cache_index: i32, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_kerning_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Vector2i);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i, kerning: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, Vector2i, Vector2);
            let args = (cache_index, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_kerning(&self, cache_index: i32, size: i32, glyph_pair: Vector2i,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32, i32, Vector2i);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn render_range(&mut self, cache_index: i32, size: Vector2i, start: i64, end: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i64, i64);
            let args = (cache_index, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "render_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn render_glyph(&mut self, cache_index: i32, size: Vector2i, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "render_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language_support_override(&mut self, language: GString, supported: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (language, supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_support_override(&self, language: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_language_support_override(&mut self, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_support_overrides(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_script_support_override(&mut self, script: GString, supported: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (script, supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_support_override(&self, script: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_script_support_override(&mut self, script: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_support_overrides(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_opentype_feature_overrides(&mut self, overrides: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (overrides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_opentype_feature_overrides(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_index(&self, size: i32, char: i64, variation_selector: i64,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i64, i64);
            let args = (size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_char_from_glyph_index(&self, size: i32, glyph_index: i32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, i32, i32);
            let args = (size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_char_from_glyph_index", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FontFile {
        type Base = crate::engine::Font;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"FontFile\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FontFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for FontFile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Font > for FontFile {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for FontFile {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for FontFile {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for FontFile {
        
    }
    impl crate::obj::ExportableObject for FontFile {
        
    }
    impl crate::obj::cap::GodotDefault for FontFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FontFile {
        type Target = crate::engine::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FontFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_FontFile {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::FontFile > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Font > for $Class {
                
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