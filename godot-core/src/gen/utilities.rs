#![doc = r" Global utility functions."]
#![doc = r""]
#![doc = r" A list of global-scope built-in functions."]
#![doc = r" For global enums and constants, check out the [`global` module][crate::engine::global]."]
#![doc = r""]
#![doc = r" See also [Godot docs for `@GlobalScope`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#methods)."]
use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
pub fn sin(angle_rad: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sin;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "sin", args)
    }
}
pub fn cos(angle_rad: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cos;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cos", args)
    }
}
pub fn tan(angle_rad: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tan;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "tan", args)
    }
}
pub fn sinh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sinh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "sinh", args)
    }
}
pub fn cosh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cosh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cosh", args)
    }
}
pub fn tanh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tanh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "tanh", args)
    }
}
pub fn asin(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asin;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "asin", args)
    }
}
pub fn acos(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acos;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "acos", args)
    }
}
pub fn atan(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "atan", args)
    }
}
pub fn atan2(y: f64, x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (y, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan2;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "atan2", args)
    }
}
pub fn asinh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asinh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "asinh", args)
    }
}
pub fn acosh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acosh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "acosh", args)
    }
}
pub fn atanh(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atanh;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "atanh", args)
    }
}
pub fn sqrt(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sqrt;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "sqrt", args)
    }
}
pub fn fmod(x: f64, y: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fmod;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "fmod", args)
    }
}
pub fn fposmod(x: f64, y: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fposmod;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "fposmod", args)
    }
}
pub fn posmod(x: i64, y: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . posmod;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "posmod", args)
    }
}
pub fn floor(x: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floor;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "floor", args)
    }
}
pub fn floorf(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floorf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "floorf", args)
    }
}
pub fn floori(x: f64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floori;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "floori", args)
    }
}
pub fn ceil(x: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceil;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "ceil", args)
    }
}
pub fn ceilf(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceilf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "ceilf", args)
    }
}
pub fn ceili(x: f64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceili;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "ceili", args)
    }
}
pub fn round(x: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . round;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "round", args)
    }
}
pub fn roundf(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "roundf", args)
    }
}
pub fn roundi(x: f64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "roundi", args)
    }
}
pub fn abs(x: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . abs;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "abs", args)
    }
}
pub fn absf(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "absf", args)
    }
}
pub fn absi(x: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "absi", args)
    }
}
pub fn sign(x: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sign;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "sign", args)
    }
}
pub fn signf(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "signf", args)
    }
}
pub fn signi(x: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "signi", args)
    }
}
pub fn snapped(x: Variant, step: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant, Variant);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snapped;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "snapped", args)
    }
}
pub fn snappedf(x: f64, step: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "snappedf", args)
    }
}
pub fn snappedi(x: f64, step: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, f64, i64);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "snappedi", args)
    }
}
pub fn pow(base: f64, exp: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (base, exp,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pow;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "pow", args)
    }
}
pub fn log(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . log;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "log", args)
    }
}
pub fn exp(x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . exp;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "exp", args)
    }
}
pub fn is_nan(x: f64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_nan;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_nan", args)
    }
}
pub fn is_inf(x: f64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_inf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_inf", args)
    }
}
pub fn is_equal_approx(a: f64, b: f64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, f64, f64);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_equal_approx;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_equal_approx", args)
    }
}
pub fn is_zero_approx(x: f64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_zero_approx;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_zero_approx", args)
    }
}
pub fn is_finite(x: f64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_finite;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_finite", args)
    }
}
pub fn ease(x: f64, curve: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (x, curve,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ease;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "ease", args)
    }
}
pub fn step_decimals(x: f64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, f64);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . step_decimals;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "step_decimals", args)
    }
}
pub fn lerp(from: Variant, to: Variant, weight: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant, Variant, Variant);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "lerp", args)
    }
}
pub fn lerpf(from: f64, to: f64, weight: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerpf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "lerpf", args)
    }
}
pub fn cubic_interpolate(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cubic_interpolate", args)
    }
}
pub fn cubic_interpolate_angle(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle", args)
    }
}
pub fn cubic_interpolate_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64, f64, f64, f64);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_in_time;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_in_time", args)
    }
}
pub fn cubic_interpolate_angle_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64, f64, f64, f64);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle_in_time;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle_in_time", args)
    }
}
pub fn bezier_interpolate(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_interpolate;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "bezier_interpolate", args)
    }
}
pub fn bezier_derivative(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_derivative;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "bezier_derivative", args)
    }
}
pub fn angle_difference(from: f64, to: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . angle_difference;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "angle_difference", args)
    }
}
pub fn lerp_angle(from: f64, to: f64, weight: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp_angle;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "lerp_angle", args)
    }
}
pub fn inverse_lerp(from: f64, to: f64, weight: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . inverse_lerp;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "inverse_lerp", args)
    }
}
pub fn remap(value: f64, istart: f64, istop: f64, ostart: f64, ostop: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64, f64, f64);
    let args = (value, istart, istop, ostart, ostop,);
    unsafe {
        let utility_fn = sys::utility_function_table() . remap;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "remap", args)
    }
}
pub fn smoothstep(from: f64, to: f64, x: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . smoothstep;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "smoothstep", args)
    }
}
pub fn move_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . move_toward;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "move_toward", args)
    }
}
pub fn rotate_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rotate_toward;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "rotate_toward", args)
    }
}
pub fn deg_to_rad(deg: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (deg,);
    unsafe {
        let utility_fn = sys::utility_function_table() . deg_to_rad;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "deg_to_rad", args)
    }
}
pub fn rad_to_deg(rad: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rad_to_deg;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "rad_to_deg", args)
    }
}
pub fn linear_to_db(lin: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (lin,);
    unsafe {
        let utility_fn = sys::utility_function_table() . linear_to_db;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "linear_to_db", args)
    }
}
pub fn db_to_linear(db: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64);
    let args = (db,);
    unsafe {
        let utility_fn = sys::utility_function_table() . db_to_linear;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "db_to_linear", args)
    }
}
pub fn wrap(value: Variant, min: Variant, max: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant, Variant, Variant);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrap;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "wrap", args)
    }
}
pub fn wrapi(value: i64, min: i64, max: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64, i64);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "wrapi", args)
    }
}
pub fn wrapf(value: f64, min: f64, max: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "wrapf", args)
    }
}
pub fn max(arg1: Variant, arg2: Variant, varargs: &[Variant]) -> Variant {
    type CallSig = (Variant, Variant, Variant);
    let args = (arg1, arg2,);
    unsafe {
        let utility_fn = sys::utility_function_table() . max;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "max", args, varargs)
    }
}
pub fn maxi(a: i64, b: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "maxi", args)
    }
}
pub fn maxf(a: f64, b: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "maxf", args)
    }
}
pub fn min(arg1: Variant, arg2: Variant, varargs: &[Variant]) -> Variant {
    type CallSig = (Variant, Variant, Variant);
    let args = (arg1, arg2,);
    unsafe {
        let utility_fn = sys::utility_function_table() . min;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "min", args, varargs)
    }
}
pub fn mini(a: i64, b: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . mini;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "mini", args)
    }
}
pub fn minf(a: f64, b: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . minf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "minf", args)
    }
}
pub fn clamp(value: Variant, min: Variant, max: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant, Variant, Variant);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clamp;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "clamp", args)
    }
}
pub fn clampi(value: i64, min: i64, max: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64, i64);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "clampi", args)
    }
}
pub fn clampf(value: f64, min: f64, max: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64, f64);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "clampf", args)
    }
}
pub fn nearest_po2(value: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64);
    let args = (value,);
    unsafe {
        let utility_fn = sys::utility_function_table() . nearest_po2;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "nearest_po2", args)
    }
}
pub fn pingpong(value: f64, length: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (value, length,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pingpong;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "pingpong", args)
    }
}
pub fn randomize() {
    type RetMarshal = PtrcallReturnUnit;
    type CallSig = ((),);
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randomize;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randomize", args)
    }
}
pub fn randi() -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64,);
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randi;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randi", args)
    }
}
pub fn randf() -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64,);
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randf;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randf", args)
    }
}
pub fn randi_range(from: i64, to: i64,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, i64, i64);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randi_range;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randi_range", args)
    }
}
pub fn randf_range(from: f64, to: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randf_range;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randf_range", args)
    }
}
pub fn randfn(mean: f64, deviation: f64,) -> f64 {
    type RetMarshal = PtrcallReturnT < f64 >;
    type CallSig = (f64, f64, f64);
    let args = (mean, deviation,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randfn;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "randfn", args)
    }
}
pub fn seed(base: i64,) {
    type RetMarshal = PtrcallReturnUnit;
    type CallSig = ((), i64);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . seed;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "seed", args)
    }
}
pub fn rand_from_seed(seed: i64,) -> PackedInt64Array {
    type RetMarshal = PtrcallReturnT < PackedInt64Array >;
    type CallSig = (PackedInt64Array, i64);
    let args = (seed,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rand_from_seed;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "rand_from_seed", args)
    }
}
pub fn weakref(obj: Variant,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant);
    let args = (obj,);
    unsafe {
        let utility_fn = sys::utility_function_table() . weakref;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "weakref", args)
    }
}
pub fn typeof_(variable: Variant,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, Variant);
    let args = (variable,);
    unsafe {
        let utility_fn = sys::utility_function_table() . typeof_;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "typeof", args)
    }
}
pub fn type_convert(variant: Variant, type_: i64,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, Variant, i64);
    let args = (variant, type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_convert;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "type_convert", args)
    }
}
pub fn str(arg1: Variant, varargs: &[Variant]) -> GString {
    type CallSig = (GString, Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . str;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "str", args, varargs)
    }
}
pub fn error_string(error: i64,) -> GString {
    type RetMarshal = PtrcallReturnT < GString >;
    type CallSig = (GString, i64);
    let args = (error,);
    unsafe {
        let utility_fn = sys::utility_function_table() . error_string;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "error_string", args)
    }
}
pub fn type_string(type_: i64,) -> GString {
    type RetMarshal = PtrcallReturnT < GString >;
    type CallSig = (GString, i64);
    let args = (type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_string;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "type_string", args)
    }
}
pub fn print(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . print;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "print", args, varargs)
    }
}
pub fn print_rich(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . print_rich;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "print_rich", args, varargs)
    }
}
pub fn printerr(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . printerr;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "printerr", args, varargs)
    }
}
pub fn printt(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . printt;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "printt", args, varargs)
    }
}
pub fn prints(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . prints;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "prints", args, varargs)
    }
}
pub fn printraw(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . printraw;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "printraw", args, varargs)
    }
}
pub fn print_verbose(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . print_verbose;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "print_verbose", args, varargs)
    }
}
pub fn push_error(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . push_error;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "push_error", args, varargs)
    }
}
pub fn push_warning(arg1: Variant, varargs: &[Variant]) {
    type CallSig = ((), Variant);
    let args = (arg1,);
    unsafe {
        let utility_fn = sys::utility_function_table() . push_warning;
        < CallSig as VarcallSignatureTuple > ::out_utility_ptrcall_varargs(utility_fn, "push_warning", args, varargs)
    }
}
pub fn var_to_str(variable: Variant,) -> GString {
    type RetMarshal = PtrcallReturnT < GString >;
    type CallSig = (GString, Variant);
    let args = (variable,);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_str;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "var_to_str", args)
    }
}
pub fn str_to_var(string: GString,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, GString);
    let args = (string,);
    unsafe {
        let utility_fn = sys::utility_function_table() . str_to_var;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "str_to_var", args)
    }
}
pub fn var_to_bytes(variable: Variant,) -> PackedByteArray {
    type RetMarshal = PtrcallReturnT < PackedByteArray >;
    type CallSig = (PackedByteArray, Variant);
    let args = (variable,);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "var_to_bytes", args)
    }
}
pub fn bytes_to_var(bytes: PackedByteArray,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, PackedByteArray);
    let args = (bytes,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "bytes_to_var", args)
    }
}
pub fn var_to_bytes_with_objects(variable: Variant,) -> PackedByteArray {
    type RetMarshal = PtrcallReturnT < PackedByteArray >;
    type CallSig = (PackedByteArray, Variant);
    let args = (variable,);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes_with_objects;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "var_to_bytes_with_objects", args)
    }
}
pub fn bytes_to_var_with_objects(bytes: PackedByteArray,) -> Variant {
    type RetMarshal = PtrcallReturnT < Variant >;
    type CallSig = (Variant, PackedByteArray);
    let args = (bytes,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var_with_objects;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "bytes_to_var_with_objects", args)
    }
}
pub fn hash(variable: Variant,) -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64, Variant);
    let args = (variable,);
    unsafe {
        let utility_fn = sys::utility_function_table() . hash;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "hash", args)
    }
}
pub fn instance_from_id(instance_id: i64,) -> Option < Gd < crate::engine::Object > > {
    type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
    type CallSig = (Option < Gd < crate::engine::Object > >, i64);
    let args = (instance_id,);
    unsafe {
        let utility_fn = sys::utility_function_table() . instance_from_id;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "instance_from_id", args)
    }
}
pub fn is_instance_id_valid(id: i64,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, i64);
    let args = (id,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_id_valid;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_instance_id_valid", args)
    }
}
pub fn is_instance_valid(instance: Variant,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, Variant);
    let args = (instance,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_valid;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_instance_valid", args)
    }
}
pub fn rid_allocate_id() -> i64 {
    type RetMarshal = PtrcallReturnT < i64 >;
    type CallSig = (i64,);
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_allocate_id;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "rid_allocate_id", args)
    }
}
pub fn rid_from_int64(base: i64,) -> Rid {
    type RetMarshal = PtrcallReturnT < Rid >;
    type CallSig = (Rid, i64);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_from_int64;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "rid_from_int64", args)
    }
}
pub fn is_same(a: Variant, b: Variant,) -> bool {
    type RetMarshal = PtrcallReturnT < bool >;
    type CallSig = (bool, Variant, Variant);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_same;
        < CallSig as PtrcallSignatureTuple > ::out_utility_ptrcall(utility_fn, "is_same", args)
    }
}