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
pub struct InnerVector2i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector2i < 'a > {
    pub fn from_outer(outer: &Vector2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn aspect(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(146usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "aspect", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(147usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(148usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(149usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(150usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length_squared", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector2i {
        type RetMarshal = PtrcallReturnT < Vector2i >;
        type CallSig = (Vector2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(151usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sign", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector2i {
        type RetMarshal = PtrcallReturnT < Vector2i >;
        type CallSig = (Vector2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(152usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector2i, max: Vector2i,) -> Vector2i {
        type RetMarshal = PtrcallReturnT < Vector2i >;
        type CallSig = (Vector2i, Vector2i, Vector2i);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(153usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clamp", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector2i,) -> Vector2i {
        type RetMarshal = PtrcallReturnT < Vector2i >;
        type CallSig = (Vector2i, Vector2i);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(154usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "snapped", self.sys_ptr, args)
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