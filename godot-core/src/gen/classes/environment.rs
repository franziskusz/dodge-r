#![doc = "Sidecar module for class [`Environment`][crate::engine::Environment].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Environment` enums](https://docs.godotengine.org/en/stable/classes/class_environment.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Environment.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`environment`][crate::engine::environment]: sidecar module with related enum/flag types\n* [`IEnvironment`][crate::engine::IEnvironment]: virtual methods\n\n\nSee also [Godot docs for `Environment`](https://docs.godotengine.org/en/stable/classes/class_environment.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Environment {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Environment`][crate::engine::Environment].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Environment` methods](https://docs.godotengine.org/en/stable/classes/class_environment.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEnvironment: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Environment {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_background(&mut self, mode: crate::engine::environment::BGMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::BGMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_background(&self,) -> crate::engine::environment::BGMode {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::BGMode >;
            type CallSig = (crate::engine::environment::BGMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sky(&mut self, sky: Gd < crate::engine::Sky >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Sky >);
            let args = (sky,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sky(&self,) -> Option < Gd < crate::engine::Sky > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Sky > >;
            type CallSig = (Option < Gd < crate::engine::Sky > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sky_custom_fov(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sky_custom_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sky_custom_fov(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sky_custom_fov", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sky_rotation(&mut self, euler_radians: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sky_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sky_rotation(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sky_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bg_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bg_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bg_energy_multiplier(&mut self, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bg_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bg_energy_multiplier(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bg_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bg_intensity(&mut self, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bg_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bg_intensity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bg_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_max_layer(&mut self, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_canvas_max_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_max_layer(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_canvas_max_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_camera_feed_id(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_camera_feed_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_feed_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_feed_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ambient_light_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ambient_light_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ambient_light_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ambient_light_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ambient_source(&mut self, source: crate::engine::environment::AmbientSource,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::AmbientSource);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ambient_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ambient_source(&self,) -> crate::engine::environment::AmbientSource {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::AmbientSource >;
            type CallSig = (crate::engine::environment::AmbientSource,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ambient_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ambient_light_energy(&mut self, energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ambient_light_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ambient_light_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ambient_light_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ambient_light_sky_contribution(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ambient_light_sky_contribution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ambient_light_sky_contribution(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ambient_light_sky_contribution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reflection_source(&mut self, source: crate::engine::environment::ReflectionSource,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::ReflectionSource);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_reflection_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reflection_source(&self,) -> crate::engine::environment::ReflectionSource {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::ReflectionSource >;
            type CallSig = (crate::engine::environment::ReflectionSource,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_reflection_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tonemapper(&mut self, mode: crate::engine::environment::ToneMapper,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::ToneMapper);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tonemapper", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tonemapper(&self,) -> crate::engine::environment::ToneMapper {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::ToneMapper >;
            type CallSig = (crate::engine::environment::ToneMapper,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tonemapper", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tonemap_exposure(&mut self, exposure: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (exposure,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tonemap_exposure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tonemap_exposure(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tonemap_exposure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tonemap_white(&mut self, white: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (white,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tonemap_white", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tonemap_white(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tonemap_white", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssr_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssr_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ssr_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ssr_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssr_max_steps(&mut self, max_steps: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssr_max_steps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssr_max_steps(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssr_max_steps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssr_fade_in(&mut self, fade_in: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (fade_in,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssr_fade_in", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssr_fade_in(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssr_fade_in", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssr_fade_out(&mut self, fade_out: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (fade_out,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssr_fade_out", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssr_fade_out(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssr_fade_out", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssr_depth_tolerance(&mut self, depth_tolerance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (depth_tolerance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssr_depth_tolerance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssr_depth_tolerance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssr_depth_tolerance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ssao_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ssao_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_intensity(&mut self, intensity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_intensity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_power(&mut self, power: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_power", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_power(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_power", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_detail(&mut self, detail: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (detail,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_detail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_detail(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_detail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_horizon(&mut self, horizon: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (horizon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_horizon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_horizon(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_horizon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_sharpness(&mut self, sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_sharpness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_direct_light_affect(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_direct_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_direct_light_affect(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_direct_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssao_ao_channel_affect(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssao_ao_channel_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssao_ao_channel_affect(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssao_ao_channel_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssil_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssil_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ssil_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ssil_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssil_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssil_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssil_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssil_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssil_intensity(&mut self, intensity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssil_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssil_intensity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssil_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssil_sharpness(&mut self, sharpness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssil_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssil_sharpness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssil_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ssil_normal_rejection(&mut self, normal_rejection: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (normal_rejection,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ssil_normal_rejection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ssil_normal_rejection(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ssil_normal_rejection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sdfgi_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sdfgi_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_cascades(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_cascades", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_cascades(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_cascades", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_min_cell_size(&mut self, size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_min_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_min_cell_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_min_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_max_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_max_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_cascade0_distance(&mut self, distance: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_cascade0_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_cascade0_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_cascade0_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_y_scale(&mut self, scale: crate::engine::environment::SDFGIYScale,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::SDFGIYScale);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_y_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_y_scale(&self,) -> crate::engine::environment::SDFGIYScale {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::SDFGIYScale >;
            type CallSig = (crate::engine::environment::SDFGIYScale,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_y_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_use_occlusion(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_use_occlusion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sdfgi_using_occlusion(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sdfgi_using_occlusion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_bounce_feedback(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_bounce_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_bounce_feedback(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_bounce_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_read_sky_light(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_read_sky_light", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sdfgi_reading_sky_light(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sdfgi_reading_sky_light", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_energy(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_normal_bias(&mut self, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_normal_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_normal_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_normal_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdfgi_probe_bias(&mut self, bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sdfgi_probe_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdfgi_probe_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sdfgi_probe_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_glow_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_glow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_level(&mut self, idx: i32, intensity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (idx, intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_level(&self, idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_normalized(&mut self, normalize: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_glow_normalized(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_glow_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_intensity(&mut self, intensity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_intensity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_strength(&mut self, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_mix(&mut self, mix: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (mix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_mix(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_bloom(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_bloom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_bloom(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_bloom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_blend_mode(&mut self, mode: crate::engine::environment::GlowBlendMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::environment::GlowBlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_blend_mode(&self,) -> crate::engine::environment::GlowBlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::environment::GlowBlendMode >;
            type CallSig = (crate::engine::environment::GlowBlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_hdr_bleed_threshold(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_hdr_bleed_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_hdr_bleed_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_hdr_bleed_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_hdr_bleed_scale(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_hdr_bleed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_hdr_bleed_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_hdr_bleed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_hdr_luminance_cap(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_hdr_luminance_cap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_hdr_luminance_cap(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_hdr_luminance_cap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_map_strength(&mut self, strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_map_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_map_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_map_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glow_map(&mut self, mode: Gd < crate::engine::Texture >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture >);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_glow_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glow_map(&self,) -> Option < Gd < crate::engine::Texture > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture > >;
            type CallSig = (Option < Gd < crate::engine::Texture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_glow_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fog_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_fog_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_light_color(&mut self, light_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (light_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_light_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_light_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_light_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_light_energy(&mut self, light_energy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (light_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_light_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_light_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_light_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_sun_scatter(&mut self, sun_scatter: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sun_scatter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_sun_scatter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_sun_scatter(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_sun_scatter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_density(&mut self, density: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_density(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_height(&mut self, height: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_height(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_height_density(&mut self, height_density: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (height_density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_height_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_height_density(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_height_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_aerial_perspective(&mut self, aerial_perspective: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (aerial_perspective,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_aerial_perspective", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_aerial_perspective(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_aerial_perspective", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fog_sky_affect(&mut self, sky_affect: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sky_affect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fog_sky_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fog_sky_affect(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fog_sky_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_volumetric_fog_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_volumetric_fog_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_emission(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_emission(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_albedo(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_albedo(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_density(&mut self, density: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_density(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_emission_energy(&mut self, begin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (begin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_emission_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_emission_energy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_emission_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_anisotropy(&mut self, anisotropy: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (anisotropy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_anisotropy(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_length(&mut self, length: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_detail_spread(&mut self, detail_spread: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (detail_spread,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_detail_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_detail_spread(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_detail_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_gi_inject(&mut self, gi_inject: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (gi_inject,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_gi_inject", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_gi_inject(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_gi_inject", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_ambient_inject(&mut self, enabled: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_ambient_inject", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_ambient_inject(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_ambient_inject", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_sky_affect(&mut self, sky_affect: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (sky_affect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_sky_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_sky_affect(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_sky_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_temporal_reprojection_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_temporal_reprojection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_volumetric_fog_temporal_reprojection_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_volumetric_fog_temporal_reprojection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volumetric_fog_temporal_reprojection_amount(&mut self, temporal_reprojection_amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (temporal_reprojection_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volumetric_fog_temporal_reprojection_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volumetric_fog_temporal_reprojection_amount(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volumetric_fog_temporal_reprojection_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_adjustment_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_adjustment_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_adjustment_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_adjustment_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_adjustment_brightness(&mut self, brightness: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (brightness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_adjustment_brightness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_adjustment_brightness(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_adjustment_brightness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_adjustment_contrast(&mut self, contrast: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (contrast,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_adjustment_contrast", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_adjustment_contrast(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_adjustment_contrast", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_adjustment_saturation(&mut self, saturation: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_adjustment_saturation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_adjustment_saturation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_adjustment_saturation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_adjustment_color_correction(&mut self, color_correction: Gd < crate::engine::Texture >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Texture >);
            let args = (color_correction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_adjustment_color_correction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_adjustment_color_correction(&self,) -> Option < Gd < crate::engine::Texture > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Texture > >;
            type CallSig = (Option < Gd < crate::engine::Texture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_adjustment_color_correction", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Environment {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Environment\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Environment {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Environment {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Environment {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Environment {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Environment {
        
    }
    impl crate::obj::ExportableObject for Environment {
        
    }
    impl crate::obj::cap::GodotDefault for Environment {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Environment {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Environment {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Environment {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Environment > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BGMode {
    ord: i32
}
impl BGMode {
    pub const BG_CLEAR_COLOR: Self = Self {
        ord: 0i32
    };
    pub const BG_COLOR: Self = Self {
        ord: 1i32
    };
    pub const BG_SKY: Self = Self {
        ord: 2i32
    };
    pub const BG_CANVAS: Self = Self {
        ord: 3i32
    };
    pub const BG_KEEP: Self = Self {
        ord: 4i32
    };
    pub const BG_CAMERA_FEED: Self = Self {
        ord: 5i32
    };
    pub const BG_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for BGMode {
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
impl crate::obj::IndexEnum for BGMode {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for BGMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for BGMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for BGMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AmbientSource {
    ord: i32
}
impl AmbientSource {
    pub const AMBIENT_SOURCE_BG: Self = Self {
        ord: 0i32
    };
    pub const AMBIENT_SOURCE_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const AMBIENT_SOURCE_COLOR: Self = Self {
        ord: 2i32
    };
    pub const AMBIENT_SOURCE_SKY: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AmbientSource {
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
impl crate::builtin::meta::GodotConvert for AmbientSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AmbientSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AmbientSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReflectionSource {
    ord: i32
}
impl ReflectionSource {
    pub const REFLECTION_SOURCE_BG: Self = Self {
        ord: 0i32
    };
    pub const REFLECTION_SOURCE_DISABLED: Self = Self {
        ord: 1i32
    };
    pub const REFLECTION_SOURCE_SKY: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for ReflectionSource {
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
impl crate::builtin::meta::GodotConvert for ReflectionSource {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ReflectionSource {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ReflectionSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ToneMapper {
    ord: i32
}
impl ToneMapper {
    pub const TONE_MAPPER_LINEAR: Self = Self {
        ord: 0i32
    };
    pub const TONE_MAPPER_REINHARDT: Self = Self {
        ord: 1i32
    };
    pub const TONE_MAPPER_FILMIC: Self = Self {
        ord: 2i32
    };
    pub const TONE_MAPPER_ACES: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for ToneMapper {
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
impl crate::builtin::meta::GodotConvert for ToneMapper {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ToneMapper {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ToneMapper {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GlowBlendMode {
    ord: i32
}
impl GlowBlendMode {
    pub const GLOW_BLEND_MODE_ADDITIVE: Self = Self {
        ord: 0i32
    };
    pub const GLOW_BLEND_MODE_SCREEN: Self = Self {
        ord: 1i32
    };
    pub const GLOW_BLEND_MODE_SOFTLIGHT: Self = Self {
        ord: 2i32
    };
    pub const GLOW_BLEND_MODE_REPLACE: Self = Self {
        ord: 3i32
    };
    pub const GLOW_BLEND_MODE_MIX: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for GlowBlendMode {
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
impl crate::builtin::meta::GodotConvert for GlowBlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for GlowBlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for GlowBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SDFGIYScale {
    ord: i32
}
impl SDFGIYScale {
    pub const SDFGI_Y_SCALE_50_PERCENT: Self = Self {
        ord: 0i32
    };
    pub const SDFGI_Y_SCALE_75_PERCENT: Self = Self {
        ord: 1i32
    };
    pub const SDFGI_Y_SCALE_100_PERCENT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for SDFGIYScale {
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
impl crate::builtin::meta::GodotConvert for SDFGIYScale {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SDFGIYScale {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SDFGIYScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}