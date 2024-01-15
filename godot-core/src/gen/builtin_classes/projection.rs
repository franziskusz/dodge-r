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
pub struct InnerProjection < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerProjection < 'a > {
    pub fn from_outer(outer: &Projection) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn create_depth_correction(flip_y: bool,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, bool);
        let args = (flip_y,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(375usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_depth_correction", std::ptr::null_mut(), args)
        }
    }
    pub fn create_light_atlas_rect(rect: Rect2,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, Rect2);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(376usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_light_atlas_rect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_perspective(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, f64, f64, bool);
        let args = (fovy, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(377usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_perspective", std::ptr::null_mut(), args)
        }
    }
    pub fn create_perspective_hmd(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool, eye: i64, intraocular_dist: f64, convergence_dist: f64,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, f64, f64, bool, i64, f64, f64);
        let args = (fovy, aspect, z_near, z_far, flip_fov, eye, intraocular_dist, convergence_dist,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(378usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_perspective_hmd", std::ptr::null_mut(), args)
        }
    }
    pub fn create_for_hmd(eye: i64, aspect: f64, intraocular_dist: f64, display_width: f64, display_to_lens: f64, oversample: f64, z_near: f64, z_far: f64,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, i64, f64, f64, f64, f64, f64, f64, f64);
        let args = (eye, aspect, intraocular_dist, display_width, display_to_lens, oversample, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(379usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_for_hmd", std::ptr::null_mut(), args)
        }
    }
    pub fn create_orthogonal(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, f64, f64, f64, f64);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(380usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_orthogonal", std::ptr::null_mut(), args)
        }
    }
    pub fn create_orthogonal_aspect(size: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, f64, f64, bool);
        let args = (size, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(381usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_orthogonal_aspect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_frustum(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, f64, f64, f64, f64);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(382usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_frustum", std::ptr::null_mut(), args)
        }
    }
    pub fn create_frustum_aspect(size: f64, aspect: f64, offset: Vector2, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64, f64, Vector2, f64, f64, bool);
        let args = (size, aspect, offset, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(383usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_frustum_aspect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_fit_aabb(aabb: Aabb,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, Aabb);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(384usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "create_fit_aabb", std::ptr::null_mut(), args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(385usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "determinant", self.sys_ptr, args)
        }
    }
    pub fn perspective_znear_adjusted(&self, new_znear: f64,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, f64);
        let args = (new_znear,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(386usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "perspective_znear_adjusted", self.sys_ptr, args)
        }
    }
    pub fn get_projection_plane(&self, plane: i64,) -> Plane {
        type RetMarshal = PtrcallReturnT < Plane >;
        type CallSig = (Plane, i64);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(387usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_projection_plane", self.sys_ptr, args)
        }
    }
    pub fn flipped_y(&self,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(388usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "flipped_y", self.sys_ptr, args)
        }
    }
    pub fn jitter_offseted(&self, offset: Vector2,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection, Vector2);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(389usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "jitter_offseted", self.sys_ptr, args)
        }
    }
    pub fn get_fovy(fovx: f64, aspect: f64,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, f64, f64);
        let args = (fovx, aspect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(390usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_fovy", std::ptr::null_mut(), args)
        }
    }
    pub fn get_z_far(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(391usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_z_far", self.sys_ptr, args)
        }
    }
    pub fn get_z_near(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(392usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_z_near", self.sys_ptr, args)
        }
    }
    pub fn get_aspect(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(393usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_aspect", self.sys_ptr, args)
        }
    }
    pub fn get_fov(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(394usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_fov", self.sys_ptr, args)
        }
    }
    pub fn is_orthogonal(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(395usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_orthogonal", self.sys_ptr, args)
        }
    }
    pub fn get_viewport_half_extents(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(396usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_viewport_half_extents", self.sys_ptr, args)
        }
    }
    pub fn get_far_plane_half_extents(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(397usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_far_plane_half_extents", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Projection {
        type RetMarshal = PtrcallReturnT < Projection >;
        type CallSig = (Projection,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(398usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn get_pixels_per_meter(&self, for_pixel_width: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (for_pixel_width,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(399usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_pixels_per_meter", self.sys_ptr, args)
        }
    }
    pub fn get_lod_multiplier(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(400usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_lod_multiplier", self.sys_ptr, args)
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Planes {
    ord: i32
}
impl Planes {
    pub const PLANE_NEAR: Self = Self {
        ord: 0i32
    };
    pub const PLANE_FAR: Self = Self {
        ord: 1i32
    };
    pub const PLANE_LEFT: Self = Self {
        ord: 2i32
    };
    pub const PLANE_TOP: Self = Self {
        ord: 3i32
    };
    pub const PLANE_RIGHT: Self = Self {
        ord: 4i32
    };
    pub const PLANE_BOTTOM: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for Planes {
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
impl crate::builtin::meta::GodotConvert for Planes {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Planes {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Planes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}