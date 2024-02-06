use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerVector2 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector2 < 'a > {
    pub fn from_outer(outer: &Vector2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn angle(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(104usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "angle", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(105usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "angle_to", self.sys_ptr, args)
        }
    }
    pub fn angle_to_point(&self, to: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(106usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "angle_to_point", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(107usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(108usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(109usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(110usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(111usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length_squared", self.sys_ptr, args)
        }
    }
    pub fn limit_length(&self, length: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, f64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(112usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "limit_length", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(113usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(114usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector2,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(115usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(116usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(117usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, f64);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(118usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(119usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmodv", self.sys_ptr, args)
        }
    }
    pub fn project(&self, b: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(120usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "project", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(121usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lerp", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(122usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, Vector2, Vector2, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(123usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, Vector2, Vector2, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(124usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn bezier_interpolate(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, Vector2, Vector2, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(125usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bezier_interpolate", self.sys_ptr, args)
        }
    }
    pub fn bezier_derivative(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, Vector2, Vector2, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(126usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bezier_derivative", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(127usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(128usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn move_toward(&self, to: Vector2, delta: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, f64);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(129usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "move_toward", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, angle: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, f64);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(130usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated", self.sys_ptr, args)
        }
    }
    pub fn orthogonal(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(131usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "orthogonal", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(132usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(133usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(134usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "round", self.sys_ptr, args)
        }
    }
    pub fn aspect(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(135usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "aspect", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(136usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "dot", self.sys_ptr, args)
        }
    }
    pub fn slide(&self, n: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(137usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slide", self.sys_ptr, args)
        }
    }
    pub fn bounce(&self, n: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(138usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bounce", self.sys_ptr, args)
        }
    }
    pub fn reflect(&self, n: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(139usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reflect", self.sys_ptr, args)
        }
    }
    pub fn cross(&self, with: Vector2,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector2);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(140usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cross", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(141usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(142usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sign", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector2, max: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2, Vector2);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(143usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clamp", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(144usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "snapped", self.sys_ptr, args)
        }
    }
    pub fn from_angle(angle: f64,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, f64);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(145usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_angle", std::ptr::null_mut(), args)
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Axis {
    ord: i32
}
impl Axis {
    pub const AXIS_X: Self = Self {
        ord: 0i32
    };
    pub const AXIS_Y: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for Axis {
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
impl crate::builtin::meta::GodotConvert for Axis {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Axis {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Axis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}