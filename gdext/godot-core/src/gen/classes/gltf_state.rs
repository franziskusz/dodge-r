#![doc = "Sidecar module for class [`GltfState`][crate::engine::GltfState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFState` enums](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFState.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`IGltfState`][crate::engine::IGltfState]: virtual methods\n\n\nSee also [Godot docs for `GLTFState`](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfState`][crate::engine::GltfState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFState` methods](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfState: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfState {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_used_extension(&mut self, extension_name: GString, required: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (extension_name, required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_used_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_json(&mut self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_json", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_json(&mut self, json: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (json,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_json", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_major_version(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_major_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_major_version(&mut self, major_version: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (major_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_major_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minor_version(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_minor_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_minor_version(&mut self, minor_version: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (minor_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_minor_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_copyright(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_copyright", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_copyright(&mut self, copyright: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (copyright,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_copyright", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glb_data(&mut self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glb_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glb_data(&mut self, glb_data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedByteArray);
            let args = (glb_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glb_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_named_skin_binds(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_named_skin_binds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_named_skin_binds(&mut self, use_named_skin_binds: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (use_named_skin_binds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_named_skin_binds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_nodes(&mut self,) -> Array < Gd < crate::engine::GltfNode > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfNode > > >;
            type CallSig = (Array < Gd < crate::engine::GltfNode > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_nodes(&mut self, nodes: Array < Gd < crate::engine::GltfNode > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfNode > >);
            let args = (nodes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffers(&mut self,) -> Array < PackedByteArray > {
            type RetMarshal = PtrcallReturnT < Array < PackedByteArray > >;
            type CallSig = (Array < PackedByteArray >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffers(&mut self, buffers: Array < PackedByteArray >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < PackedByteArray >);
            let args = (buffers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_buffers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer_views(&mut self,) -> Array < Gd < crate::engine::GltfBufferView > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfBufferView > > >;
            type CallSig = (Array < Gd < crate::engine::GltfBufferView > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffer_views", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer_views(&mut self, buffer_views: Array < Gd < crate::engine::GltfBufferView > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfBufferView > >);
            let args = (buffer_views,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_buffer_views", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accessors(&mut self,) -> Array < Gd < crate::engine::GltfAccessor > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfAccessor > > >;
            type CallSig = (Array < Gd < crate::engine::GltfAccessor > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_accessors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accessors(&mut self, accessors: Array < Gd < crate::engine::GltfAccessor > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfAccessor > >);
            let args = (accessors,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_accessors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meshes(&mut self,) -> Array < Gd < crate::engine::GltfMesh > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfMesh > > >;
            type CallSig = (Array < Gd < crate::engine::GltfMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meshes(&mut self, meshes: Array < Gd < crate::engine::GltfMesh > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfMesh > >);
            let args = (meshes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_players_count(&mut self, idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_players_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_player(&mut self, idx: i32,) -> Option < Gd < crate::engine::AnimationPlayer > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationPlayer > >;
            type CallSig = (Option < Gd < crate::engine::AnimationPlayer > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_player", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_materials(&mut self,) -> Array < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Material > > >;
            type CallSig = (Array < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_materials", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_materials(&mut self, materials: Array < Gd < crate::engine::Material > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::Material > >);
            let args = (materials,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_materials", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_name(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_name(&mut self, scene_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (scene_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scene_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_path(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_base_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_base_path(&mut self, base_path: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (base_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_base_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_filename(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_filename(&mut self, filename: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (filename,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_nodes(&mut self,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_nodes(&mut self, root_nodes: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedInt32Array);
            let args = (root_nodes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_textures(&mut self,) -> Array < Gd < crate::engine::GltfTexture > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfTexture > > >;
            type CallSig = (Array < Gd < crate::engine::GltfTexture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_textures(&mut self, textures: Array < Gd < crate::engine::GltfTexture > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfTexture > >);
            let args = (textures,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_samplers(&mut self,) -> Array < Gd < crate::engine::GltfTextureSampler > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfTextureSampler > > >;
            type CallSig = (Array < Gd < crate::engine::GltfTextureSampler > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_samplers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_samplers(&mut self, texture_samplers: Array < Gd < crate::engine::GltfTextureSampler > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfTextureSampler > >);
            let args = (texture_samplers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_samplers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_images(&mut self,) -> Array < Gd < crate::engine::Texture2D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Texture2D > > >;
            type CallSig = (Array < Gd < crate::engine::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_images", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_images(&mut self, images: Array < Gd < crate::engine::Texture2D > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::Texture2D > >);
            let args = (images,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_images", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skins(&mut self,) -> Array < Gd < crate::engine::GltfSkin > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfSkin > > >;
            type CallSig = (Array < Gd < crate::engine::GltfSkin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skins(&mut self, skins: Array < Gd < crate::engine::GltfSkin > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfSkin > >);
            let args = (skins,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cameras(&mut self,) -> Array < Gd < crate::engine::GltfCamera > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfCamera > > >;
            type CallSig = (Array < Gd < crate::engine::GltfCamera > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cameras", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cameras(&mut self, cameras: Array < Gd < crate::engine::GltfCamera > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfCamera > >);
            let args = (cameras,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cameras", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lights(&mut self,) -> Array < Gd < crate::engine::GltfLight > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfLight > > >;
            type CallSig = (Array < Gd < crate::engine::GltfLight > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lights(&mut self, lights: Array < Gd < crate::engine::GltfLight > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfLight > >);
            let args = (lights,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_names(&mut self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_names(&mut self, unique_names: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (unique_names,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_animation_names(&mut self,) -> Array < GString > {
            type RetMarshal = PtrcallReturnT < Array < GString > >;
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unique_animation_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_animation_names(&mut self, unique_animation_names: Array < GString >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < GString >);
            let args = (unique_animation_names,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unique_animation_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeletons(&mut self,) -> Array < Gd < crate::engine::GltfSkeleton > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfSkeleton > > >;
            type CallSig = (Array < Gd < crate::engine::GltfSkeleton > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skeletons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeletons(&mut self, skeletons: Array < Gd < crate::engine::GltfSkeleton > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfSkeleton > >);
            let args = (skeletons,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skeletons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_create_animations(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_create_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_create_animations(&mut self, create_animations: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (create_animations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_create_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animations(&mut self,) -> Array < Gd < crate::engine::GltfAnimation > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::GltfAnimation > > >;
            type CallSig = (Array < Gd < crate::engine::GltfAnimation > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animations(&mut self, animations: Array < Gd < crate::engine::GltfAnimation > >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Gd < crate::engine::GltfAnimation > >);
            let args = (animations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_node(&mut self, idx: i32,) -> Option < Gd < crate::engine::Node > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Node > >;
            type CallSig = (Option < Gd < crate::engine::Node > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scene_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_index(&mut self, scene_node: Gd < crate::engine::Node >,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, Gd < crate::engine::Node >);
            let args = (scene_node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_node_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_additional_data(&mut self, extension_name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (extension_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_additional_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_additional_data(&mut self, extension_name: StringName, additional_data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (extension_name, additional_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_additional_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handle_binary_image(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_handle_binary_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_binary_image(&mut self, method: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_handle_binary_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const HANDLE_BINARY_DISCARD_TEXTURES: i32 = 0i32;
        pub const HANDLE_BINARY_EXTRACT_TEXTURES: i32 = 1i32;
        pub const HANDLE_BINARY_EMBED_AS_BASISU: i32 = 2i32;
        pub const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED: i32 = 3i32;
        
    }
    impl crate::obj::GodotClass for GltfState {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"GLTFState\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for GltfState {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for GltfState {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for GltfState {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for GltfState {
        
    }
    impl crate::obj::ExportableObject for GltfState {
        
    }
    impl crate::obj::cap::GodotDefault for GltfState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfState {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_GltfState {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::GltfState > for $Class {
                
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