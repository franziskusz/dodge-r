#![doc = "Sidecar module for class [`TextServer`][crate::engine::TextServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServer` enums](https://docs.godotengine.org/en/stable/classes/class_textserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServer.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`text_server`][crate::engine::text_server]: sidecar module with related enum/flag types\n* [`ITextServer`][crate::engine::ITextServer]: virtual methods\n\n\nSee also [Godot docs for `TextServer`](https://docs.godotengine.org/en/stable/classes/class_textserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextServer`][crate::engine::TextServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextServer` methods](https://docs.godotengine.org/en/stable/classes/class_textserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TextServer {
        pub fn has_feature(&self, feature: crate::engine::text_server::Feature,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::text_server::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_features(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_support_data(&mut self, filename: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (filename,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_support_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_support_data_filename(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_support_data_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_support_data_info(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_support_data_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_support_data(&self, filename: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (filename,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_support_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_locale_right_to_left(&self, locale: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_locale_right_to_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn name_to_tag(&self, name: GString,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "name_to_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tag_to_name(&self, tag: i64,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i64);
            let args = (tag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "tag_to_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has(&mut self, rid: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_font(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_font_linked_variation(&mut self, font_rid: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_font_linked_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_data(&mut self, font_rid: Rid, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedByteArray);
            let args = (font_rid, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, face_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_face_index(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_face_count(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_face_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_style(&mut self, font_rid: Rid, style: crate::engine::text_server::FontStyle,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::FontStyle);
            let args = (font_rid, style,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_style(&self, font_rid: Rid,) -> crate::engine::text_server::FontStyle {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FontStyle >;
            type CallSig = (crate::engine::text_server::FontStyle, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_name(&mut self, font_rid: Rid, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (font_rid, name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_name(&self, font_rid: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_ot_name_strings(&self, font_rid: Rid,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_ot_name_strings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_style_name(&mut self, font_rid: Rid, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (font_rid, name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_style_name(&self, font_rid: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_weight(&mut self, font_rid: Rid, weight: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_weight(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_stretch(&mut self, font_rid: Rid, weight: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_stretch(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_antialiasing(&mut self, font_rid: Rid, antialiasing: crate::engine::text_server::FontAntialiasing,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::FontAntialiasing);
            let args = (font_rid, antialiasing,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_antialiasing(&self, font_rid: Rid,) -> crate::engine::text_server::FontAntialiasing {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FontAntialiasing >;
            type CallSig = (crate::engine::text_server::FontAntialiasing, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (font_rid, generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_generate_mipmaps(&self, font_rid: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (font_rid, msdf,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_is_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_msdf_pixel_range(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, msdf_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_msdf_size(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, fixed_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_fixed_size(&self, font_rid: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_fixed_size_scale_mode(&mut self, font_rid: Rid, fixed_size_scale_mode: crate::engine::text_server::FixedSizeScaleMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::FixedSizeScaleMode);
            let args = (font_rid, fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_fixed_size_scale_mode(&self, font_rid: Rid,) -> crate::engine::text_server::FixedSizeScaleMode {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::FixedSizeScaleMode >;
            type CallSig = (crate::engine::text_server::FixedSizeScaleMode, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (font_rid, allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_allow_system_fallback(&self, font_rid: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_is_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_force_autohinter(&self, font_rid: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_is_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_hinting(&mut self, font_rid: Rid, hinting: crate::engine::text_server::Hinting,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::Hinting);
            let args = (font_rid, hinting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_hinting(&self, font_rid: Rid,) -> crate::engine::text_server::Hinting {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Hinting >;
            type CallSig = (crate::engine::text_server::Hinting, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_subpixel_positioning(&mut self, font_rid: Rid, subpixel_positioning: crate::engine::text_server::SubpixelPositioning,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::SubpixelPositioning);
            let args = (font_rid, subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_subpixel_positioning(&self, font_rid: Rid,) -> crate::engine::text_server::SubpixelPositioning {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::SubpixelPositioning >;
            type CallSig = (crate::engine::text_server::SubpixelPositioning, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_embolden(&mut self, font_rid: Rid, strength: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64);
            let args = (font_rid, strength,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_embolden(&self, font_rid: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_spacing(&mut self, font_rid: Rid, spacing: crate::engine::text_server::SpacingType, value: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::SpacingType, i64);
            let args = (font_rid, spacing, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_spacing(&self, font_rid: Rid, spacing: crate::engine::text_server::SpacingType,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, crate::engine::text_server::SpacingType);
            let args = (font_rid, spacing,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (font_rid, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_transform(&self, font_rid: Rid,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_variation_coordinates(&mut self, font_rid: Rid, variation_coordinates: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Dictionary);
            let args = (font_rid, variation_coordinates,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_variation_coordinates(&self, font_rid: Rid,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64);
            let args = (font_rid, oversampling,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_oversampling(&self, font_rid: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_size_cache_list(&self, font_rid: Rid,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_size_cache_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_size_cache(&mut self, font_rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_clear_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, ascent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_ascent(&self, font_rid: Rid, size: i64,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, descent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_descent(&self, font_rid: Rid, size: i64,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_underline_position(&mut self, font_rid: Rid, size: i64, underline_position: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, underline_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_underline_position(&self, font_rid: Rid, size: i64,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_underline_thickness(&mut self, font_rid: Rid, size: i64, underline_thickness: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_underline_thickness(&self, font_rid: Rid, size: i64,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_scale(&self, font_rid: Rid, size: i64,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_texture_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_clear_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_texture_image(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, image: Gd < crate::engine::Image >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, Gd < crate::engine::Image >);
            let args = (font_rid, size, texture_index, image,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_image(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_texture_offsets(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, offset: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, PackedInt32Array);
            let args = (font_rid, size, texture_index, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_offsets(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_clear_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, i64, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_advance(&mut self, font_rid: Rid, size: i64, glyph: i64, advance: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, i64, Vector2);
            let args = (font_rid, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_offset(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, Vector2);
            let args = (font_rid, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_size(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, gl_size: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, Vector2);
            let args = (font_rid, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_uv_rect(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, uv_rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, Rect2);
            let args = (font_rid, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_texture_idx(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, texture_idx: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, i64);
            let args = (font_rid, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_texture_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_contours(&self, font: Rid, size: i64, index: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid, i64, i64);
            let args = (font, size, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_contours", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_kerning_list(&self, font_rid: Rid, size: i64,) -> Array < Vector2i > {
            type RetMarshal = PtrcallReturnT < Array < Vector2i > >;
            type CallSig = (Array < Vector2i >, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_kerning_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_clear_kerning_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, Vector2i);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i, kerning: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, Vector2i, Vector2);
            let args = (font_rid, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, i64, Vector2i);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_index(&self, font_rid: Rid, size: i64, char: i64, variation_selector: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64, i64, i64);
            let args = (font_rid, size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_char_from_glyph_index(&self, font_rid: Rid, size: i64, glyph_index: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64, i64);
            let args = (font_rid, size, glyph_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_char_from_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_has_char(&self, font_rid: Rid, char: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, i64);
            let args = (font_rid, char,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_has_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_supported_chars(&self, font_rid: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_supported_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64, i64);
            let args = (font_rid, size, start, end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_render_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_render_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn font_draw_glyph_full(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, i64, Vector2, i64, Color);
            let args = (font_rid, canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_draw_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn font_draw_glyph(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_ex(font_rid, canvas, size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_ex(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyph < '_ > {
            ExFontDrawGlyph::new(self, font_rid, canvas, size, pos, index,)
        }
        pub(crate) fn font_draw_glyph_outline_full(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, i64, i64, Vector2, i64, Color);
            let args = (font_rid, canvas, size, outline_size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_draw_glyph_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn font_draw_glyph_outline(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_outline_ex(font_rid, canvas, size, outline_size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_outline_ex(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyphOutline < '_ > {
            ExFontDrawGlyphOutline::new(self, font_rid, canvas, size, outline_size, pos, index,)
        }
        pub fn font_is_language_supported(&self, font_rid: Rid, language: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, GString);
            let args = (font_rid, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_is_language_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_language_support_override(&mut self, font_rid: Rid, language: GString, supported: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString, bool);
            let args = (font_rid, language, supported,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_language_support_override(&mut self, font_rid: Rid, language: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, GString);
            let args = (font_rid, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_language_support_override(&mut self, font_rid: Rid, language: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (font_rid, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_language_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_language_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_script_supported(&self, font_rid: Rid, script: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, GString);
            let args = (font_rid, script,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_is_script_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_script_support_override(&mut self, font_rid: Rid, script: GString, supported: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString, bool);
            let args = (font_rid, script, supported,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_script_support_override(&mut self, font_rid: Rid, script: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, GString);
            let args = (font_rid, script,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_script_support_override(&mut self, font_rid: Rid, script: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (font_rid, script,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_remove_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_script_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_script_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Dictionary);
            let args = (font_rid, overrides,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_opentype_feature_overrides(&self, font_rid: Rid,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_supported_feature_list(&self, font_rid: Rid,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_supported_feature_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_supported_variation_list(&self, font_rid: Rid,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_supported_variation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_global_oversampling(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_get_global_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_global_oversampling(&mut self, oversampling: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "font_set_global_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hex_code_box_size(&self, size: i64, index: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i64, i64);
            let args = (size, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hex_code_box_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_hex_code_box(&self, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, Vector2, i64, Color);
            let args = (canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "draw_hex_code_box", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_shaped_text_full(&mut self, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, crate::engine::text_server::Direction, crate::engine::text_server::Orientation);
            let args = (direction, orientation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_shaped_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_shaped_text(&mut self,) -> Rid {
            self.create_shaped_text_ex() . done()
        }
        #[inline]
        pub fn create_shaped_text_ex(&mut self,) -> ExCreateShapedText < '_ > {
            ExCreateShapedText::new(self,)
        }
        pub fn shaped_text_clear(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_set_direction_full(&mut self, shaped: Rid, direction: crate::engine::text_server::Direction,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::Direction);
            let args = (shaped, direction,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_set_direction(&mut self, shaped: Rid,) {
            self.shaped_text_set_direction_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_direction_ex(&mut self, shaped: Rid,) -> ExShapedTextSetDirection < '_ > {
            ExShapedTextSetDirection::new(self, shaped,)
        }
        pub fn shaped_text_get_direction(&self, shaped: Rid,) -> crate::engine::text_server::Direction {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Direction >;
            type CallSig = (crate::engine::text_server::Direction, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_inferred_direction(&self, shaped: Rid,) -> crate::engine::text_server::Direction {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Direction >;
            type CallSig = (crate::engine::text_server::Direction, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_inferred_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, VariantArray);
            let args = (shaped, override_,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (shaped, punct,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_custom_punctuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_custom_punctuation(&self, shaped: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_custom_punctuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_set_orientation_full(&mut self, shaped: Rid, orientation: crate::engine::text_server::Orientation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::Orientation);
            let args = (shaped, orientation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_set_orientation(&mut self, shaped: Rid,) {
            self.shaped_text_set_orientation_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_orientation_ex(&mut self, shaped: Rid,) -> ExShapedTextSetOrientation < '_ > {
            ExShapedTextSetOrientation::new(self, shaped,)
        }
        pub fn shaped_text_get_orientation(&self, shaped: Rid,) -> crate::engine::text_server::Orientation {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Orientation >;
            type CallSig = (crate::engine::text_server::Orientation, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_preserve_invalid(&self, shaped: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_preserve_control(&self, shaped: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_spacing(&mut self, shaped: Rid, spacing: crate::engine::text_server::SpacingType, value: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::text_server::SpacingType, i64);
            let args = (shaped, spacing, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_set_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_spacing(&self, shaped: Rid, spacing: crate::engine::text_server::SpacingType,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, crate::engine::text_server::SpacingType);
            let args = (shaped, spacing,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_add_string_full(&mut self, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64, opentype_features: Dictionary, language: GString, meta: Variant,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, GString, Array < Rid >, i64, Dictionary, GString, Variant);
            let args = (shaped, text, fonts, size, opentype_features, language, meta,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_add_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_add_string(&mut self, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64,) -> bool {
            self.shaped_text_add_string_ex(shaped, text, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_string_ex(&mut self, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64,) -> ExShapedTextAddString < '_ > {
            ExShapedTextAddString::new(self, shaped, text, fonts, size,)
        }
        pub(crate) fn shaped_text_add_object_full(&mut self, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, length: i64, baseline: f64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, Variant, Vector2, crate::engine::global::InlineAlignment, i64, f64);
            let args = (shaped, key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_add_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_add_object(&mut self, shaped: Rid, key: Variant, size: Vector2,) -> bool {
            self.shaped_text_add_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_object_ex(&mut self, shaped: Rid, key: Variant, size: Vector2,) -> ExShapedTextAddObject < '_ > {
            ExShapedTextAddObject::new(self, shaped, key, size,)
        }
        pub(crate) fn shaped_text_resize_object_full(&mut self, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, baseline: f64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid, Variant, Vector2, crate::engine::global::InlineAlignment, f64);
            let args = (shaped, key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_resize_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_resize_object(&mut self, shaped: Rid, key: Variant, size: Vector2,) -> bool {
            self.shaped_text_resize_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_resize_object_ex(&mut self, shaped: Rid, key: Variant, size: Vector2,) -> ExShapedTextResizeObject < '_ > {
            ExShapedTextResizeObject::new(self, shaped, key, size,)
        }
        pub fn shaped_get_span_count(&self, shaped: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_get_span_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_get_span_meta(&self, shaped: Rid, index: i64,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, i64);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_get_span_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_set_span_update_font_full(&mut self, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64, opentype_features: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i64, Array < Rid >, i64, Dictionary);
            let args = (shaped, index, fonts, size, opentype_features,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_set_span_update_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_set_span_update_font(&mut self, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64,) {
            self.shaped_set_span_update_font_ex(shaped, index, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_set_span_update_font_ex(&mut self, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64,) -> ExShapedSetSpanUpdateFont < '_ > {
            ExShapedSetSpanUpdateFont::new(self, shaped, index, fonts, size,)
        }
        pub fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i64, i64);
            let args = (shaped, start, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_substr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_parent(&self, shaped: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_fit_to_width_full(&mut self, shaped: Rid, width: f64, justification_flags: crate::engine::text_server::JustificationFlag,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, f64, crate::engine::text_server::JustificationFlag);
            let args = (shaped, width, justification_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_fit_to_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_fit_to_width(&mut self, shaped: Rid, width: f64,) -> f64 {
            self.shaped_text_fit_to_width_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_fit_to_width_ex(&mut self, shaped: Rid, width: f64,) -> ExShapedTextFitToWidth < '_ > {
            ExShapedTextFitToWidth::new(self, shaped, width,)
        }
        pub fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: PackedFloat32Array,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid, PackedFloat32Array);
            let args = (shaped, tab_stops,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_tab_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_shape(&mut self, shaped: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_is_ready(&self, shaped: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_is_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_has_visible_chars(&self, shaped: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_has_visible_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_glyphs(&self, shaped: Rid,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_sort_logical(&mut self, shaped: Rid,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_sort_logical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_glyph_count(&self, shaped: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_range(&self, shaped: Rid,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_get_line_breaks_adv_full(&self, shaped: Rid, width: PackedFloat32Array, start: i64, once: bool, break_flags: crate::engine::text_server::LineBreakFlag,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid, PackedFloat32Array, i64, bool, crate::engine::text_server::LineBreakFlag);
            let args = (shaped, width, start, once, break_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_line_breaks_adv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_adv(&self, shaped: Rid, width: PackedFloat32Array,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_adv_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_adv_ex(&self, shaped: Rid, width: PackedFloat32Array,) -> ExShapedTextGetLineBreaksAdv < '_ > {
            ExShapedTextGetLineBreaksAdv::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_line_breaks_full(&self, shaped: Rid, width: f64, start: i64, break_flags: crate::engine::text_server::LineBreakFlag,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid, f64, i64, crate::engine::text_server::LineBreakFlag);
            let args = (shaped, width, start, break_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_line_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_get_line_breaks(&self, shaped: Rid, width: f64,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_ex(&self, shaped: Rid, width: f64,) -> ExShapedTextGetLineBreaks < '_ > {
            ExShapedTextGetLineBreaks::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_word_breaks_full(&self, shaped: Rid, grapheme_flags: crate::engine::text_server::GraphemeFlag,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid, crate::engine::text_server::GraphemeFlag);
            let args = (shaped, grapheme_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_word_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_get_word_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            self.shaped_text_get_word_breaks_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_get_word_breaks_ex(&self, shaped: Rid,) -> ExShapedTextGetWordBreaks < '_ > {
            ExShapedTextGetWordBreaks::new(self, shaped,)
        }
        pub fn shaped_text_get_trim_pos(&self, shaped: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_trim_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_pos(&self, shaped: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_ellipsis_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyphs(&self, shaped: Rid,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_ellipsis_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_ellipsis_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_overrun_trim_to_width_full(&mut self, shaped: Rid, width: f64, overrun_trim_flags: crate::engine::text_server::TextOverrunFlag,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64, crate::engine::text_server::TextOverrunFlag);
            let args = (shaped, width, overrun_trim_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_overrun_trim_to_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_overrun_trim_to_width(&mut self, shaped: Rid,) {
            self.shaped_text_overrun_trim_to_width_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_overrun_trim_to_width_ex(&mut self, shaped: Rid,) -> ExShapedTextOverrunTrimToWidth < '_ > {
            ExShapedTextOverrunTrimToWidth::new(self, shaped,)
        }
        pub fn shaped_text_get_objects(&self, shaped: Rid,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_object_rect(&self, shaped: Rid, key: Variant,) -> Rect2 {
            type RetMarshal = PtrcallReturnT < Rect2 >;
            type CallSig = (Rect2, Rid, Variant);
            let args = (shaped, key,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_object_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_size(&self, shaped: Rid,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ascent(&self, shaped: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_descent(&self, shaped: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_width(&self, shaped: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_underline_position(&self, shaped: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_underline_thickness(&self, shaped: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_carets(&self, shaped: Rid, position: i64,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid, i64);
            let args = (shaped, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_selection(&self, shaped: Rid, start: i64, end: i64,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Rid, i64, i64);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coords: f64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, f64);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_hit_test_grapheme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_hit_test_position(&self, shaped: Rid, coords: f64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, f64);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_hit_test_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_grapheme_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_next_grapheme_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_prev_grapheme_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_character_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_character_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_next_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_next_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_prev_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_prev_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_closest_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_closest_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_draw_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Vector2, f64, f64, Color);
            let args = (shaped, canvas, pos, clip_l, clip_r, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_draw(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_ex(&self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDraw < '_ > {
            ExShapedTextDraw::new(self, shaped, canvas, pos,)
        }
        pub(crate) fn shaped_text_draw_outline_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Vector2, f64, f64, i64, Color);
            let args = (shaped, canvas, pos, clip_l, clip_r, outline_size, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_draw_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shaped_text_draw_outline(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_outline_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_outline_ex(&self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDrawOutline < '_ > {
            ExShapedTextDrawOutline::new(self, shaped, canvas, pos,)
        }
        pub fn shaped_text_get_dominant_direction_in_range(&self, shaped: Rid, start: i64, end: i64,) -> crate::engine::text_server::Direction {
            type RetMarshal = PtrcallReturnT < crate::engine::text_server::Direction >;
            type CallSig = (crate::engine::text_server::Direction, Rid, i64, i64);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shaped_text_get_dominant_direction_in_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn format_number_full(&self, number: GString, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, GString);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "format_number", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn format_number(&self, number: GString,) -> GString {
            self.format_number_ex(number,) . done()
        }
        #[inline]
        pub fn format_number_ex(&self, number: GString,) -> ExFormatNumber < '_ > {
            ExFormatNumber::new(self, number,)
        }
        pub(crate) fn parse_number_full(&self, number: GString, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, GString);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_number", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn parse_number(&self, number: GString,) -> GString {
            self.parse_number_ex(number,) . done()
        }
        #[inline]
        pub fn parse_number_ex(&self, number: GString,) -> ExParseNumber < '_ > {
            ExParseNumber::new(self, number,)
        }
        pub(crate) fn percent_sign_full(&self, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "percent_sign", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn percent_sign(&self,) -> GString {
            self.percent_sign_ex() . done()
        }
        #[inline]
        pub fn percent_sign_ex(&self,) -> ExPercentSign < '_ > {
            ExPercentSign::new(self,)
        }
        pub(crate) fn string_get_word_breaks_full(&self, string: GString, language: GString, chars_per_line: i64,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, GString, GString, i64);
            let args = (string, language, chars_per_line,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "string_get_word_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn string_get_word_breaks(&self, string: GString,) -> PackedInt32Array {
            self.string_get_word_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_word_breaks_ex(&self, string: GString,) -> ExStringGetWordBreaks < '_ > {
            ExStringGetWordBreaks::new(self, string,)
        }
        pub(crate) fn string_get_character_breaks_full(&self, string: GString, language: GString,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, GString, GString);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "string_get_character_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn string_get_character_breaks(&self, string: GString,) -> PackedInt32Array {
            self.string_get_character_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_character_breaks_ex(&self, string: GString,) -> ExStringGetCharacterBreaks < '_ > {
            ExStringGetCharacterBreaks::new(self, string,)
        }
        pub fn is_confusable(&self, string: GString, dict: PackedStringArray,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, GString, PackedStringArray);
            let args = (string, dict,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_confusable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn spoof_check(&self, string: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "spoof_check", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn strip_diacritics(&self, string: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "strip_diacritics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid_identifier(&self, string: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_valid_identifier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn string_to_upper_full(&self, string: GString, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, GString);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "string_to_upper", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn string_to_upper(&self, string: GString,) -> GString {
            self.string_to_upper_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_upper_ex(&self, string: GString,) -> ExStringToUpper < '_ > {
            ExStringToUpper::new(self, string,)
        }
        pub(crate) fn string_to_lower_full(&self, string: GString, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString, GString);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "string_to_lower", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn string_to_lower(&self, string: GString,) -> GString {
            self.string_to_lower_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_lower_ex(&self, string: GString,) -> ExStringToLower < '_ > {
            ExStringToLower::new(self, string,)
        }
        pub fn parse_structured_text(&self, parser_type: crate::engine::text_server::StructuredTextParser, args: VariantArray, text: GString,) -> Array < Vector3i > {
            type RetMarshal = PtrcallReturnT < Array < Vector3i > >;
            type CallSig = (Array < Vector3i >, crate::engine::text_server::StructuredTextParser, VariantArray, GString);
            let args = (parser_type, args, text,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_structured_text", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextServer {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TextServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for TextServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TextServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TextServer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TextServer {
        
    }
    impl std::ops::Deref for TextServer {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TextServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TextServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_ex`][super::TextServer::font_draw_glyph_ex]."]
#[must_use]
pub struct ExFontDrawGlyph < 'a > {
    surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyph < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> Self {
        Self {
            surround_object, font_rid, canvas, size, pos, index, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
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
        re_export::TextServer::font_draw_glyph_full(self.surround_object, self.font_rid, self.canvas, self.size, self.pos, self.index, self.color,)
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_outline_ex`][super::TextServer::font_draw_glyph_outline_ex]."]
#[must_use]
pub struct ExFontDrawGlyphOutline < 'a > {
    surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyphOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> Self {
        Self {
            surround_object, font_rid, canvas, size, outline_size, pos, index, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
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
        re_export::TextServer::font_draw_glyph_outline_full(self.surround_object, self.font_rid, self.canvas, self.size, self.outline_size, self.pos, self.index, self.color,)
    }
}
#[doc = "Default-param extender for [`TextServer::create_shaped_text_ex`][super::TextServer::create_shaped_text_ex]."]
#[must_use]
pub struct ExCreateShapedText < 'a > {
    surround_object: &'a mut re_export::TextServer, direction: crate::engine::text_server::Direction, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateShapedText < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer,) -> Self {
        Self {
            surround_object, direction: crate::obj::EngineEnum::from_ord(0), orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::TextServer::create_shaped_text_full(self.surround_object, self.direction, self.orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_direction_ex`][super::TextServer::shaped_text_set_direction_ex]."]
#[must_use]
pub struct ExShapedTextSetDirection < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, direction: crate::engine::text_server::Direction,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetDirection < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        Self {
            surround_object, shaped, direction: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn direction(self, value: crate::engine::text_server::Direction) -> Self {
        Self {
            direction: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextServer::shaped_text_set_direction_full(self.surround_object, self.shaped, self.direction,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_orientation_ex`][super::TextServer::shaped_text_set_orientation_ex]."]
#[must_use]
pub struct ExShapedTextSetOrientation < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, orientation: crate::engine::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        Self {
            surround_object, shaped, orientation: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn orientation(self, value: crate::engine::text_server::Orientation) -> Self {
        Self {
            orientation: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextServer::shaped_text_set_orientation_full(self.surround_object, self.shaped, self.orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_string_ex`][super::TextServer::shaped_text_add_string_ex]."]
#[must_use]
pub struct ExShapedTextAddString < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64, opentype_features: Dictionary, language: GString, meta: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64,) -> Self {
        Self {
            surround_object, shaped, text, fonts, size, opentype_features: Dictionary::new(), language: GString::from(""), meta: Variant::nil(),
        }
    }
    #[inline]
    pub fn opentype_features(self, value: Dictionary) -> Self {
        Self {
            opentype_features: value, .. self
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn meta(self, value: Variant) -> Self {
        Self {
            meta: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextServer::shaped_text_add_string_full(self.surround_object, self.shaped, self.text, self.fonts, self.size, self.opentype_features, self.language, self.meta,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_object_ex`][super::TextServer::shaped_text_add_object_ex]."]
#[must_use]
pub struct ExShapedTextAddObject < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, length: i64, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: Variant, size: Vector2,) -> Self {
        Self {
            surround_object, shaped, key, size, inline_align: crate::obj::EngineEnum::from_ord(5), length: 1i64, baseline: 0f64,
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn length(self, value: i64) -> Self {
        Self {
            length: value, .. self
        }
    }
    #[inline]
    pub fn baseline(self, value: f64) -> Self {
        Self {
            baseline: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextServer::shaped_text_add_object_full(self.surround_object, self.shaped, self.key, self.size, self.inline_align, self.length, self.baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_resize_object_ex`][super::TextServer::shaped_text_resize_object_ex]."]
#[must_use]
pub struct ExShapedTextResizeObject < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::engine::global::InlineAlignment, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: Variant, size: Vector2,) -> Self {
        Self {
            surround_object, shaped, key, size, inline_align: crate::obj::EngineEnum::from_ord(5), baseline: 0f64,
        }
    }
    #[inline]
    pub fn inline_align(self, value: crate::engine::global::InlineAlignment) -> Self {
        Self {
            inline_align: value, .. self
        }
    }
    #[inline]
    pub fn baseline(self, value: f64) -> Self {
        Self {
            baseline: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::TextServer::shaped_text_resize_object_full(self.surround_object, self.shaped, self.key, self.size, self.inline_align, self.baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_set_span_update_font_ex`][super::TextServer::shaped_set_span_update_font_ex]."]
#[must_use]
pub struct ExShapedSetSpanUpdateFont < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64, opentype_features: Dictionary,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedSetSpanUpdateFont < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64,) -> Self {
        Self {
            surround_object, shaped, index, fonts, size, opentype_features: Dictionary::new(),
        }
    }
    #[inline]
    pub fn opentype_features(self, value: Dictionary) -> Self {
        Self {
            opentype_features: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextServer::shaped_set_span_update_font_full(self.surround_object, self.shaped, self.index, self.fonts, self.size, self.opentype_features,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_fit_to_width_ex`][super::TextServer::shaped_text_fit_to_width_ex]."]
#[must_use]
pub struct ExShapedTextFitToWidth < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, justification_flags: crate::engine::text_server::JustificationFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextFitToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        Self {
            surround_object, shaped, width, justification_flags: crate::obj::EngineBitfield::from_ord(3),
        }
    }
    #[inline]
    pub fn justification_flags(self, value: crate::engine::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        re_export::TextServer::shaped_text_fit_to_width_full(self.surround_object, self.shaped, self.width, self.justification_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_adv_ex`][super::TextServer::shaped_text_get_line_breaks_adv_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaksAdv < 'a > {
    surround_object: &'a re_export::TextServer, shaped: Rid, width: PackedFloat32Array, start: i64, once: bool, break_flags: crate::engine::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaksAdv < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: PackedFloat32Array,) -> Self {
        Self {
            surround_object, shaped, width, start: 0i64, once: true, break_flags: crate::obj::EngineBitfield::from_ord(3),
        }
    }
    #[inline]
    pub fn start(self, value: i64) -> Self {
        Self {
            start: value, .. self
        }
    }
    #[inline]
    pub fn once(self, value: bool) -> Self {
        Self {
            once: value, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::TextServer::shaped_text_get_line_breaks_adv_full(self.surround_object, self.shaped, self.width, self.start, self.once, self.break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_ex`][super::TextServer::shaped_text_get_line_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaks < 'a > {
    surround_object: &'a re_export::TextServer, shaped: Rid, width: f64, start: i64, break_flags: crate::engine::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        Self {
            surround_object, shaped, width, start: 0i64, break_flags: crate::obj::EngineBitfield::from_ord(3),
        }
    }
    #[inline]
    pub fn start(self, value: i64) -> Self {
        Self {
            start: value, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, value: crate::engine::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::TextServer::shaped_text_get_line_breaks_full(self.surround_object, self.shaped, self.width, self.start, self.break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_word_breaks_ex`][super::TextServer::shaped_text_get_word_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetWordBreaks < 'a > {
    surround_object: &'a re_export::TextServer, shaped: Rid, grapheme_flags: crate::engine::text_server::GraphemeFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid,) -> Self {
        Self {
            surround_object, shaped, grapheme_flags: crate::obj::EngineBitfield::from_ord(264),
        }
    }
    #[inline]
    pub fn grapheme_flags(self, value: crate::engine::text_server::GraphemeFlag) -> Self {
        Self {
            grapheme_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::TextServer::shaped_text_get_word_breaks_full(self.surround_object, self.shaped, self.grapheme_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_overrun_trim_to_width_ex`][super::TextServer::shaped_text_overrun_trim_to_width_ex]."]
#[must_use]
pub struct ExShapedTextOverrunTrimToWidth < 'a > {
    surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, overrun_trim_flags: crate::engine::text_server::TextOverrunFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextOverrunTrimToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        Self {
            surround_object, shaped, width: 0f64, overrun_trim_flags: crate::obj::EngineBitfield::from_ord(0),
        }
    }
    #[inline]
    pub fn width(self, value: f64) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn overrun_trim_flags(self, value: crate::engine::text_server::TextOverrunFlag) -> Self {
        Self {
            overrun_trim_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::TextServer::shaped_text_overrun_trim_to_width_full(self.surround_object, self.shaped, self.width, self.overrun_trim_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_ex`][super::TextServer::shaped_text_draw_ex]."]
#[must_use]
pub struct ExShapedTextDraw < 'a > {
    surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDraw < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        Self {
            surround_object, shaped, canvas, pos, clip_l: - 1f64, clip_r: - 1f64, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn clip_l(self, value: f64) -> Self {
        Self {
            clip_l: value, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, value: f64) -> Self {
        Self {
            clip_r: value, .. self
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
        re_export::TextServer::shaped_text_draw_full(self.surround_object, self.shaped, self.canvas, self.pos, self.clip_l, self.clip_r, self.color,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_outline_ex`][super::TextServer::shaped_text_draw_outline_ex]."]
#[must_use]
pub struct ExShapedTextDrawOutline < 'a > {
    surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        Self {
            surround_object, shaped, canvas, pos, clip_l: - 1f64, clip_r: - 1f64, outline_size: 1i64, color: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn clip_l(self, value: f64) -> Self {
        Self {
            clip_l: value, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, value: f64) -> Self {
        Self {
            clip_r: value, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, value: i64) -> Self {
        Self {
            outline_size: value, .. self
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
        re_export::TextServer::shaped_text_draw_outline_full(self.surround_object, self.shaped, self.canvas, self.pos, self.clip_l, self.clip_r, self.outline_size, self.color,)
    }
}
#[doc = "Default-param extender for [`TextServer::format_number_ex`][super::TextServer::format_number_ex]."]
#[must_use]
pub struct ExFormatNumber < 'a > {
    surround_object: &'a re_export::TextServer, number: GString, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFormatNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: GString,) -> Self {
        Self {
            surround_object, number, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextServer::format_number_full(self.surround_object, self.number, self.language,)
    }
}
#[doc = "Default-param extender for [`TextServer::parse_number_ex`][super::TextServer::parse_number_ex]."]
#[must_use]
pub struct ExParseNumber < 'a > {
    surround_object: &'a re_export::TextServer, number: GString, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExParseNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: GString,) -> Self {
        Self {
            surround_object, number, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextServer::parse_number_full(self.surround_object, self.number, self.language,)
    }
}
#[doc = "Default-param extender for [`TextServer::percent_sign_ex`][super::TextServer::percent_sign_ex]."]
#[must_use]
pub struct ExPercentSign < 'a > {
    surround_object: &'a re_export::TextServer, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPercentSign < 'a > {
    fn new(surround_object: &'a re_export::TextServer,) -> Self {
        Self {
            surround_object, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextServer::percent_sign_full(self.surround_object, self.language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_word_breaks_ex`][super::TextServer::string_get_word_breaks_ex]."]
#[must_use]
pub struct ExStringGetWordBreaks < 'a > {
    surround_object: &'a re_export::TextServer, string: GString, language: GString, chars_per_line: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: GString,) -> Self {
        Self {
            surround_object, string, language: GString::from(""), chars_per_line: 0i64,
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn chars_per_line(self, value: i64) -> Self {
        Self {
            chars_per_line: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::TextServer::string_get_word_breaks_full(self.surround_object, self.string, self.language, self.chars_per_line,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_character_breaks_ex`][super::TextServer::string_get_character_breaks_ex]."]
#[must_use]
pub struct ExStringGetCharacterBreaks < 'a > {
    surround_object: &'a re_export::TextServer, string: GString, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetCharacterBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: GString,) -> Self {
        Self {
            surround_object, string, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::TextServer::string_get_character_breaks_full(self.surround_object, self.string, self.language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_upper_ex`][super::TextServer::string_to_upper_ex]."]
#[must_use]
pub struct ExStringToUpper < 'a > {
    surround_object: &'a re_export::TextServer, string: GString, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToUpper < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: GString,) -> Self {
        Self {
            surround_object, string, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextServer::string_to_upper_full(self.surround_object, self.string, self.language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_lower_ex`][super::TextServer::string_to_lower_ex]."]
#[must_use]
pub struct ExStringToLower < 'a > {
    surround_object: &'a re_export::TextServer, string: GString, language: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToLower < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: GString,) -> Self {
        Self {
            surround_object, string, language: GString::from(""),
        }
    }
    #[inline]
    pub fn language(self, value: GString) -> Self {
        Self {
            language: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::TextServer::string_to_lower_full(self.surround_object, self.string, self.language,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FontAntialiasing {
    ord: i32
}
impl FontAntialiasing {
    pub const FONT_ANTIALIASING_NONE: Self = Self {
        ord: 0i32
    };
    pub const FONT_ANTIALIASING_GRAY: Self = Self {
        ord: 1i32
    };
    pub const FONT_ANTIALIASING_LCD: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for FontAntialiasing {
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
impl crate::builtin::meta::GodotConvert for FontAntialiasing {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FontAntialiasing {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FontAntialiasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FontLCDSubpixelLayout {
    ord: i32
}
impl FontLCDSubpixelLayout {
    pub const FONT_LCD_SUBPIXEL_LAYOUT_NONE: Self = Self {
        ord: 0i32
    };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_HRGB: Self = Self {
        ord: 1i32
    };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_HBGR: Self = Self {
        ord: 2i32
    };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_VRGB: Self = Self {
        ord: 3i32
    };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_VBGR: Self = Self {
        ord: 4i32
    };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for FontLCDSubpixelLayout {
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
impl crate::obj::IndexEnum for FontLCDSubpixelLayout {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for FontLCDSubpixelLayout {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FontLCDSubpixelLayout {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FontLCDSubpixelLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Direction {
    ord: i32
}
impl Direction {
    pub const DIRECTION_AUTO: Self = Self {
        ord: 0i32
    };
    pub const DIRECTION_LTR: Self = Self {
        ord: 1i32
    };
    pub const DIRECTION_RTL: Self = Self {
        ord: 2i32
    };
    pub const DIRECTION_INHERITED: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for Direction {
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
impl crate::builtin::meta::GodotConvert for Direction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Direction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Direction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Orientation {
    ord: i32
}
impl Orientation {
    pub const ORIENTATION_HORIZONTAL: Self = Self {
        ord: 0i32
    };
    pub const ORIENTATION_VERTICAL: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for Orientation {
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
impl crate::builtin::meta::GodotConvert for Orientation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Orientation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Orientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct JustificationFlag {
    ord: u64
}
impl JustificationFlag {
    pub const JUSTIFICATION_NONE: Self = Self {
        ord: 0u64
    };
    pub const JUSTIFICATION_KASHIDA: Self = Self {
        ord: 1u64
    };
    pub const JUSTIFICATION_WORD_BOUND: Self = Self {
        ord: 2u64
    };
    pub const JUSTIFICATION_TRIM_EDGE_SPACES: Self = Self {
        ord: 4u64
    };
    pub const JUSTIFICATION_AFTER_LAST_TAB: Self = Self {
        ord: 8u64
    };
    pub const JUSTIFICATION_CONSTRAIN_ELLIPSIS: Self = Self {
        ord: 16u64
    };
    pub const JUSTIFICATION_SKIP_LAST_LINE: Self = Self {
        ord: 32u64
    };
    pub const JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS: Self = Self {
        ord: 64u64
    };
    pub const JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE: Self = Self {
        ord: 128u64
    };
    
}
impl crate::obj::EngineBitfield for JustificationFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for JustificationFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for JustificationFlag {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for JustificationFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for JustificationFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AutowrapMode {
    ord: i32
}
impl AutowrapMode {
    pub const AUTOWRAP_OFF: Self = Self {
        ord: 0i32
    };
    pub const AUTOWRAP_ARBITRARY: Self = Self {
        ord: 1i32
    };
    pub const AUTOWRAP_WORD: Self = Self {
        ord: 2i32
    };
    pub const AUTOWRAP_WORD_SMART: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AutowrapMode {
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
impl crate::builtin::meta::GodotConvert for AutowrapMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AutowrapMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AutowrapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct LineBreakFlag {
    ord: u64
}
impl LineBreakFlag {
    pub const BREAK_NONE: Self = Self {
        ord: 0u64
    };
    pub const BREAK_MANDATORY: Self = Self {
        ord: 1u64
    };
    pub const BREAK_WORD_BOUND: Self = Self {
        ord: 2u64
    };
    pub const BREAK_GRAPHEME_BOUND: Self = Self {
        ord: 4u64
    };
    pub const BREAK_ADAPTIVE: Self = Self {
        ord: 8u64
    };
    pub const BREAK_TRIM_EDGE_SPACES: Self = Self {
        ord: 16u64
    };
    
}
impl crate::obj::EngineBitfield for LineBreakFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for LineBreakFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for LineBreakFlag {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for LineBreakFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LineBreakFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VisibleCharactersBehavior {
    ord: i32
}
impl VisibleCharactersBehavior {
    pub const VC_CHARS_BEFORE_SHAPING: Self = Self {
        ord: 0i32
    };
    pub const VC_CHARS_AFTER_SHAPING: Self = Self {
        ord: 1i32
    };
    pub const VC_GLYPHS_AUTO: Self = Self {
        ord: 2i32
    };
    pub const VC_GLYPHS_LTR: Self = Self {
        ord: 3i32
    };
    pub const VC_GLYPHS_RTL: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for VisibleCharactersBehavior {
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
impl crate::builtin::meta::GodotConvert for VisibleCharactersBehavior {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibleCharactersBehavior {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibleCharactersBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct OverrunBehavior {
    ord: i32
}
impl OverrunBehavior {
    pub const OVERRUN_NO_TRIMMING: Self = Self {
        ord: 0i32
    };
    pub const OVERRUN_TRIM_CHAR: Self = Self {
        ord: 1i32
    };
    pub const OVERRUN_TRIM_WORD: Self = Self {
        ord: 2i32
    };
    pub const OVERRUN_TRIM_ELLIPSIS: Self = Self {
        ord: 3i32
    };
    pub const OVERRUN_TRIM_WORD_ELLIPSIS: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for OverrunBehavior {
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
impl crate::builtin::meta::GodotConvert for OverrunBehavior {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for OverrunBehavior {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for OverrunBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TextOverrunFlag {
    ord: u64
}
impl TextOverrunFlag {
    pub const OVERRUN_NO_TRIM: Self = Self {
        ord: 0u64
    };
    pub const OVERRUN_TRIM: Self = Self {
        ord: 1u64
    };
    pub const OVERRUN_TRIM_WORD_ONLY: Self = Self {
        ord: 2u64
    };
    pub const OVERRUN_ADD_ELLIPSIS: Self = Self {
        ord: 4u64
    };
    pub const OVERRUN_ENFORCE_ELLIPSIS: Self = Self {
        ord: 8u64
    };
    pub const OVERRUN_JUSTIFICATION_AWARE: Self = Self {
        ord: 16u64
    };
    
}
impl crate::obj::EngineBitfield for TextOverrunFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for TextOverrunFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for TextOverrunFlag {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for TextOverrunFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextOverrunFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct GraphemeFlag {
    ord: u64
}
impl GraphemeFlag {
    pub const GRAPHEME_IS_VALID: Self = Self {
        ord: 1u64
    };
    pub const GRAPHEME_IS_RTL: Self = Self {
        ord: 2u64
    };
    pub const GRAPHEME_IS_VIRTUAL: Self = Self {
        ord: 4u64
    };
    pub const GRAPHEME_IS_SPACE: Self = Self {
        ord: 8u64
    };
    pub const GRAPHEME_IS_BREAK_HARD: Self = Self {
        ord: 16u64
    };
    pub const GRAPHEME_IS_BREAK_SOFT: Self = Self {
        ord: 32u64
    };
    pub const GRAPHEME_IS_TAB: Self = Self {
        ord: 64u64
    };
    pub const GRAPHEME_IS_ELONGATION: Self = Self {
        ord: 128u64
    };
    pub const GRAPHEME_IS_PUNCTUATION: Self = Self {
        ord: 256u64
    };
    pub const GRAPHEME_IS_UNDERSCORE: Self = Self {
        ord: 512u64
    };
    pub const GRAPHEME_IS_CONNECTED: Self = Self {
        ord: 1024u64
    };
    pub const GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL: Self = Self {
        ord: 2048u64
    };
    pub const GRAPHEME_IS_EMBEDDED_OBJECT: Self = Self {
        ord: 4096u64
    };
    
}
impl crate::obj::EngineBitfield for GraphemeFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for GraphemeFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for GraphemeFlag {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for GraphemeFlag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GraphemeFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Hinting {
    ord: i32
}
impl Hinting {
    pub const HINTING_NONE: Self = Self {
        ord: 0i32
    };
    pub const HINTING_LIGHT: Self = Self {
        ord: 1i32
    };
    pub const HINTING_NORMAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Hinting {
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
impl crate::builtin::meta::GodotConvert for Hinting {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Hinting {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Hinting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SubpixelPositioning {
    ord: i32
}
impl SubpixelPositioning {
    pub const SUBPIXEL_POSITIONING_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SUBPIXEL_POSITIONING_AUTO: Self = Self {
        ord: 1i32
    };
    pub const SUBPIXEL_POSITIONING_ONE_HALF: Self = Self {
        ord: 2i32
    };
    pub const SUBPIXEL_POSITIONING_ONE_QUARTER: Self = Self {
        ord: 3i32
    };
    pub const SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE: Self = Self {
        ord: 20i32
    };
    pub const SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE: Self = Self {
        ord: 16i32
    };
    
}
impl crate::obj::EngineEnum for SubpixelPositioning {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 16i32 | ord @ 20i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SubpixelPositioning {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SubpixelPositioning {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SubpixelPositioning {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Feature {
    ord: i32
}
impl Feature {
    pub const FEATURE_SIMPLE_LAYOUT: Self = Self {
        ord: 1i32
    };
    pub const FEATURE_BIDI_LAYOUT: Self = Self {
        ord: 2i32
    };
    pub const FEATURE_VERTICAL_LAYOUT: Self = Self {
        ord: 4i32
    };
    pub const FEATURE_SHAPING: Self = Self {
        ord: 8i32
    };
    pub const FEATURE_KASHIDA_JUSTIFICATION: Self = Self {
        ord: 16i32
    };
    pub const FEATURE_BREAK_ITERATORS: Self = Self {
        ord: 32i32
    };
    pub const FEATURE_FONT_BITMAP: Self = Self {
        ord: 64i32
    };
    pub const FEATURE_FONT_DYNAMIC: Self = Self {
        ord: 128i32
    };
    pub const FEATURE_FONT_MSDF: Self = Self {
        ord: 256i32
    };
    pub const FEATURE_FONT_SYSTEM: Self = Self {
        ord: 512i32
    };
    pub const FEATURE_FONT_VARIABLE: Self = Self {
        ord: 1024i32
    };
    pub const FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION: Self = Self {
        ord: 2048i32
    };
    pub const FEATURE_USE_SUPPORT_DATA: Self = Self {
        ord: 4096i32
    };
    pub const FEATURE_UNICODE_IDENTIFIERS: Self = Self {
        ord: 8192i32
    };
    pub const FEATURE_UNICODE_SECURITY: Self = Self {
        ord: 16384i32
    };
    
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 128i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 | ord @ 2048i32 | ord @ 4096i32 | ord @ 8192i32 | ord @ 16384i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Feature {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContourPointTag {
    ord: i32
}
impl ContourPointTag {
    pub const CONTOUR_CURVE_TAG_ON: Self = Self {
        ord: 1i32
    };
    pub const CONTOUR_CURVE_TAG_OFF_CONIC: Self = Self {
        ord: 0i32
    };
    pub const CONTOUR_CURVE_TAG_OFF_CUBIC: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ContourPointTag {
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
impl crate::builtin::meta::GodotConvert for ContourPointTag {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ContourPointTag {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ContourPointTag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpacingType {
    ord: i32
}
impl SpacingType {
    pub const SPACING_GLYPH: Self = Self {
        ord: 0i32
    };
    pub const SPACING_SPACE: Self = Self {
        ord: 1i32
    };
    pub const SPACING_TOP: Self = Self {
        ord: 2i32
    };
    pub const SPACING_BOTTOM: Self = Self {
        ord: 3i32
    };
    pub const SPACING_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SpacingType {
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
impl crate::obj::IndexEnum for SpacingType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for SpacingType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpacingType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpacingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FontStyle {
    ord: u64
}
impl FontStyle {
    pub const FONT_BOLD: Self = Self {
        ord: 1u64
    };
    pub const FONT_ITALIC: Self = Self {
        ord: 2u64
    };
    pub const FONT_FIXED_WIDTH: Self = Self {
        ord: 4u64
    };
    
}
impl crate::obj::EngineBitfield for FontStyle {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for FontStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for FontStyle {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for FontStyle {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FontStyle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StructuredTextParser {
    ord: i32
}
impl StructuredTextParser {
    pub const STRUCTURED_TEXT_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const STRUCTURED_TEXT_URI: Self = Self {
        ord: 1i32
    };
    pub const STRUCTURED_TEXT_FILE: Self = Self {
        ord: 2i32
    };
    pub const STRUCTURED_TEXT_EMAIL: Self = Self {
        ord: 3i32
    };
    pub const STRUCTURED_TEXT_LIST: Self = Self {
        ord: 4i32
    };
    pub const STRUCTURED_TEXT_GDSCRIPT: Self = Self {
        ord: 5i32
    };
    pub const STRUCTURED_TEXT_CUSTOM: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for StructuredTextParser {
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
impl crate::builtin::meta::GodotConvert for StructuredTextParser {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for StructuredTextParser {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for StructuredTextParser {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FixedSizeScaleMode {
    ord: i32
}
impl FixedSizeScaleMode {
    pub const FIXED_SIZE_SCALE_DISABLE: Self = Self {
        ord: 0i32
    };
    pub const FIXED_SIZE_SCALE_INTEGER_ONLY: Self = Self {
        ord: 1i32
    };
    pub const FIXED_SIZE_SCALE_ENABLED: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for FixedSizeScaleMode {
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
impl crate::builtin::meta::GodotConvert for FixedSizeScaleMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FixedSizeScaleMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FixedSizeScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}