
extends Node

var property_i64: int
var property_array_i64: Array[int]
@export var export_i64: int
@export var export_array_i64: Array[int]
var property_i32: int
var property_array_i32: Array[int]
@export var export_i32: int
@export var export_array_i32: Array[int]
var property_u32: int
var property_array_u32: Array[int]
@export var export_u32: int
@export var export_array_u32: Array[int]
var property_i16: int
var property_array_i16: Array[int]
@export var export_i16: int
@export var export_array_i16: Array[int]
var property_u16: int
var property_array_u16: Array[int]
@export var export_u16: int
@export var export_array_u16: Array[int]
var property_i8: int
var property_array_i8: Array[int]
@export var export_i8: int
@export var export_array_i8: Array[int]
var property_u8: int
var property_array_u8: Array[int]
@export var export_u8: int
@export var export_array_u8: Array[int]
var property_f32: float
var property_array_f32: Array[float]
@export var export_f32: float
@export var export_array_f32: Array[float]
var property_f64: float
var property_array_f64: Array[float]
@export var export_f64: float
@export var export_array_f64: Array[float]
var property_bool: bool
var property_array_bool: Array[bool]
@export var export_bool: bool
@export var export_array_bool: Array[bool]
var property_color: Color
var property_array_color: Array[Color]
@export var export_color: Color
@export var export_array_color: Array[Color]
var property_gstring: String
var property_array_gstring: Array[String]
@export var export_gstring: String
@export var export_array_gstring: Array[String]
var property_stringname: StringName
var property_array_stringname: Array[StringName]
@export var export_stringname: StringName
@export var export_array_stringname: Array[StringName]
var property_nodepath: NodePath
var property_array_nodepath: Array[NodePath]
@export var export_nodepath: NodePath
@export var export_array_nodepath: Array[NodePath]
var property_vector2: Vector2
var property_array_vector2: Array[Vector2]
@export var export_vector2: Vector2
@export var export_array_vector2: Array[Vector2]
var property_vector3: Vector3
var property_array_vector3: Array[Vector3]
@export var export_vector3: Vector3
@export var export_array_vector3: Array[Vector3]
var property_vector4: Vector4
var property_array_vector4: Array[Vector4]
@export var export_vector4: Vector4
@export var export_array_vector4: Array[Vector4]
var property_vector2i: Vector2i
var property_array_vector2i: Array[Vector2i]
@export var export_vector2i: Vector2i
@export var export_array_vector2i: Array[Vector2i]
var property_vector3i: Vector3i
var property_array_vector3i: Array[Vector3i]
@export var export_vector3i: Vector3i
@export var export_array_vector3i: Array[Vector3i]
var property_vector4i: Vector4i
var property_array_vector4i: Array[Vector4i]
@export var export_vector4i: Vector4i
@export var export_array_vector4i: Array[Vector4i]
var property_callable: Callable
var property_array_callable: Array[Callable]
var property_rect2: Rect2
var property_array_rect2: Array[Rect2]
@export var export_rect2: Rect2
@export var export_array_rect2: Array[Rect2]
var property_rect2i: Rect2i
var property_array_rect2i: Array[Rect2i]
@export var export_rect2i: Rect2i
@export var export_array_rect2i: Array[Rect2i]
var property_transform2d: Transform2D
var property_array_transform2d: Array[Transform2D]
@export var export_transform2d: Transform2D
@export var export_array_transform2d: Array[Transform2D]
var property_plane: Plane
var property_array_plane: Array[Plane]
@export var export_plane: Plane
@export var export_array_plane: Array[Plane]
var property_quaternion: Quaternion
var property_array_quaternion: Array[Quaternion]
@export var export_quaternion: Quaternion
@export var export_array_quaternion: Array[Quaternion]
var property_aabb: AABB
var property_array_aabb: Array[AABB]
@export var export_aabb: AABB
@export var export_array_aabb: Array[AABB]
var property_basis: Basis
var property_array_basis: Array[Basis]
@export var export_basis: Basis
@export var export_array_basis: Array[Basis]
var property_transform3d: Transform3D
var property_array_transform3d: Array[Transform3D]
@export var export_transform3d: Transform3D
@export var export_array_transform3d: Array[Transform3D]
var property_projection: Projection
var property_array_projection: Array[Projection]
@export var export_projection: Projection
@export var export_array_projection: Array[Projection]
var property_rid: RID
var property_array_rid: Array[RID]
var property_option_gd_node: Node
var property_array_option_gd_node: Array[Node]
@export var export_option_gd_node: Node
@export var export_array_option_gd_node: Array[Node]
var property_option_gd_resource: Resource
var property_array_option_gd_resource: Array[Resource]
@export var export_option_gd_resource: Resource
@export var export_array_option_gd_resource: Array[Resource]
var property_packedbytearray: PackedByteArray
var property_array_packedbytearray: Array[PackedByteArray]
@export var export_packedbytearray: PackedByteArray
@export var export_array_packedbytearray: Array[PackedByteArray]
var property_packedint32array: PackedInt32Array
var property_array_packedint32array: Array[PackedInt32Array]
@export var export_packedint32array: PackedInt32Array
@export var export_array_packedint32array: Array[PackedInt32Array]
var property_packedint64array: PackedInt64Array
var property_array_packedint64array: Array[PackedInt64Array]
@export var export_packedint64array: PackedInt64Array
@export var export_array_packedint64array: Array[PackedInt64Array]
var property_packedfloat32array: PackedFloat32Array
var property_array_packedfloat32array: Array[PackedFloat32Array]
@export var export_packedfloat32array: PackedFloat32Array
@export var export_array_packedfloat32array: Array[PackedFloat32Array]
var property_packedfloat64array: PackedFloat64Array
var property_array_packedfloat64array: Array[PackedFloat64Array]
@export var export_packedfloat64array: PackedFloat64Array
@export var export_array_packedfloat64array: Array[PackedFloat64Array]
var property_packedstringarray: PackedStringArray
var property_array_packedstringarray: Array[PackedStringArray]
@export var export_packedstringarray: PackedStringArray
@export var export_array_packedstringarray: Array[PackedStringArray]
var property_packedvector2array: PackedVector2Array
var property_array_packedvector2array: Array[PackedVector2Array]
@export var export_packedvector2array: PackedVector2Array
@export var export_array_packedvector2array: Array[PackedVector2Array]
var property_packedvector3array: PackedVector3Array
var property_array_packedvector3array: Array[PackedVector3Array]
@export var export_packedvector3array: PackedVector3Array
@export var export_array_packedvector3array: Array[PackedVector3Array]
var property_packedcolorarray: PackedColorArray
var property_array_packedcolorarray: Array[PackedColorArray]
@export var export_packedcolorarray: PackedColorArray
@export var export_array_packedcolorarray: Array[PackedColorArray]
var property_variantarray: Array
var property_array_variantarray: Array[Array]
@export var export_variantarray: Array
@export var export_array_variantarray: Array[Array]
var property_dictionary: Dictionary
var property_array_dictionary: Array[Dictionary]
@export var export_dictionary: Dictionary
@export var export_array_dictionary: Array[Dictionary]
@export_file var export_file: String
@export_file("*.txt") var export_file_wildcard_txt: String
@export_global_file var export_global_file: String
@export_global_file("*.png") var export_global_file_wildcard_png: String
@export_dir var export_dir: String
@export_global_dir var export_global_dir: String
@export_multiline var export_multiline: String
@export_range(0, 20) var export_range_float_0_20: float
@export_range(-10, 20, 0.2) var export_range_float_neg10_20_02: float
@export_range(0, 100, 1, "or_greater", "or_less") var export_range_int_0_100_1_or_greater_or_less: int
@export_exp_easing var export_exp_easing: float
@export_color_no_alpha var export_color_no_alpha: Color
@export_node_path("Button", "TouchScreenButton") var export_node_path_button_touch_screen_button: NodePath
@export_flags("Fire", "Water", "Earth", "Wind") var export_flags_fire_water_earth_wind: int
@export_flags("Self:4", "Allies:8", "Foes:16") var export_flags_self_4_allies_8_foes_16: int
@export_flags_2d_physics var export_flags_2d_physics: int
@export_flags_2d_render var export_flags_2d_render: int
@export_flags_2d_navigation var export_flags_2d_navigation: int
@export_flags_3d_physics var export_flags_3d_physics: int
@export_flags_3d_render var export_flags_3d_render: int
@export_flags_3d_navigation var export_flags_3d_navigation: int
@export_enum("Warrior", "Magician", "Thief") var export_enum_int_warrior_magician_thief: int
@export_enum("Slow:30", "Average:60", "VeryFast:200") var export_enum_int_slow_30_average_60_very_fast_200: int
@export_enum("Rebecca", "Mary", "Leah") var export_enum_string_rebecca_mary_leah: String
