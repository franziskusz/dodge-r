use crate::builtin::*;
use crate::engine::Object;
use crate::obj::Gd;
#[allow(dead_code)]
pub enum VariantDispatch {
    Nil, Bool(bool), Int(i64), Float(f64), String(GString), Vector2(Vector2), Vector2i(Vector2i), Rect2(Rect2), Rect2i(Rect2i), Vector3(Vector3), Vector3i(Vector3i), Transform2D(Transform2D), Vector4(Vector4), Vector4i(Vector4i), Plane(Plane), Quaternion(Quaternion), Aabb(Aabb), Basis(Basis), Transform3D(Transform3D), Projection(Projection), Color(Color), StringName(StringName), NodePath(NodePath), Rid(Rid), Object(Gd < crate::engine::Object >), Callable(Callable), Signal(Signal), Dictionary(Dictionary), Array(VariantArray), PackedByteArray(PackedByteArray), PackedInt32Array(PackedInt32Array), PackedInt64Array(PackedInt64Array), PackedFloat32Array(PackedFloat32Array), PackedFloat64Array(PackedFloat64Array), PackedStringArray(PackedStringArray), PackedVector2Array(PackedVector2Array), PackedVector3Array(PackedVector3Array), PackedColorArray(PackedColorArray),
}
#[cfg(FALSE)]
impl FromVariant for VariantDispatch {
    fn try_from_variant(variant: &Variant) -> Result < Self, VariantConversionError > {
        let dispatch = match variant.get_type() {
            VariantType::Nil => Self::Nil, VariantType::Bool => Self::Bool(variant.to::< bool > ()), VariantType::Int => Self::Int(variant.to::< i64 > ()), VariantType::Float => Self::Float(variant.to::< f64 > ()), VariantType::String => Self::String(variant.to::< GString > ()), VariantType::Vector2 => Self::Vector2(variant.to::< Vector2 > ()), VariantType::Vector2i => Self::Vector2i(variant.to::< Vector2i > ()), VariantType::Rect2 => Self::Rect2(variant.to::< Rect2 > ()), VariantType::Rect2i => Self::Rect2i(variant.to::< Rect2i > ()), VariantType::Vector3 => Self::Vector3(variant.to::< Vector3 > ()), VariantType::Vector3i => Self::Vector3i(variant.to::< Vector3i > ()), VariantType::Transform2D => Self::Transform2D(variant.to::< Transform2D > ()), VariantType::Vector4 => Self::Vector4(variant.to::< Vector4 > ()), VariantType::Vector4i => Self::Vector4i(variant.to::< Vector4i > ()), VariantType::Plane => Self::Plane(variant.to::< Plane > ()), VariantType::Quaternion => Self::Quaternion(variant.to::< Quaternion > ()), VariantType::Aabb => Self::Aabb(variant.to::< Aabb > ()), VariantType::Basis => Self::Basis(variant.to::< Basis > ()), VariantType::Transform3D => Self::Transform3D(variant.to::< Transform3D > ()), VariantType::Projection => Self::Projection(variant.to::< Projection > ()), VariantType::Color => Self::Color(variant.to::< Color > ()), VariantType::StringName => Self::StringName(variant.to::< StringName > ()), VariantType::NodePath => Self::NodePath(variant.to::< NodePath > ()), VariantType::Rid => Self::Rid(variant.to::< Rid > ()), VariantType::Object => Self::Object(variant.to::< Gd < crate::engine::Object > > ()), VariantType::Callable => Self::Callable(variant.to::< Callable > ()), VariantType::Signal => Self::Signal(variant.to::< Signal > ()), VariantType::Dictionary => Self::Dictionary(variant.to::< Dictionary > ()), VariantType::Array => Self::Array(variant.to::< VariantArray > ()), VariantType::PackedByteArray => Self::PackedByteArray(variant.to::< PackedByteArray > ()), VariantType::PackedInt32Array => Self::PackedInt32Array(variant.to::< PackedInt32Array > ()), VariantType::PackedInt64Array => Self::PackedInt64Array(variant.to::< PackedInt64Array > ()), VariantType::PackedFloat32Array => Self::PackedFloat32Array(variant.to::< PackedFloat32Array > ()), VariantType::PackedFloat64Array => Self::PackedFloat64Array(variant.to::< PackedFloat64Array > ()), VariantType::PackedStringArray => Self::PackedStringArray(variant.to::< PackedStringArray > ()), VariantType::PackedVector2Array => Self::PackedVector2Array(variant.to::< PackedVector2Array > ()), VariantType::PackedVector3Array => Self::PackedVector3Array(variant.to::< PackedVector3Array > ()), VariantType::PackedColorArray => Self::PackedColorArray(variant.to::< PackedColorArray > ()),
        };
        Ok(dispatch)
    }
}
#[doc = r" Global enums and constants."]
#[doc = r""]
#[doc = r" A list of global-scope enumerated constants."]
#[doc = r" For global built-in functions, check out the [`utilities` module][crate::engine::utilities]."]
#[doc = r""]
#[doc = r" See also [Godot docs for `@GlobalScope`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enumerations)."]
pub mod global {
    use crate::sys;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Side {
        ord: i32
    }
    impl Side {
        pub const SIDE_LEFT: Self = Self {
            ord: 0i32
        };
        pub const SIDE_TOP: Self = Self {
            ord: 1i32
        };
        pub const SIDE_RIGHT: Self = Self {
            ord: 2i32
        };
        pub const SIDE_BOTTOM: Self = Self {
            ord: 3i32
        };
        
    }
    impl crate::obj::EngineEnum for Side {
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
    impl crate::builtin::meta::GodotConvert for Side {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for Side {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for Side {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Corner {
        ord: i32
    }
    impl Corner {
        pub const CORNER_TOP_LEFT: Self = Self {
            ord: 0i32
        };
        pub const CORNER_TOP_RIGHT: Self = Self {
            ord: 1i32
        };
        pub const CORNER_BOTTOM_RIGHT: Self = Self {
            ord: 2i32
        };
        pub const CORNER_BOTTOM_LEFT: Self = Self {
            ord: 3i32
        };
        
    }
    impl crate::obj::EngineEnum for Corner {
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
    impl crate::builtin::meta::GodotConvert for Corner {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for Corner {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for Corner {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Orientation {
        ord: i32
    }
    impl Orientation {
        pub const VERTICAL: Self = Self {
            ord: 1i32
        };
        pub const HORIZONTAL: Self = Self {
            ord: 0i32
        };
        
    }
    impl crate::obj::EngineEnum for Orientation {
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
    impl crate::builtin::meta::GodotConvert for Orientation {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for Orientation {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for Orientation {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ClockDirection {
        ord: i32
    }
    impl ClockDirection {
        pub const CLOCKWISE: Self = Self {
            ord: 0i32
        };
        pub const COUNTERCLOCKWISE: Self = Self {
            ord: 1i32
        };
        
    }
    impl crate::obj::EngineEnum for ClockDirection {
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
    impl crate::builtin::meta::GodotConvert for ClockDirection {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for ClockDirection {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for ClockDirection {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct HorizontalAlignment {
        ord: i32
    }
    impl HorizontalAlignment {
        pub const HORIZONTAL_ALIGNMENT_LEFT: Self = Self {
            ord: 0i32
        };
        pub const HORIZONTAL_ALIGNMENT_CENTER: Self = Self {
            ord: 1i32
        };
        pub const HORIZONTAL_ALIGNMENT_RIGHT: Self = Self {
            ord: 2i32
        };
        pub const HORIZONTAL_ALIGNMENT_FILL: Self = Self {
            ord: 3i32
        };
        
    }
    impl crate::obj::EngineEnum for HorizontalAlignment {
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
    impl crate::builtin::meta::GodotConvert for HorizontalAlignment {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for HorizontalAlignment {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for HorizontalAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct VerticalAlignment {
        ord: i32
    }
    impl VerticalAlignment {
        pub const VERTICAL_ALIGNMENT_TOP: Self = Self {
            ord: 0i32
        };
        pub const VERTICAL_ALIGNMENT_CENTER: Self = Self {
            ord: 1i32
        };
        pub const VERTICAL_ALIGNMENT_BOTTOM: Self = Self {
            ord: 2i32
        };
        pub const VERTICAL_ALIGNMENT_FILL: Self = Self {
            ord: 3i32
        };
        
    }
    impl crate::obj::EngineEnum for VerticalAlignment {
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
    impl crate::builtin::meta::GodotConvert for VerticalAlignment {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for VerticalAlignment {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for VerticalAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct InlineAlignment {
        ord: i32
    }
    impl InlineAlignment {
        pub const INLINE_ALIGNMENT_TOP_TO: Self = Self {
            ord: 0i32
        };
        pub const INLINE_ALIGNMENT_CENTER_TO: Self = Self {
            ord: 1i32
        };
        pub const INLINE_ALIGNMENT_BASELINE_TO: Self = Self {
            ord: 3i32
        };
        pub const INLINE_ALIGNMENT_BOTTOM_TO: Self = Self {
            ord: 2i32
        };
        pub const INLINE_ALIGNMENT_TO_TOP: Self = Self {
            ord: 0i32
        };
        pub const INLINE_ALIGNMENT_TO_CENTER: Self = Self {
            ord: 4i32
        };
        pub const INLINE_ALIGNMENT_TO_BASELINE: Self = Self {
            ord: 8i32
        };
        pub const INLINE_ALIGNMENT_TO_BOTTOM: Self = Self {
            ord: 12i32
        };
        pub const INLINE_ALIGNMENT_TOP: Self = Self {
            ord: 0i32
        };
        pub const INLINE_ALIGNMENT_CENTER: Self = Self {
            ord: 5i32
        };
        pub const INLINE_ALIGNMENT_BOTTOM: Self = Self {
            ord: 14i32
        };
        pub const INLINE_ALIGNMENT_IMAGE_MASK: Self = Self {
            ord: 3i32
        };
        pub const INLINE_ALIGNMENT_TEXT_MASK: Self = Self {
            ord: 12i32
        };
        
    }
    impl crate::obj::EngineEnum for InlineAlignment {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 8i32 | ord @ 12i32 | ord @ 14i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for InlineAlignment {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for InlineAlignment {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for InlineAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct EulerOrder {
        ord: i32
    }
    impl EulerOrder {
        pub const EULER_ORDER_XYZ: Self = Self {
            ord: 0i32
        };
        pub const EULER_ORDER_XZY: Self = Self {
            ord: 1i32
        };
        pub const EULER_ORDER_YXZ: Self = Self {
            ord: 2i32
        };
        pub const EULER_ORDER_YZX: Self = Self {
            ord: 3i32
        };
        pub const EULER_ORDER_ZXY: Self = Self {
            ord: 4i32
        };
        pub const EULER_ORDER_ZYX: Self = Self {
            ord: 5i32
        };
        
    }
    impl crate::obj::EngineEnum for EulerOrder {
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
    impl crate::builtin::meta::GodotConvert for EulerOrder {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for EulerOrder {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for EulerOrder {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Key {
        ord: i32
    }
    impl Key {
        pub const KEY_NONE: Self = Self {
            ord: 0i32
        };
        pub const KEY_SPECIAL: Self = Self {
            ord: 4194304i32
        };
        pub const KEY_ESCAPE: Self = Self {
            ord: 4194305i32
        };
        pub const KEY_TAB: Self = Self {
            ord: 4194306i32
        };
        pub const KEY_BACKTAB: Self = Self {
            ord: 4194307i32
        };
        pub const KEY_BACKSPACE: Self = Self {
            ord: 4194308i32
        };
        pub const KEY_ENTER: Self = Self {
            ord: 4194309i32
        };
        pub const KEY_KP_ENTER: Self = Self {
            ord: 4194310i32
        };
        pub const KEY_INSERT: Self = Self {
            ord: 4194311i32
        };
        pub const KEY_DELETE: Self = Self {
            ord: 4194312i32
        };
        pub const KEY_PAUSE: Self = Self {
            ord: 4194313i32
        };
        pub const KEY_PRINT: Self = Self {
            ord: 4194314i32
        };
        pub const KEY_SYSREQ: Self = Self {
            ord: 4194315i32
        };
        pub const KEY_CLEAR: Self = Self {
            ord: 4194316i32
        };
        pub const KEY_HOME: Self = Self {
            ord: 4194317i32
        };
        pub const KEY_END: Self = Self {
            ord: 4194318i32
        };
        pub const KEY_LEFT: Self = Self {
            ord: 4194319i32
        };
        pub const KEY_UP: Self = Self {
            ord: 4194320i32
        };
        pub const KEY_RIGHT: Self = Self {
            ord: 4194321i32
        };
        pub const KEY_DOWN: Self = Self {
            ord: 4194322i32
        };
        pub const KEY_PAGEUP: Self = Self {
            ord: 4194323i32
        };
        pub const KEY_PAGEDOWN: Self = Self {
            ord: 4194324i32
        };
        pub const KEY_SHIFT: Self = Self {
            ord: 4194325i32
        };
        pub const KEY_CTRL: Self = Self {
            ord: 4194326i32
        };
        pub const KEY_META: Self = Self {
            ord: 4194327i32
        };
        pub const KEY_ALT: Self = Self {
            ord: 4194328i32
        };
        pub const KEY_CAPSLOCK: Self = Self {
            ord: 4194329i32
        };
        pub const KEY_NUMLOCK: Self = Self {
            ord: 4194330i32
        };
        pub const KEY_SCROLLLOCK: Self = Self {
            ord: 4194331i32
        };
        pub const KEY_F1: Self = Self {
            ord: 4194332i32
        };
        pub const KEY_F2: Self = Self {
            ord: 4194333i32
        };
        pub const KEY_F3: Self = Self {
            ord: 4194334i32
        };
        pub const KEY_F4: Self = Self {
            ord: 4194335i32
        };
        pub const KEY_F5: Self = Self {
            ord: 4194336i32
        };
        pub const KEY_F6: Self = Self {
            ord: 4194337i32
        };
        pub const KEY_F7: Self = Self {
            ord: 4194338i32
        };
        pub const KEY_F8: Self = Self {
            ord: 4194339i32
        };
        pub const KEY_F9: Self = Self {
            ord: 4194340i32
        };
        pub const KEY_F10: Self = Self {
            ord: 4194341i32
        };
        pub const KEY_F11: Self = Self {
            ord: 4194342i32
        };
        pub const KEY_F12: Self = Self {
            ord: 4194343i32
        };
        pub const KEY_F13: Self = Self {
            ord: 4194344i32
        };
        pub const KEY_F14: Self = Self {
            ord: 4194345i32
        };
        pub const KEY_F15: Self = Self {
            ord: 4194346i32
        };
        pub const KEY_F16: Self = Self {
            ord: 4194347i32
        };
        pub const KEY_F17: Self = Self {
            ord: 4194348i32
        };
        pub const KEY_F18: Self = Self {
            ord: 4194349i32
        };
        pub const KEY_F19: Self = Self {
            ord: 4194350i32
        };
        pub const KEY_F20: Self = Self {
            ord: 4194351i32
        };
        pub const KEY_F21: Self = Self {
            ord: 4194352i32
        };
        pub const KEY_F22: Self = Self {
            ord: 4194353i32
        };
        pub const KEY_F23: Self = Self {
            ord: 4194354i32
        };
        pub const KEY_F24: Self = Self {
            ord: 4194355i32
        };
        pub const KEY_F25: Self = Self {
            ord: 4194356i32
        };
        pub const KEY_F26: Self = Self {
            ord: 4194357i32
        };
        pub const KEY_F27: Self = Self {
            ord: 4194358i32
        };
        pub const KEY_F28: Self = Self {
            ord: 4194359i32
        };
        pub const KEY_F29: Self = Self {
            ord: 4194360i32
        };
        pub const KEY_F30: Self = Self {
            ord: 4194361i32
        };
        pub const KEY_F31: Self = Self {
            ord: 4194362i32
        };
        pub const KEY_F32: Self = Self {
            ord: 4194363i32
        };
        pub const KEY_F33: Self = Self {
            ord: 4194364i32
        };
        pub const KEY_F34: Self = Self {
            ord: 4194365i32
        };
        pub const KEY_F35: Self = Self {
            ord: 4194366i32
        };
        pub const KEY_KP_MULTIPLY: Self = Self {
            ord: 4194433i32
        };
        pub const KEY_KP_DIVIDE: Self = Self {
            ord: 4194434i32
        };
        pub const KEY_KP_SUBTRACT: Self = Self {
            ord: 4194435i32
        };
        pub const KEY_KP_PERIOD: Self = Self {
            ord: 4194436i32
        };
        pub const KEY_KP_ADD: Self = Self {
            ord: 4194437i32
        };
        pub const KEY_KP_0: Self = Self {
            ord: 4194438i32
        };
        pub const KEY_KP_1: Self = Self {
            ord: 4194439i32
        };
        pub const KEY_KP_2: Self = Self {
            ord: 4194440i32
        };
        pub const KEY_KP_3: Self = Self {
            ord: 4194441i32
        };
        pub const KEY_KP_4: Self = Self {
            ord: 4194442i32
        };
        pub const KEY_KP_5: Self = Self {
            ord: 4194443i32
        };
        pub const KEY_KP_6: Self = Self {
            ord: 4194444i32
        };
        pub const KEY_KP_7: Self = Self {
            ord: 4194445i32
        };
        pub const KEY_KP_8: Self = Self {
            ord: 4194446i32
        };
        pub const KEY_KP_9: Self = Self {
            ord: 4194447i32
        };
        pub const KEY_MENU: Self = Self {
            ord: 4194370i32
        };
        pub const KEY_HYPER: Self = Self {
            ord: 4194371i32
        };
        pub const KEY_HELP: Self = Self {
            ord: 4194373i32
        };
        pub const KEY_BACK: Self = Self {
            ord: 4194376i32
        };
        pub const KEY_FORWARD: Self = Self {
            ord: 4194377i32
        };
        pub const KEY_STOP: Self = Self {
            ord: 4194378i32
        };
        pub const KEY_REFRESH: Self = Self {
            ord: 4194379i32
        };
        pub const KEY_VOLUMEDOWN: Self = Self {
            ord: 4194380i32
        };
        pub const KEY_VOLUMEMUTE: Self = Self {
            ord: 4194381i32
        };
        pub const KEY_VOLUMEUP: Self = Self {
            ord: 4194382i32
        };
        pub const KEY_MEDIAPLAY: Self = Self {
            ord: 4194388i32
        };
        pub const KEY_MEDIASTOP: Self = Self {
            ord: 4194389i32
        };
        pub const KEY_MEDIAPREVIOUS: Self = Self {
            ord: 4194390i32
        };
        pub const KEY_MEDIANEXT: Self = Self {
            ord: 4194391i32
        };
        pub const KEY_MEDIARECORD: Self = Self {
            ord: 4194392i32
        };
        pub const KEY_HOMEPAGE: Self = Self {
            ord: 4194393i32
        };
        pub const KEY_FAVORITES: Self = Self {
            ord: 4194394i32
        };
        pub const KEY_SEARCH: Self = Self {
            ord: 4194395i32
        };
        pub const KEY_STANDBY: Self = Self {
            ord: 4194396i32
        };
        pub const KEY_OPENURL: Self = Self {
            ord: 4194397i32
        };
        pub const KEY_LAUNCHMAIL: Self = Self {
            ord: 4194398i32
        };
        pub const KEY_LAUNCHMEDIA: Self = Self {
            ord: 4194399i32
        };
        pub const KEY_LAUNCH0: Self = Self {
            ord: 4194400i32
        };
        pub const KEY_LAUNCH1: Self = Self {
            ord: 4194401i32
        };
        pub const KEY_LAUNCH2: Self = Self {
            ord: 4194402i32
        };
        pub const KEY_LAUNCH3: Self = Self {
            ord: 4194403i32
        };
        pub const KEY_LAUNCH4: Self = Self {
            ord: 4194404i32
        };
        pub const KEY_LAUNCH5: Self = Self {
            ord: 4194405i32
        };
        pub const KEY_LAUNCH6: Self = Self {
            ord: 4194406i32
        };
        pub const KEY_LAUNCH7: Self = Self {
            ord: 4194407i32
        };
        pub const KEY_LAUNCH8: Self = Self {
            ord: 4194408i32
        };
        pub const KEY_LAUNCH9: Self = Self {
            ord: 4194409i32
        };
        pub const KEY_LAUNCHA: Self = Self {
            ord: 4194410i32
        };
        pub const KEY_LAUNCHB: Self = Self {
            ord: 4194411i32
        };
        pub const KEY_LAUNCHC: Self = Self {
            ord: 4194412i32
        };
        pub const KEY_LAUNCHD: Self = Self {
            ord: 4194413i32
        };
        pub const KEY_LAUNCHE: Self = Self {
            ord: 4194414i32
        };
        pub const KEY_LAUNCHF: Self = Self {
            ord: 4194415i32
        };
        pub const KEY_GLOBE: Self = Self {
            ord: 4194416i32
        };
        pub const KEY_KEYBOARD: Self = Self {
            ord: 4194417i32
        };
        pub const KEY_JIS_EISU: Self = Self {
            ord: 4194418i32
        };
        pub const KEY_JIS_KANA: Self = Self {
            ord: 4194419i32
        };
        pub const KEY_UNKNOWN: Self = Self {
            ord: 8388607i32
        };
        pub const KEY_SPACE: Self = Self {
            ord: 32i32
        };
        pub const KEY_EXCLAM: Self = Self {
            ord: 33i32
        };
        pub const KEY_QUOTEDBL: Self = Self {
            ord: 34i32
        };
        pub const KEY_NUMBERSIGN: Self = Self {
            ord: 35i32
        };
        pub const KEY_DOLLAR: Self = Self {
            ord: 36i32
        };
        pub const KEY_PERCENT: Self = Self {
            ord: 37i32
        };
        pub const KEY_AMPERSAND: Self = Self {
            ord: 38i32
        };
        pub const KEY_APOSTROPHE: Self = Self {
            ord: 39i32
        };
        pub const KEY_PARENLEFT: Self = Self {
            ord: 40i32
        };
        pub const KEY_PARENRIGHT: Self = Self {
            ord: 41i32
        };
        pub const KEY_ASTERISK: Self = Self {
            ord: 42i32
        };
        pub const KEY_PLUS: Self = Self {
            ord: 43i32
        };
        pub const KEY_COMMA: Self = Self {
            ord: 44i32
        };
        pub const KEY_MINUS: Self = Self {
            ord: 45i32
        };
        pub const KEY_PERIOD: Self = Self {
            ord: 46i32
        };
        pub const KEY_SLASH: Self = Self {
            ord: 47i32
        };
        pub const KEY_0: Self = Self {
            ord: 48i32
        };
        pub const KEY_1: Self = Self {
            ord: 49i32
        };
        pub const KEY_2: Self = Self {
            ord: 50i32
        };
        pub const KEY_3: Self = Self {
            ord: 51i32
        };
        pub const KEY_4: Self = Self {
            ord: 52i32
        };
        pub const KEY_5: Self = Self {
            ord: 53i32
        };
        pub const KEY_6: Self = Self {
            ord: 54i32
        };
        pub const KEY_7: Self = Self {
            ord: 55i32
        };
        pub const KEY_8: Self = Self {
            ord: 56i32
        };
        pub const KEY_9: Self = Self {
            ord: 57i32
        };
        pub const KEY_COLON: Self = Self {
            ord: 58i32
        };
        pub const KEY_SEMICOLON: Self = Self {
            ord: 59i32
        };
        pub const KEY_LESS: Self = Self {
            ord: 60i32
        };
        pub const KEY_EQUAL: Self = Self {
            ord: 61i32
        };
        pub const KEY_GREATER: Self = Self {
            ord: 62i32
        };
        pub const KEY_QUESTION: Self = Self {
            ord: 63i32
        };
        pub const KEY_AT: Self = Self {
            ord: 64i32
        };
        pub const KEY_A: Self = Self {
            ord: 65i32
        };
        pub const KEY_B: Self = Self {
            ord: 66i32
        };
        pub const KEY_C: Self = Self {
            ord: 67i32
        };
        pub const KEY_D: Self = Self {
            ord: 68i32
        };
        pub const KEY_E: Self = Self {
            ord: 69i32
        };
        pub const KEY_F: Self = Self {
            ord: 70i32
        };
        pub const KEY_G: Self = Self {
            ord: 71i32
        };
        pub const KEY_H: Self = Self {
            ord: 72i32
        };
        pub const KEY_I: Self = Self {
            ord: 73i32
        };
        pub const KEY_J: Self = Self {
            ord: 74i32
        };
        pub const KEY_K: Self = Self {
            ord: 75i32
        };
        pub const KEY_L: Self = Self {
            ord: 76i32
        };
        pub const KEY_M: Self = Self {
            ord: 77i32
        };
        pub const KEY_N: Self = Self {
            ord: 78i32
        };
        pub const KEY_O: Self = Self {
            ord: 79i32
        };
        pub const KEY_P: Self = Self {
            ord: 80i32
        };
        pub const KEY_Q: Self = Self {
            ord: 81i32
        };
        pub const KEY_R: Self = Self {
            ord: 82i32
        };
        pub const KEY_S: Self = Self {
            ord: 83i32
        };
        pub const KEY_T: Self = Self {
            ord: 84i32
        };
        pub const KEY_U: Self = Self {
            ord: 85i32
        };
        pub const KEY_V: Self = Self {
            ord: 86i32
        };
        pub const KEY_W: Self = Self {
            ord: 87i32
        };
        pub const KEY_X: Self = Self {
            ord: 88i32
        };
        pub const KEY_Y: Self = Self {
            ord: 89i32
        };
        pub const KEY_Z: Self = Self {
            ord: 90i32
        };
        pub const KEY_BRACKETLEFT: Self = Self {
            ord: 91i32
        };
        pub const KEY_BACKSLASH: Self = Self {
            ord: 92i32
        };
        pub const KEY_BRACKETRIGHT: Self = Self {
            ord: 93i32
        };
        pub const KEY_ASCIICIRCUM: Self = Self {
            ord: 94i32
        };
        pub const KEY_UNDERSCORE: Self = Self {
            ord: 95i32
        };
        pub const KEY_QUOTELEFT: Self = Self {
            ord: 96i32
        };
        pub const KEY_BRACELEFT: Self = Self {
            ord: 123i32
        };
        pub const KEY_BAR: Self = Self {
            ord: 124i32
        };
        pub const KEY_BRACERIGHT: Self = Self {
            ord: 125i32
        };
        pub const KEY_ASCIITILDE: Self = Self {
            ord: 126i32
        };
        pub const KEY_YEN: Self = Self {
            ord: 165i32
        };
        pub const KEY_SECTION: Self = Self {
            ord: 167i32
        };
        
    }
    impl crate::obj::EngineEnum for Key {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 | ord @ 60i32 | ord @ 61i32 | ord @ 62i32 | ord @ 63i32 | ord @ 64i32 | ord @ 65i32 | ord @ 66i32 | ord @ 67i32 | ord @ 68i32 | ord @ 69i32 | ord @ 70i32 | ord @ 71i32 | ord @ 72i32 | ord @ 73i32 | ord @ 74i32 | ord @ 75i32 | ord @ 76i32 | ord @ 77i32 | ord @ 78i32 | ord @ 79i32 | ord @ 80i32 | ord @ 81i32 | ord @ 82i32 | ord @ 83i32 | ord @ 84i32 | ord @ 85i32 | ord @ 86i32 | ord @ 87i32 | ord @ 88i32 | ord @ 89i32 | ord @ 90i32 | ord @ 91i32 | ord @ 92i32 | ord @ 93i32 | ord @ 94i32 | ord @ 95i32 | ord @ 96i32 | ord @ 123i32 | ord @ 124i32 | ord @ 125i32 | ord @ 126i32 | ord @ 165i32 | ord @ 167i32 | ord @ 4194304i32 | ord @ 4194305i32 | ord @ 4194306i32 | ord @ 4194307i32 | ord @ 4194308i32 | ord @ 4194309i32 | ord @ 4194310i32 | ord @ 4194311i32 | ord @ 4194312i32 | ord @ 4194313i32 | ord @ 4194314i32 | ord @ 4194315i32 | ord @ 4194316i32 | ord @ 4194317i32 | ord @ 4194318i32 | ord @ 4194319i32 | ord @ 4194320i32 | ord @ 4194321i32 | ord @ 4194322i32 | ord @ 4194323i32 | ord @ 4194324i32 | ord @ 4194325i32 | ord @ 4194326i32 | ord @ 4194327i32 | ord @ 4194328i32 | ord @ 4194329i32 | ord @ 4194330i32 | ord @ 4194331i32 | ord @ 4194332i32 | ord @ 4194333i32 | ord @ 4194334i32 | ord @ 4194335i32 | ord @ 4194336i32 | ord @ 4194337i32 | ord @ 4194338i32 | ord @ 4194339i32 | ord @ 4194340i32 | ord @ 4194341i32 | ord @ 4194342i32 | ord @ 4194343i32 | ord @ 4194344i32 | ord @ 4194345i32 | ord @ 4194346i32 | ord @ 4194347i32 | ord @ 4194348i32 | ord @ 4194349i32 | ord @ 4194350i32 | ord @ 4194351i32 | ord @ 4194352i32 | ord @ 4194353i32 | ord @ 4194354i32 | ord @ 4194355i32 | ord @ 4194356i32 | ord @ 4194357i32 | ord @ 4194358i32 | ord @ 4194359i32 | ord @ 4194360i32 | ord @ 4194361i32 | ord @ 4194362i32 | ord @ 4194363i32 | ord @ 4194364i32 | ord @ 4194365i32 | ord @ 4194366i32 | ord @ 4194370i32 | ord @ 4194371i32 | ord @ 4194373i32 | ord @ 4194376i32 | ord @ 4194377i32 | ord @ 4194378i32 | ord @ 4194379i32 | ord @ 4194380i32 | ord @ 4194381i32 | ord @ 4194382i32 | ord @ 4194388i32 | ord @ 4194389i32 | ord @ 4194390i32 | ord @ 4194391i32 | ord @ 4194392i32 | ord @ 4194393i32 | ord @ 4194394i32 | ord @ 4194395i32 | ord @ 4194396i32 | ord @ 4194397i32 | ord @ 4194398i32 | ord @ 4194399i32 | ord @ 4194400i32 | ord @ 4194401i32 | ord @ 4194402i32 | ord @ 4194403i32 | ord @ 4194404i32 | ord @ 4194405i32 | ord @ 4194406i32 | ord @ 4194407i32 | ord @ 4194408i32 | ord @ 4194409i32 | ord @ 4194410i32 | ord @ 4194411i32 | ord @ 4194412i32 | ord @ 4194413i32 | ord @ 4194414i32 | ord @ 4194415i32 | ord @ 4194416i32 | ord @ 4194417i32 | ord @ 4194418i32 | ord @ 4194419i32 | ord @ 4194433i32 | ord @ 4194434i32 | ord @ 4194435i32 | ord @ 4194436i32 | ord @ 4194437i32 | ord @ 4194438i32 | ord @ 4194439i32 | ord @ 4194440i32 | ord @ 4194441i32 | ord @ 4194442i32 | ord @ 4194443i32 | ord @ 4194444i32 | ord @ 4194445i32 | ord @ 4194446i32 | ord @ 4194447i32 | ord @ 8388607i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for Key {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for Key {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for Key {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct KeyModifierMask {
        ord: u64
    }
    impl KeyModifierMask {
        pub const KEY_CODE_MASK: Self = Self {
            ord: 8388607u64
        };
        pub const KEY_MODIFIER_MASK: Self = Self {
            ord: 532676608u64
        };
        pub const KEY_MASK_CMD_OR_CTRL: Self = Self {
            ord: 16777216u64
        };
        pub const KEY_MASK_SHIFT: Self = Self {
            ord: 33554432u64
        };
        pub const KEY_MASK_ALT: Self = Self {
            ord: 67108864u64
        };
        pub const KEY_MASK_META: Self = Self {
            ord: 134217728u64
        };
        pub const KEY_MASK_CTRL: Self = Self {
            ord: 268435456u64
        };
        pub const KEY_MASK_KPAD: Self = Self {
            ord: 536870912u64
        };
        pub const KEY_MASK_GROUP_SWITCH: Self = Self {
            ord: 1073741824u64
        };
        
    }
    impl crate::obj::EngineBitfield for KeyModifierMask {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for KeyModifierMask {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::builtin::meta::GodotConvert for KeyModifierMask {
        type Via = u64;
        
    }
    impl crate::builtin::meta::ToGodot for KeyModifierMask {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for KeyModifierMask {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MouseButton {
        ord: i32
    }
    impl MouseButton {
        pub const MOUSE_BUTTON_NONE: Self = Self {
            ord: 0i32
        };
        pub const MOUSE_BUTTON_LEFT: Self = Self {
            ord: 1i32
        };
        pub const MOUSE_BUTTON_RIGHT: Self = Self {
            ord: 2i32
        };
        pub const MOUSE_BUTTON_MIDDLE: Self = Self {
            ord: 3i32
        };
        pub const MOUSE_BUTTON_WHEEL_UP: Self = Self {
            ord: 4i32
        };
        pub const MOUSE_BUTTON_WHEEL_DOWN: Self = Self {
            ord: 5i32
        };
        pub const MOUSE_BUTTON_WHEEL_LEFT: Self = Self {
            ord: 6i32
        };
        pub const MOUSE_BUTTON_WHEEL_RIGHT: Self = Self {
            ord: 7i32
        };
        pub const MOUSE_BUTTON_XBUTTON1: Self = Self {
            ord: 8i32
        };
        pub const MOUSE_BUTTON_XBUTTON2: Self = Self {
            ord: 9i32
        };
        
    }
    impl crate::obj::EngineEnum for MouseButton {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for MouseButton {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for MouseButton {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for MouseButton {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct MouseButtonMask {
        ord: u64
    }
    impl MouseButtonMask {
        pub const MOUSE_BUTTON_MASK_LEFT: Self = Self {
            ord: 1u64
        };
        pub const MOUSE_BUTTON_MASK_RIGHT: Self = Self {
            ord: 2u64
        };
        pub const MOUSE_BUTTON_MASK_MIDDLE: Self = Self {
            ord: 4u64
        };
        pub const MOUSE_BUTTON_MASK_MB_XBUTTON1: Self = Self {
            ord: 128u64
        };
        pub const MOUSE_BUTTON_MASK_MB_XBUTTON2: Self = Self {
            ord: 256u64
        };
        
    }
    impl crate::obj::EngineBitfield for MouseButtonMask {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for MouseButtonMask {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::builtin::meta::GodotConvert for MouseButtonMask {
        type Via = u64;
        
    }
    impl crate::builtin::meta::ToGodot for MouseButtonMask {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for MouseButtonMask {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct JoyButton {
        ord: i32
    }
    impl JoyButton {
        pub const JOY_BUTTON_INVALID: Self = Self {
            ord: - 1i32
        };
        pub const JOY_BUTTON_A: Self = Self {
            ord: 0i32
        };
        pub const JOY_BUTTON_B: Self = Self {
            ord: 1i32
        };
        pub const JOY_BUTTON_X: Self = Self {
            ord: 2i32
        };
        pub const JOY_BUTTON_Y: Self = Self {
            ord: 3i32
        };
        pub const JOY_BUTTON_BACK: Self = Self {
            ord: 4i32
        };
        pub const JOY_BUTTON_GUIDE: Self = Self {
            ord: 5i32
        };
        pub const JOY_BUTTON_START: Self = Self {
            ord: 6i32
        };
        pub const JOY_BUTTON_LEFT_STICK: Self = Self {
            ord: 7i32
        };
        pub const JOY_BUTTON_RIGHT_STICK: Self = Self {
            ord: 8i32
        };
        pub const JOY_BUTTON_LEFT_SHOULDER: Self = Self {
            ord: 9i32
        };
        pub const JOY_BUTTON_RIGHT_SHOULDER: Self = Self {
            ord: 10i32
        };
        pub const JOY_BUTTON_DPAD_UP: Self = Self {
            ord: 11i32
        };
        pub const JOY_BUTTON_DPAD_DOWN: Self = Self {
            ord: 12i32
        };
        pub const JOY_BUTTON_DPAD_LEFT: Self = Self {
            ord: 13i32
        };
        pub const JOY_BUTTON_DPAD_RIGHT: Self = Self {
            ord: 14i32
        };
        pub const JOY_BUTTON_MISC1: Self = Self {
            ord: 15i32
        };
        pub const JOY_BUTTON_PADDLE1: Self = Self {
            ord: 16i32
        };
        pub const JOY_BUTTON_PADDLE2: Self = Self {
            ord: 17i32
        };
        pub const JOY_BUTTON_PADDLE3: Self = Self {
            ord: 18i32
        };
        pub const JOY_BUTTON_PADDLE4: Self = Self {
            ord: 19i32
        };
        pub const JOY_BUTTON_TOUCHPAD: Self = Self {
            ord: 20i32
        };
        pub const JOY_BUTTON_SDL_MAX: Self = Self {
            ord: 21i32
        };
        pub const JOY_BUTTON_MAX: Self = Self {
            ord: 128i32
        };
        
    }
    impl crate::obj::EngineEnum for JoyButton {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 128i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for JoyButton {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for JoyButton {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for JoyButton {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct JoyAxis {
        ord: i32
    }
    impl JoyAxis {
        pub const JOY_AXIS_INVALID: Self = Self {
            ord: - 1i32
        };
        pub const JOY_AXIS_LEFT_X: Self = Self {
            ord: 0i32
        };
        pub const JOY_AXIS_LEFT_Y: Self = Self {
            ord: 1i32
        };
        pub const JOY_AXIS_RIGHT_X: Self = Self {
            ord: 2i32
        };
        pub const JOY_AXIS_RIGHT_Y: Self = Self {
            ord: 3i32
        };
        pub const JOY_AXIS_TRIGGER_LEFT: Self = Self {
            ord: 4i32
        };
        pub const JOY_AXIS_TRIGGER_RIGHT: Self = Self {
            ord: 5i32
        };
        pub const JOY_AXIS_SDL_MAX: Self = Self {
            ord: 6i32
        };
        pub const JOY_AXIS_MAX: Self = Self {
            ord: 10i32
        };
        
    }
    impl crate::obj::EngineEnum for JoyAxis {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 10i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for JoyAxis {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for JoyAxis {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for JoyAxis {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MIDIMessage {
        ord: i32
    }
    impl MIDIMessage {
        pub const MIDI_MESSAGE_NONE: Self = Self {
            ord: 0i32
        };
        pub const MIDI_MESSAGE_NOTE_OFF: Self = Self {
            ord: 8i32
        };
        pub const MIDI_MESSAGE_NOTE_ON: Self = Self {
            ord: 9i32
        };
        pub const MIDI_MESSAGE_AFTERTOUCH: Self = Self {
            ord: 10i32
        };
        pub const MIDI_MESSAGE_CONTROL_CHANGE: Self = Self {
            ord: 11i32
        };
        pub const MIDI_MESSAGE_PROGRAM_CHANGE: Self = Self {
            ord: 12i32
        };
        pub const MIDI_MESSAGE_CHANNEL_PRESSURE: Self = Self {
            ord: 13i32
        };
        pub const MIDI_MESSAGE_PITCH_BEND: Self = Self {
            ord: 14i32
        };
        pub const MIDI_MESSAGE_SYSTEM_EXCLUSIVE: Self = Self {
            ord: 240i32
        };
        pub const MIDI_MESSAGE_QUARTER_FRAME: Self = Self {
            ord: 241i32
        };
        pub const MIDI_MESSAGE_SONG_POSITION_POINTER: Self = Self {
            ord: 242i32
        };
        pub const MIDI_MESSAGE_SONG_SELECT: Self = Self {
            ord: 243i32
        };
        pub const MIDI_MESSAGE_TUNE_REQUEST: Self = Self {
            ord: 246i32
        };
        pub const MIDI_MESSAGE_TIMING_CLOCK: Self = Self {
            ord: 248i32
        };
        pub const MIDI_MESSAGE_START: Self = Self {
            ord: 250i32
        };
        pub const MIDI_MESSAGE_CONTINUE: Self = Self {
            ord: 251i32
        };
        pub const MIDI_MESSAGE_STOP: Self = Self {
            ord: 252i32
        };
        pub const MIDI_MESSAGE_ACTIVE_SENSING: Self = Self {
            ord: 254i32
        };
        pub const MIDI_MESSAGE_SYSTEM_RESET: Self = Self {
            ord: 255i32
        };
        
    }
    impl crate::obj::EngineEnum for MIDIMessage {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 240i32 | ord @ 241i32 | ord @ 242i32 | ord @ 243i32 | ord @ 246i32 | ord @ 248i32 | ord @ 250i32 | ord @ 251i32 | ord @ 252i32 | ord @ 254i32 | ord @ 255i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for MIDIMessage {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for MIDIMessage {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for MIDIMessage {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Error {
        ord: i32
    }
    impl Error {
        pub const OK: Self = Self {
            ord: 0i32
        };
        pub const FAILED: Self = Self {
            ord: 1i32
        };
        pub const ERR_UNAVAILABLE: Self = Self {
            ord: 2i32
        };
        pub const ERR_UNCONFIGURED: Self = Self {
            ord: 3i32
        };
        pub const ERR_UNAUTHORIZED: Self = Self {
            ord: 4i32
        };
        pub const ERR_PARAMETER_RANGE_ERROR: Self = Self {
            ord: 5i32
        };
        pub const ERR_OUT_OF_MEMORY: Self = Self {
            ord: 6i32
        };
        pub const ERR_FILE_NOT_FOUND: Self = Self {
            ord: 7i32
        };
        pub const ERR_FILE_BAD_DRIVE: Self = Self {
            ord: 8i32
        };
        pub const ERR_FILE_BAD_PATH: Self = Self {
            ord: 9i32
        };
        pub const ERR_FILE_NO_PERMISSION: Self = Self {
            ord: 10i32
        };
        pub const ERR_FILE_ALREADY_IN_USE: Self = Self {
            ord: 11i32
        };
        pub const ERR_FILE_CANT_OPEN: Self = Self {
            ord: 12i32
        };
        pub const ERR_FILE_CANT_WRITE: Self = Self {
            ord: 13i32
        };
        pub const ERR_FILE_CANT_READ: Self = Self {
            ord: 14i32
        };
        pub const ERR_FILE_UNRECOGNIZED: Self = Self {
            ord: 15i32
        };
        pub const ERR_FILE_CORRUPT: Self = Self {
            ord: 16i32
        };
        pub const ERR_FILE_MISSING_DEPENDENCIES: Self = Self {
            ord: 17i32
        };
        pub const ERR_FILE_EOF: Self = Self {
            ord: 18i32
        };
        pub const ERR_CANT_OPEN: Self = Self {
            ord: 19i32
        };
        pub const ERR_CANT_CREATE: Self = Self {
            ord: 20i32
        };
        pub const ERR_QUERY_FAILED: Self = Self {
            ord: 21i32
        };
        pub const ERR_ALREADY_IN_USE: Self = Self {
            ord: 22i32
        };
        pub const ERR_LOCKED: Self = Self {
            ord: 23i32
        };
        pub const ERR_TIMEOUT: Self = Self {
            ord: 24i32
        };
        pub const ERR_CANT_CONNECT: Self = Self {
            ord: 25i32
        };
        pub const ERR_CANT_RESOLVE: Self = Self {
            ord: 26i32
        };
        pub const ERR_CONNECTION_ERROR: Self = Self {
            ord: 27i32
        };
        pub const ERR_CANT_ACQUIRE_RESOURCE: Self = Self {
            ord: 28i32
        };
        pub const ERR_CANT_FORK: Self = Self {
            ord: 29i32
        };
        pub const ERR_INVALID_DATA: Self = Self {
            ord: 30i32
        };
        pub const ERR_INVALID_PARAMETER: Self = Self {
            ord: 31i32
        };
        pub const ERR_ALREADY_EXISTS: Self = Self {
            ord: 32i32
        };
        pub const ERR_DOES_NOT_EXIST: Self = Self {
            ord: 33i32
        };
        pub const ERR_DATABASE_CANT_READ: Self = Self {
            ord: 34i32
        };
        pub const ERR_DATABASE_CANT_WRITE: Self = Self {
            ord: 35i32
        };
        pub const ERR_COMPILATION_FAILED: Self = Self {
            ord: 36i32
        };
        pub const ERR_METHOD_NOT_FOUND: Self = Self {
            ord: 37i32
        };
        pub const ERR_LINK_FAILED: Self = Self {
            ord: 38i32
        };
        pub const ERR_SCRIPT_FAILED: Self = Self {
            ord: 39i32
        };
        pub const ERR_CYCLIC_LINK: Self = Self {
            ord: 40i32
        };
        pub const ERR_INVALID_DECLARATION: Self = Self {
            ord: 41i32
        };
        pub const ERR_DUPLICATE_SYMBOL: Self = Self {
            ord: 42i32
        };
        pub const ERR_PARSE_ERROR: Self = Self {
            ord: 43i32
        };
        pub const ERR_BUSY: Self = Self {
            ord: 44i32
        };
        pub const ERR_SKIP: Self = Self {
            ord: 45i32
        };
        pub const ERR_HELP: Self = Self {
            ord: 46i32
        };
        pub const ERR_BUG: Self = Self {
            ord: 47i32
        };
        pub const ERR_PRINTER_ON_FIRE: Self = Self {
            ord: 48i32
        };
        
    }
    impl crate::obj::EngineEnum for Error {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::builtin::meta::GodotConvert for Error {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for Error {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for Error {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct PropertyHint {
        ord: i32
    }
    impl PropertyHint {
        pub const PROPERTY_HINT_NONE: Self = Self {
            ord: 0i32
        };
        pub const PROPERTY_HINT_RANGE: Self = Self {
            ord: 1i32
        };
        pub const PROPERTY_HINT_ENUM: Self = Self {
            ord: 2i32
        };
        pub const PROPERTY_HINT_ENUM_SUGGESTION: Self = Self {
            ord: 3i32
        };
        pub const PROPERTY_HINT_EXP_EASING: Self = Self {
            ord: 4i32
        };
        pub const PROPERTY_HINT_LINK: Self = Self {
            ord: 5i32
        };
        pub const PROPERTY_HINT_FLAGS: Self = Self {
            ord: 6i32
        };
        pub const PROPERTY_HINT_LAYERS_2D_RENDER: Self = Self {
            ord: 7i32
        };
        pub const PROPERTY_HINT_LAYERS_2D_PHYSICS: Self = Self {
            ord: 8i32
        };
        pub const PROPERTY_HINT_LAYERS_2D_NAVIGATION: Self = Self {
            ord: 9i32
        };
        pub const PROPERTY_HINT_LAYERS_3D_RENDER: Self = Self {
            ord: 10i32
        };
        pub const PROPERTY_HINT_LAYERS_3D_PHYSICS: Self = Self {
            ord: 11i32
        };
        pub const PROPERTY_HINT_LAYERS_3D_NAVIGATION: Self = Self {
            ord: 12i32
        };
        pub const PROPERTY_HINT_LAYERS_AVOIDANCE: Self = Self {
            ord: 37i32
        };
        pub const PROPERTY_HINT_FILE: Self = Self {
            ord: 13i32
        };
        pub const PROPERTY_HINT_DIR: Self = Self {
            ord: 14i32
        };
        pub const PROPERTY_HINT_GLOBAL_FILE: Self = Self {
            ord: 15i32
        };
        pub const PROPERTY_HINT_GLOBAL_DIR: Self = Self {
            ord: 16i32
        };
        pub const PROPERTY_HINT_RESOURCE_TYPE: Self = Self {
            ord: 17i32
        };
        pub const PROPERTY_HINT_MULTILINE_TEXT: Self = Self {
            ord: 18i32
        };
        pub const PROPERTY_HINT_EXPRESSION: Self = Self {
            ord: 19i32
        };
        pub const PROPERTY_HINT_PLACEHOLDER_TEXT: Self = Self {
            ord: 20i32
        };
        pub const PROPERTY_HINT_COLOR_NO_ALPHA: Self = Self {
            ord: 21i32
        };
        pub const PROPERTY_HINT_OBJECT_ID: Self = Self {
            ord: 22i32
        };
        pub const PROPERTY_HINT_TYPE_STRING: Self = Self {
            ord: 23i32
        };
        pub const PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE: Self = Self {
            ord: 24i32
        };
        pub const PROPERTY_HINT_OBJECT_TOO_BIG: Self = Self {
            ord: 25i32
        };
        pub const PROPERTY_HINT_NODE_PATH_VALID_TYPES: Self = Self {
            ord: 26i32
        };
        pub const PROPERTY_HINT_SAVE_FILE: Self = Self {
            ord: 27i32
        };
        pub const PROPERTY_HINT_GLOBAL_SAVE_FILE: Self = Self {
            ord: 28i32
        };
        pub const PROPERTY_HINT_INT_IS_OBJECTID: Self = Self {
            ord: 29i32
        };
        pub const PROPERTY_HINT_INT_IS_POINTER: Self = Self {
            ord: 30i32
        };
        pub const PROPERTY_HINT_ARRAY_TYPE: Self = Self {
            ord: 31i32
        };
        pub const PROPERTY_HINT_LOCALE_ID: Self = Self {
            ord: 32i32
        };
        pub const PROPERTY_HINT_LOCALIZABLE_STRING: Self = Self {
            ord: 33i32
        };
        pub const PROPERTY_HINT_NODE_TYPE: Self = Self {
            ord: 34i32
        };
        pub const PROPERTY_HINT_HIDE_QUATERNION_EDIT: Self = Self {
            ord: 35i32
        };
        pub const PROPERTY_HINT_PASSWORD: Self = Self {
            ord: 36i32
        };
        pub const PROPERTY_HINT_MAX: Self = Self {
            ord: 38i32
        };
        
    }
    impl crate::obj::EngineEnum for PropertyHint {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 => Some(Self {
                    ord
                }), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    impl crate::obj::IndexEnum for PropertyHint {
        const ENUMERATOR_COUNT: usize = 38usize;
        
    }
    impl crate::builtin::meta::GodotConvert for PropertyHint {
        type Via = i32;
        
    }
    impl crate::builtin::meta::ToGodot for PropertyHint {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for PropertyHint {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct PropertyUsageFlags {
        ord: u64
    }
    impl PropertyUsageFlags {
        pub const PROPERTY_USAGE_NONE: Self = Self {
            ord: 0u64
        };
        pub const PROPERTY_USAGE_STORAGE: Self = Self {
            ord: 2u64
        };
        pub const PROPERTY_USAGE_EDITOR: Self = Self {
            ord: 4u64
        };
        pub const PROPERTY_USAGE_INTERNAL: Self = Self {
            ord: 8u64
        };
        pub const PROPERTY_USAGE_CHECKABLE: Self = Self {
            ord: 16u64
        };
        pub const PROPERTY_USAGE_CHECKED: Self = Self {
            ord: 32u64
        };
        pub const PROPERTY_USAGE_GROUP: Self = Self {
            ord: 64u64
        };
        pub const PROPERTY_USAGE_CATEGORY: Self = Self {
            ord: 128u64
        };
        pub const PROPERTY_USAGE_SUBGROUP: Self = Self {
            ord: 256u64
        };
        pub const PROPERTY_USAGE_CLASS_IS_BITFIELD: Self = Self {
            ord: 512u64
        };
        pub const PROPERTY_USAGE_NO_INSTANCE_STATE: Self = Self {
            ord: 1024u64
        };
        pub const PROPERTY_USAGE_RESTART_IF_CHANGED: Self = Self {
            ord: 2048u64
        };
        pub const PROPERTY_USAGE_SCRIPT_VARIABLE: Self = Self {
            ord: 4096u64
        };
        pub const PROPERTY_USAGE_STORE_IF_NULL: Self = Self {
            ord: 8192u64
        };
        pub const PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED: Self = Self {
            ord: 16384u64
        };
        pub const PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE: Self = Self {
            ord: 32768u64
        };
        pub const PROPERTY_USAGE_CLASS_IS_ENUM: Self = Self {
            ord: 65536u64
        };
        pub const PROPERTY_USAGE_NIL_IS_VARIANT: Self = Self {
            ord: 131072u64
        };
        pub const PROPERTY_USAGE_ARRAY: Self = Self {
            ord: 262144u64
        };
        pub const PROPERTY_USAGE_ALWAYS_DUPLICATE: Self = Self {
            ord: 524288u64
        };
        pub const PROPERTY_USAGE_NEVER_DUPLICATE: Self = Self {
            ord: 1048576u64
        };
        pub const PROPERTY_USAGE_HIGH_END_GFX: Self = Self {
            ord: 2097152u64
        };
        pub const PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT: Self = Self {
            ord: 4194304u64
        };
        pub const PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT: Self = Self {
            ord: 8388608u64
        };
        pub const PROPERTY_USAGE_KEYING_INCREMENTS: Self = Self {
            ord: 16777216u64
        };
        pub const PROPERTY_USAGE_DEFERRED_SET_RESOURCE: Self = Self {
            ord: 33554432u64
        };
        pub const PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT: Self = Self {
            ord: 67108864u64
        };
        pub const PROPERTY_USAGE_EDITOR_BASIC_SETTING: Self = Self {
            ord: 134217728u64
        };
        pub const PROPERTY_USAGE_READ_ONLY: Self = Self {
            ord: 268435456u64
        };
        pub const PROPERTY_USAGE_SECRET: Self = Self {
            ord: 536870912u64
        };
        pub const PROPERTY_USAGE_DEFAULT: Self = Self {
            ord: 6u64
        };
        pub const PROPERTY_USAGE_NO_EDITOR: Self = Self {
            ord: 2u64
        };
        
    }
    impl crate::obj::EngineBitfield for PropertyUsageFlags {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for PropertyUsageFlags {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::builtin::meta::GodotConvert for PropertyUsageFlags {
        type Via = u64;
        
    }
    impl crate::builtin::meta::ToGodot for PropertyUsageFlags {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for PropertyUsageFlags {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct MethodFlags {
        ord: u64
    }
    impl MethodFlags {
        pub const METHOD_FLAG_NORMAL: Self = Self {
            ord: 1u64
        };
        pub const METHOD_FLAG_EDITOR: Self = Self {
            ord: 2u64
        };
        pub const METHOD_FLAG_CONST: Self = Self {
            ord: 4u64
        };
        pub const METHOD_FLAG_VIRTUAL: Self = Self {
            ord: 8u64
        };
        pub const METHOD_FLAG_VARARG: Self = Self {
            ord: 16u64
        };
        pub const METHOD_FLAG_STATIC: Self = Self {
            ord: 32u64
        };
        pub const METHOD_FLAG_OBJECT_CORE: Self = Self {
            ord: 64u64
        };
        pub const METHOD_FLAGS_DEFAULT: Self = Self {
            ord: 1u64
        };
        
    }
    impl crate::obj::EngineBitfield for MethodFlags {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for MethodFlags {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::builtin::meta::GodotConvert for MethodFlags {
        type Via = u64;
        
    }
    impl crate::builtin::meta::ToGodot for MethodFlags {
        fn to_godot(&self) -> Self::Via {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::builtin::meta::FromGodot for MethodFlags {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
        }
    }
}