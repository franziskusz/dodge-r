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
pub struct InnerVector3 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector3 < 'a > {
    pub fn from_outer(outer: &Vector3) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(183usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(184usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(185usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "angle_to", self.sys_ptr, args)
        }
    }
    pub fn signed_angle_to(&self, to: Vector3, axis: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3, Vector3);
        let args = (to, axis,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(186usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "signed_angle_to", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(187usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(188usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(189usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(190usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(191usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length_squared", self.sys_ptr, args)
        }
    }
    pub fn limit_length(&self, length: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, f64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(192usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "limit_length", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(193usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(194usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector3,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(195usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(196usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(197usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(198usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector3, max: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, Vector3);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(199usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clamp", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(200usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "snapped", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(201usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(202usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lerp", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(203usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(204usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(205usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn bezier_interpolate(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(206usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bezier_interpolate", self.sys_ptr, args)
        }
    }
    pub fn bezier_derivative(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(207usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bezier_derivative", self.sys_ptr, args)
        }
    }
    pub fn move_toward(&self, to: Vector3, delta: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(208usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "move_toward", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(209usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "dot", self.sys_ptr, args)
        }
    }
    pub fn cross(&self, with: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(210usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cross", self.sys_ptr, args)
        }
    }
    pub fn outer(&self, with: Vector3,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(211usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "outer", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(212usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(213usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(214usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(215usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "round", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, f64);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(216usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(217usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmodv", self.sys_ptr, args)
        }
    }
    pub fn project(&self, b: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(218usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "project", self.sys_ptr, args)
        }
    }
    pub fn slide(&self, n: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(219usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slide", self.sys_ptr, args)
        }
    }
    pub fn bounce(&self, n: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(220usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bounce", self.sys_ptr, args)
        }
    }
    pub fn reflect(&self, n: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(221usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reflect", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(222usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sign", self.sys_ptr, args)
        }
    }
    pub fn octahedron_encode(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(223usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "octahedron_encode", self.sys_ptr, args)
        }
    }
    pub fn octahedron_decode(uv: Vector2,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector2);
        let args = (uv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(224usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "octahedron_decode", std::ptr::null_mut(), args)
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
    pub const AXIS_Z: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Axis {
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