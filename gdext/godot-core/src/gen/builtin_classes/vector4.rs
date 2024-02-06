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
pub struct InnerVector4 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector4 < 'a > {
    pub fn from_outer(outer: &Vector4) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(254usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(255usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(256usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(257usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length_squared", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(258usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(259usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sign", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(260usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(261usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(262usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "round", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector4, weight: f64,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(263usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4, Vector4, Vector4, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(264usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4, Vector4, Vector4, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(265usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, f64);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(266usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector4,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(267usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "posmodv", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector4,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(268usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "snapped", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector4, max: Vector4,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4, Vector4);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(269usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clamp", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(270usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(271usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector4,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(272usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector4,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(273usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector4,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(274usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector4,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector4);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(275usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "dot", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Vector4 {
        type RetMarshal = PtrcallReturnT < Vector4 >;
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(276usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector4,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(277usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(278usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(279usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
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
    pub const AXIS_W: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for Axis {
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