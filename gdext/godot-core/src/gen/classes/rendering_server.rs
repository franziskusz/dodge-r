#![doc = "Sidecar module for class [`RenderingServer`][crate::engine::RenderingServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingServer` enums](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`rendering_server`][crate::engine::rendering_server]: sidecar module with related enum/flag types\n* [`IRenderingServer`][crate::engine::IRenderingServer]: virtual methods\n\n\nSee also [Godot docs for `RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RenderingServer`][crate::engine::RenderingServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderingServer` methods](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderingServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderingServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"RenderingServer\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn texture_2d_create(&mut self, image: Gd < crate::engine::Image >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Gd < crate::engine::Image >);
            let args = (image,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_layered_create(&mut self, layers: Array < Gd < crate::engine::Image > >, layered_type: crate::engine::rendering_server::TextureLayeredType,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Array < Gd < crate::engine::Image > >, crate::engine::rendering_server::TextureLayeredType);
            let args = (layers, layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_layered_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_3d_create(&mut self, format: crate::engine::image::Format, width: i32, height: i32, depth: i32, mipmaps: bool, data: Array < Gd < crate::engine::Image > >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, crate::engine::image::Format, i32, i32, i32, bool, Array < Gd < crate::engine::Image > >);
            let args = (format, width, height, depth, mipmaps, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_3d_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_proxy_create(&mut self, base: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_proxy_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_update(&mut self, texture: Rid, image: Gd < crate::engine::Image >, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Gd < crate::engine::Image >, i32);
            let args = (texture, image, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_3d_update(&mut self, texture: Rid, data: Array < Gd < crate::engine::Image > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Array < Gd < crate::engine::Image > >);
            let args = (texture, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_3d_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_proxy_update(&mut self, texture: Rid, proxy_to: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (texture, proxy_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_proxy_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_placeholder_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_placeholder_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_layered_placeholder_create(&mut self, layered_type: crate::engine::rendering_server::TextureLayeredType,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, crate::engine::rendering_server::TextureLayeredType);
            let args = (layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_layered_placeholder_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_3d_placeholder_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_3d_placeholder_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_get(&self, texture: Rid,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_2d_layer_get(&self, texture: Rid, layer: i32,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rid, i32);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_2d_layer_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_3d_get(&self, texture: Rid,) -> Array < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Image > > >;
            type CallSig = (Array < Gd < crate::engine::Image > >, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_3d_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_replace(&mut self, texture: Rid, by_texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (texture, by_texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_replace", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_set_size_override(&mut self, texture: Rid, width: i32, height: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32);
            let args = (texture, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_set_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_set_path(&mut self, texture: Rid, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (texture, path,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_set_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_path(&self, texture: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_format(&self, texture: Rid,) -> crate::engine::image::Format {
            type RetMarshal = PtrcallReturnT < crate::engine::image::Format >;
            type CallSig = (crate::engine::image::Format, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_set_force_redraw_if_visible(&mut self, texture: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (texture, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_set_force_redraw_if_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn texture_rd_create_full(&mut self, rd_texture: Rid, layer_type: crate::engine::rendering_server::TextureLayeredType,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, crate::engine::rendering_server::TextureLayeredType);
            let args = (rd_texture, layer_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_rd_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_rd_create(&mut self, rd_texture: Rid,) -> Rid {
            self.texture_rd_create_ex(rd_texture,) . done()
        }
        #[inline]
        pub fn texture_rd_create_ex(&mut self, rd_texture: Rid,) -> ExTextureRdCreate < '_ > {
            ExTextureRdCreate::new(self, rd_texture,)
        }
        pub(crate) fn texture_get_rd_texture_full(&self, texture: Rid, srgb: bool,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, bool);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_rd_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_get_rd_texture(&self, texture: Rid,) -> Rid {
            self.texture_get_rd_texture_ex(texture,) . done()
        }
        #[inline]
        pub fn texture_get_rd_texture_ex(&self, texture: Rid,) -> ExTextureGetRdTexture < '_ > {
            ExTextureGetRdTexture::new(self, texture,)
        }
        pub(crate) fn texture_get_native_handle_full(&self, texture: Rid, srgb: bool,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, Rid, bool);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "texture_get_native_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn texture_get_native_handle(&self, texture: Rid,) -> u64 {
            self.texture_get_native_handle_ex(texture,) . done()
        }
        #[inline]
        pub fn texture_get_native_handle_ex(&self, texture: Rid,) -> ExTextureGetNativeHandle < '_ > {
            ExTextureGetNativeHandle::new(self, texture,)
        }
        pub fn shader_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_set_code(&mut self, shader: Rid, code: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (shader, code,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_set_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_set_path_hint(&mut self, shader: Rid, path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, GString);
            let args = (shader, path,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_set_path_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_get_code(&self, shader: Rid,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Rid);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_get_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shader_parameter_list(&self, shader: Rid,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shader_parameter_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_get_parameter_default(&self, shader: Rid, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, StringName);
            let args = (shader, name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_get_parameter_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shader_set_default_texture_parameter_full(&mut self, shader: Rid, name: StringName, texture: Rid, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, StringName, Rid, i32);
            let args = (shader, name, texture, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_set_default_texture_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_set_default_texture_parameter(&mut self, shader: Rid, name: StringName, texture: Rid,) {
            self.shader_set_default_texture_parameter_ex(shader, name, texture,) . done()
        }
        #[inline]
        pub fn shader_set_default_texture_parameter_ex(&mut self, shader: Rid, name: StringName, texture: Rid,) -> ExShaderSetDefaultTextureParameter < '_ > {
            ExShaderSetDefaultTextureParameter::new(self, shader, name, texture,)
        }
        pub(crate) fn shader_get_default_texture_parameter_full(&self, shader: Rid, name: StringName, index: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, StringName, i32);
            let args = (shader, name, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "shader_get_default_texture_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn shader_get_default_texture_parameter(&self, shader: Rid, name: StringName,) -> Rid {
            self.shader_get_default_texture_parameter_ex(shader, name,) . done()
        }
        #[inline]
        pub fn shader_get_default_texture_parameter_ex(&self, shader: Rid, name: StringName,) -> ExShaderGetDefaultTextureParameter < '_ > {
            ExShaderGetDefaultTextureParameter::new(self, shader, name,)
        }
        pub fn material_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn material_set_shader(&mut self, shader_material: Rid, shader: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (shader_material, shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_set_shader", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn material_set_param(&mut self, material: Rid, parameter: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, StringName, Variant);
            let args = (material, parameter, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn material_get_param(&self, material: Rid, parameter: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, StringName);
            let args = (material, parameter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn material_set_render_priority(&mut self, material: Rid, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (material, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_set_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn material_set_next_pass(&mut self, material: Rid, next_material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (material, next_material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "material_set_next_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn mesh_create_from_surfaces_full(&mut self, surfaces: Array < Dictionary >, blend_shape_count: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Array < Dictionary >, i32);
            let args = (surfaces, blend_shape_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_create_from_surfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn mesh_create_from_surfaces(&mut self, surfaces: Array < Dictionary >,) -> Rid {
            self.mesh_create_from_surfaces_ex(surfaces,) . done()
        }
        #[inline]
        pub fn mesh_create_from_surfaces_ex(&mut self, surfaces: Array < Dictionary >,) -> ExMeshCreateFromSurfaces < '_ > {
            ExMeshCreateFromSurfaces::new(self, surfaces,)
        }
        pub fn mesh_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_format_offset(&self, format: crate::engine::rendering_server::ArrayFormat, vertex_count: i32, array_index: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, crate::engine::rendering_server::ArrayFormat, i32, i32);
            let args = (format, vertex_count, array_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_format_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_format_vertex_stride(&self, format: crate::engine::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, crate::engine::rendering_server::ArrayFormat, i32);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_format_vertex_stride", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_format_normal_tangent_stride(&self, format: crate::engine::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, crate::engine::rendering_server::ArrayFormat, i32);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_format_normal_tangent_stride", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_format_attribute_stride(&self, format: crate::engine::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, crate::engine::rendering_server::ArrayFormat, i32);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_format_attribute_stride", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_format_skin_stride(&self, format: crate::engine::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32, crate::engine::rendering_server::ArrayFormat, i32);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_format_skin_stride", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_add_surface(&mut self, mesh: Rid, surface: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Dictionary);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_add_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn mesh_add_surface_from_arrays_full(&mut self, mesh: Rid, primitive: crate::engine::rendering_server::PrimitiveType, arrays: VariantArray, blend_shapes: VariantArray, lods: Dictionary, compress_format: crate::engine::rendering_server::ArrayFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::PrimitiveType, VariantArray, VariantArray, Dictionary, crate::engine::rendering_server::ArrayFormat);
            let args = (mesh, primitive, arrays, blend_shapes, lods, compress_format,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_add_surface_from_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn mesh_add_surface_from_arrays(&mut self, mesh: Rid, primitive: crate::engine::rendering_server::PrimitiveType, arrays: VariantArray,) {
            self.mesh_add_surface_from_arrays_ex(mesh, primitive, arrays,) . done()
        }
        #[inline]
        pub fn mesh_add_surface_from_arrays_ex(&mut self, mesh: Rid, primitive: crate::engine::rendering_server::PrimitiveType, arrays: VariantArray,) -> ExMeshAddSurfaceFromArrays < '_ > {
            ExMeshAddSurfaceFromArrays::new(self, mesh, primitive, arrays,)
        }
        pub fn mesh_get_blend_shape_count(&self, mesh: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_set_blend_shape_mode(&mut self, mesh: Rid, mode: crate::engine::rendering_server::BlendShapeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::BlendShapeMode);
            let args = (mesh, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_set_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_get_blend_shape_mode(&self, mesh: Rid,) -> crate::engine::rendering_server::BlendShapeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_server::BlendShapeMode >;
            type CallSig = (crate::engine::rendering_server::BlendShapeMode, Rid);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_get_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_set_material(&mut self, mesh: Rid, surface: i32, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Rid);
            let args = (mesh, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_material(&self, mesh: Rid, surface: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, i32);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_get_surface(&mut self, mesh: Rid, surface: i32,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Rid, i32);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_get_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_arrays(&self, mesh: Rid, surface: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, Rid, i32);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_get_blend_shape_arrays(&self, mesh: Rid, surface: i32,) -> Array < VariantArray > {
            type RetMarshal = PtrcallReturnT < Array < VariantArray > >;
            type CallSig = (Array < VariantArray >, Rid, i32);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_get_blend_shape_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_get_surface_count(&self, mesh: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_get_surface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_set_custom_aabb(&mut self, mesh: Rid, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Aabb);
            let args = (mesh, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_get_custom_aabb(&self, mesh: Rid,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb, Rid);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_clear(&mut self, mesh: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_update_vertex_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32, PackedByteArray);
            let args = (mesh, surface, offset, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_update_vertex_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_update_attribute_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32, PackedByteArray);
            let args = (mesh, surface, offset, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_update_attribute_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_surface_update_skin_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32, PackedByteArray);
            let args = (mesh, surface, offset, data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_surface_update_skin_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mesh_set_shadow_mesh(&mut self, mesh: Rid, shadow_mesh: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (mesh, shadow_mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mesh_set_shadow_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn multimesh_allocate_data_full(&mut self, multimesh: Rid, instances: i32, transform_format: crate::engine::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, crate::engine::rendering_server::MultimeshTransformFormat, bool, bool);
            let args = (multimesh, instances, transform_format, color_format, custom_data_format,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_allocate_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn multimesh_allocate_data(&mut self, multimesh: Rid, instances: i32, transform_format: crate::engine::rendering_server::MultimeshTransformFormat,) {
            self.multimesh_allocate_data_ex(multimesh, instances, transform_format,) . done()
        }
        #[inline]
        pub fn multimesh_allocate_data_ex(&mut self, multimesh: Rid, instances: i32, transform_format: crate::engine::rendering_server::MultimeshTransformFormat,) -> ExMultimeshAllocateData < '_ > {
            ExMultimeshAllocateData::new(self, multimesh, instances, transform_format,)
        }
        pub fn multimesh_get_instance_count(&self, multimesh: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_get_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_set_mesh(&mut self, multimesh: Rid, mesh: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (multimesh, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_set_transform(&mut self, multimesh: Rid, index: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_set_transform_2d(&mut self, multimesh: Rid, index: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_set_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_set_color(&mut self, multimesh: Rid, index: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Color);
            let args = (multimesh, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_set_custom_data(&mut self, multimesh: Rid, index: i32, custom_data: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Color);
            let args = (multimesh, index, custom_data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_set_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_get_mesh(&self, multimesh: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_get_aabb(&self, multimesh: Rid,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb, Rid);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_get_transform(&self, multimesh: Rid, index: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid, i32);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_get_transform_2d(&self, multimesh: Rid, index: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid, i32);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_get_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_get_color(&self, multimesh: Rid, index: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, Rid, i32);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_instance_get_custom_data(&self, multimesh: Rid, index: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, Rid, i32);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_instance_get_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_set_visible_instances(&mut self, multimesh: Rid, visible: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (multimesh, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_set_visible_instances", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_get_visible_instances(&self, multimesh: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_get_visible_instances", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_set_buffer(&mut self, multimesh: Rid, buffer: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedFloat32Array);
            let args = (multimesh, buffer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_set_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn multimesh_get_buffer(&self, multimesh: Rid,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array, Rid);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "multimesh_get_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn skeleton_allocate_data_full(&mut self, skeleton: Rid, bones: i32, is_2d_skeleton: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (skeleton, bones, is_2d_skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_allocate_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn skeleton_allocate_data(&mut self, skeleton: Rid, bones: i32,) {
            self.skeleton_allocate_data_ex(skeleton, bones,) . done()
        }
        #[inline]
        pub fn skeleton_allocate_data_ex(&mut self, skeleton: Rid, bones: i32,) -> ExSkeletonAllocateData < '_ > {
            ExSkeletonAllocateData::new(self, skeleton, bones,)
        }
        pub fn skeleton_get_bone_count(&self, skeleton: Rid,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid);
            let args = (skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_get_bone_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_bone_set_transform(&mut self, skeleton: Rid, bone: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_bone_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_bone_get_transform(&self, skeleton: Rid, bone: i32,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid, i32);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_bone_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_bone_set_transform_2d(&mut self, skeleton: Rid, bone: i32, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_bone_set_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_bone_get_transform_2d(&self, skeleton: Rid, bone: i32,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Rid, i32);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_bone_get_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skeleton_set_base_transform_2d(&mut self, skeleton: Rid, base_transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (skeleton, base_transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "skeleton_set_base_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn directional_light_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "directional_light_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn omni_light_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "omni_light_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn spot_light_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "spot_light_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_color(&mut self, light: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_param(&mut self, light: Rid, param: crate::engine::rendering_server::LightParam, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::LightParam, f32);
            let args = (light, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_shadow(&mut self, light: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_projector(&mut self, light: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_projector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_negative(&mut self, light: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_negative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_cull_mask(&mut self, light: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, shadow: f32, length: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32, f32);
            let args = (decal, enabled, begin, shadow, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_reverse_cull_face_mode(&mut self, light: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_reverse_cull_face_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_bake_mode(&mut self, light: Rid, bake_mode: crate::engine::rendering_server::LightBakeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::LightBakeMode);
            let args = (light, bake_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_bake_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_set_max_sdfgi_cascade(&mut self, light: Rid, cascade: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (light, cascade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_set_max_sdfgi_cascade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_omni_set_shadow_mode(&mut self, light: Rid, mode: crate::engine::rendering_server::LightOmniShadowMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::LightOmniShadowMode);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_omni_set_shadow_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_directional_set_shadow_mode(&mut self, light: Rid, mode: crate::engine::rendering_server::LightDirectionalShadowMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::LightDirectionalShadowMode);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_directional_set_shadow_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_directional_set_blend_splits(&mut self, light: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_directional_set_blend_splits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_directional_set_sky_mode(&mut self, light: Rid, mode: crate::engine::rendering_server::LightDirectionalSkyMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::LightDirectionalSkyMode);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_directional_set_sky_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn light_projectors_set_filter(&mut self, filter: crate::engine::rendering_server::LightProjectorFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::LightProjectorFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "light_projectors_set_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn positional_soft_shadow_filter_set_quality(&mut self, quality: crate::engine::rendering_server::ShadowQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::ShadowQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "positional_soft_shadow_filter_set_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn directional_soft_shadow_filter_set_quality(&mut self, quality: crate::engine::rendering_server::ShadowQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::ShadowQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "directional_soft_shadow_filter_set_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn directional_shadow_atlas_set_size(&mut self, size: i32, is_16bits: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (size, is_16bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "directional_shadow_atlas_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_update_mode(&mut self, probe: Rid, mode: crate::engine::rendering_server::ReflectionProbeUpdateMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ReflectionProbeUpdateMode);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_intensity(&mut self, probe: Rid, intensity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (probe, intensity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_ambient_mode(&mut self, probe: Rid, mode: crate::engine::rendering_server::ReflectionProbeAmbientMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ReflectionProbeAmbientMode);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_ambient_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_ambient_color(&mut self, probe: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (probe, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_ambient_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_ambient_energy(&mut self, probe: Rid, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (probe, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_ambient_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_max_distance(&mut self, probe: Rid, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (probe, distance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_size(&mut self, probe: Rid, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (probe, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_origin_offset(&mut self, probe: Rid, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (probe, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_origin_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_as_interior(&mut self, probe: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_as_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_enable_box_projection(&mut self, probe: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_enable_box_projection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_enable_shadows(&mut self, probe: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_enable_shadows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_cull_mask(&mut self, probe: Rid, layers: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (probe, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_resolution(&mut self, probe: Rid, resolution: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (probe, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reflection_probe_set_mesh_lod_threshold(&mut self, probe: Rid, pixels: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (probe, pixels,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reflection_probe_set_mesh_lod_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_size(&mut self, decal: Rid, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (decal, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_texture(&mut self, decal: Rid, type_: crate::engine::rendering_server::DecalTexture, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::DecalTexture, Rid);
            let args = (decal, type_, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_emission_energy(&mut self, decal: Rid, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (decal, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_emission_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_albedo_mix(&mut self, decal: Rid, albedo_mix: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (decal, albedo_mix,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_albedo_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_modulate(&mut self, decal: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (decal, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_cull_mask(&mut self, decal: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (decal, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, length: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32);
            let args = (decal, enabled, begin, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_fade(&mut self, decal: Rid, above: f32, below: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32);
            let args = (decal, above, below,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decal_set_normal_fade(&mut self, decal: Rid, fade: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (decal, fade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decal_set_normal_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decals_set_filter(&mut self, filter: crate::engine::rendering_server::DecalFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::DecalFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decals_set_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gi_set_use_half_resolution(&mut self, half_resolution: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (half_resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "gi_set_use_half_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_allocate_data(&mut self, voxel_gi: Rid, to_cell_xform: Transform3D, aabb: Aabb, octree_size: Vector3i, octree_cells: PackedByteArray, data_cells: PackedByteArray, distance_field: PackedByteArray, level_counts: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D, Aabb, Vector3i, PackedByteArray, PackedByteArray, PackedByteArray, PackedInt32Array);
            let args = (voxel_gi, to_cell_xform, aabb, octree_size, octree_cells, data_cells, distance_field, level_counts,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_allocate_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_octree_size(&self, voxel_gi: Rid,) -> Vector3i {
            type RetMarshal = PtrcallReturnT < Vector3i >;
            type CallSig = (Vector3i, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_octree_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_octree_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_octree_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_data_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_data_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_distance_field(&self, voxel_gi: Rid,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_level_counts(&self, voxel_gi: Rid,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_level_counts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_get_to_cell_xform(&self, voxel_gi: Rid,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, Rid);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_get_to_cell_xform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_dynamic_range(&mut self, voxel_gi: Rid, range: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, range,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_dynamic_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_propagation(&mut self, voxel_gi: Rid, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_propagation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_energy(&mut self, voxel_gi: Rid, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_baked_exposure_normalization(&mut self, voxel_gi: Rid, baked_exposure: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_baked_exposure_normalization", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_normal_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_normal_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_interior(&mut self, voxel_gi: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_use_two_bounces(&mut self, voxel_gi: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_use_two_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn voxel_gi_set_quality(&mut self, quality: crate::engine::rendering_server::VoxelGIQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::VoxelGIQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "voxel_gi_set_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_textures(&mut self, lightmap: Rid, light: Rid, uses_sh: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, bool);
            let args = (lightmap, light, uses_sh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_probe_bounds(&mut self, lightmap: Rid, bounds: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Aabb);
            let args = (lightmap, bounds,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_probe_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_probe_interior(&mut self, lightmap: Rid, interior: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (lightmap, interior,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_probe_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_probe_capture_data(&mut self, lightmap: Rid, points: PackedVector3Array, point_sh: PackedColorArray, tetrahedra: PackedInt32Array, bsp_tree: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector3Array, PackedColorArray, PackedInt32Array, PackedInt32Array);
            let args = (lightmap, points, point_sh, tetrahedra, bsp_tree,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_probe_capture_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_get_probe_capture_points(&self, lightmap: Rid,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Rid);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_get_probe_capture_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_get_probe_capture_sh(&self, lightmap: Rid,) -> PackedColorArray {
            type RetMarshal = PtrcallReturnT < PackedColorArray >;
            type CallSig = (PackedColorArray, Rid);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_get_probe_capture_sh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_get_probe_capture_tetrahedra(&self, lightmap: Rid,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_get_probe_capture_tetrahedra", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_get_probe_capture_bsp_tree(&self, lightmap: Rid,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, Rid);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_get_probe_capture_bsp_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_baked_exposure_normalization(&mut self, lightmap: Rid, baked_exposure: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (lightmap, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_baked_exposure_normalization", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_set_probe_capture_update_speed(&mut self, speed: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_set_probe_capture_update_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_mode(&mut self, particles: Rid, mode: crate::engine::rendering_server::ParticlesMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ParticlesMode);
            let args = (particles, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_emitting(&mut self, particles: Rid, emitting: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (particles, emitting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_get_emitting(&mut self, particles: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_get_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_amount(&mut self, particles: Rid, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (particles, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_amount_ratio(&mut self, particles: Rid, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_amount_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_lifetime(&mut self, particles: Rid, lifetime: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64);
            let args = (particles, lifetime,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_one_shot(&mut self, particles: Rid, one_shot: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (particles, one_shot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_pre_process_time(&mut self, particles: Rid, time: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64);
            let args = (particles, time,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_explosiveness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_randomness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_interp_to_end(&mut self, particles: Rid, factor: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles, factor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_emitter_velocity(&mut self, particles: Rid, velocity: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (particles, velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_emitter_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_custom_aabb(&mut self, particles: Rid, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Aabb);
            let args = (particles, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_speed_scale(&mut self, particles: Rid, scale: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64);
            let args = (particles, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_use_local_coordinates(&mut self, particles: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_process_material(&mut self, particles: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (particles, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_fixed_fps(&mut self, particles: Rid, fps: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (particles, fps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_interpolate(&mut self, particles: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_fractional_delta(&mut self, particles: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_collision_base_size(&mut self, particles: Rid, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_transform_align(&mut self, particles: Rid, align: crate::engine::rendering_server::ParticlesTransformAlign,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ParticlesTransformAlign);
            let args = (particles, align,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_transform_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_trails(&mut self, particles: Rid, enable: bool, length_sec: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32);
            let args = (particles, enable, length_sec,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_trails", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_trail_bind_poses(&mut self, particles: Rid, bind_poses: Array < Transform3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Array < Transform3D >);
            let args = (particles, bind_poses,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_trail_bind_poses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_is_inactive(&mut self, particles: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_is_inactive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_request_process(&mut self, particles: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_request_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_restart(&mut self, particles: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_subemitter(&mut self, particles: Rid, subemitter_particles: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (particles, subemitter_particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_subemitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_emit(&mut self, particles: Rid, transform: Transform3D, velocity: Vector3, color: Color, custom: Color, emit_flags: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D, Vector3, Color, Color, u32);
            let args = (particles, transform, velocity, color, custom, emit_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_emit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_draw_order(&mut self, particles: Rid, order: crate::engine::rendering_server::ParticlesDrawOrder,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ParticlesDrawOrder);
            let args = (particles, order,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_draw_passes(&mut self, particles: Rid, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (particles, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_draw_passes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_draw_pass_mesh(&mut self, particles: Rid, pass: i32, mesh: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Rid);
            let args = (particles, pass, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_draw_pass_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_get_current_aabb(&mut self, particles: Rid,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb, Rid);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_get_current_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_set_emission_transform(&mut self, particles: Rid, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D);
            let args = (particles, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_set_emission_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_collision_type(&mut self, particles_collision: Rid, type_: crate::engine::rendering_server::ParticlesCollisionType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ParticlesCollisionType);
            let args = (particles_collision, type_,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_collision_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_cull_mask(&mut self, particles_collision: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (particles_collision, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_sphere_radius(&mut self, particles_collision: Rid, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles_collision, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_box_extents(&mut self, particles_collision: Rid, extents: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (particles_collision, extents,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_attractor_strength(&mut self, particles_collision: Rid, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles_collision, strength,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_attractor_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_attractor_directionality(&mut self, particles_collision: Rid, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles_collision, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_attractor_directionality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_attractor_attenuation(&mut self, particles_collision: Rid, curve: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (particles_collision, curve,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_attractor_attenuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_field_texture(&mut self, particles_collision: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (particles_collision, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_field_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_height_field_update(&mut self, particles_collision: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (particles_collision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_height_field_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn particles_collision_set_height_field_resolution(&mut self, particles_collision: Rid, resolution: crate::engine::rendering_server::ParticlesCollisionHeightfieldResolution,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ParticlesCollisionHeightfieldResolution);
            let args = (particles_collision, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "particles_collision_set_height_field_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fog_volume_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fog_volume_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fog_volume_set_shape(&mut self, fog_volume: Rid, shape: crate::engine::rendering_server::FogVolumeShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::FogVolumeShape);
            let args = (fog_volume, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fog_volume_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fog_volume_set_size(&mut self, fog_volume: Rid, size: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector3);
            let args = (fog_volume, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fog_volume_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fog_volume_set_material(&mut self, fog_volume: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (fog_volume, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "fog_volume_set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn visibility_notifier_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "visibility_notifier_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn visibility_notifier_set_aabb(&mut self, notifier: Rid, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Aabb);
            let args = (notifier, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "visibility_notifier_set_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn visibility_notifier_set_callbacks(&mut self, notifier: Rid, enter_callable: Callable, exit_callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Callable, Callable);
            let args = (notifier, enter_callable, exit_callable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "visibility_notifier_set_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn occluder_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "occluder_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn occluder_set_mesh(&mut self, occluder: Rid, vertices: PackedVector3Array, indices: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector3Array, PackedInt32Array);
            let args = (occluder, vertices, indices,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "occluder_set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_perspective(&mut self, camera: Rid, fovy_degrees: f32, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32, f32);
            let args = (camera, fovy_degrees, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_perspective", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_orthogonal(&mut self, camera: Rid, size: f32, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32, f32);
            let args = (camera, size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_orthogonal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_frustum(&mut self, camera: Rid, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, Vector2, f32, f32);
            let args = (camera, size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_frustum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_transform(&mut self, camera: Rid, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D);
            let args = (camera, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_cull_mask(&mut self, camera: Rid, layers: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (camera, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_environment(&mut self, camera: Rid, env: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (camera, env,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_camera_attributes(&mut self, camera: Rid, effects: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (camera, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_set_use_vertical_aspect(&mut self, camera: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (camera, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_set_use_vertical_aspect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_use_xr(&mut self, viewport: Rid, use_xr: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, use_xr,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_use_xr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_size(&mut self, viewport: Rid, width: i32, height: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32);
            let args = (viewport, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_active(&mut self, viewport: Rid, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_parent_viewport(&mut self, viewport: Rid, parent_viewport: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, parent_viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_parent_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn viewport_attach_to_screen_full(&mut self, viewport: Rid, rect: Rect2, screen: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, i32);
            let args = (viewport, rect, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_attach_to_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn viewport_attach_to_screen(&mut self, viewport: Rid,) {
            self.viewport_attach_to_screen_ex(viewport,) . done()
        }
        #[inline]
        pub fn viewport_attach_to_screen_ex(&mut self, viewport: Rid,) -> ExViewportAttachToScreen < '_ > {
            ExViewportAttachToScreen::new(self, viewport,)
        }
        pub fn viewport_set_render_direct_to_screen(&mut self, viewport: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_render_direct_to_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_canvas_cull_mask(&mut self, viewport: Rid, canvas_cull_mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (viewport, canvas_cull_mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_canvas_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_scaling_3d_mode(&mut self, viewport: Rid, scaling_3d_mode: crate::engine::rendering_server::ViewportScaling3DMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportScaling3DMode);
            let args = (viewport, scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_scaling_3d_scale(&mut self, viewport: Rid, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (viewport, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_scaling_3d_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_fsr_sharpness(&mut self, viewport: Rid, sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (viewport, sharpness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_texture_mipmap_bias(&mut self, viewport: Rid, mipmap_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (viewport, mipmap_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_texture_mipmap_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_update_mode(&mut self, viewport: Rid, update_mode: crate::engine::rendering_server::ViewportUpdateMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportUpdateMode);
            let args = (viewport, update_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_clear_mode(&mut self, viewport: Rid, clear_mode: crate::engine::rendering_server::ViewportClearMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportClearMode);
            let args = (viewport, clear_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_clear_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_get_render_target(&self, viewport: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_get_render_target", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_get_texture(&self, viewport: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_disable_3d(&mut self, viewport: Rid, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_disable_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_disable_2d(&mut self, viewport: Rid, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_disable_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_environment_mode(&mut self, viewport: Rid, mode: crate::engine::rendering_server::ViewportEnvironmentMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportEnvironmentMode);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_environment_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_attach_camera(&mut self, viewport: Rid, camera: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, camera,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_attach_camera", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_scenario(&mut self, viewport: Rid, scenario: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_scenario", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_attach_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_attach_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_remove_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_remove_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_snap_2d_transforms_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_snap_2d_transforms_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_snap_2d_vertices_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_snap_2d_vertices_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_default_canvas_item_texture_filter(&mut self, viewport: Rid, filter: crate::engine::rendering_server::CanvasItemTextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureFilter);
            let args = (viewport, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_default_canvas_item_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_default_canvas_item_texture_repeat(&mut self, viewport: Rid, repeat: crate::engine::rendering_server::CanvasItemTextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureRepeat);
            let args = (viewport, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_default_canvas_item_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_canvas_transform(&mut self, viewport: Rid, canvas: Rid, offset: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform2D);
            let args = (viewport, canvas, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_canvas_stacking(&mut self, viewport: Rid, canvas: Rid, layer: i32, sublayer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, i32, i32);
            let args = (viewport, canvas, layer, sublayer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_canvas_stacking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_transparent_background(&mut self, viewport: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_transparent_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_global_canvas_transform(&mut self, viewport: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (viewport, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_global_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_sdf_oversize_and_scale(&mut self, viewport: Rid, oversize: crate::engine::rendering_server::ViewportSDFOversize, scale: crate::engine::rendering_server::ViewportSDFScale,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportSDFOversize, crate::engine::rendering_server::ViewportSDFScale);
            let args = (viewport, oversize, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_sdf_oversize_and_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn viewport_set_positional_shadow_atlas_size_full(&mut self, viewport: Rid, size: i32, use_16_bits: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, bool);
            let args = (viewport, size, use_16_bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_positional_shadow_atlas_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size(&mut self, viewport: Rid, size: i32,) {
            self.viewport_set_positional_shadow_atlas_size_ex(viewport, size,) . done()
        }
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size_ex(&mut self, viewport: Rid, size: i32,) -> ExViewportSetPositionalShadowAtlasSize < '_ > {
            ExViewportSetPositionalShadowAtlasSize::new(self, viewport, size,)
        }
        pub fn viewport_set_positional_shadow_atlas_quadrant_subdivision(&mut self, viewport: Rid, quadrant: i32, subdivision: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32);
            let args = (viewport, quadrant, subdivision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_positional_shadow_atlas_quadrant_subdivision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_msaa_3d(&mut self, viewport: Rid, msaa: crate::engine::rendering_server::ViewportMSAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportMSAA);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_msaa_2d(&mut self, viewport: Rid, msaa: crate::engine::rendering_server::ViewportMSAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportMSAA);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_msaa_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_use_hdr_2d(&mut self, viewport: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_use_hdr_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_screen_space_aa(&mut self, viewport: Rid, mode: crate::engine::rendering_server::ViewportScreenSpaceAA,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportScreenSpaceAA);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_use_taa(&mut self, viewport: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_use_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_use_debanding(&mut self, viewport: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_use_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_use_occlusion_culling(&mut self, viewport: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_use_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_occlusion_rays_per_thread(&mut self, rays_per_thread: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (rays_per_thread,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_occlusion_rays_per_thread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_occlusion_culling_build_quality(&mut self, quality: crate::engine::rendering_server::ViewportOcclusionCullingBuildQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::ViewportOcclusionCullingBuildQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_occlusion_culling_build_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_get_render_info(&mut self, viewport: Rid, type_: crate::engine::rendering_server::ViewportRenderInfoType, info: crate::engine::rendering_server::ViewportRenderInfo,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Rid, crate::engine::rendering_server::ViewportRenderInfoType, crate::engine::rendering_server::ViewportRenderInfo);
            let args = (viewport, type_, info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_get_render_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_debug_draw(&mut self, viewport: Rid, draw: crate::engine::rendering_server::ViewportDebugDraw,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportDebugDraw);
            let args = (viewport, draw,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_debug_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_measure_render_time(&mut self, viewport: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_measure_render_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_get_measured_render_time_cpu(&self, viewport: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_get_measured_render_time_cpu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_get_measured_render_time_gpu(&self, viewport: Rid,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, Rid);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_get_measured_render_time_gpu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_vrs_mode(&mut self, viewport: Rid, mode: crate::engine::rendering_server::ViewportVRSMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ViewportVRSMode);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_vrs_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn viewport_set_vrs_texture(&mut self, viewport: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (viewport, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "viewport_set_vrs_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sky_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sky_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sky_set_radiance_size(&mut self, sky: Rid, radiance_size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (sky, radiance_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sky_set_radiance_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sky_set_mode(&mut self, sky: Rid, mode: crate::engine::rendering_server::SkyMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::SkyMode);
            let args = (sky, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sky_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sky_set_material(&mut self, sky: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (sky, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sky_set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sky_bake_panorama(&mut self, sky: Rid, energy: f32, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rid, f32, bool, Vector2i);
            let args = (sky, energy, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sky_bake_panorama", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_background(&mut self, env: Rid, bg: crate::engine::rendering_server::EnvironmentBG,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::EnvironmentBG);
            let args = (env, bg,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sky(&mut self, env: Rid, sky: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (env, sky,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sky_custom_fov(&mut self, env: Rid, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (env, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sky_custom_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sky_orientation(&mut self, env: Rid, orientation: Basis,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Basis);
            let args = (env, orientation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sky_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_bg_color(&mut self, env: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (env, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_bg_energy(&mut self, env: Rid, multiplier: f32, exposure_value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32);
            let args = (env, multiplier, exposure_value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_bg_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_canvas_max_layer(&mut self, env: Rid, max_layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (env, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_canvas_max_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn environment_set_ambient_light_full(&mut self, env: Rid, color: Color, ambient: crate::engine::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contibution: f32, reflection_source: crate::engine::rendering_server::EnvironmentReflectionSource,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color, crate::engine::rendering_server::EnvironmentAmbientSource, f32, f32, crate::engine::rendering_server::EnvironmentReflectionSource);
            let args = (env, color, ambient, energy, sky_contibution, reflection_source,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ambient_light", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn environment_set_ambient_light(&mut self, env: Rid, color: Color,) {
            self.environment_set_ambient_light_ex(env, color,) . done()
        }
        #[inline]
        pub fn environment_set_ambient_light_ex(&mut self, env: Rid, color: Color,) -> ExEnvironmentSetAmbientLight < '_ > {
            ExEnvironmentSetAmbientLight::new(self, env, color,)
        }
        pub fn environment_set_glow(&mut self, env: Rid, enable: bool, levels: PackedFloat32Array, intensity: f32, strength: f32, mix: f32, bloom_threshold: f32, blend_mode: crate::engine::rendering_server::EnvironmentGlowBlendMode, hdr_bleed_threshold: f32, hdr_bleed_scale: f32, hdr_luminance_cap: f32, glow_map_strength: f32, glow_map: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, PackedFloat32Array, f32, f32, f32, f32, crate::engine::rendering_server::EnvironmentGlowBlendMode, f32, f32, f32, f32, Rid);
            let args = (env, enable, levels, intensity, strength, mix, bloom_threshold, blend_mode, hdr_bleed_threshold, hdr_bleed_scale, hdr_luminance_cap, glow_map_strength, glow_map,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_glow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_tonemap(&mut self, env: Rid, tone_mapper: crate::engine::rendering_server::EnvironmentToneMapper, exposure: f32, white: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::EnvironmentToneMapper, f32, f32);
            let args = (env, tone_mapper, exposure, white,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_tonemap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_adjustment(&mut self, env: Rid, enable: bool, brightness: f32, contrast: f32, saturation: f32, use_1d_color_correction: bool, color_correction: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32, f32, bool, Rid);
            let args = (env, enable, brightness, contrast, saturation, use_1d_color_correction, color_correction,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_adjustment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_ssr(&mut self, env: Rid, enable: bool, max_steps: i32, fade_in: f32, fade_out: f32, depth_tolerance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, i32, f32, f32, f32);
            let args = (env, enable, max_steps, fade_in, fade_out, depth_tolerance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ssr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_ssao(&mut self, env: Rid, enable: bool, radius: f32, intensity: f32, power: f32, detail: f32, horizon: f32, sharpness: f32, light_affect: f32, ao_channel_affect: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32, f32, f32, f32, f32, f32, f32);
            let args = (env, enable, radius, intensity, power, detail, horizon, sharpness, light_affect, ao_channel_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ssao", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_fog(&mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, Color, f32, f32, f32, f32, f32, f32, f32);
            let args = (env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_fog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sdfgi(&mut self, env: Rid, enable: bool, cascades: i32, min_cell_size: f32, y_scale: crate::engine::rendering_server::EnvironmentSDFGIYScale, use_occlusion: bool, bounce_feedback: f32, read_sky: bool, energy: f32, normal_bias: f32, probe_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, i32, f32, crate::engine::rendering_server::EnvironmentSDFGIYScale, bool, f32, bool, f32, f32, f32);
            let args = (env, enable, cascades, min_cell_size, y_scale, use_occlusion, bounce_feedback, read_sky, energy, normal_bias, probe_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sdfgi", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_volumetric_fog(&mut self, env: Rid, enable: bool, density: f32, albedo: Color, emission: Color, emission_energy: f32, anisotropy: f32, length: f32, p_detail_spread: f32, gi_inject: f32, temporal_reprojection: bool, temporal_reprojection_amount: f32, ambient_inject: f32, sky_affect: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, Color, Color, f32, f32, f32, f32, f32, bool, f32, f32, f32);
            let args = (env, enable, density, albedo, emission, emission_energy, anisotropy, length, p_detail_spread, gi_inject, temporal_reprojection, temporal_reprojection_amount, ambient_inject, sky_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_volumetric_fog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_glow_set_use_bicubic_upscale(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_glow_set_use_bicubic_upscale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_ssr_roughness_quality(&mut self, quality: crate::engine::rendering_server::EnvironmentSSRRoughnessQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSSRRoughnessQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ssr_roughness_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_ssao_quality(&mut self, quality: crate::engine::rendering_server::EnvironmentSSAOQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSSAOQuality, bool, f32, i32, f32, f32);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ssao_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_ssil_quality(&mut self, quality: crate::engine::rendering_server::EnvironmentSSILQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSSILQuality, bool, f32, i32, f32, f32);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_ssil_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sdfgi_ray_count(&mut self, ray_count: crate::engine::rendering_server::EnvironmentSDFGIRayCount,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSDFGIRayCount);
            let args = (ray_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sdfgi_ray_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sdfgi_frames_to_converge(&mut self, frames: crate::engine::rendering_server::EnvironmentSDFGIFramesToConverge,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSDFGIFramesToConverge);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sdfgi_frames_to_converge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_sdfgi_frames_to_update_light(&mut self, frames: crate::engine::rendering_server::EnvironmentSDFGIFramesToUpdateLight,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::EnvironmentSDFGIFramesToUpdateLight);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_sdfgi_frames_to_update_light", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_volumetric_fog_volume_size(&mut self, size: i32, depth: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (size, depth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_volumetric_fog_volume_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_set_volumetric_fog_filter_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_set_volumetric_fog_filter_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn environment_bake_panorama(&mut self, environment: Rid, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Image > >;
            type CallSig = (Option < Gd < crate::engine::Image > >, Rid, bool, Vector2i);
            let args = (environment, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "environment_bake_panorama", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn screen_space_roughness_limiter_set_active(&mut self, enable: bool, amount: f32, limit: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, f32, f32);
            let args = (enable, amount, limit,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "screen_space_roughness_limiter_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sub_surface_scattering_set_quality(&mut self, quality: crate::engine::rendering_server::SubSurfaceScatteringQuality,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::SubSurfaceScatteringQuality);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sub_surface_scattering_set_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sub_surface_scattering_set_scale(&mut self, scale: f32, depth_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, f32);
            let args = (scale, depth_scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "sub_surface_scattering_set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur_quality(&mut self, quality: crate::engine::rendering_server::DOFBlurQuality, use_jitter: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::DOFBlurQuality, bool);
            let args = (quality, use_jitter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_set_dof_blur_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur_bokeh_shape(&mut self, shape: crate::engine::rendering_server::DOFBokehShape,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_server::DOFBokehShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_set_dof_blur_bokeh_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur(&mut self, camera_attributes: Rid, far_enable: bool, far_distance: f32, far_transition: f32, near_enable: bool, near_distance: f32, near_transition: f32, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32, bool, f32, f32, f32);
            let args = (camera_attributes, far_enable, far_distance, far_transition, near_enable, near_distance, near_transition, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_set_dof_blur", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_set_exposure(&mut self, camera_attributes: Rid, multiplier: f32, normalization: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32);
            let args = (camera_attributes, multiplier, normalization,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_set_exposure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn camera_attributes_set_auto_exposure(&mut self, camera_attributes: Rid, enable: bool, min_sensitivity: f32, max_sensitivity: f32, speed: f32, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, f32, f32, f32, f32);
            let args = (camera_attributes, enable, min_sensitivity, max_sensitivity, speed, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "camera_attributes_set_auto_exposure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scenario_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scenario_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scenario_set_environment(&mut self, scenario: Rid, environment: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scenario_set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scenario_set_fallback_environment(&mut self, scenario: Rid, environment: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scenario_set_fallback_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scenario_set_camera_attributes(&mut self, scenario: Rid, effects: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (scenario, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "scenario_set_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_create2(&mut self, base: Rid, scenario: Rid,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, Rid, Rid);
            let args = (base, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_create2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_base(&mut self, instance: Rid, base: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_base", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_scenario(&mut self, instance: Rid, scenario: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_scenario", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_layer_mask(&mut self, instance: Rid, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (instance, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_layer_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_pivot_data(&mut self, instance: Rid, sorting_offset: f32, use_aabb_center: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, bool);
            let args = (instance, sorting_offset, use_aabb_center,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_pivot_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_transform(&mut self, instance: Rid, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform3D);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_attach_object_instance_id(&mut self, instance: Rid, id: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u64);
            let args = (instance, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_blend_shape_weight(&mut self, instance: Rid, shape: i32, weight: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, f32);
            let args = (instance, shape, weight,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_blend_shape_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_surface_override_material(&mut self, instance: Rid, surface: i32, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, Rid);
            let args = (instance, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_surface_override_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_visible(&mut self, instance: Rid, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (instance, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_transparency(&mut self, instance: Rid, transparency: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (instance, transparency,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_custom_aabb(&mut self, instance: Rid, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Aabb);
            let args = (instance, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_attach_skeleton(&mut self, instance: Rid, skeleton: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_attach_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_extra_visibility_margin(&mut self, instance: Rid, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (instance, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_extra_visibility_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_visibility_parent(&mut self, instance: Rid, parent: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_visibility_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_set_ignore_culling(&mut self, instance: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (instance, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_set_ignore_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_flag(&mut self, instance: Rid, flag: crate::engine::rendering_server::InstanceFlags, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::InstanceFlags, bool);
            let args = (instance, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_cast_shadows_setting(&mut self, instance: Rid, shadow_casting_setting: crate::engine::rendering_server::ShadowCastingSetting,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::ShadowCastingSetting);
            let args = (instance, shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_material_override(&mut self, instance: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_material_overlay(&mut self, instance: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_material_overlay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_visibility_range(&mut self, instance: Rid, min: f32, max: f32, min_margin: f32, max_margin: f32, fade_mode: crate::engine::rendering_server::VisibilityRangeFadeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32, f32, f32, f32, crate::engine::rendering_server::VisibilityRangeFadeMode);
            let args = (instance, min, max, min_margin, max_margin, fade_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_visibility_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_lightmap(&mut self, instance: Rid, lightmap: Rid, lightmap_uv_scale: Rect2, lightmap_slice: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Rect2, i32);
            let args = (instance, lightmap, lightmap_uv_scale, lightmap_slice,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_lightmap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_lod_bias(&mut self, instance: Rid, lod_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (instance, lod_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_set_shader_parameter(&mut self, instance: Rid, parameter: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, StringName, Variant);
            let args = (instance, parameter, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_set_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter(&self, instance: Rid, parameter: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, StringName);
            let args = (instance, parameter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_get_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter_default_value(&self, instance: Rid, parameter: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Rid, StringName);
            let args = (instance, parameter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_get_shader_parameter_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter_list(&self, instance: Rid,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Rid);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instance_geometry_get_shader_parameter_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn instances_cull_aabb_full(&self, aabb: Aabb, scenario: Rid,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, Aabb, Rid);
            let args = (aabb, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instances_cull_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn instances_cull_aabb(&self, aabb: Aabb,) -> PackedInt64Array {
            self.instances_cull_aabb_ex(aabb,) . done()
        }
        #[inline]
        pub fn instances_cull_aabb_ex(&self, aabb: Aabb,) -> ExInstancesCullAabb < '_ > {
            ExInstancesCullAabb::new(self, aabb,)
        }
        pub(crate) fn instances_cull_ray_full(&self, from: Vector3, to: Vector3, scenario: Rid,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, Vector3, Vector3, Rid);
            let args = (from, to, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instances_cull_ray", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn instances_cull_ray(&self, from: Vector3, to: Vector3,) -> PackedInt64Array {
            self.instances_cull_ray_ex(from, to,) . done()
        }
        #[inline]
        pub fn instances_cull_ray_ex(&self, from: Vector3, to: Vector3,) -> ExInstancesCullRay < '_ > {
            ExInstancesCullRay::new(self, from, to,)
        }
        pub(crate) fn instances_cull_convex_full(&self, convex: Array < Plane >, scenario: Rid,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, Array < Plane >, Rid);
            let args = (convex, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "instances_cull_convex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn instances_cull_convex(&self, convex: Array < Plane >,) -> PackedInt64Array {
            self.instances_cull_convex_ex(convex,) . done()
        }
        #[inline]
        pub fn instances_cull_convex_ex(&self, convex: Array < Plane >,) -> ExInstancesCullConvex < '_ > {
            ExInstancesCullConvex::new(self, convex,)
        }
        pub fn bake_render_uv2(&mut self, base: Rid, material_overrides: Array < Rid >, image_size: Vector2i,) -> Array < Gd < crate::engine::Image > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Image > > >;
            type CallSig = (Array < Gd < crate::engine::Image > >, Rid, Array < Rid >, Vector2i);
            let args = (base, material_overrides, image_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "bake_render_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_set_item_mirroring(&mut self, canvas: Rid, item: Rid, mirroring: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Vector2);
            let args = (canvas, item, mirroring,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_set_item_mirroring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_set_modulate(&mut self, canvas: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (canvas, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_set_disable_scale(&mut self, disable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_set_disable_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_texture_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_texture_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_texture_set_channel(&mut self, canvas_texture: Rid, channel: crate::engine::rendering_server::CanvasTextureChannel, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasTextureChannel, Rid);
            let args = (canvas_texture, channel, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_texture_set_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_texture_set_shading_parameters(&mut self, canvas_texture: Rid, base_color: Color, shininess: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color, f32);
            let args = (canvas_texture, base_color, shininess,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_texture_set_shading_parameters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_texture_set_texture_filter(&mut self, canvas_texture: Rid, filter: crate::engine::rendering_server::CanvasItemTextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureFilter);
            let args = (canvas_texture, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_texture_set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_texture_set_texture_repeat(&mut self, canvas_texture: Rid, repeat: crate::engine::rendering_server::CanvasItemTextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureRepeat);
            let args = (canvas_texture, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_texture_set_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_parent(&mut self, item: Rid, parent: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (item, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_default_texture_filter(&mut self, item: Rid, filter: crate::engine::rendering_server::CanvasItemTextureFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureFilter);
            let args = (item, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_default_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_default_texture_repeat(&mut self, item: Rid, repeat: crate::engine::rendering_server::CanvasItemTextureRepeat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasItemTextureRepeat);
            let args = (item, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_default_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_visible(&mut self, item: Rid, visible: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_light_mask(&mut self, item: Rid, mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (item, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_visibility_layer(&mut self, item: Rid, visibility_layer: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, u32);
            let args = (item, visibility_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_visibility_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_clip(&mut self, item: Rid, clip: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, clip,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_distance_field_mode(&mut self, item: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_distance_field_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_set_custom_rect_full(&mut self, item: Rid, use_custom_rect: bool, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, Rect2);
            let args = (item, use_custom_rect, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_custom_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_set_custom_rect(&mut self, item: Rid, use_custom_rect: bool,) {
            self.canvas_item_set_custom_rect_ex(item, use_custom_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_set_custom_rect_ex(&mut self, item: Rid, use_custom_rect: bool,) -> ExCanvasItemSetCustomRect < '_ > {
            ExCanvasItemSetCustomRect::new(self, item, use_custom_rect,)
        }
        pub fn canvas_item_set_modulate(&mut self, item: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_self_modulate(&mut self, item: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_self_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_draw_behind_parent(&mut self, item: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_draw_behind_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_add_line_full(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, Vector2, Color, f32, bool);
            let args = (item, from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_line(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) {
            self.canvas_item_add_line_ex(item, from, to, color,) . done()
        }
        #[inline]
        pub fn canvas_item_add_line_ex(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) -> ExCanvasItemAddLine < '_ > {
            ExCanvasItemAddLine::new(self, item, from, to, color,)
        }
        pub(crate) fn canvas_item_add_polyline_full(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray, width: f32, antialiased: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector2Array, PackedColorArray, f32, bool);
            let args = (item, points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_polyline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_polyline(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) {
            self.canvas_item_add_polyline_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_polyline_ex(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> ExCanvasItemAddPolyline < '_ > {
            ExCanvasItemAddPolyline::new(self, item, points, colors,)
        }
        pub(crate) fn canvas_item_add_multiline_full(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray, width: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector2Array, PackedColorArray, f32);
            let args = (item, points, colors, width,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_multiline(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) {
            self.canvas_item_add_multiline_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_multiline_ex(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> ExCanvasItemAddMultiline < '_ > {
            ExCanvasItemAddMultiline::new(self, item, points, colors,)
        }
        pub fn canvas_item_add_rect(&mut self, item: Rid, rect: Rect2, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Color);
            let args = (item, rect, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_add_circle(&mut self, item: Rid, pos: Vector2, radius: f32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2, f32, Color);
            let args = (item, pos, radius, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_add_texture_rect_full(&mut self, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rid, bool, Color, bool);
            let args = (item, rect, texture, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_texture_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_texture_rect(&mut self, item: Rid, rect: Rect2, texture: Rid,) {
            self.canvas_item_add_texture_rect_ex(item, rect, texture,) . done()
        }
        #[inline]
        pub fn canvas_item_add_texture_rect_ex(&mut self, item: Rid, rect: Rect2, texture: Rid,) -> ExCanvasItemAddTextureRect < '_ > {
            ExCanvasItemAddTextureRect::new(self, item, rect, texture,)
        }
        pub(crate) fn canvas_item_add_msdf_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rid, Rect2, Color, i32, f32, f32);
            let args = (item, rect, texture, src_rect, modulate, outline_size, px_range, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_msdf_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_msdf_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region_ex(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddMsdfTextureRectRegion < '_ > {
            ExCanvasItemAddMsdfTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        pub fn canvas_item_add_lcd_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rid, Rect2, Color);
            let args = (item, rect, texture, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_lcd_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_add_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rid, Rect2, Color, bool, bool);
            let args = (item, rect, texture, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_texture_rect_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_add_texture_rect_region_ex(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddTextureRectRegion < '_ > {
            ExCanvasItemAddTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        pub(crate) fn canvas_item_add_nine_patch_full(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::engine::rendering_server::NinePatchAxisMode, y_axis_mode: crate::engine::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rect2, Rect2, Rid, Vector2, Vector2, crate::engine::rendering_server::NinePatchAxisMode, crate::engine::rendering_server::NinePatchAxisMode, bool, Color);
            let args = (item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_nine_patch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_nine_patch(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) {
            self.canvas_item_add_nine_patch_ex(item, rect, source, texture, topleft, bottomright,) . done()
        }
        #[inline]
        pub fn canvas_item_add_nine_patch_ex(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> ExCanvasItemAddNinePatch < '_ > {
            ExCanvasItemAddNinePatch::new(self, item, rect, source, texture, topleft, bottomright,)
        }
        pub fn canvas_item_add_primitive(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector2Array, PackedColorArray, PackedVector2Array, Rid);
            let args = (item, points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_primitive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_add_polygon_full(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector2Array, PackedColorArray, PackedVector2Array, Rid);
            let args = (item, points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_polygon(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) {
            self.canvas_item_add_polygon_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_polygon_ex(&mut self, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> ExCanvasItemAddPolygon < '_ > {
            ExCanvasItemAddPolygon::new(self, item, points, colors,)
        }
        pub(crate) fn canvas_item_add_triangle_array_full(&mut self, item: Rid, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, bones: PackedInt32Array, weights: PackedFloat32Array, texture: Rid, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedInt32Array, PackedVector2Array, PackedColorArray, PackedVector2Array, PackedInt32Array, PackedFloat32Array, Rid, i32);
            let args = (item, indices, points, colors, uvs, bones, weights, texture, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_triangle_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_triangle_array(&mut self, item: Rid, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray,) {
            self.canvas_item_add_triangle_array_ex(item, indices, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_triangle_array_ex(&mut self, item: Rid, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray,) -> ExCanvasItemAddTriangleArray < '_ > {
            ExCanvasItemAddTriangleArray::new(self, item, indices, points, colors,)
        }
        pub(crate) fn canvas_item_add_mesh_full(&mut self, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Transform2D, Color, Rid);
            let args = (item, mesh, transform, modulate, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_mesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_mesh_ex(item, mesh,) . done()
        }
        #[inline]
        pub fn canvas_item_add_mesh_ex(&mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMesh < '_ > {
            ExCanvasItemAddMesh::new(self, item, mesh,)
        }
        pub(crate) fn canvas_item_add_multimesh_full(&mut self, item: Rid, mesh: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Rid);
            let args = (item, mesh, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_multimesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_multimesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_multimesh_ex(item, mesh,) . done()
        }
        #[inline]
        pub fn canvas_item_add_multimesh_ex(&mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMultimesh < '_ > {
            ExCanvasItemAddMultimesh::new(self, item, mesh,)
        }
        pub fn canvas_item_add_particles(&mut self, item: Rid, particles: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid, Rid);
            let args = (item, particles, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_particles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_add_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_add_clip_ignore(&mut self, item: Rid, ignore: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, ignore,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_clip_ignore", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_add_animation_slice_full(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f64, f64, f64, f64);
            let args = (item, animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_add_animation_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_add_animation_slice(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.canvas_item_add_animation_slice_ex(item, animation_length, slice_begin, slice_end,) . done()
        }
        #[inline]
        pub fn canvas_item_add_animation_slice_ex(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExCanvasItemAddAnimationSlice < '_ > {
            ExCanvasItemAddAnimationSlice::new(self, item, animation_length, slice_begin, slice_end,)
        }
        pub fn canvas_item_set_sort_children_by_y(&mut self, item: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_sort_children_by_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_z_index(&mut self, item: Rid, z_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (item, z_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_z_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_z_as_relative_to_parent(&mut self, item: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_z_as_relative_to_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_copy_to_backbuffer(&mut self, item: Rid, enabled: bool, rect: Rect2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, Rect2);
            let args = (item, enabled, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_copy_to_backbuffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_clear(&mut self, item: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_draw_index(&mut self, item: Rid, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (item, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_draw_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_material(&mut self, item: Rid, material: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (item, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_use_parent_material(&mut self, item: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_use_parent_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_item_set_visibility_notifier(&mut self, item: Rid, enable: bool, area: Rect2, enter_callable: Callable, exit_callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool, Rect2, Callable, Callable);
            let args = (item, enable, area, enter_callable, exit_callable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_visibility_notifier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn canvas_item_set_canvas_group_mode_full(&mut self, item: Rid, mode: crate::engine::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasGroupMode, f32, bool, f32, bool);
            let args = (item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_item_set_canvas_group_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn canvas_item_set_canvas_group_mode(&mut self, item: Rid, mode: crate::engine::rendering_server::CanvasGroupMode,) {
            self.canvas_item_set_canvas_group_mode_ex(item, mode,) . done()
        }
        #[inline]
        pub fn canvas_item_set_canvas_group_mode_ex(&mut self, item: Rid, mode: crate::engine::rendering_server::CanvasGroupMode,) -> ExCanvasItemSetCanvasGroupMode < '_ > {
            ExCanvasItemSetCanvasGroupMode::new(self, item, mode,)
        }
        pub fn canvas_light_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_attach_to_canvas(&mut self, light: Rid, canvas: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (light, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_attach_to_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_enabled(&mut self, light: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_texture_scale(&mut self, light: Rid, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (light, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_texture_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_transform(&mut self, light: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (light, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_texture(&mut self, light: Rid, texture: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_texture_offset(&mut self, light: Rid, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Vector2);
            let args = (light, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_texture_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_color(&mut self, light: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_height(&mut self, light: Rid, height: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (light, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_energy(&mut self, light: Rid, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (light, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_z_range(&mut self, light: Rid, min_z: i32, max_z: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32);
            let args = (light, min_z, max_z,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_z_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_layer_range(&mut self, light: Rid, min_layer: i32, max_layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32, i32);
            let args = (light, min_layer, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_layer_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_item_cull_mask(&mut self, light: Rid, mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_item_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_item_shadow_cull_mask(&mut self, light: Rid, mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_item_shadow_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_mode(&mut self, light: Rid, mode: crate::engine::rendering_server::CanvasLightMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasLightMode);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_shadow_enabled(&mut self, light: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_shadow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_shadow_filter(&mut self, light: Rid, filter: crate::engine::rendering_server::CanvasLightShadowFilter,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasLightShadowFilter);
            let args = (light, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_shadow_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_shadow_color(&mut self, light: Rid, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Color);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_shadow_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_shadow_smooth(&mut self, light: Rid, smooth: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, f32);
            let args = (light, smooth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_shadow_smooth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_set_blend_mode(&mut self, light: Rid, mode: crate::engine::rendering_server::CanvasLightBlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasLightBlendMode);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_attach_to_canvas(&mut self, occluder: Rid, canvas: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (occluder, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_attach_to_canvas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_set_enabled(&mut self, occluder: Rid, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (occluder, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_set_polygon(&mut self, occluder: Rid, polygon: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Rid);
            let args = (occluder, polygon,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_set_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_set_as_sdf_collision(&mut self, occluder: Rid, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, bool);
            let args = (occluder, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_set_as_sdf_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_set_transform(&mut self, occluder: Rid, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, Transform2D);
            let args = (occluder, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_light_occluder_set_light_mask(&mut self, occluder: Rid, mask: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, i32);
            let args = (occluder, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_light_occluder_set_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_occluder_polygon_create(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_occluder_polygon_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_occluder_polygon_set_shape(&mut self, occluder_polygon: Rid, shape: PackedVector2Array, closed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, PackedVector2Array, bool);
            let args = (occluder_polygon, shape, closed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_occluder_polygon_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_occluder_polygon_set_cull_mode(&mut self, occluder_polygon: Rid, mode: crate::engine::rendering_server::CanvasOccluderPolygonCullMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid, crate::engine::rendering_server::CanvasOccluderPolygonCullMode);
            let args = (occluder_polygon, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_occluder_polygon_set_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn canvas_set_shadow_texture_size(&mut self, size: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "canvas_set_shadow_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_add(&mut self, name: StringName, type_: crate::engine::rendering_server::GlobalShaderParameterType, default_value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, crate::engine::rendering_server::GlobalShaderParameterType, Variant);
            let args = (name, type_, default_value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_add", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_remove(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_remove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_get_list(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_get_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_set(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_set_override(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_set_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_get(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_shader_parameter_get_type(&self, name: StringName,) -> crate::engine::rendering_server::GlobalShaderParameterType {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_server::GlobalShaderParameterType >;
            type CallSig = (crate::engine::rendering_server::GlobalShaderParameterType, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_shader_parameter_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_frame_drawn_callback(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "request_frame_drawn_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_changed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rendering_info(&mut self, info: crate::engine::rendering_server::RenderingInfo,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, crate::engine::rendering_server::RenderingInfo);
            let args = (info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rendering_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_name(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_video_adapter_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_vendor(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_video_adapter_vendor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_type(&self,) -> crate::engine::rendering_device::DeviceType {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::DeviceType >;
            type CallSig = (crate::engine::rendering_device::DeviceType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_video_adapter_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_api_version(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_video_adapter_api_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_sphere_mesh(&mut self, latitudes: i32, longitudes: i32, radius: f32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32, i32, f32);
            let args = (latitudes, longitudes, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_sphere_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_test_cube(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_test_cube", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_test_texture(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_test_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_white_texture(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_white_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_boot_image_full(&mut self, image: Gd < crate::engine::Image >, color: Color, scale: bool, use_filter: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, Color, bool, bool);
            let args = (image, color, scale, use_filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_boot_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_boot_image(&mut self, image: Gd < crate::engine::Image >, color: Color, scale: bool,) {
            self.set_boot_image_ex(image, color, scale,) . done()
        }
        #[inline]
        pub fn set_boot_image_ex(&mut self, image: Gd < crate::engine::Image >, color: Color, scale: bool,) -> ExSetBootImage < '_ > {
            ExSetBootImage::new(self, image, color, scale,)
        }
        pub fn get_default_clear_color(&mut self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_clear_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_clear_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_clear_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_feature(&self, feature: crate::engine::rendering_server::Features,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::rendering_server::Features);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_os_feature(&self, feature: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_os_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_generate_wireframes(&mut self, generate: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (generate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_generate_wireframes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_render_loop_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_render_loop_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_loop_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_render_loop_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_setup_time_cpu(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frame_setup_time_cpu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_sync(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn force_draw_full(&mut self, swap_buffers: bool, frame_step: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, f64);
            let args = (swap_buffers, frame_step,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn force_draw(&mut self,) {
            self.force_draw_ex() . done()
        }
        #[inline]
        pub fn force_draw_ex(&mut self,) -> ExForceDraw < '_ > {
            ExForceDraw::new(self,)
        }
        pub fn get_rendering_device(&self,) -> Option < Gd < crate::engine::RenderingDevice > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RenderingDevice > >;
            type CallSig = (Option < Gd < crate::engine::RenderingDevice > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rendering_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_local_rendering_device(&self,) -> Option < Gd < crate::engine::RenderingDevice > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RenderingDevice > >;
            type CallSig = (Option < Gd < crate::engine::RenderingDevice > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_local_rendering_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn call_on_render_thread(&mut self, callable: Callable,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Callable);
            let args = (callable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "call_on_render_thread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const NO_INDEX_ARRAY: i32 = - 1i32;
        pub const ARRAY_WEIGHTS_SIZE: i32 = 4i32;
        pub const CANVAS_ITEM_Z_MIN: i32 = - 4096i32;
        pub const CANVAS_ITEM_Z_MAX: i32 = 4096i32;
        pub const MAX_GLOW_LEVELS: i32 = 7i32;
        pub const MAX_CURSORS: i32 = 8i32;
        pub const MAX_2D_DIRECTIONAL_LIGHTS: i32 = 8i32;
        pub const MATERIAL_RENDER_PRIORITY_MIN: i32 = - 128i32;
        pub const MATERIAL_RENDER_PRIORITY_MAX: i32 = 127i32;
        pub const ARRAY_CUSTOM_COUNT: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_POSITION: i32 = 1i32;
        pub const PARTICLES_EMIT_FLAG_ROTATION_SCALE: i32 = 2i32;
        pub const PARTICLES_EMIT_FLAG_VELOCITY: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_COLOR: i32 = 8i32;
        pub const PARTICLES_EMIT_FLAG_CUSTOM: i32 = 16i32;
        
    }
    impl crate::obj::GodotClass for RenderingServer {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RenderingServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for RenderingServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RenderingServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for RenderingServer {
        
    }
    impl std::ops::Deref for RenderingServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RenderingServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RenderingServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_rd_create_ex`][super::RenderingServer::texture_rd_create_ex]."]
#[must_use]
pub struct ExTextureRdCreate < 'a > {
    surround_object: &'a mut re_export::RenderingServer, rd_texture: Rid, layer_type: crate::engine::rendering_server::TextureLayeredType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureRdCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, rd_texture: Rid,) -> Self {
        Self {
            surround_object, rd_texture, layer_type: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn layer_type(self, value: crate::engine::rendering_server::TextureLayeredType) -> Self {
        Self {
            layer_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingServer::texture_rd_create_full(self.surround_object, self.rd_texture, self.layer_type,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_rd_texture_ex`][super::RenderingServer::texture_get_rd_texture_ex]."]
#[must_use]
pub struct ExTextureGetRdTexture < 'a > {
    surround_object: &'a re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureGetRdTexture < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, texture: Rid,) -> Self {
        Self {
            surround_object, texture, srgb: false,
        }
    }
    #[inline]
    pub fn srgb(self, value: bool) -> Self {
        Self {
            srgb: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingServer::texture_get_rd_texture_full(self.surround_object, self.texture, self.srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_native_handle_ex`][super::RenderingServer::texture_get_native_handle_ex]."]
#[must_use]
pub struct ExTextureGetNativeHandle < 'a > {
    surround_object: &'a re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureGetNativeHandle < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, texture: Rid,) -> Self {
        Self {
            surround_object, texture, srgb: false,
        }
    }
    #[inline]
    pub fn srgb(self, value: bool) -> Self {
        Self {
            srgb: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        re_export::RenderingServer::texture_get_native_handle_full(self.surround_object, self.texture, self.srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_set_default_texture_parameter_ex`][super::RenderingServer::shader_set_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderSetDefaultTextureParameter < 'a > {
    surround_object: &'a mut re_export::RenderingServer, shader: Rid, name: StringName, texture: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderSetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, shader: Rid, name: StringName, texture: Rid,) -> Self {
        Self {
            surround_object, shader, name, texture, index: 0i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::shader_set_default_texture_parameter_full(self.surround_object, self.shader, self.name, self.texture, self.index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_get_default_texture_parameter_ex`][super::RenderingServer::shader_get_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderGetDefaultTextureParameter < 'a > {
    surround_object: &'a re_export::RenderingServer, shader: Rid, name: StringName, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderGetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, shader: Rid, name: StringName,) -> Self {
        Self {
            surround_object, shader, name, index: 0i32,
        }
    }
    #[inline]
    pub fn index(self, value: i32) -> Self {
        Self {
            index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingServer::shader_get_default_texture_parameter_full(self.surround_object, self.shader, self.name, self.index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_create_from_surfaces_ex`][super::RenderingServer::mesh_create_from_surfaces_ex]."]
#[must_use]
pub struct ExMeshCreateFromSurfaces < 'a > {
    surround_object: &'a mut re_export::RenderingServer, surfaces: Array < Dictionary >, blend_shape_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMeshCreateFromSurfaces < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, surfaces: Array < Dictionary >,) -> Self {
        Self {
            surround_object, surfaces, blend_shape_count: 0i32,
        }
    }
    #[inline]
    pub fn blend_shape_count(self, value: i32) -> Self {
        Self {
            blend_shape_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::RenderingServer::mesh_create_from_surfaces_full(self.surround_object, self.surfaces, self.blend_shape_count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_add_surface_from_arrays_ex`][super::RenderingServer::mesh_add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExMeshAddSurfaceFromArrays < 'a > {
    surround_object: &'a mut re_export::RenderingServer, mesh: Rid, primitive: crate::engine::rendering_server::PrimitiveType, arrays: VariantArray, blend_shapes: VariantArray, lods: Dictionary, compress_format: crate::engine::rendering_server::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMeshAddSurfaceFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, mesh: Rid, primitive: crate::engine::rendering_server::PrimitiveType, arrays: VariantArray,) -> Self {
        Self {
            surround_object, mesh, primitive, arrays, blend_shapes: Array::new(), lods: Dictionary::new(), compress_format: crate::obj::EngineBitfield::from_ord(0),
        }
    }
    #[inline]
    pub fn blend_shapes(self, value: VariantArray) -> Self {
        Self {
            blend_shapes: value, .. self
        }
    }
    #[inline]
    pub fn lods(self, value: Dictionary) -> Self {
        Self {
            lods: value, .. self
        }
    }
    #[inline]
    pub fn compress_format(self, value: crate::engine::rendering_server::ArrayFormat) -> Self {
        Self {
            compress_format: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::mesh_add_surface_from_arrays_full(self.surround_object, self.mesh, self.primitive, self.arrays, self.blend_shapes, self.lods, self.compress_format,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::multimesh_allocate_data_ex`][super::RenderingServer::multimesh_allocate_data_ex]."]
#[must_use]
pub struct ExMultimeshAllocateData < 'a > {
    surround_object: &'a mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::engine::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMultimeshAllocateData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::engine::rendering_server::MultimeshTransformFormat,) -> Self {
        Self {
            surround_object, multimesh, instances, transform_format, color_format: false, custom_data_format: false,
        }
    }
    #[inline]
    pub fn color_format(self, value: bool) -> Self {
        Self {
            color_format: value, .. self
        }
    }
    #[inline]
    pub fn custom_data_format(self, value: bool) -> Self {
        Self {
            custom_data_format: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::multimesh_allocate_data_full(self.surround_object, self.multimesh, self.instances, self.transform_format, self.color_format, self.custom_data_format,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::skeleton_allocate_data_ex`][super::RenderingServer::skeleton_allocate_data_ex]."]
#[must_use]
pub struct ExSkeletonAllocateData < 'a > {
    surround_object: &'a mut re_export::RenderingServer, skeleton: Rid, bones: i32, is_2d_skeleton: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSkeletonAllocateData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, skeleton: Rid, bones: i32,) -> Self {
        Self {
            surround_object, skeleton, bones, is_2d_skeleton: false,
        }
    }
    #[inline]
    pub fn is_2d_skeleton(self, value: bool) -> Self {
        Self {
            is_2d_skeleton: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::skeleton_allocate_data_full(self.surround_object, self.skeleton, self.bones, self.is_2d_skeleton,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_attach_to_screen_ex`][super::RenderingServer::viewport_attach_to_screen_ex]."]
#[must_use]
pub struct ExViewportAttachToScreen < 'a > {
    surround_object: &'a mut re_export::RenderingServer, viewport: Rid, rect: Rect2, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExViewportAttachToScreen < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, viewport: Rid,) -> Self {
        Self {
            surround_object, viewport, rect: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _), screen: 0i32,
        }
    }
    #[inline]
    pub fn rect(self, value: Rect2) -> Self {
        Self {
            rect: value, .. self
        }
    }
    #[inline]
    pub fn screen(self, value: i32) -> Self {
        Self {
            screen: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::viewport_attach_to_screen_full(self.surround_object, self.viewport, self.rect, self.screen,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_set_positional_shadow_atlas_size_ex`][super::RenderingServer::viewport_set_positional_shadow_atlas_size_ex]."]
#[must_use]
pub struct ExViewportSetPositionalShadowAtlasSize < 'a > {
    surround_object: &'a mut re_export::RenderingServer, viewport: Rid, size: i32, use_16_bits: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExViewportSetPositionalShadowAtlasSize < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, viewport: Rid, size: i32,) -> Self {
        Self {
            surround_object, viewport, size, use_16_bits: false,
        }
    }
    #[inline]
    pub fn use_16_bits(self, value: bool) -> Self {
        Self {
            use_16_bits: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::viewport_set_positional_shadow_atlas_size_full(self.surround_object, self.viewport, self.size, self.use_16_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::environment_set_ambient_light_ex`][super::RenderingServer::environment_set_ambient_light_ex]."]
#[must_use]
pub struct ExEnvironmentSetAmbientLight < 'a > {
    surround_object: &'a mut re_export::RenderingServer, env: Rid, color: Color, ambient: crate::engine::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contibution: f32, reflection_source: crate::engine::rendering_server::EnvironmentReflectionSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEnvironmentSetAmbientLight < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, env: Rid, color: Color,) -> Self {
        Self {
            surround_object, env, color, ambient: crate::obj::EngineEnum::from_ord(0), energy: 1f32, sky_contibution: 0f32, reflection_source: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn ambient(self, value: crate::engine::rendering_server::EnvironmentAmbientSource) -> Self {
        Self {
            ambient: value, .. self
        }
    }
    #[inline]
    pub fn energy(self, value: f32) -> Self {
        Self {
            energy: value, .. self
        }
    }
    #[inline]
    pub fn sky_contibution(self, value: f32) -> Self {
        Self {
            sky_contibution: value, .. self
        }
    }
    #[inline]
    pub fn reflection_source(self, value: crate::engine::rendering_server::EnvironmentReflectionSource) -> Self {
        Self {
            reflection_source: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::environment_set_ambient_light_full(self.surround_object, self.env, self.color, self.ambient, self.energy, self.sky_contibution, self.reflection_source,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_aabb_ex`][super::RenderingServer::instances_cull_aabb_ex]."]
#[must_use]
pub struct ExInstancesCullAabb < 'a > {
    surround_object: &'a re_export::RenderingServer, aabb: Aabb, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullAabb < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, aabb: Aabb,) -> Self {
        Self {
            surround_object, aabb, scenario: Rid::Invalid,
        }
    }
    #[inline]
    pub fn scenario(self, value: Rid) -> Self {
        Self {
            scenario: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        re_export::RenderingServer::instances_cull_aabb_full(self.surround_object, self.aabb, self.scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_ray_ex`][super::RenderingServer::instances_cull_ray_ex]."]
#[must_use]
pub struct ExInstancesCullRay < 'a > {
    surround_object: &'a re_export::RenderingServer, from: Vector3, to: Vector3, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullRay < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, from: Vector3, to: Vector3,) -> Self {
        Self {
            surround_object, from, to, scenario: Rid::Invalid,
        }
    }
    #[inline]
    pub fn scenario(self, value: Rid) -> Self {
        Self {
            scenario: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        re_export::RenderingServer::instances_cull_ray_full(self.surround_object, self.from, self.to, self.scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_convex_ex`][super::RenderingServer::instances_cull_convex_ex]."]
#[must_use]
pub struct ExInstancesCullConvex < 'a > {
    surround_object: &'a re_export::RenderingServer, convex: Array < Plane >, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullConvex < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, convex: Array < Plane >,) -> Self {
        Self {
            surround_object, convex, scenario: Rid::Invalid,
        }
    }
    #[inline]
    pub fn scenario(self, value: Rid) -> Self {
        Self {
            scenario: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        re_export::RenderingServer::instances_cull_convex_full(self.surround_object, self.convex, self.scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_custom_rect_ex`][super::RenderingServer::canvas_item_set_custom_rect_ex]."]
#[must_use]
pub struct ExCanvasItemSetCustomRect < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, use_custom_rect: bool, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemSetCustomRect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, use_custom_rect: bool,) -> Self {
        Self {
            surround_object, item, use_custom_rect, rect: Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _),
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
        re_export::RenderingServer::canvas_item_set_custom_rect_full(self.surround_object, self.item, self.use_custom_rect, self.rect,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_line_ex`][super::RenderingServer::canvas_item_add_line_ex]."]
#[must_use]
pub struct ExCanvasItemAddLine < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddLine < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color,) -> Self {
        Self {
            surround_object, item, from, to, color, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_line_full(self.surround_object, self.item, self.from, self.to, self.color, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polyline_ex`][super::RenderingServer::canvas_item_add_polyline_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolyline < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, item, points, colors, width: - 1f32, antialiased: false,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, value: bool) -> Self {
        Self {
            antialiased: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_polyline_full(self.surround_object, self.item, self.points, self.colors, self.width, self.antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multiline_ex`][super::RenderingServer::canvas_item_add_multiline_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultiline < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray, width: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMultiline < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, item, points, colors, width: - 1f32,
        }
    }
    #[inline]
    pub fn width(self, value: f32) -> Self {
        Self {
            width: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_multiline_full(self.surround_object, self.item, self.points, self.colors, self.width,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_ex`][super::RenderingServer::canvas_item_add_texture_rect_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRect < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTextureRect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid,) -> Self {
        Self {
            surround_object, item, rect, texture, tile: false, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false,
        }
    }
    #[inline]
    pub fn tile(self, value: bool) -> Self {
        Self {
            tile: value, .. self
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
        re_export::RenderingServer::canvas_item_add_texture_rect_full(self.surround_object, self.item, self.rect, self.texture, self.tile, self.modulate, self.transpose,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_msdf_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddMsdfTextureRectRegion < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMsdfTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        Self {
            surround_object, item, rect, texture, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), outline_size: 0i32, px_range: 1f32, scale: 1f32,
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, value: i32) -> Self {
        Self {
            outline_size: value, .. self
        }
    }
    #[inline]
    pub fn px_range(self, value: f32) -> Self {
        Self {
            px_range: value, .. self
        }
    }
    #[inline]
    pub fn scale(self, value: f32) -> Self {
        Self {
            scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_msdf_texture_rect_region_full(self.surround_object, self.item, self.rect, self.texture, self.src_rect, self.modulate, self.outline_size, self.px_range, self.scale,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRectRegion < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        Self {
            surround_object, item, rect, texture, src_rect, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), transpose: false, clip_uv: true,
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
        re_export::RenderingServer::canvas_item_add_texture_rect_region_full(self.surround_object, self.item, self.rect, self.texture, self.src_rect, self.modulate, self.transpose, self.clip_uv,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_nine_patch_ex`][super::RenderingServer::canvas_item_add_nine_patch_ex]."]
#[must_use]
pub struct ExCanvasItemAddNinePatch < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::engine::rendering_server::NinePatchAxisMode, y_axis_mode: crate::engine::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddNinePatch < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> Self {
        Self {
            surround_object, item, rect, source, texture, topleft, bottomright, x_axis_mode: crate::obj::EngineEnum::from_ord(0), y_axis_mode: crate::obj::EngineEnum::from_ord(0), draw_center: true, modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _),
        }
    }
    #[inline]
    pub fn x_axis_mode(self, value: crate::engine::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            x_axis_mode: value, .. self
        }
    }
    #[inline]
    pub fn y_axis_mode(self, value: crate::engine::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            y_axis_mode: value, .. self
        }
    }
    #[inline]
    pub fn draw_center(self, value: bool) -> Self {
        Self {
            draw_center: value, .. self
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
        re_export::RenderingServer::canvas_item_add_nine_patch_full(self.surround_object, self.item, self.rect, self.source, self.texture, self.topleft, self.bottomright, self.x_axis_mode, self.y_axis_mode, self.draw_center, self.modulate,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polygon_ex`][super::RenderingServer::canvas_item_add_polygon_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolygon < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, item, points, colors, uvs: PackedVector2Array::new(), texture: Rid::Invalid,
        }
    }
    #[inline]
    pub fn uvs(self, value: PackedVector2Array) -> Self {
        Self {
            uvs: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Rid) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_polygon_full(self.surround_object, self.item, self.points, self.colors, self.uvs, self.texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_triangle_array_ex`][super::RenderingServer::canvas_item_add_triangle_array_ex]."]
#[must_use]
pub struct ExCanvasItemAddTriangleArray < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, bones: PackedInt32Array, weights: PackedFloat32Array, texture: Rid, count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTriangleArray < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray,) -> Self {
        Self {
            surround_object, item, indices, points, colors, uvs: PackedVector2Array::new(), bones: PackedInt32Array::new(), weights: PackedFloat32Array::new(), texture: Rid::Invalid, count: - 1i32,
        }
    }
    #[inline]
    pub fn uvs(self, value: PackedVector2Array) -> Self {
        Self {
            uvs: value, .. self
        }
    }
    #[inline]
    pub fn bones(self, value: PackedInt32Array) -> Self {
        Self {
            bones: value, .. self
        }
    }
    #[inline]
    pub fn weights(self, value: PackedFloat32Array) -> Self {
        Self {
            weights: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Rid) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn count(self, value: i32) -> Self {
        Self {
            count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_triangle_array_full(self.surround_object, self.item, self.indices, self.points, self.colors, self.uvs, self.bones, self.weights, self.texture, self.count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_mesh_ex`][super::RenderingServer::canvas_item_add_mesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMesh < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMesh < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        Self {
            surround_object, item, mesh, transform: Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _), modulate: Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _), texture: Rid::Invalid,
        }
    }
    #[inline]
    pub fn transform(self, value: Transform2D) -> Self {
        Self {
            transform: value, .. self
        }
    }
    #[inline]
    pub fn modulate(self, value: Color) -> Self {
        Self {
            modulate: value, .. self
        }
    }
    #[inline]
    pub fn texture(self, value: Rid) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_mesh_full(self.surround_object, self.item, self.mesh, self.transform, self.modulate, self.texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multimesh_ex`][super::RenderingServer::canvas_item_add_multimesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultimesh < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMultimesh < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        Self {
            surround_object, item, mesh, texture: Rid::Invalid,
        }
    }
    #[inline]
    pub fn texture(self, value: Rid) -> Self {
        Self {
            texture: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_multimesh_full(self.surround_object, self.item, self.mesh, self.texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_animation_slice_ex`][super::RenderingServer::canvas_item_add_animation_slice_ex]."]
#[must_use]
pub struct ExCanvasItemAddAnimationSlice < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddAnimationSlice < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        Self {
            surround_object, item, animation_length, slice_begin, slice_end, offset: 0f64,
        }
    }
    #[inline]
    pub fn offset(self, value: f64) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_add_animation_slice_full(self.surround_object, self.item, self.animation_length, self.slice_begin, self.slice_end, self.offset,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_canvas_group_mode_ex`][super::RenderingServer::canvas_item_set_canvas_group_mode_ex]."]
#[must_use]
pub struct ExCanvasItemSetCanvasGroupMode < 'a > {
    surround_object: &'a mut re_export::RenderingServer, item: Rid, mode: crate::engine::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemSetCanvasGroupMode < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mode: crate::engine::rendering_server::CanvasGroupMode,) -> Self {
        Self {
            surround_object, item, mode, clear_margin: 5f32, fit_empty: false, fit_margin: 0f32, blur_mipmaps: false,
        }
    }
    #[inline]
    pub fn clear_margin(self, value: f32) -> Self {
        Self {
            clear_margin: value, .. self
        }
    }
    #[inline]
    pub fn fit_empty(self, value: bool) -> Self {
        Self {
            fit_empty: value, .. self
        }
    }
    #[inline]
    pub fn fit_margin(self, value: f32) -> Self {
        Self {
            fit_margin: value, .. self
        }
    }
    #[inline]
    pub fn blur_mipmaps(self, value: bool) -> Self {
        Self {
            blur_mipmaps: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::canvas_item_set_canvas_group_mode_full(self.surround_object, self.item, self.mode, self.clear_margin, self.fit_empty, self.fit_margin, self.blur_mipmaps,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::set_boot_image_ex`][super::RenderingServer::set_boot_image_ex]."]
#[must_use]
pub struct ExSetBootImage < 'a > {
    surround_object: &'a mut re_export::RenderingServer, image: Gd < crate::engine::Image >, color: Color, scale: bool, use_filter: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBootImage < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, image: Gd < crate::engine::Image >, color: Color, scale: bool,) -> Self {
        Self {
            surround_object, image, color, scale, use_filter: true,
        }
    }
    #[inline]
    pub fn use_filter(self, value: bool) -> Self {
        Self {
            use_filter: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::set_boot_image_full(self.surround_object, self.image, self.color, self.scale, self.use_filter,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::force_draw_ex`][super::RenderingServer::force_draw_ex]."]
#[must_use]
pub struct ExForceDraw < 'a > {
    surround_object: &'a mut re_export::RenderingServer, swap_buffers: bool, frame_step: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExForceDraw < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer,) -> Self {
        Self {
            surround_object, swap_buffers: true, frame_step: 0f64,
        }
    }
    #[inline]
    pub fn swap_buffers(self, value: bool) -> Self {
        Self {
            swap_buffers: value, .. self
        }
    }
    #[inline]
    pub fn frame_step(self, value: f64) -> Self {
        Self {
            frame_step: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RenderingServer::force_draw_full(self.surround_object, self.swap_buffers, self.frame_step,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureLayeredType {
    ord: i32
}
impl TextureLayeredType {
    pub const TEXTURE_LAYERED_2D_ARRAY: Self = Self {
        ord: 0i32
    };
    pub const TEXTURE_LAYERED_CUBEMAP: Self = Self {
        ord: 1i32
    };
    pub const TEXTURE_LAYERED_CUBEMAP_ARRAY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TextureLayeredType {
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
impl crate::builtin::meta::GodotConvert for TextureLayeredType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TextureLayeredType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TextureLayeredType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CubeMapLayer {
    ord: i32
}
impl CubeMapLayer {
    pub const CUBEMAP_LAYER_LEFT: Self = Self {
        ord: 0i32
    };
    pub const CUBEMAP_LAYER_RIGHT: Self = Self {
        ord: 1i32
    };
    pub const CUBEMAP_LAYER_BOTTOM: Self = Self {
        ord: 2i32
    };
    pub const CUBEMAP_LAYER_TOP: Self = Self {
        ord: 3i32
    };
    pub const CUBEMAP_LAYER_FRONT: Self = Self {
        ord: 4i32
    };
    pub const CUBEMAP_LAYER_BACK: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for CubeMapLayer {
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
impl crate::builtin::meta::GodotConvert for CubeMapLayer {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CubeMapLayer {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CubeMapLayer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShaderMode {
    ord: i32
}
impl ShaderMode {
    pub const SHADER_SPATIAL: Self = Self {
        ord: 0i32
    };
    pub const SHADER_CANVAS_ITEM: Self = Self {
        ord: 1i32
    };
    pub const SHADER_PARTICLES: Self = Self {
        ord: 2i32
    };
    pub const SHADER_SKY: Self = Self {
        ord: 3i32
    };
    pub const SHADER_FOG: Self = Self {
        ord: 4i32
    };
    pub const SHADER_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for ShaderMode {
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
impl crate::obj::IndexEnum for ShaderMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for ShaderMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShaderMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShaderMode {
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
    pub const ARRAY_FLAG_FORMAT_VERSION_BASE: Self = Self {
        ord: 35u64
    };
    pub const ARRAY_FLAG_FORMAT_VERSION_SHIFT: Self = Self {
        ord: 35u64
    };
    pub const ARRAY_FLAG_FORMAT_VERSION_1: Self = Self {
        ord: 0u64
    };
    pub const ARRAY_FLAG_FORMAT_VERSION_2: Self = Self {
        ord: 34359738368u64
    };
    pub const ARRAY_FLAG_FORMAT_CURRENT_VERSION: Self = Self {
        ord: 34359738368u64
    };
    pub const ARRAY_FLAG_FORMAT_VERSION_MASK: Self = Self {
        ord: 255u64
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
    pub const PRIMITIVE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for PrimitiveType {
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
impl crate::obj::IndexEnum for PrimitiveType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MultimeshTransformFormat {
    ord: i32
}
impl MultimeshTransformFormat {
    pub const MULTIMESH_TRANSFORM_2D: Self = Self {
        ord: 0i32
    };
    pub const MULTIMESH_TRANSFORM_3D: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for MultimeshTransformFormat {
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
impl crate::builtin::meta::GodotConvert for MultimeshTransformFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MultimeshTransformFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MultimeshTransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightProjectorFilter {
    ord: i32
}
impl LightProjectorFilter {
    pub const LIGHT_PROJECTOR_FILTER_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS: Self = Self {
        ord: 2i32
    };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 4i32
    };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for LightProjectorFilter {
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
impl crate::builtin::meta::GodotConvert for LightProjectorFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightProjectorFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightProjectorFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightType {
    ord: i32
}
impl LightType {
    pub const LIGHT_DIRECTIONAL: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_OMNI: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_SPOT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LightType {
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
impl crate::builtin::meta::GodotConvert for LightType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightParam {
    ord: i32
}
impl LightParam {
    pub const LIGHT_PARAM_ENERGY: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_PARAM_INDIRECT_ENERGY: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY: Self = Self {
        ord: 2i32
    };
    pub const LIGHT_PARAM_SPECULAR: Self = Self {
        ord: 3i32
    };
    pub const LIGHT_PARAM_RANGE: Self = Self {
        ord: 4i32
    };
    pub const LIGHT_PARAM_SIZE: Self = Self {
        ord: 5i32
    };
    pub const LIGHT_PARAM_ATTENUATION: Self = Self {
        ord: 6i32
    };
    pub const LIGHT_PARAM_SPOT_ANGLE: Self = Self {
        ord: 7i32
    };
    pub const LIGHT_PARAM_SPOT_ATTENUATION: Self = Self {
        ord: 8i32
    };
    pub const LIGHT_PARAM_SHADOW_MAX_DISTANCE: Self = Self {
        ord: 9i32
    };
    pub const LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET: Self = Self {
        ord: 10i32
    };
    pub const LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET: Self = Self {
        ord: 11i32
    };
    pub const LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET: Self = Self {
        ord: 12i32
    };
    pub const LIGHT_PARAM_SHADOW_FADE_START: Self = Self {
        ord: 13i32
    };
    pub const LIGHT_PARAM_SHADOW_NORMAL_BIAS: Self = Self {
        ord: 14i32
    };
    pub const LIGHT_PARAM_SHADOW_BIAS: Self = Self {
        ord: 15i32
    };
    pub const LIGHT_PARAM_SHADOW_PANCAKE_SIZE: Self = Self {
        ord: 16i32
    };
    pub const LIGHT_PARAM_SHADOW_OPACITY: Self = Self {
        ord: 17i32
    };
    pub const LIGHT_PARAM_SHADOW_BLUR: Self = Self {
        ord: 18i32
    };
    pub const LIGHT_PARAM_TRANSMITTANCE_BIAS: Self = Self {
        ord: 19i32
    };
    pub const LIGHT_PARAM_INTENSITY: Self = Self {
        ord: 20i32
    };
    pub const LIGHT_PARAM_MAX: Self = Self {
        ord: 21i32
    };
    
}
impl crate::obj::EngineEnum for LightParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for LightParam {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::builtin::meta::GodotConvert for LightParam {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightParam {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightBakeMode {
    ord: i32
}
impl LightBakeMode {
    pub const LIGHT_BAKE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_BAKE_STATIC: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_BAKE_DYNAMIC: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LightBakeMode {
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
impl crate::builtin::meta::GodotConvert for LightBakeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightBakeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightBakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightOmniShadowMode {
    ord: i32
}
impl LightOmniShadowMode {
    pub const LIGHT_OMNI_SHADOW_DUAL_PARABOLOID: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_OMNI_SHADOW_CUBE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for LightOmniShadowMode {
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
impl crate::builtin::meta::GodotConvert for LightOmniShadowMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightOmniShadowMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightOmniShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightDirectionalShadowMode {
    ord: i32
}
impl LightDirectionalShadowMode {
    pub const LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LightDirectionalShadowMode {
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
impl crate::builtin::meta::GodotConvert for LightDirectionalShadowMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightDirectionalShadowMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightDirectionalShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LightDirectionalSkyMode {
    ord: i32
}
impl LightDirectionalSkyMode {
    pub const LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY: Self = Self {
        ord: 0i32
    };
    pub const LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY: Self = Self {
        ord: 1i32
    };
    pub const LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for LightDirectionalSkyMode {
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
impl crate::builtin::meta::GodotConvert for LightDirectionalSkyMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LightDirectionalSkyMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LightDirectionalSkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShadowQuality {
    ord: i32
}
impl ShadowQuality {
    pub const SHADOW_QUALITY_HARD: Self = Self {
        ord: 0i32
    };
    pub const SHADOW_QUALITY_SOFT_VERY_LOW: Self = Self {
        ord: 1i32
    };
    pub const SHADOW_QUALITY_SOFT_LOW: Self = Self {
        ord: 2i32
    };
    pub const SHADOW_QUALITY_SOFT_MEDIUM: Self = Self {
        ord: 3i32
    };
    pub const SHADOW_QUALITY_SOFT_HIGH: Self = Self {
        ord: 4i32
    };
    pub const SHADOW_QUALITY_SOFT_ULTRA: Self = Self {
        ord: 5i32
    };
    pub const SHADOW_QUALITY_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for ShadowQuality {
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
impl crate::obj::IndexEnum for ShadowQuality {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for ShadowQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShadowQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShadowQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReflectionProbeUpdateMode {
    ord: i32
}
impl ReflectionProbeUpdateMode {
    pub const REFLECTION_PROBE_UPDATE_ONCE: Self = Self {
        ord: 0i32
    };
    pub const REFLECTION_PROBE_UPDATE_ALWAYS: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for ReflectionProbeUpdateMode {
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
impl crate::builtin::meta::GodotConvert for ReflectionProbeUpdateMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ReflectionProbeUpdateMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ReflectionProbeUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReflectionProbeAmbientMode {
    ord: i32
}
impl ReflectionProbeAmbientMode {
    pub const REFLECTION_PROBE_AMBIENT_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const REFLECTION_PROBE_AMBIENT_ENVIRONMENT: Self = Self {
        ord: 1i32
    };
    pub const REFLECTION_PROBE_AMBIENT_COLOR: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ReflectionProbeAmbientMode {
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
impl crate::builtin::meta::GodotConvert for ReflectionProbeAmbientMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ReflectionProbeAmbientMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ReflectionProbeAmbientMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecalTexture {
    ord: i32
}
impl DecalTexture {
    pub const DECAL_TEXTURE_ALBEDO: Self = Self {
        ord: 0i32
    };
    pub const DECAL_TEXTURE_NORMAL: Self = Self {
        ord: 1i32
    };
    pub const DECAL_TEXTURE_ORM: Self = Self {
        ord: 2i32
    };
    pub const DECAL_TEXTURE_EMISSION: Self = Self {
        ord: 3i32
    };
    pub const DECAL_TEXTURE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for DecalTexture {
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
impl crate::obj::IndexEnum for DecalTexture {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for DecalTexture {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DecalTexture {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DecalTexture {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecalFilter {
    ord: i32
}
impl DecalFilter {
    pub const DECAL_FILTER_NEAREST: Self = Self {
        ord: 0i32
    };
    pub const DECAL_FILTER_LINEAR: Self = Self {
        ord: 1i32
    };
    pub const DECAL_FILTER_NEAREST_MIPMAPS: Self = Self {
        ord: 2i32
    };
    pub const DECAL_FILTER_LINEAR_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 4i32
    };
    pub const DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for DecalFilter {
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
impl crate::builtin::meta::GodotConvert for DecalFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DecalFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DecalFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VoxelGIQuality {
    ord: i32
}
impl VoxelGIQuality {
    pub const VOXEL_GI_QUALITY_LOW: Self = Self {
        ord: 0i32
    };
    pub const VOXEL_GI_QUALITY_HIGH: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for VoxelGIQuality {
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
impl crate::builtin::meta::GodotConvert for VoxelGIQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VoxelGIQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VoxelGIQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticlesMode {
    ord: i32
}
impl ParticlesMode {
    pub const PARTICLES_MODE_2D: Self = Self {
        ord: 0i32
    };
    pub const PARTICLES_MODE_3D: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for ParticlesMode {
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
impl crate::builtin::meta::GodotConvert for ParticlesMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticlesMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticlesMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticlesTransformAlign {
    ord: i32
}
impl ParticlesTransformAlign {
    pub const PARTICLES_TRANSFORM_ALIGN_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD: Self = Self {
        ord: 1i32
    };
    pub const PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY: Self = Self {
        ord: 2i32
    };
    pub const PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ParticlesTransformAlign {
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
impl crate::builtin::meta::GodotConvert for ParticlesTransformAlign {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticlesTransformAlign {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticlesTransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticlesDrawOrder {
    ord: i32
}
impl ParticlesDrawOrder {
    pub const PARTICLES_DRAW_ORDER_INDEX: Self = Self {
        ord: 0i32
    };
    pub const PARTICLES_DRAW_ORDER_LIFETIME: Self = Self {
        ord: 1i32
    };
    pub const PARTICLES_DRAW_ORDER_REVERSE_LIFETIME: Self = Self {
        ord: 2i32
    };
    pub const PARTICLES_DRAW_ORDER_VIEW_DEPTH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ParticlesDrawOrder {
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
impl crate::builtin::meta::GodotConvert for ParticlesDrawOrder {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticlesDrawOrder {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticlesDrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticlesCollisionType {
    ord: i32
}
impl ParticlesCollisionType {
    pub const PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT: Self = Self {
        ord: 0i32
    };
    pub const PARTICLES_COLLISION_TYPE_BOX_ATTRACT: Self = Self {
        ord: 1i32
    };
    pub const PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT: Self = Self {
        ord: 2i32
    };
    pub const PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE: Self = Self {
        ord: 3i32
    };
    pub const PARTICLES_COLLISION_TYPE_BOX_COLLIDE: Self = Self {
        ord: 4i32
    };
    pub const PARTICLES_COLLISION_TYPE_SDF_COLLIDE: Self = Self {
        ord: 5i32
    };
    pub const PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for ParticlesCollisionType {
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
impl crate::builtin::meta::GodotConvert for ParticlesCollisionType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticlesCollisionType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticlesCollisionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParticlesCollisionHeightfieldResolution {
    ord: i32
}
impl ParticlesCollisionHeightfieldResolution {
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256: Self = Self {
        ord: 0i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512: Self = Self {
        ord: 1i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024: Self = Self {
        ord: 2i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048: Self = Self {
        ord: 3i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096: Self = Self {
        ord: 4i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192: Self = Self {
        ord: 5i32
    };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for ParticlesCollisionHeightfieldResolution {
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
impl crate::obj::IndexEnum for ParticlesCollisionHeightfieldResolution {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for ParticlesCollisionHeightfieldResolution {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ParticlesCollisionHeightfieldResolution {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ParticlesCollisionHeightfieldResolution {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FogVolumeShape {
    ord: i32
}
impl FogVolumeShape {
    pub const FOG_VOLUME_SHAPE_ELLIPSOID: Self = Self {
        ord: 0i32
    };
    pub const FOG_VOLUME_SHAPE_CONE: Self = Self {
        ord: 1i32
    };
    pub const FOG_VOLUME_SHAPE_CYLINDER: Self = Self {
        ord: 2i32
    };
    pub const FOG_VOLUME_SHAPE_BOX: Self = Self {
        ord: 3i32
    };
    pub const FOG_VOLUME_SHAPE_WORLD: Self = Self {
        ord: 4i32
    };
    pub const FOG_VOLUME_SHAPE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for FogVolumeShape {
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
impl crate::obj::IndexEnum for FogVolumeShape {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for FogVolumeShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FogVolumeShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FogVolumeShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportScaling3DMode {
    ord: i32
}
impl ViewportScaling3DMode {
    pub const VIEWPORT_SCALING_3D_MODE_BILINEAR: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_SCALING_3D_MODE_FSR: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_SCALING_3D_MODE_FSR2: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_SCALING_3D_MODE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ViewportScaling3DMode {
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
impl crate::obj::IndexEnum for ViewportScaling3DMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportScaling3DMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportScaling3DMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportScaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportUpdateMode {
    ord: i32
}
impl ViewportUpdateMode {
    pub const VIEWPORT_UPDATE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_UPDATE_ONCE: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_UPDATE_WHEN_VISIBLE: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE: Self = Self {
        ord: 3i32
    };
    pub const VIEWPORT_UPDATE_ALWAYS: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ViewportUpdateMode {
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
impl crate::builtin::meta::GodotConvert for ViewportUpdateMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportUpdateMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportClearMode {
    ord: i32
}
impl ViewportClearMode {
    pub const VIEWPORT_CLEAR_ALWAYS: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_CLEAR_NEVER: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_CLEAR_ONLY_NEXT_FRAME: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ViewportClearMode {
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
impl crate::builtin::meta::GodotConvert for ViewportClearMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportClearMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportClearMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportEnvironmentMode {
    ord: i32
}
impl ViewportEnvironmentMode {
    pub const VIEWPORT_ENVIRONMENT_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_ENVIRONMENT_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_ENVIRONMENT_INHERIT: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_ENVIRONMENT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ViewportEnvironmentMode {
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
impl crate::obj::IndexEnum for ViewportEnvironmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportEnvironmentMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportEnvironmentMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportEnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportSDFOversize {
    ord: i32
}
impl ViewportSDFOversize {
    pub const VIEWPORT_SDF_OVERSIZE_100_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_SDF_OVERSIZE_120_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_SDF_OVERSIZE_150_PERCENT: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_SDF_OVERSIZE_200_PERCENT: Self = Self {
        ord: 3i32
    };
    pub const VIEWPORT_SDF_OVERSIZE_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ViewportSDFOversize {
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
impl crate::obj::IndexEnum for ViewportSDFOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportSDFOversize {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportSDFOversize {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportSDFOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportSDFScale {
    ord: i32
}
impl ViewportSDFScale {
    pub const VIEWPORT_SDF_SCALE_100_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_SDF_SCALE_50_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_SDF_SCALE_25_PERCENT: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_SDF_SCALE_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ViewportSDFScale {
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
impl crate::obj::IndexEnum for ViewportSDFScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportSDFScale {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportSDFScale {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportSDFScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportMSAA {
    ord: i32
}
impl ViewportMSAA {
    pub const VIEWPORT_MSAA_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_MSAA_2X: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_MSAA_4X: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_MSAA_8X: Self = Self {
        ord: 3i32
    };
    pub const VIEWPORT_MSAA_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for ViewportMSAA {
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
impl crate::obj::IndexEnum for ViewportMSAA {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportMSAA {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportMSAA {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportMSAA {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportScreenSpaceAA {
    ord: i32
}
impl ViewportScreenSpaceAA {
    pub const VIEWPORT_SCREEN_SPACE_AA_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_SCREEN_SPACE_AA_FXAA: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_SCREEN_SPACE_AA_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ViewportScreenSpaceAA {
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
impl crate::obj::IndexEnum for ViewportScreenSpaceAA {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportScreenSpaceAA {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportScreenSpaceAA {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportScreenSpaceAA {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportOcclusionCullingBuildQuality {
    ord: i32
}
impl ViewportOcclusionCullingBuildQuality {
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ViewportOcclusionCullingBuildQuality {
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
impl crate::builtin::meta::GodotConvert for ViewportOcclusionCullingBuildQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportOcclusionCullingBuildQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportOcclusionCullingBuildQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportRenderInfo {
    ord: i32
}
impl ViewportRenderInfo {
    pub const VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_RENDER_INFO_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ViewportRenderInfo {
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
impl crate::obj::IndexEnum for ViewportRenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportRenderInfo {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportRenderInfo {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportRenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportRenderInfoType {
    ord: i32
}
impl ViewportRenderInfoType {
    pub const VIEWPORT_RENDER_INFO_TYPE_VISIBLE: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_RENDER_INFO_TYPE_SHADOW: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_RENDER_INFO_TYPE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ViewportRenderInfoType {
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
impl crate::obj::IndexEnum for ViewportRenderInfoType {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportRenderInfoType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportRenderInfoType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportRenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportDebugDraw {
    ord: i32
}
impl ViewportDebugDraw {
    pub const VIEWPORT_DEBUG_DRAW_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_DEBUG_DRAW_UNSHADED: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_DEBUG_DRAW_LIGHTING: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_DEBUG_DRAW_OVERDRAW: Self = Self {
        ord: 3i32
    };
    pub const VIEWPORT_DEBUG_DRAW_WIREFRAME: Self = Self {
        ord: 4i32
    };
    pub const VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER: Self = Self {
        ord: 5i32
    };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO: Self = Self {
        ord: 6i32
    };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING: Self = Self {
        ord: 7i32
    };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION: Self = Self {
        ord: 8i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS: Self = Self {
        ord: 9i32
    };
    pub const VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS: Self = Self {
        ord: 10i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE: Self = Self {
        ord: 11i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SSAO: Self = Self {
        ord: 12i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SSIL: Self = Self {
        ord: 13i32
    };
    pub const VIEWPORT_DEBUG_DRAW_PSSM_SPLITS: Self = Self {
        ord: 14i32
    };
    pub const VIEWPORT_DEBUG_DRAW_DECAL_ATLAS: Self = Self {
        ord: 15i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SDFGI: Self = Self {
        ord: 16i32
    };
    pub const VIEWPORT_DEBUG_DRAW_SDFGI_PROBES: Self = Self {
        ord: 17i32
    };
    pub const VIEWPORT_DEBUG_DRAW_GI_BUFFER: Self = Self {
        ord: 18i32
    };
    pub const VIEWPORT_DEBUG_DRAW_DISABLE_LOD: Self = Self {
        ord: 19i32
    };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS: Self = Self {
        ord: 20i32
    };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS: Self = Self {
        ord: 21i32
    };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS: Self = Self {
        ord: 22i32
    };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES: Self = Self {
        ord: 23i32
    };
    pub const VIEWPORT_DEBUG_DRAW_OCCLUDERS: Self = Self {
        ord: 24i32
    };
    pub const VIEWPORT_DEBUG_DRAW_MOTION_VECTORS: Self = Self {
        ord: 25i32
    };
    pub const VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER: Self = Self {
        ord: 26i32
    };
    
}
impl crate::obj::EngineEnum for ViewportDebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ViewportDebugDraw {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportDebugDraw {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportDebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ViewportVRSMode {
    ord: i32
}
impl ViewportVRSMode {
    pub const VIEWPORT_VRS_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VIEWPORT_VRS_TEXTURE: Self = Self {
        ord: 1i32
    };
    pub const VIEWPORT_VRS_XR: Self = Self {
        ord: 2i32
    };
    pub const VIEWPORT_VRS_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ViewportVRSMode {
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
impl crate::obj::IndexEnum for ViewportVRSMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for ViewportVRSMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ViewportVRSMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ViewportVRSMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SkyMode {
    ord: i32
}
impl SkyMode {
    pub const SKY_MODE_AUTOMATIC: Self = Self {
        ord: 0i32
    };
    pub const SKY_MODE_QUALITY: Self = Self {
        ord: 1i32
    };
    pub const SKY_MODE_INCREMENTAL: Self = Self {
        ord: 2i32
    };
    pub const SKY_MODE_REALTIME: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for SkyMode {
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
impl crate::builtin::meta::GodotConvert for SkyMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SkyMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentBG {
    ord: i32
}
impl EnvironmentBG {
    pub const ENV_BG_CLEAR_COLOR: Self = Self {
        ord: 0i32
    };
    pub const ENV_BG_COLOR: Self = Self {
        ord: 1i32
    };
    pub const ENV_BG_SKY: Self = Self {
        ord: 2i32
    };
    pub const ENV_BG_CANVAS: Self = Self {
        ord: 3i32
    };
    pub const ENV_BG_KEEP: Self = Self {
        ord: 4i32
    };
    pub const ENV_BG_CAMERA_FEED: Self = Self {
        ord: 5i32
    };
    pub const ENV_BG_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentBG {
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
impl crate::obj::IndexEnum for EnvironmentBG {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for EnvironmentBG {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentBG {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentBG {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentAmbientSource {
    ord: i32
}
impl EnvironmentAmbientSource {
    pub const ENV_AMBIENT_SOURCE_BG: Self = Self {
        ord: 0i32
    };
    pub const ENV_AMBIENT_SOURCE_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const ENV_AMBIENT_SOURCE_COLOR: Self = Self {
        ord: 2i32
    };
    pub const ENV_AMBIENT_SOURCE_SKY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentAmbientSource {
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
impl crate::builtin::meta::GodotConvert for EnvironmentAmbientSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentAmbientSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentAmbientSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentReflectionSource {
    ord: i32
}
impl EnvironmentReflectionSource {
    pub const ENV_REFLECTION_SOURCE_BG: Self = Self {
        ord: 0i32
    };
    pub const ENV_REFLECTION_SOURCE_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const ENV_REFLECTION_SOURCE_SKY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentReflectionSource {
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
impl crate::builtin::meta::GodotConvert for EnvironmentReflectionSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentReflectionSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentReflectionSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentGlowBlendMode {
    ord: i32
}
impl EnvironmentGlowBlendMode {
    pub const ENV_GLOW_BLEND_MODE_ADDITIVE: Self = Self {
        ord: 0i32
    };
    pub const ENV_GLOW_BLEND_MODE_SCREEN: Self = Self {
        ord: 1i32
    };
    pub const ENV_GLOW_BLEND_MODE_SOFTLIGHT: Self = Self {
        ord: 2i32
    };
    pub const ENV_GLOW_BLEND_MODE_REPLACE: Self = Self {
        ord: 3i32
    };
    pub const ENV_GLOW_BLEND_MODE_MIX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentGlowBlendMode {
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
impl crate::builtin::meta::GodotConvert for EnvironmentGlowBlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentGlowBlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentGlowBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentToneMapper {
    ord: i32
}
impl EnvironmentToneMapper {
    pub const ENV_TONE_MAPPER_LINEAR: Self = Self {
        ord: 0i32
    };
    pub const ENV_TONE_MAPPER_REINHARD: Self = Self {
        ord: 1i32
    };
    pub const ENV_TONE_MAPPER_FILMIC: Self = Self {
        ord: 2i32
    };
    pub const ENV_TONE_MAPPER_ACES: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentToneMapper {
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
impl crate::builtin::meta::GodotConvert for EnvironmentToneMapper {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentToneMapper {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentToneMapper {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSSRRoughnessQuality {
    ord: i32
}
impl EnvironmentSSRRoughnessQuality {
    pub const ENV_SSR_ROUGHNESS_QUALITY_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const ENV_SSR_ROUGHNESS_QUALITY_LOW: Self = Self {
        ord: 1i32
    };
    pub const ENV_SSR_ROUGHNESS_QUALITY_MEDIUM: Self = Self {
        ord: 2i32
    };
    pub const ENV_SSR_ROUGHNESS_QUALITY_HIGH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSSRRoughnessQuality {
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
impl crate::builtin::meta::GodotConvert for EnvironmentSSRRoughnessQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSSRRoughnessQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSSRRoughnessQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSSAOQuality {
    ord: i32
}
impl EnvironmentSSAOQuality {
    pub const ENV_SSAO_QUALITY_VERY_LOW: Self = Self {
        ord: 0i32
    };
    pub const ENV_SSAO_QUALITY_LOW: Self = Self {
        ord: 1i32
    };
    pub const ENV_SSAO_QUALITY_MEDIUM: Self = Self {
        ord: 2i32
    };
    pub const ENV_SSAO_QUALITY_HIGH: Self = Self {
        ord: 3i32
    };
    pub const ENV_SSAO_QUALITY_ULTRA: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSSAOQuality {
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
impl crate::builtin::meta::GodotConvert for EnvironmentSSAOQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSSAOQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSSAOQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSSILQuality {
    ord: i32
}
impl EnvironmentSSILQuality {
    pub const ENV_SSIL_QUALITY_VERY_LOW: Self = Self {
        ord: 0i32
    };
    pub const ENV_SSIL_QUALITY_LOW: Self = Self {
        ord: 1i32
    };
    pub const ENV_SSIL_QUALITY_MEDIUM: Self = Self {
        ord: 2i32
    };
    pub const ENV_SSIL_QUALITY_HIGH: Self = Self {
        ord: 3i32
    };
    pub const ENV_SSIL_QUALITY_ULTRA: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSSILQuality {
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
impl crate::builtin::meta::GodotConvert for EnvironmentSSILQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSSILQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSSILQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSDFGIYScale {
    ord: i32
}
impl EnvironmentSDFGIYScale {
    pub const ENV_SDFGI_Y_SCALE_50_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const ENV_SDFGI_Y_SCALE_75_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const ENV_SDFGI_Y_SCALE_100_PERCENT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSDFGIYScale {
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
impl crate::builtin::meta::GodotConvert for EnvironmentSDFGIYScale {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSDFGIYScale {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSDFGIYScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSDFGIRayCount {
    ord: i32
}
impl EnvironmentSDFGIRayCount {
    pub const ENV_SDFGI_RAY_COUNT_4: Self = Self {
        ord: 0i32
    };
    pub const ENV_SDFGI_RAY_COUNT_8: Self = Self {
        ord: 1i32
    };
    pub const ENV_SDFGI_RAY_COUNT_16: Self = Self {
        ord: 2i32
    };
    pub const ENV_SDFGI_RAY_COUNT_32: Self = Self {
        ord: 3i32
    };
    pub const ENV_SDFGI_RAY_COUNT_64: Self = Self {
        ord: 4i32
    };
    pub const ENV_SDFGI_RAY_COUNT_96: Self = Self {
        ord: 5i32
    };
    pub const ENV_SDFGI_RAY_COUNT_128: Self = Self {
        ord: 6i32
    };
    pub const ENV_SDFGI_RAY_COUNT_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSDFGIRayCount {
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
impl crate::obj::IndexEnum for EnvironmentSDFGIRayCount {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for EnvironmentSDFGIRayCount {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSDFGIRayCount {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSDFGIRayCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSDFGIFramesToConverge {
    ord: i32
}
impl EnvironmentSDFGIFramesToConverge {
    pub const ENV_SDFGI_CONVERGE_IN_5_FRAMES: Self = Self {
        ord: 0i32
    };
    pub const ENV_SDFGI_CONVERGE_IN_10_FRAMES: Self = Self {
        ord: 1i32
    };
    pub const ENV_SDFGI_CONVERGE_IN_15_FRAMES: Self = Self {
        ord: 2i32
    };
    pub const ENV_SDFGI_CONVERGE_IN_20_FRAMES: Self = Self {
        ord: 3i32
    };
    pub const ENV_SDFGI_CONVERGE_IN_25_FRAMES: Self = Self {
        ord: 4i32
    };
    pub const ENV_SDFGI_CONVERGE_IN_30_FRAMES: Self = Self {
        ord: 5i32
    };
    pub const ENV_SDFGI_CONVERGE_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSDFGIFramesToConverge {
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
impl crate::obj::IndexEnum for EnvironmentSDFGIFramesToConverge {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for EnvironmentSDFGIFramesToConverge {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSDFGIFramesToConverge {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSDFGIFramesToConverge {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentSDFGIFramesToUpdateLight {
    ord: i32
}
impl EnvironmentSDFGIFramesToUpdateLight {
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME: Self = Self {
        ord: 0i32
    };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES: Self = Self {
        ord: 1i32
    };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES: Self = Self {
        ord: 2i32
    };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES: Self = Self {
        ord: 3i32
    };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES: Self = Self {
        ord: 4i32
    };
    pub const ENV_SDFGI_UPDATE_LIGHT_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentSDFGIFramesToUpdateLight {
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
impl crate::obj::IndexEnum for EnvironmentSDFGIFramesToUpdateLight {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for EnvironmentSDFGIFramesToUpdateLight {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentSDFGIFramesToUpdateLight {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentSDFGIFramesToUpdateLight {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SubSurfaceScatteringQuality {
    ord: i32
}
impl SubSurfaceScatteringQuality {
    pub const SUB_SURFACE_SCATTERING_QUALITY_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SUB_SURFACE_SCATTERING_QUALITY_LOW: Self = Self {
        ord: 1i32
    };
    pub const SUB_SURFACE_SCATTERING_QUALITY_MEDIUM: Self = Self {
        ord: 2i32
    };
    pub const SUB_SURFACE_SCATTERING_QUALITY_HIGH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for SubSurfaceScatteringQuality {
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
impl crate::builtin::meta::GodotConvert for SubSurfaceScatteringQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SubSurfaceScatteringQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SubSurfaceScatteringQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DOFBokehShape {
    ord: i32
}
impl DOFBokehShape {
    pub const DOF_BOKEH_BOX: Self = Self {
        ord: 0i32
    };
    pub const DOF_BOKEH_HEXAGON: Self = Self {
        ord: 1i32
    };
    pub const DOF_BOKEH_CIRCLE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DOFBokehShape {
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
impl crate::builtin::meta::GodotConvert for DOFBokehShape {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DOFBokehShape {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DOFBokehShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DOFBlurQuality {
    ord: i32
}
impl DOFBlurQuality {
    pub const DOF_BLUR_QUALITY_VERY_LOW: Self = Self {
        ord: 0i32
    };
    pub const DOF_BLUR_QUALITY_LOW: Self = Self {
        ord: 1i32
    };
    pub const DOF_BLUR_QUALITY_MEDIUM: Self = Self {
        ord: 2i32
    };
    pub const DOF_BLUR_QUALITY_HIGH: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for DOFBlurQuality {
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
impl crate::builtin::meta::GodotConvert for DOFBlurQuality {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DOFBlurQuality {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DOFBlurQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InstanceType {
    ord: i32
}
impl InstanceType {
    pub const INSTANCE_NONE: Self = Self {
        ord: 0i32
    };
    pub const INSTANCE_MESH: Self = Self {
        ord: 1i32
    };
    pub const INSTANCE_MULTIMESH: Self = Self {
        ord: 2i32
    };
    pub const INSTANCE_PARTICLES: Self = Self {
        ord: 3i32
    };
    pub const INSTANCE_PARTICLES_COLLISION: Self = Self {
        ord: 4i32
    };
    pub const INSTANCE_LIGHT: Self = Self {
        ord: 5i32
    };
    pub const INSTANCE_REFLECTION_PROBE: Self = Self {
        ord: 6i32
    };
    pub const INSTANCE_DECAL: Self = Self {
        ord: 7i32
    };
    pub const INSTANCE_VOXEL_GI: Self = Self {
        ord: 8i32
    };
    pub const INSTANCE_LIGHTMAP: Self = Self {
        ord: 9i32
    };
    pub const INSTANCE_OCCLUDER: Self = Self {
        ord: 10i32
    };
    pub const INSTANCE_VISIBLITY_NOTIFIER: Self = Self {
        ord: 11i32
    };
    pub const INSTANCE_FOG_VOLUME: Self = Self {
        ord: 12i32
    };
    pub const INSTANCE_MAX: Self = Self {
        ord: 13i32
    };
    pub const INSTANCE_GEOMETRY_MASK: Self = Self {
        ord: 14i32
    };
    
}
impl crate::obj::EngineEnum for InstanceType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for InstanceType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InstanceType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InstanceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InstanceFlags {
    ord: i32
}
impl InstanceFlags {
    pub const INSTANCE_FLAG_USE_BAKED_LIGHT: Self = Self {
        ord: 0i32
    };
    pub const INSTANCE_FLAG_USE_DYNAMIC_GI: Self = Self {
        ord: 1i32
    };
    pub const INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE: Self = Self {
        ord: 2i32
    };
    pub const INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING: Self = Self {
        ord: 3i32
    };
    pub const INSTANCE_FLAG_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for InstanceFlags {
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
impl crate::obj::IndexEnum for InstanceFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for InstanceFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for InstanceFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for InstanceFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    pub const SHADOW_CASTING_SETTING_OFF: Self = Self {
        ord: 0i32
    };
    pub const SHADOW_CASTING_SETTING_ON: Self = Self {
        ord: 1i32
    };
    pub const SHADOW_CASTING_SETTING_DOUBLE_SIDED: Self = Self {
        ord: 2i32
    };
    pub const SHADOW_CASTING_SETTING_SHADOWS_ONLY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
impl crate::builtin::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ShadowCastingSetting {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    pub const VISIBILITY_RANGE_FADE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const VISIBILITY_RANGE_FADE_SELF: Self = Self {
        ord: 1i32
    };
    pub const VISIBILITY_RANGE_FADE_DEPENDENCIES: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
impl crate::builtin::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for VisibilityRangeFadeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BakeChannels {
    ord: i32
}
impl BakeChannels {
    pub const BAKE_CHANNEL_ALBEDO_ALPHA: Self = Self {
        ord: 0i32
    };
    pub const BAKE_CHANNEL_NORMAL: Self = Self {
        ord: 1i32
    };
    pub const BAKE_CHANNEL_ORM: Self = Self {
        ord: 2i32
    };
    pub const BAKE_CHANNEL_EMISSION: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for BakeChannels {
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
impl crate::builtin::meta::GodotConvert for BakeChannels {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BakeChannels {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BakeChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasTextureChannel {
    ord: i32
}
impl CanvasTextureChannel {
    pub const CANVAS_TEXTURE_CHANNEL_DIFFUSE: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_TEXTURE_CHANNEL_NORMAL: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_TEXTURE_CHANNEL_SPECULAR: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CanvasTextureChannel {
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
impl crate::builtin::meta::GodotConvert for CanvasTextureChannel {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasTextureChannel {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasTextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NinePatchAxisMode {
    ord: i32
}
impl NinePatchAxisMode {
    pub const NINE_PATCH_STRETCH: Self = Self {
        ord: 0i32
    };
    pub const NINE_PATCH_TILE: Self = Self {
        ord: 1i32
    };
    pub const NINE_PATCH_TILE_FIT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for NinePatchAxisMode {
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
impl crate::builtin::meta::GodotConvert for NinePatchAxisMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for NinePatchAxisMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for NinePatchAxisMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasItemTextureFilter {
    ord: i32
}
impl CanvasItemTextureFilter {
    pub const CANVAS_ITEM_TEXTURE_FILTER_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR: Self = Self {
        ord: 2i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self {
        ord: 3i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self {
        ord: 4i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 5i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self {
        ord: 6i32
    };
    pub const CANVAS_ITEM_TEXTURE_FILTER_MAX: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for CanvasItemTextureFilter {
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
impl crate::obj::IndexEnum for CanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::builtin::meta::GodotConvert for CanvasItemTextureFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasItemTextureFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasItemTextureRepeat {
    ord: i32
}
impl CanvasItemTextureRepeat {
    pub const CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_ENABLED: Self = Self {
        ord: 2i32
    };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_MIRROR: Self = Self {
        ord: 3i32
    };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_MAX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for CanvasItemTextureRepeat {
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
impl crate::obj::IndexEnum for CanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::builtin::meta::GodotConvert for CanvasItemTextureRepeat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasItemTextureRepeat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasGroupMode {
    ord: i32
}
impl CanvasGroupMode {
    pub const CANVAS_GROUP_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_GROUP_MODE_CLIP_ONLY: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_GROUP_MODE_CLIP_AND_DRAW: Self = Self {
        ord: 2i32
    };
    pub const CANVAS_GROUP_MODE_TRANSPARENT: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for CanvasGroupMode {
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
impl crate::builtin::meta::GodotConvert for CanvasGroupMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasGroupMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasGroupMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasLightMode {
    ord: i32
}
impl CanvasLightMode {
    pub const CANVAS_LIGHT_MODE_POINT: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_LIGHT_MODE_DIRECTIONAL: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for CanvasLightMode {
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
impl crate::builtin::meta::GodotConvert for CanvasLightMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasLightMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasLightMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasLightBlendMode {
    ord: i32
}
impl CanvasLightBlendMode {
    pub const CANVAS_LIGHT_BLEND_MODE_ADD: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_LIGHT_BLEND_MODE_SUB: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_LIGHT_BLEND_MODE_MIX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CanvasLightBlendMode {
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
impl crate::builtin::meta::GodotConvert for CanvasLightBlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasLightBlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasLightBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasLightShadowFilter {
    ord: i32
}
impl CanvasLightShadowFilter {
    pub const CANVAS_LIGHT_FILTER_NONE: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_LIGHT_FILTER_PCF5: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_LIGHT_FILTER_PCF13: Self = Self {
        ord: 2i32
    };
    pub const CANVAS_LIGHT_FILTER_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for CanvasLightShadowFilter {
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
impl crate::obj::IndexEnum for CanvasLightShadowFilter {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for CanvasLightShadowFilter {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasLightShadowFilter {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasLightShadowFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanvasOccluderPolygonCullMode {
    ord: i32
}
impl CanvasOccluderPolygonCullMode {
    pub const CANVAS_OCCLUDER_POLYGON_CULL_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE: Self = Self {
        ord: 1i32
    };
    pub const CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CanvasOccluderPolygonCullMode {
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
impl crate::builtin::meta::GodotConvert for CanvasOccluderPolygonCullMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CanvasOccluderPolygonCullMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CanvasOccluderPolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GlobalShaderParameterType {
    ord: i32
}
impl GlobalShaderParameterType {
    pub const GLOBAL_VAR_TYPE_BOOL: Self = Self {
        ord: 0i32
    };
    pub const GLOBAL_VAR_TYPE_BVEC2: Self = Self {
        ord: 1i32
    };
    pub const GLOBAL_VAR_TYPE_BVEC3: Self = Self {
        ord: 2i32
    };
    pub const GLOBAL_VAR_TYPE_BVEC4: Self = Self {
        ord: 3i32
    };
    pub const GLOBAL_VAR_TYPE_INT: Self = Self {
        ord: 4i32
    };
    pub const GLOBAL_VAR_TYPE_IVEC2: Self = Self {
        ord: 5i32
    };
    pub const GLOBAL_VAR_TYPE_IVEC3: Self = Self {
        ord: 6i32
    };
    pub const GLOBAL_VAR_TYPE_IVEC4: Self = Self {
        ord: 7i32
    };
    pub const GLOBAL_VAR_TYPE_RECT2I: Self = Self {
        ord: 8i32
    };
    pub const GLOBAL_VAR_TYPE_UINT: Self = Self {
        ord: 9i32
    };
    pub const GLOBAL_VAR_TYPE_UVEC2: Self = Self {
        ord: 10i32
    };
    pub const GLOBAL_VAR_TYPE_UVEC3: Self = Self {
        ord: 11i32
    };
    pub const GLOBAL_VAR_TYPE_UVEC4: Self = Self {
        ord: 12i32
    };
    pub const GLOBAL_VAR_TYPE_FLOAT: Self = Self {
        ord: 13i32
    };
    pub const GLOBAL_VAR_TYPE_VEC2: Self = Self {
        ord: 14i32
    };
    pub const GLOBAL_VAR_TYPE_VEC3: Self = Self {
        ord: 15i32
    };
    pub const GLOBAL_VAR_TYPE_VEC4: Self = Self {
        ord: 16i32
    };
    pub const GLOBAL_VAR_TYPE_COLOR: Self = Self {
        ord: 17i32
    };
    pub const GLOBAL_VAR_TYPE_RECT2: Self = Self {
        ord: 18i32
    };
    pub const GLOBAL_VAR_TYPE_MAT2: Self = Self {
        ord: 19i32
    };
    pub const GLOBAL_VAR_TYPE_MAT3: Self = Self {
        ord: 20i32
    };
    pub const GLOBAL_VAR_TYPE_MAT4: Self = Self {
        ord: 21i32
    };
    pub const GLOBAL_VAR_TYPE_TRANSFORM_2D: Self = Self {
        ord: 22i32
    };
    pub const GLOBAL_VAR_TYPE_TRANSFORM: Self = Self {
        ord: 23i32
    };
    pub const GLOBAL_VAR_TYPE_SAMPLER2D: Self = Self {
        ord: 24i32
    };
    pub const GLOBAL_VAR_TYPE_SAMPLER2DARRAY: Self = Self {
        ord: 25i32
    };
    pub const GLOBAL_VAR_TYPE_SAMPLER3D: Self = Self {
        ord: 26i32
    };
    pub const GLOBAL_VAR_TYPE_SAMPLERCUBE: Self = Self {
        ord: 27i32
    };
    pub const GLOBAL_VAR_TYPE_MAX: Self = Self {
        ord: 28i32
    };
    
}
impl crate::obj::EngineEnum for GlobalShaderParameterType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for GlobalShaderParameterType {
    const ENUMERATOR_COUNT: usize = 28usize;
    
}
impl crate::builtin::meta::GodotConvert for GlobalShaderParameterType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GlobalShaderParameterType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GlobalShaderParameterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderingInfo {
    ord: i32
}
impl RenderingInfo {
    pub const RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME: Self = Self {
        ord: 0i32
    };
    pub const RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME: Self = Self {
        ord: 1i32
    };
    pub const RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME: Self = Self {
        ord: 2i32
    };
    pub const RENDERING_INFO_TEXTURE_MEM_USED: Self = Self {
        ord: 3i32
    };
    pub const RENDERING_INFO_BUFFER_MEM_USED: Self = Self {
        ord: 4i32
    };
    pub const RENDERING_INFO_VIDEO_MEM_USED: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for RenderingInfo {
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
impl crate::builtin::meta::GodotConvert for RenderingInfo {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for RenderingInfo {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for RenderingInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Features {
    ord: i32
}
impl Features {
    pub const FEATURE_SHADERS: Self = Self {
        ord: 0i32
    };
    pub const FEATURE_MULTITHREADED: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for Features {
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
impl crate::builtin::meta::GodotConvert for Features {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Features {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Features {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}