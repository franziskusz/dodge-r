#![doc = "Sidecar module for class [`FastNoiseLite`][crate::engine::FastNoiseLite].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FastNoiseLite` enums](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FastNoiseLite.`\n\nInherits [`Noise`][crate::engine::Noise].\n\nRelated symbols:\n\n* [`fast_noise_lite`][crate::engine::fast_noise_lite]: sidecar module with related enum/flag types\n* [`IFastNoiseLite`][crate::engine::IFastNoiseLite]: virtual methods\n\n\nSee also [Godot docs for `FastNoiseLite`](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FastNoiseLite {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FastNoiseLite`][crate::engine::FastNoiseLite].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FastNoiseLite` methods](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFastNoiseLite: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FastNoiseLite {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_noise_type(&mut self, type_: crate::engine::fast_noise_lite::NoiseType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::NoiseType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_noise_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_type(&self,) -> crate::engine::fast_noise_lite::NoiseType {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::NoiseType >;
            type CallSig = (crate::engine::fast_noise_lite::NoiseType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_noise_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_seed(&mut self, seed: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seed(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frequency(&mut self, freq: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (freq,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frequency(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_type(&mut self, type_: crate::engine::fast_noise_lite::FractalType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::FractalType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_type(&self,) -> crate::engine::fast_noise_lite::FractalType {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::FractalType >;
            type CallSig = (crate::engine::fast_noise_lite::FractalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_octaves(&mut self, octave_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (octave_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_octaves(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_lacunarity(&mut self, lacunarity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (lacunarity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_lacunarity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_gain(&mut self, gain: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_gain(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_weighted_strength(&mut self, weighted_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (weighted_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_weighted_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_weighted_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_weighted_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_ping_pong_strength(&mut self, ping_pong_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ping_pong_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fractal_ping_pong_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_ping_pong_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fractal_ping_pong_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_distance_function(&mut self, func: crate::engine::fast_noise_lite::CellularDistanceFunction,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::CellularDistanceFunction);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cellular_distance_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_distance_function(&self,) -> crate::engine::fast_noise_lite::CellularDistanceFunction {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::CellularDistanceFunction >;
            type CallSig = (crate::engine::fast_noise_lite::CellularDistanceFunction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cellular_distance_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_jitter(&mut self, jitter: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (jitter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cellular_jitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_jitter(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cellular_jitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_return_type(&mut self, ret: crate::engine::fast_noise_lite::CellularReturnType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::CellularReturnType);
            let args = (ret,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_cellular_return_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_return_type(&self,) -> crate::engine::fast_noise_lite::CellularReturnType {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::CellularReturnType >;
            type CallSig = (crate::engine::fast_noise_lite::CellularReturnType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_cellular_return_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_enabled(&mut self, domain_warp_enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (domain_warp_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_domain_warp_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_domain_warp_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_type(&mut self, domain_warp_type: crate::engine::fast_noise_lite::DomainWarpType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::DomainWarpType);
            let args = (domain_warp_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_type(&self,) -> crate::engine::fast_noise_lite::DomainWarpType {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::DomainWarpType >;
            type CallSig = (crate::engine::fast_noise_lite::DomainWarpType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_amplitude(&mut self, domain_warp_amplitude: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (domain_warp_amplitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_amplitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_amplitude(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_amplitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_frequency(&mut self, domain_warp_frequency: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (domain_warp_frequency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_frequency(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_type(&mut self, domain_warp_fractal_type: crate::engine::fast_noise_lite::DomainWarpFractalType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::fast_noise_lite::DomainWarpFractalType);
            let args = (domain_warp_fractal_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_type(&self,) -> crate::engine::fast_noise_lite::DomainWarpFractalType {
            type RetMarshal = PtrcallReturnT < crate::engine::fast_noise_lite::DomainWarpFractalType >;
            type CallSig = (crate::engine::fast_noise_lite::DomainWarpFractalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_octaves(&mut self, domain_warp_octave_count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (domain_warp_octave_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_octaves(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_lacunarity(&mut self, domain_warp_lacunarity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (domain_warp_lacunarity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_lacunarity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_gain(&mut self, domain_warp_gain: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (domain_warp_gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_domain_warp_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_gain(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_domain_warp_fractal_gain", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FastNoiseLite {
        type Base = crate::engine::Noise;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"FastNoiseLite\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FastNoiseLite {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for FastNoiseLite {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Noise > for FastNoiseLite {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for FastNoiseLite {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for FastNoiseLite {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for FastNoiseLite {
        
    }
    impl crate::obj::ExportableObject for FastNoiseLite {
        
    }
    impl crate::obj::cap::GodotDefault for FastNoiseLite {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FastNoiseLite {
        type Target = crate::engine::Noise;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FastNoiseLite {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_FastNoiseLite {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::FastNoiseLite > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Noise > for $Class {
                
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
pub struct NoiseType {
    ord: i32
}
impl NoiseType {
    pub const TYPE_VALUE: Self = Self {
        ord: 5i32
    };
    pub const TYPE_VALUE_CUBIC: Self = Self {
        ord: 4i32
    };
    pub const TYPE_PERLIN: Self = Self {
        ord: 3i32
    };
    pub const TYPE_CELLULAR: Self = Self {
        ord: 2i32
    };
    pub const TYPE_SIMPLEX: Self = Self {
        ord: 0i32
    };
    pub const TYPE_SIMPLEX_SMOOTH: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for NoiseType {
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
impl crate::builtin::meta::GodotConvert for NoiseType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for NoiseType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for NoiseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FractalType {
    ord: i32
}
impl FractalType {
    pub const FRACTAL_NONE: Self = Self {
        ord: 0i32
    };
    pub const FRACTAL_FBM: Self = Self {
        ord: 1i32
    };
    pub const FRACTAL_RIDGED: Self = Self {
        ord: 2i32
    };
    pub const FRACTAL_PING_PONG: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for FractalType {
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
impl crate::builtin::meta::GodotConvert for FractalType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FractalType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FractalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CellularDistanceFunction {
    ord: i32
}
impl CellularDistanceFunction {
    pub const DISTANCE_EUCLIDEAN: Self = Self {
        ord: 0i32
    };
    pub const DISTANCE_EUCLIDEAN_SQUARED: Self = Self {
        ord: 1i32
    };
    pub const DISTANCE_MANHATTAN: Self = Self {
        ord: 2i32
    };
    pub const DISTANCE_HYBRID: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for CellularDistanceFunction {
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
impl crate::builtin::meta::GodotConvert for CellularDistanceFunction {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CellularDistanceFunction {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CellularDistanceFunction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CellularReturnType {
    ord: i32
}
impl CellularReturnType {
    pub const RETURN_CELL_VALUE: Self = Self {
        ord: 0i32
    };
    pub const RETURN_DISTANCE: Self = Self {
        ord: 1i32
    };
    pub const RETURN_DISTANCE2: Self = Self {
        ord: 2i32
    };
    pub const RETURN_DISTANCE2_ADD: Self = Self {
        ord: 3i32
    };
    pub const RETURN_DISTANCE2_SUB: Self = Self {
        ord: 4i32
    };
    pub const RETURN_DISTANCE2_MUL: Self = Self {
        ord: 5i32
    };
    pub const RETURN_DISTANCE2_DIV: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for CellularReturnType {
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
impl crate::builtin::meta::GodotConvert for CellularReturnType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CellularReturnType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CellularReturnType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DomainWarpType {
    ord: i32
}
impl DomainWarpType {
    pub const DOMAIN_WARP_SIMPLEX: Self = Self {
        ord: 0i32
    };
    pub const DOMAIN_WARP_SIMPLEX_REDUCED: Self = Self {
        ord: 1i32
    };
    pub const DOMAIN_WARP_BASIC_GRID: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DomainWarpType {
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
impl crate::builtin::meta::GodotConvert for DomainWarpType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DomainWarpType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DomainWarpType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DomainWarpFractalType {
    ord: i32
}
impl DomainWarpFractalType {
    pub const DOMAIN_WARP_FRACTAL_NONE: Self = Self {
        ord: 0i32
    };
    pub const DOMAIN_WARP_FRACTAL_PROGRESSIVE: Self = Self {
        ord: 1i32
    };
    pub const DOMAIN_WARP_FRACTAL_INDEPENDENT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DomainWarpFractalType {
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
impl crate::builtin::meta::GodotConvert for DomainWarpFractalType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DomainWarpFractalType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DomainWarpFractalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}