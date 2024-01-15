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
pub struct InnerColor < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerColor < 'a > {
    pub fn from_outer(outer: &Color) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn to_argb32(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(401usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_argb32", self.sys_ptr, args)
        }
    }
    pub fn to_abgr32(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(402usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_abgr32", self.sys_ptr, args)
        }
    }
    pub fn to_rgba32(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(403usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_rgba32", self.sys_ptr, args)
        }
    }
    pub fn to_argb64(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(404usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_argb64", self.sys_ptr, args)
        }
    }
    pub fn to_abgr64(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(405usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_abgr64", self.sys_ptr, args)
        }
    }
    pub fn to_rgba64(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(406usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_rgba64", self.sys_ptr, args)
        }
    }
    pub fn to_html(&self, with_alpha: bool,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, bool);
        let args = (with_alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(407usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_html", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Color, max: Color,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, Color, Color);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(408usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clamp", self.sys_ptr, args)
        }
    }
    pub fn inverted(&self,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(409usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverted", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Color, weight: f64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, Color, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(410usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lerp", self.sys_ptr, args)
        }
    }
    pub fn lightened(&self, amount: f64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, f64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(411usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lightened", self.sys_ptr, args)
        }
    }
    pub fn darkened(&self, amount: f64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, f64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(412usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "darkened", self.sys_ptr, args)
        }
    }
    pub fn blend(&self, over: Color,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, Color);
        let args = (over,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(413usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "blend", self.sys_ptr, args)
        }
    }
    pub fn get_luminance(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(414usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_luminance", self.sys_ptr, args)
        }
    }
    pub fn srgb_to_linear(&self,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(415usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "srgb_to_linear", self.sys_ptr, args)
        }
    }
    pub fn linear_to_srgb(&self,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(416usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "linear_to_srgb", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Color,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Color);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(417usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn hex(hex: i64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, i64);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(418usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hex", std::ptr::null_mut(), args)
        }
    }
    pub fn hex64(hex: i64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, i64);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(419usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hex64", std::ptr::null_mut(), args)
        }
    }
    pub fn html(rgba: GString,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, GString);
        let args = (rgba,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(420usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "html", std::ptr::null_mut(), args)
        }
    }
    pub fn html_is_valid(color: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (color,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(421usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "html_is_valid", std::ptr::null_mut(), args)
        }
    }
    pub fn from_string(str: GString, default: Color,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, GString, Color);
        let args = (str, default,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(422usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_string", std::ptr::null_mut(), args)
        }
    }
    pub fn from_hsv(h: f64, s: f64, v: f64, alpha: f64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, f64, f64, f64, f64);
        let args = (h, s, v, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(423usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_hsv", std::ptr::null_mut(), args)
        }
    }
    pub fn from_ok_hsl(h: f64, s: f64, l: f64, alpha: f64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, f64, f64, f64, f64);
        let args = (h, s, l, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(424usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_ok_hsl", std::ptr::null_mut(), args)
        }
    }
    pub fn from_rgbe9995(rgbe: i64,) -> Color {
        type RetMarshal = PtrcallReturnT < Color >;
        type CallSig = (Color, i64);
        let args = (rgbe,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(425usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_rgbe9995", std::ptr::null_mut(), args)
        }
    }
}