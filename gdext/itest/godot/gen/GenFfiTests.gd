# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

extends TestSuite

# Note: GDScript only uses ptrcalls if it has the full type information available at "compile" (parse) time.
# That includes all arguments (including receiver) as well as function signature (parameters + return type).
# Otherwise, GDScript will use varcall. Both are tested below.
# It is thus important that `ffi` is initialized using = for varcalls, and using := for ptrcalls.


func test_varcall_i64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i64()
	assert_that(ffi.accept_i64(from_rust), "ffi.accept_i64(from_rust)")

	var from_gdscript: Variant = -922337203685477580
	var mirrored: Variant = ffi.mirror_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i32()
	assert_that(ffi.accept_i32(from_rust), "ffi.accept_i32(from_rust)")

	var from_gdscript: Variant = -2147483648
	var mirrored: Variant = ffi.mirror_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u32()
	assert_that(ffi.accept_u32(from_rust), "ffi.accept_u32(from_rust)")

	var from_gdscript: Variant = 4294967295
	var mirrored: Variant = ffi.mirror_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i16()
	assert_that(ffi.accept_i16(from_rust), "ffi.accept_i16(from_rust)")

	var from_gdscript: Variant = -32767
	var mirrored: Variant = ffi.mirror_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u16()
	assert_that(ffi.accept_u16(from_rust), "ffi.accept_u16(from_rust)")

	var from_gdscript: Variant = 65535
	var mirrored: Variant = ffi.mirror_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i8()
	assert_that(ffi.accept_i8(from_rust), "ffi.accept_i8(from_rust)")

	var from_gdscript: Variant = -128
	var mirrored: Variant = ffi.mirror_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u8()
	assert_that(ffi.accept_u8(from_rust), "ffi.accept_u8(from_rust)")

	var from_gdscript: Variant = 255
	var mirrored: Variant = ffi.mirror_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_f32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_f32()
	assert_that(ffi.accept_f32(from_rust), "ffi.accept_f32(from_rust)")

	var from_gdscript: Variant = 12.5
	var mirrored: Variant = ffi.mirror_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_f64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_f64()
	assert_that(ffi.accept_f64(from_rust), "ffi.accept_f64(from_rust)")

	var from_gdscript: Variant = 127.83156478
	var mirrored: Variant = ffi.mirror_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_bool():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_bool()
	assert_that(ffi.accept_bool(from_rust), "ffi.accept_bool(from_rust)")

	var from_gdscript: Variant = true
	var mirrored: Variant = ffi.mirror_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_color():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_color()
	assert_that(ffi.accept_color(from_rust), "ffi.accept_color(from_rust)")

	var from_gdscript: Variant = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Variant = ffi.mirror_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_gstring():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_gstring()
	assert_that(ffi.accept_gstring(from_rust), "ffi.accept_gstring(from_rust)")

	var from_gdscript: Variant = "hello"
	var mirrored: Variant = ffi.mirror_gstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_stringname():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_stringname()
	assert_that(ffi.accept_stringname(from_rust), "ffi.accept_stringname(from_rust)")

	var from_gdscript: Variant = &"hello"
	var mirrored: Variant = ffi.mirror_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_nodepath():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_nodepath()
	assert_that(ffi.accept_nodepath(from_rust), "ffi.accept_nodepath(from_rust)")

	var from_gdscript: Variant = ^"hello"
	var mirrored: Variant = ffi.mirror_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector2():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector2()
	assert_that(ffi.accept_vector2(from_rust), "ffi.accept_vector2(from_rust)")

	var from_gdscript: Variant = Vector2(12.5, -3.5)
	var mirrored: Variant = ffi.mirror_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector3():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector3()
	assert_that(ffi.accept_vector3(from_rust), "ffi.accept_vector3(from_rust)")

	var from_gdscript: Variant = Vector3(117.5, 100.0, -323.25)
	var mirrored: Variant = ffi.mirror_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector4():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector4()
	assert_that(ffi.accept_vector4(from_rust), "ffi.accept_vector4(from_rust)")

	var from_gdscript: Variant = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Variant = ffi.mirror_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector2i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector2i()
	assert_that(ffi.accept_vector2i(from_rust), "ffi.accept_vector2i(from_rust)")

	var from_gdscript: Variant = Vector2i(-2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector3i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector3i()
	assert_that(ffi.accept_vector3i(from_rust), "ffi.accept_vector3i(from_rust)")

	var from_gdscript: Variant = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector4i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector4i()
	assert_that(ffi.accept_vector4i(from_rust), "ffi.accept_vector4i(from_rust)")

	var from_gdscript: Variant = Vector4i(-1, -2147483648, 2147483647, 1000)
	var mirrored: Variant = ffi.mirror_vector4i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_callable():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_callable()
	assert_that(ffi.accept_callable(from_rust), "ffi.accept_callable(from_rust)")

	var from_gdscript: Variant = Callable()
	var mirrored: Variant = ffi.mirror_callable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_rect2():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_rect2()
	assert_that(ffi.accept_rect2(from_rust), "ffi.accept_rect2(from_rust)")

	var from_gdscript: Variant = Rect2()
	var mirrored: Variant = ffi.mirror_rect2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_rect2i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_rect2i()
	assert_that(ffi.accept_rect2i(from_rust), "ffi.accept_rect2i(from_rust)")

	var from_gdscript: Variant = Rect2i()
	var mirrored: Variant = ffi.mirror_rect2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_transform2d():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_transform2d()
	assert_that(ffi.accept_transform2d(from_rust), "ffi.accept_transform2d(from_rust)")

	var from_gdscript: Variant = Transform2D()
	var mirrored: Variant = ffi.mirror_transform2d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_plane():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_plane()
	assert_that(ffi.accept_plane(from_rust), "ffi.accept_plane(from_rust)")

	var from_gdscript: Variant = Plane()
	var mirrored: Variant = ffi.mirror_plane(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_quaternion():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_quaternion()
	assert_that(ffi.accept_quaternion(from_rust), "ffi.accept_quaternion(from_rust)")

	var from_gdscript: Variant = Quaternion()
	var mirrored: Variant = ffi.mirror_quaternion(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_aabb():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_aabb()
	assert_that(ffi.accept_aabb(from_rust), "ffi.accept_aabb(from_rust)")

	var from_gdscript: Variant = AABB()
	var mirrored: Variant = ffi.mirror_aabb(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_basis():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_basis()
	assert_that(ffi.accept_basis(from_rust), "ffi.accept_basis(from_rust)")

	var from_gdscript: Variant = Basis()
	var mirrored: Variant = ffi.mirror_basis(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_transform3d():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_transform3d()
	assert_that(ffi.accept_transform3d(from_rust), "ffi.accept_transform3d(from_rust)")

	var from_gdscript: Variant = Transform3D()
	var mirrored: Variant = ffi.mirror_transform3d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_projection():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_projection()
	assert_that(ffi.accept_projection(from_rust), "ffi.accept_projection(from_rust)")

	var from_gdscript: Variant = Projection()
	var mirrored: Variant = ffi.mirror_projection(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_rid():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_rid()
	assert_that(ffi.accept_rid(from_rust), "ffi.accept_rid(from_rust)")

	var from_gdscript: Variant = RID()
	var mirrored: Variant = ffi.mirror_rid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_option_gd_node():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_option_gd_node()
	assert_that(ffi.accept_option_gd_node(from_rust), "ffi.accept_option_gd_node(from_rust)")

	var from_gdscript: Variant = null
	var mirrored: Variant = ffi.mirror_option_gd_node(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_option_gd_resource():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_option_gd_resource()
	assert_that(ffi.accept_option_gd_resource(from_rust), "ffi.accept_option_gd_resource(from_rust)")

	var from_gdscript: Variant = null
	var mirrored: Variant = ffi.mirror_option_gd_resource(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedbytearray():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedbytearray()
	assert_that(ffi.accept_packedbytearray(from_rust), "ffi.accept_packedbytearray(from_rust)")

	var from_gdscript: Variant = PackedByteArray()
	var mirrored: Variant = ffi.mirror_packedbytearray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedint32array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedint32array()
	assert_that(ffi.accept_packedint32array(from_rust), "ffi.accept_packedint32array(from_rust)")

	var from_gdscript: Variant = PackedInt32Array()
	var mirrored: Variant = ffi.mirror_packedint32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedint64array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedint64array()
	assert_that(ffi.accept_packedint64array(from_rust), "ffi.accept_packedint64array(from_rust)")

	var from_gdscript: Variant = PackedInt64Array()
	var mirrored: Variant = ffi.mirror_packedint64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedfloat32array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedfloat32array()
	assert_that(ffi.accept_packedfloat32array(from_rust), "ffi.accept_packedfloat32array(from_rust)")

	var from_gdscript: Variant = PackedFloat32Array()
	var mirrored: Variant = ffi.mirror_packedfloat32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedfloat64array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedfloat64array()
	assert_that(ffi.accept_packedfloat64array(from_rust), "ffi.accept_packedfloat64array(from_rust)")

	var from_gdscript: Variant = PackedFloat64Array()
	var mirrored: Variant = ffi.mirror_packedfloat64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedstringarray():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedstringarray()
	assert_that(ffi.accept_packedstringarray(from_rust), "ffi.accept_packedstringarray(from_rust)")

	var from_gdscript: Variant = PackedStringArray()
	var mirrored: Variant = ffi.mirror_packedstringarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedvector2array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedvector2array()
	assert_that(ffi.accept_packedvector2array(from_rust), "ffi.accept_packedvector2array(from_rust)")

	var from_gdscript: Variant = PackedVector2Array()
	var mirrored: Variant = ffi.mirror_packedvector2array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedvector3array():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedvector3array()
	assert_that(ffi.accept_packedvector3array(from_rust), "ffi.accept_packedvector3array(from_rust)")

	var from_gdscript: Variant = PackedVector3Array()
	var mirrored: Variant = ffi.mirror_packedvector3array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_packedcolorarray():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_packedcolorarray()
	assert_that(ffi.accept_packedcolorarray(from_rust), "ffi.accept_packedcolorarray(from_rust)")

	var from_gdscript: Variant = PackedColorArray()
	var mirrored: Variant = ffi.mirror_packedcolorarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newi64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newi64()
	assert_that(ffi.accept_newi64(from_rust), "ffi.accept_newi64(from_rust)")

	var from_gdscript: Variant = -922337203685477580
	var mirrored: Variant = ffi.mirror_newi64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newi32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newi32()
	assert_that(ffi.accept_newi32(from_rust), "ffi.accept_newi32(from_rust)")

	var from_gdscript: Variant = -2147483648
	var mirrored: Variant = ffi.mirror_newi32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newu32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newu32()
	assert_that(ffi.accept_newu32(from_rust), "ffi.accept_newu32(from_rust)")

	var from_gdscript: Variant = 4294967295
	var mirrored: Variant = ffi.mirror_newu32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newi16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newi16()
	assert_that(ffi.accept_newi16(from_rust), "ffi.accept_newi16(from_rust)")

	var from_gdscript: Variant = -32767
	var mirrored: Variant = ffi.mirror_newi16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newu16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newu16()
	assert_that(ffi.accept_newu16(from_rust), "ffi.accept_newu16(from_rust)")

	var from_gdscript: Variant = 65535
	var mirrored: Variant = ffi.mirror_newu16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newi8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newi8()
	assert_that(ffi.accept_newi8(from_rust), "ffi.accept_newi8(from_rust)")

	var from_gdscript: Variant = -128
	var mirrored: Variant = ffi.mirror_newi8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newu8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newu8()
	assert_that(ffi.accept_newu8(from_rust), "ffi.accept_newu8(from_rust)")

	var from_gdscript: Variant = 255
	var mirrored: Variant = ffi.mirror_newu8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newf32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newf32()
	assert_that(ffi.accept_newf32(from_rust), "ffi.accept_newf32(from_rust)")

	var from_gdscript: Variant = 12.5
	var mirrored: Variant = ffi.mirror_newf32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newf64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newf64()
	assert_that(ffi.accept_newf64(from_rust), "ffi.accept_newf64(from_rust)")

	var from_gdscript: Variant = 127.83156478
	var mirrored: Variant = ffi.mirror_newf64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newbool():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newbool()
	assert_that(ffi.accept_newbool(from_rust), "ffi.accept_newbool(from_rust)")

	var from_gdscript: Variant = true
	var mirrored: Variant = ffi.mirror_newbool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newcolor():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newcolor()
	assert_that(ffi.accept_newcolor(from_rust), "ffi.accept_newcolor(from_rust)")

	var from_gdscript: Variant = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Variant = ffi.mirror_newcolor(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newstring():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newstring()
	assert_that(ffi.accept_newstring(from_rust), "ffi.accept_newstring(from_rust)")

	var from_gdscript: Variant = "hello"
	var mirrored: Variant = ffi.mirror_newstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newstringname():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newstringname()
	assert_that(ffi.accept_newstringname(from_rust), "ffi.accept_newstringname(from_rust)")

	var from_gdscript: Variant = &"hello"
	var mirrored: Variant = ffi.mirror_newstringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newnodepath():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newnodepath()
	assert_that(ffi.accept_newnodepath(from_rust), "ffi.accept_newnodepath(from_rust)")

	var from_gdscript: Variant = ^"hello"
	var mirrored: Variant = ffi.mirror_newnodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newvector2():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newvector2()
	assert_that(ffi.accept_newvector2(from_rust), "ffi.accept_newvector2(from_rust)")

	var from_gdscript: Variant = Vector2(12.5, -3.5)
	var mirrored: Variant = ffi.mirror_newvector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newvector3():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newvector3()
	assert_that(ffi.accept_newvector3(from_rust), "ffi.accept_newvector3(from_rust)")

	var from_gdscript: Variant = Vector3(117.5, 100.0, -323.25)
	var mirrored: Variant = ffi.mirror_newvector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newvector4():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newvector4()
	assert_that(ffi.accept_newvector4(from_rust), "ffi.accept_newvector4(from_rust)")

	var from_gdscript: Variant = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Variant = ffi.mirror_newvector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newvector2i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newvector2i()
	assert_that(ffi.accept_newvector2i(from_rust), "ffi.accept_newvector2i(from_rust)")

	var from_gdscript: Variant = Vector2i(-2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_newvector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newvector3i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newvector3i()
	assert_that(ffi.accept_newvector3i(from_rust), "ffi.accept_newvector3i(from_rust)")

	var from_gdscript: Variant = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_newvector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_newcallable():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_newcallable()
	assert_that(ffi.accept_newcallable(from_rust), "ffi.accept_newcallable(from_rust)")

	var from_gdscript: Variant = Callable()
	var mirrored: Variant = ffi.mirror_newcallable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_variantarray():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_variantarray()
	assert_that(ffi.accept_variantarray(from_rust), "ffi.accept_variantarray(from_rust)")

	var from_gdscript: Variant = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Variant = ffi.mirror_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_dictionary():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_dictionary()
	assert_that(ffi.accept_dictionary(from_rust), "ffi.accept_dictionary(from_rust)")

	var from_gdscript: Variant = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Variant = ffi.mirror_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_instanceid():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_instanceid()
	assert_that(ffi.accept_instanceid(from_rust), "ffi.accept_instanceid(from_rust)")

	var from_gdscript: Variant = -1
	var mirrored: Variant = ffi.mirror_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_variant():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_variant()
	assert_that(ffi.accept_variant(from_rust), "ffi.accept_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = ffi.mirror_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_error():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_error()
	assert_that(ffi.accept_error(from_rust), "ffi.accept_error(from_rust)")

	var from_gdscript: Variant = 0
	var mirrored: Variant = ffi.mirror_error(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")



func test_varcall_static_i64():
	var from_rust: Variant = GenFfi.return_static_i64()
	assert_that(GenFfi.accept_static_i64(from_rust), "ffi.accept_static_i64(from_rust)")

	var from_gdscript: Variant = -922337203685477580
	var mirrored: Variant = GenFfi.mirror_static_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_i32():
	var from_rust: Variant = GenFfi.return_static_i32()
	assert_that(GenFfi.accept_static_i32(from_rust), "ffi.accept_static_i32(from_rust)")

	var from_gdscript: Variant = -2147483648
	var mirrored: Variant = GenFfi.mirror_static_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_u32():
	var from_rust: Variant = GenFfi.return_static_u32()
	assert_that(GenFfi.accept_static_u32(from_rust), "ffi.accept_static_u32(from_rust)")

	var from_gdscript: Variant = 4294967295
	var mirrored: Variant = GenFfi.mirror_static_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_i16():
	var from_rust: Variant = GenFfi.return_static_i16()
	assert_that(GenFfi.accept_static_i16(from_rust), "ffi.accept_static_i16(from_rust)")

	var from_gdscript: Variant = -32767
	var mirrored: Variant = GenFfi.mirror_static_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_u16():
	var from_rust: Variant = GenFfi.return_static_u16()
	assert_that(GenFfi.accept_static_u16(from_rust), "ffi.accept_static_u16(from_rust)")

	var from_gdscript: Variant = 65535
	var mirrored: Variant = GenFfi.mirror_static_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_i8():
	var from_rust: Variant = GenFfi.return_static_i8()
	assert_that(GenFfi.accept_static_i8(from_rust), "ffi.accept_static_i8(from_rust)")

	var from_gdscript: Variant = -128
	var mirrored: Variant = GenFfi.mirror_static_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_u8():
	var from_rust: Variant = GenFfi.return_static_u8()
	assert_that(GenFfi.accept_static_u8(from_rust), "ffi.accept_static_u8(from_rust)")

	var from_gdscript: Variant = 255
	var mirrored: Variant = GenFfi.mirror_static_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_f32():
	var from_rust: Variant = GenFfi.return_static_f32()
	assert_that(GenFfi.accept_static_f32(from_rust), "ffi.accept_static_f32(from_rust)")

	var from_gdscript: Variant = 12.5
	var mirrored: Variant = GenFfi.mirror_static_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_f64():
	var from_rust: Variant = GenFfi.return_static_f64()
	assert_that(GenFfi.accept_static_f64(from_rust), "ffi.accept_static_f64(from_rust)")

	var from_gdscript: Variant = 127.83156478
	var mirrored: Variant = GenFfi.mirror_static_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_bool():
	var from_rust: Variant = GenFfi.return_static_bool()
	assert_that(GenFfi.accept_static_bool(from_rust), "ffi.accept_static_bool(from_rust)")

	var from_gdscript: Variant = true
	var mirrored: Variant = GenFfi.mirror_static_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_color():
	var from_rust: Variant = GenFfi.return_static_color()
	assert_that(GenFfi.accept_static_color(from_rust), "ffi.accept_static_color(from_rust)")

	var from_gdscript: Variant = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Variant = GenFfi.mirror_static_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_gstring():
	var from_rust: Variant = GenFfi.return_static_gstring()
	assert_that(GenFfi.accept_static_gstring(from_rust), "ffi.accept_static_gstring(from_rust)")

	var from_gdscript: Variant = "hello"
	var mirrored: Variant = GenFfi.mirror_static_gstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_stringname():
	var from_rust: Variant = GenFfi.return_static_stringname()
	assert_that(GenFfi.accept_static_stringname(from_rust), "ffi.accept_static_stringname(from_rust)")

	var from_gdscript: Variant = &"hello"
	var mirrored: Variant = GenFfi.mirror_static_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_nodepath():
	var from_rust: Variant = GenFfi.return_static_nodepath()
	assert_that(GenFfi.accept_static_nodepath(from_rust), "ffi.accept_static_nodepath(from_rust)")

	var from_gdscript: Variant = ^"hello"
	var mirrored: Variant = GenFfi.mirror_static_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector2():
	var from_rust: Variant = GenFfi.return_static_vector2()
	assert_that(GenFfi.accept_static_vector2(from_rust), "ffi.accept_static_vector2(from_rust)")

	var from_gdscript: Variant = Vector2(12.5, -3.5)
	var mirrored: Variant = GenFfi.mirror_static_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector3():
	var from_rust: Variant = GenFfi.return_static_vector3()
	assert_that(GenFfi.accept_static_vector3(from_rust), "ffi.accept_static_vector3(from_rust)")

	var from_gdscript: Variant = Vector3(117.5, 100.0, -323.25)
	var mirrored: Variant = GenFfi.mirror_static_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector4():
	var from_rust: Variant = GenFfi.return_static_vector4()
	assert_that(GenFfi.accept_static_vector4(from_rust), "ffi.accept_static_vector4(from_rust)")

	var from_gdscript: Variant = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Variant = GenFfi.mirror_static_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector2i():
	var from_rust: Variant = GenFfi.return_static_vector2i()
	assert_that(GenFfi.accept_static_vector2i(from_rust), "ffi.accept_static_vector2i(from_rust)")

	var from_gdscript: Variant = Vector2i(-2147483648, 2147483647)
	var mirrored: Variant = GenFfi.mirror_static_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector3i():
	var from_rust: Variant = GenFfi.return_static_vector3i()
	assert_that(GenFfi.accept_static_vector3i(from_rust), "ffi.accept_static_vector3i(from_rust)")

	var from_gdscript: Variant = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Variant = GenFfi.mirror_static_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_vector4i():
	var from_rust: Variant = GenFfi.return_static_vector4i()
	assert_that(GenFfi.accept_static_vector4i(from_rust), "ffi.accept_static_vector4i(from_rust)")

	var from_gdscript: Variant = Vector4i(-1, -2147483648, 2147483647, 1000)
	var mirrored: Variant = GenFfi.mirror_static_vector4i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_callable():
	var from_rust: Variant = GenFfi.return_static_callable()
	assert_that(GenFfi.accept_static_callable(from_rust), "ffi.accept_static_callable(from_rust)")

	var from_gdscript: Variant = Callable()
	var mirrored: Variant = GenFfi.mirror_static_callable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_rect2():
	var from_rust: Variant = GenFfi.return_static_rect2()
	assert_that(GenFfi.accept_static_rect2(from_rust), "ffi.accept_static_rect2(from_rust)")

	var from_gdscript: Variant = Rect2()
	var mirrored: Variant = GenFfi.mirror_static_rect2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_rect2i():
	var from_rust: Variant = GenFfi.return_static_rect2i()
	assert_that(GenFfi.accept_static_rect2i(from_rust), "ffi.accept_static_rect2i(from_rust)")

	var from_gdscript: Variant = Rect2i()
	var mirrored: Variant = GenFfi.mirror_static_rect2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_transform2d():
	var from_rust: Variant = GenFfi.return_static_transform2d()
	assert_that(GenFfi.accept_static_transform2d(from_rust), "ffi.accept_static_transform2d(from_rust)")

	var from_gdscript: Variant = Transform2D()
	var mirrored: Variant = GenFfi.mirror_static_transform2d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_plane():
	var from_rust: Variant = GenFfi.return_static_plane()
	assert_that(GenFfi.accept_static_plane(from_rust), "ffi.accept_static_plane(from_rust)")

	var from_gdscript: Variant = Plane()
	var mirrored: Variant = GenFfi.mirror_static_plane(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_quaternion():
	var from_rust: Variant = GenFfi.return_static_quaternion()
	assert_that(GenFfi.accept_static_quaternion(from_rust), "ffi.accept_static_quaternion(from_rust)")

	var from_gdscript: Variant = Quaternion()
	var mirrored: Variant = GenFfi.mirror_static_quaternion(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_aabb():
	var from_rust: Variant = GenFfi.return_static_aabb()
	assert_that(GenFfi.accept_static_aabb(from_rust), "ffi.accept_static_aabb(from_rust)")

	var from_gdscript: Variant = AABB()
	var mirrored: Variant = GenFfi.mirror_static_aabb(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_basis():
	var from_rust: Variant = GenFfi.return_static_basis()
	assert_that(GenFfi.accept_static_basis(from_rust), "ffi.accept_static_basis(from_rust)")

	var from_gdscript: Variant = Basis()
	var mirrored: Variant = GenFfi.mirror_static_basis(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_transform3d():
	var from_rust: Variant = GenFfi.return_static_transform3d()
	assert_that(GenFfi.accept_static_transform3d(from_rust), "ffi.accept_static_transform3d(from_rust)")

	var from_gdscript: Variant = Transform3D()
	var mirrored: Variant = GenFfi.mirror_static_transform3d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_projection():
	var from_rust: Variant = GenFfi.return_static_projection()
	assert_that(GenFfi.accept_static_projection(from_rust), "ffi.accept_static_projection(from_rust)")

	var from_gdscript: Variant = Projection()
	var mirrored: Variant = GenFfi.mirror_static_projection(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_rid():
	var from_rust: Variant = GenFfi.return_static_rid()
	assert_that(GenFfi.accept_static_rid(from_rust), "ffi.accept_static_rid(from_rust)")

	var from_gdscript: Variant = RID()
	var mirrored: Variant = GenFfi.mirror_static_rid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_option_gd_node():
	var from_rust: Variant = GenFfi.return_static_option_gd_node()
	assert_that(GenFfi.accept_static_option_gd_node(from_rust), "ffi.accept_static_option_gd_node(from_rust)")

	var from_gdscript: Variant = null
	var mirrored: Variant = GenFfi.mirror_static_option_gd_node(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_option_gd_resource():
	var from_rust: Variant = GenFfi.return_static_option_gd_resource()
	assert_that(GenFfi.accept_static_option_gd_resource(from_rust), "ffi.accept_static_option_gd_resource(from_rust)")

	var from_gdscript: Variant = null
	var mirrored: Variant = GenFfi.mirror_static_option_gd_resource(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedbytearray():
	var from_rust: Variant = GenFfi.return_static_packedbytearray()
	assert_that(GenFfi.accept_static_packedbytearray(from_rust), "ffi.accept_static_packedbytearray(from_rust)")

	var from_gdscript: Variant = PackedByteArray()
	var mirrored: Variant = GenFfi.mirror_static_packedbytearray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedint32array():
	var from_rust: Variant = GenFfi.return_static_packedint32array()
	assert_that(GenFfi.accept_static_packedint32array(from_rust), "ffi.accept_static_packedint32array(from_rust)")

	var from_gdscript: Variant = PackedInt32Array()
	var mirrored: Variant = GenFfi.mirror_static_packedint32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedint64array():
	var from_rust: Variant = GenFfi.return_static_packedint64array()
	assert_that(GenFfi.accept_static_packedint64array(from_rust), "ffi.accept_static_packedint64array(from_rust)")

	var from_gdscript: Variant = PackedInt64Array()
	var mirrored: Variant = GenFfi.mirror_static_packedint64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedfloat32array():
	var from_rust: Variant = GenFfi.return_static_packedfloat32array()
	assert_that(GenFfi.accept_static_packedfloat32array(from_rust), "ffi.accept_static_packedfloat32array(from_rust)")

	var from_gdscript: Variant = PackedFloat32Array()
	var mirrored: Variant = GenFfi.mirror_static_packedfloat32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedfloat64array():
	var from_rust: Variant = GenFfi.return_static_packedfloat64array()
	assert_that(GenFfi.accept_static_packedfloat64array(from_rust), "ffi.accept_static_packedfloat64array(from_rust)")

	var from_gdscript: Variant = PackedFloat64Array()
	var mirrored: Variant = GenFfi.mirror_static_packedfloat64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedstringarray():
	var from_rust: Variant = GenFfi.return_static_packedstringarray()
	assert_that(GenFfi.accept_static_packedstringarray(from_rust), "ffi.accept_static_packedstringarray(from_rust)")

	var from_gdscript: Variant = PackedStringArray()
	var mirrored: Variant = GenFfi.mirror_static_packedstringarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedvector2array():
	var from_rust: Variant = GenFfi.return_static_packedvector2array()
	assert_that(GenFfi.accept_static_packedvector2array(from_rust), "ffi.accept_static_packedvector2array(from_rust)")

	var from_gdscript: Variant = PackedVector2Array()
	var mirrored: Variant = GenFfi.mirror_static_packedvector2array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedvector3array():
	var from_rust: Variant = GenFfi.return_static_packedvector3array()
	assert_that(GenFfi.accept_static_packedvector3array(from_rust), "ffi.accept_static_packedvector3array(from_rust)")

	var from_gdscript: Variant = PackedVector3Array()
	var mirrored: Variant = GenFfi.mirror_static_packedvector3array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_packedcolorarray():
	var from_rust: Variant = GenFfi.return_static_packedcolorarray()
	assert_that(GenFfi.accept_static_packedcolorarray(from_rust), "ffi.accept_static_packedcolorarray(from_rust)")

	var from_gdscript: Variant = PackedColorArray()
	var mirrored: Variant = GenFfi.mirror_static_packedcolorarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newi64():
	var from_rust: Variant = GenFfi.return_static_newi64()
	assert_that(GenFfi.accept_static_newi64(from_rust), "ffi.accept_static_newi64(from_rust)")

	var from_gdscript: Variant = -922337203685477580
	var mirrored: Variant = GenFfi.mirror_static_newi64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newi32():
	var from_rust: Variant = GenFfi.return_static_newi32()
	assert_that(GenFfi.accept_static_newi32(from_rust), "ffi.accept_static_newi32(from_rust)")

	var from_gdscript: Variant = -2147483648
	var mirrored: Variant = GenFfi.mirror_static_newi32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newu32():
	var from_rust: Variant = GenFfi.return_static_newu32()
	assert_that(GenFfi.accept_static_newu32(from_rust), "ffi.accept_static_newu32(from_rust)")

	var from_gdscript: Variant = 4294967295
	var mirrored: Variant = GenFfi.mirror_static_newu32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newi16():
	var from_rust: Variant = GenFfi.return_static_newi16()
	assert_that(GenFfi.accept_static_newi16(from_rust), "ffi.accept_static_newi16(from_rust)")

	var from_gdscript: Variant = -32767
	var mirrored: Variant = GenFfi.mirror_static_newi16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newu16():
	var from_rust: Variant = GenFfi.return_static_newu16()
	assert_that(GenFfi.accept_static_newu16(from_rust), "ffi.accept_static_newu16(from_rust)")

	var from_gdscript: Variant = 65535
	var mirrored: Variant = GenFfi.mirror_static_newu16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newi8():
	var from_rust: Variant = GenFfi.return_static_newi8()
	assert_that(GenFfi.accept_static_newi8(from_rust), "ffi.accept_static_newi8(from_rust)")

	var from_gdscript: Variant = -128
	var mirrored: Variant = GenFfi.mirror_static_newi8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newu8():
	var from_rust: Variant = GenFfi.return_static_newu8()
	assert_that(GenFfi.accept_static_newu8(from_rust), "ffi.accept_static_newu8(from_rust)")

	var from_gdscript: Variant = 255
	var mirrored: Variant = GenFfi.mirror_static_newu8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newf32():
	var from_rust: Variant = GenFfi.return_static_newf32()
	assert_that(GenFfi.accept_static_newf32(from_rust), "ffi.accept_static_newf32(from_rust)")

	var from_gdscript: Variant = 12.5
	var mirrored: Variant = GenFfi.mirror_static_newf32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newf64():
	var from_rust: Variant = GenFfi.return_static_newf64()
	assert_that(GenFfi.accept_static_newf64(from_rust), "ffi.accept_static_newf64(from_rust)")

	var from_gdscript: Variant = 127.83156478
	var mirrored: Variant = GenFfi.mirror_static_newf64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newbool():
	var from_rust: Variant = GenFfi.return_static_newbool()
	assert_that(GenFfi.accept_static_newbool(from_rust), "ffi.accept_static_newbool(from_rust)")

	var from_gdscript: Variant = true
	var mirrored: Variant = GenFfi.mirror_static_newbool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newcolor():
	var from_rust: Variant = GenFfi.return_static_newcolor()
	assert_that(GenFfi.accept_static_newcolor(from_rust), "ffi.accept_static_newcolor(from_rust)")

	var from_gdscript: Variant = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Variant = GenFfi.mirror_static_newcolor(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newstring():
	var from_rust: Variant = GenFfi.return_static_newstring()
	assert_that(GenFfi.accept_static_newstring(from_rust), "ffi.accept_static_newstring(from_rust)")

	var from_gdscript: Variant = "hello"
	var mirrored: Variant = GenFfi.mirror_static_newstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newstringname():
	var from_rust: Variant = GenFfi.return_static_newstringname()
	assert_that(GenFfi.accept_static_newstringname(from_rust), "ffi.accept_static_newstringname(from_rust)")

	var from_gdscript: Variant = &"hello"
	var mirrored: Variant = GenFfi.mirror_static_newstringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newnodepath():
	var from_rust: Variant = GenFfi.return_static_newnodepath()
	assert_that(GenFfi.accept_static_newnodepath(from_rust), "ffi.accept_static_newnodepath(from_rust)")

	var from_gdscript: Variant = ^"hello"
	var mirrored: Variant = GenFfi.mirror_static_newnodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newvector2():
	var from_rust: Variant = GenFfi.return_static_newvector2()
	assert_that(GenFfi.accept_static_newvector2(from_rust), "ffi.accept_static_newvector2(from_rust)")

	var from_gdscript: Variant = Vector2(12.5, -3.5)
	var mirrored: Variant = GenFfi.mirror_static_newvector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newvector3():
	var from_rust: Variant = GenFfi.return_static_newvector3()
	assert_that(GenFfi.accept_static_newvector3(from_rust), "ffi.accept_static_newvector3(from_rust)")

	var from_gdscript: Variant = Vector3(117.5, 100.0, -323.25)
	var mirrored: Variant = GenFfi.mirror_static_newvector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newvector4():
	var from_rust: Variant = GenFfi.return_static_newvector4()
	assert_that(GenFfi.accept_static_newvector4(from_rust), "ffi.accept_static_newvector4(from_rust)")

	var from_gdscript: Variant = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Variant = GenFfi.mirror_static_newvector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newvector2i():
	var from_rust: Variant = GenFfi.return_static_newvector2i()
	assert_that(GenFfi.accept_static_newvector2i(from_rust), "ffi.accept_static_newvector2i(from_rust)")

	var from_gdscript: Variant = Vector2i(-2147483648, 2147483647)
	var mirrored: Variant = GenFfi.mirror_static_newvector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newvector3i():
	var from_rust: Variant = GenFfi.return_static_newvector3i()
	assert_that(GenFfi.accept_static_newvector3i(from_rust), "ffi.accept_static_newvector3i(from_rust)")

	var from_gdscript: Variant = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Variant = GenFfi.mirror_static_newvector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_newcallable():
	var from_rust: Variant = GenFfi.return_static_newcallable()
	assert_that(GenFfi.accept_static_newcallable(from_rust), "ffi.accept_static_newcallable(from_rust)")

	var from_gdscript: Variant = Callable()
	var mirrored: Variant = GenFfi.mirror_static_newcallable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_variantarray():
	var from_rust: Variant = GenFfi.return_static_variantarray()
	assert_that(GenFfi.accept_static_variantarray(from_rust), "ffi.accept_static_variantarray(from_rust)")

	var from_gdscript: Variant = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Variant = GenFfi.mirror_static_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_dictionary():
	var from_rust: Variant = GenFfi.return_static_dictionary()
	assert_that(GenFfi.accept_static_dictionary(from_rust), "ffi.accept_static_dictionary(from_rust)")

	var from_gdscript: Variant = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Variant = GenFfi.mirror_static_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_instanceid():
	var from_rust: Variant = GenFfi.return_static_instanceid()
	assert_that(GenFfi.accept_static_instanceid(from_rust), "ffi.accept_static_instanceid(from_rust)")

	var from_gdscript: Variant = -1
	var mirrored: Variant = GenFfi.mirror_static_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_variant():
	var from_rust: Variant = GenFfi.return_static_variant()
	assert_that(GenFfi.accept_static_variant(from_rust), "ffi.accept_static_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = GenFfi.mirror_static_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_varcall_static_error():
	var from_rust: Variant = GenFfi.return_static_error()
	assert_that(GenFfi.accept_static_error(from_rust), "ffi.accept_static_error(from_rust)")

	var from_gdscript: Variant = 0
	var mirrored: Variant = GenFfi.mirror_static_error(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")



func test_ptrcall_i64():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i64()
	assert_that(ffi.accept_i64(from_rust), "ffi.accept_i64(from_rust)")

	var from_gdscript: int = -922337203685477580
	var mirrored: int = ffi.mirror_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i32()
	assert_that(ffi.accept_i32(from_rust), "ffi.accept_i32(from_rust)")

	var from_gdscript: int = -2147483648
	var mirrored: int = ffi.mirror_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u32()
	assert_that(ffi.accept_u32(from_rust), "ffi.accept_u32(from_rust)")

	var from_gdscript: int = 4294967295
	var mirrored: int = ffi.mirror_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i16()
	assert_that(ffi.accept_i16(from_rust), "ffi.accept_i16(from_rust)")

	var from_gdscript: int = -32767
	var mirrored: int = ffi.mirror_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u16()
	assert_that(ffi.accept_u16(from_rust), "ffi.accept_u16(from_rust)")

	var from_gdscript: int = 65535
	var mirrored: int = ffi.mirror_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i8()
	assert_that(ffi.accept_i8(from_rust), "ffi.accept_i8(from_rust)")

	var from_gdscript: int = -128
	var mirrored: int = ffi.mirror_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u8()
	assert_that(ffi.accept_u8(from_rust), "ffi.accept_u8(from_rust)")

	var from_gdscript: int = 255
	var mirrored: int = ffi.mirror_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_f32():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_f32()
	assert_that(ffi.accept_f32(from_rust), "ffi.accept_f32(from_rust)")

	var from_gdscript: float = 12.5
	var mirrored: float = ffi.mirror_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_f64():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_f64()
	assert_that(ffi.accept_f64(from_rust), "ffi.accept_f64(from_rust)")

	var from_gdscript: float = 127.83156478
	var mirrored: float = ffi.mirror_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_bool():
	var ffi := GenFfi.new()

	var from_rust: bool = ffi.return_bool()
	assert_that(ffi.accept_bool(from_rust), "ffi.accept_bool(from_rust)")

	var from_gdscript: bool = true
	var mirrored: bool = ffi.mirror_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_color():
	var ffi := GenFfi.new()

	var from_rust: Color = ffi.return_color()
	assert_that(ffi.accept_color(from_rust), "ffi.accept_color(from_rust)")

	var from_gdscript: Color = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Color = ffi.mirror_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_gstring():
	var ffi := GenFfi.new()

	var from_rust: String = ffi.return_gstring()
	assert_that(ffi.accept_gstring(from_rust), "ffi.accept_gstring(from_rust)")

	var from_gdscript: String = "hello"
	var mirrored: String = ffi.mirror_gstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_stringname():
	var ffi := GenFfi.new()

	var from_rust: StringName = ffi.return_stringname()
	assert_that(ffi.accept_stringname(from_rust), "ffi.accept_stringname(from_rust)")

	var from_gdscript: StringName = &"hello"
	var mirrored: StringName = ffi.mirror_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_nodepath():
	var ffi := GenFfi.new()

	var from_rust: NodePath = ffi.return_nodepath()
	assert_that(ffi.accept_nodepath(from_rust), "ffi.accept_nodepath(from_rust)")

	var from_gdscript: NodePath = ^"hello"
	var mirrored: NodePath = ffi.mirror_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector2():
	var ffi := GenFfi.new()

	var from_rust: Vector2 = ffi.return_vector2()
	assert_that(ffi.accept_vector2(from_rust), "ffi.accept_vector2(from_rust)")

	var from_gdscript: Vector2 = Vector2(12.5, -3.5)
	var mirrored: Vector2 = ffi.mirror_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector3():
	var ffi := GenFfi.new()

	var from_rust: Vector3 = ffi.return_vector3()
	assert_that(ffi.accept_vector3(from_rust), "ffi.accept_vector3(from_rust)")

	var from_gdscript: Vector3 = Vector3(117.5, 100.0, -323.25)
	var mirrored: Vector3 = ffi.mirror_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector4():
	var ffi := GenFfi.new()

	var from_rust: Vector4 = ffi.return_vector4()
	assert_that(ffi.accept_vector4(from_rust), "ffi.accept_vector4(from_rust)")

	var from_gdscript: Vector4 = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Vector4 = ffi.mirror_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector2i():
	var ffi := GenFfi.new()

	var from_rust: Vector2i = ffi.return_vector2i()
	assert_that(ffi.accept_vector2i(from_rust), "ffi.accept_vector2i(from_rust)")

	var from_gdscript: Vector2i = Vector2i(-2147483648, 2147483647)
	var mirrored: Vector2i = ffi.mirror_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector3i():
	var ffi := GenFfi.new()

	var from_rust: Vector3i = ffi.return_vector3i()
	assert_that(ffi.accept_vector3i(from_rust), "ffi.accept_vector3i(from_rust)")

	var from_gdscript: Vector3i = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Vector3i = ffi.mirror_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector4i():
	var ffi := GenFfi.new()

	var from_rust: Vector4i = ffi.return_vector4i()
	assert_that(ffi.accept_vector4i(from_rust), "ffi.accept_vector4i(from_rust)")

	var from_gdscript: Vector4i = Vector4i(-1, -2147483648, 2147483647, 1000)
	var mirrored: Vector4i = ffi.mirror_vector4i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_callable():
	var ffi := GenFfi.new()

	var from_rust: Callable = ffi.return_callable()
	assert_that(ffi.accept_callable(from_rust), "ffi.accept_callable(from_rust)")

	var from_gdscript: Callable = Callable()
	var mirrored: Callable = ffi.mirror_callable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_rect2():
	var ffi := GenFfi.new()

	var from_rust: Rect2 = ffi.return_rect2()
	assert_that(ffi.accept_rect2(from_rust), "ffi.accept_rect2(from_rust)")

	var from_gdscript: Rect2 = Rect2()
	var mirrored: Rect2 = ffi.mirror_rect2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_rect2i():
	var ffi := GenFfi.new()

	var from_rust: Rect2i = ffi.return_rect2i()
	assert_that(ffi.accept_rect2i(from_rust), "ffi.accept_rect2i(from_rust)")

	var from_gdscript: Rect2i = Rect2i()
	var mirrored: Rect2i = ffi.mirror_rect2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_transform2d():
	var ffi := GenFfi.new()

	var from_rust: Transform2D = ffi.return_transform2d()
	assert_that(ffi.accept_transform2d(from_rust), "ffi.accept_transform2d(from_rust)")

	var from_gdscript: Transform2D = Transform2D()
	var mirrored: Transform2D = ffi.mirror_transform2d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_plane():
	var ffi := GenFfi.new()

	var from_rust: Plane = ffi.return_plane()
	assert_that(ffi.accept_plane(from_rust), "ffi.accept_plane(from_rust)")

	var from_gdscript: Plane = Plane()
	var mirrored: Plane = ffi.mirror_plane(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_quaternion():
	var ffi := GenFfi.new()

	var from_rust: Quaternion = ffi.return_quaternion()
	assert_that(ffi.accept_quaternion(from_rust), "ffi.accept_quaternion(from_rust)")

	var from_gdscript: Quaternion = Quaternion()
	var mirrored: Quaternion = ffi.mirror_quaternion(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_aabb():
	var ffi := GenFfi.new()

	var from_rust: AABB = ffi.return_aabb()
	assert_that(ffi.accept_aabb(from_rust), "ffi.accept_aabb(from_rust)")

	var from_gdscript: AABB = AABB()
	var mirrored: AABB = ffi.mirror_aabb(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_basis():
	var ffi := GenFfi.new()

	var from_rust: Basis = ffi.return_basis()
	assert_that(ffi.accept_basis(from_rust), "ffi.accept_basis(from_rust)")

	var from_gdscript: Basis = Basis()
	var mirrored: Basis = ffi.mirror_basis(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_transform3d():
	var ffi := GenFfi.new()

	var from_rust: Transform3D = ffi.return_transform3d()
	assert_that(ffi.accept_transform3d(from_rust), "ffi.accept_transform3d(from_rust)")

	var from_gdscript: Transform3D = Transform3D()
	var mirrored: Transform3D = ffi.mirror_transform3d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_projection():
	var ffi := GenFfi.new()

	var from_rust: Projection = ffi.return_projection()
	assert_that(ffi.accept_projection(from_rust), "ffi.accept_projection(from_rust)")

	var from_gdscript: Projection = Projection()
	var mirrored: Projection = ffi.mirror_projection(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_rid():
	var ffi := GenFfi.new()

	var from_rust: RID = ffi.return_rid()
	assert_that(ffi.accept_rid(from_rust), "ffi.accept_rid(from_rust)")

	var from_gdscript: RID = RID()
	var mirrored: RID = ffi.mirror_rid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_option_gd_node():
	var ffi := GenFfi.new()

	var from_rust: Node = ffi.return_option_gd_node()
	assert_that(ffi.accept_option_gd_node(from_rust), "ffi.accept_option_gd_node(from_rust)")

	var from_gdscript: Node = null
	var mirrored: Node = ffi.mirror_option_gd_node(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_option_gd_resource():
	var ffi := GenFfi.new()

	var from_rust: Resource = ffi.return_option_gd_resource()
	assert_that(ffi.accept_option_gd_resource(from_rust), "ffi.accept_option_gd_resource(from_rust)")

	var from_gdscript: Resource = null
	var mirrored: Resource = ffi.mirror_option_gd_resource(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedbytearray():
	var ffi := GenFfi.new()

	var from_rust: PackedByteArray = ffi.return_packedbytearray()
	assert_that(ffi.accept_packedbytearray(from_rust), "ffi.accept_packedbytearray(from_rust)")

	var from_gdscript: PackedByteArray = PackedByteArray()
	var mirrored: PackedByteArray = ffi.mirror_packedbytearray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedint32array():
	var ffi := GenFfi.new()

	var from_rust: PackedInt32Array = ffi.return_packedint32array()
	assert_that(ffi.accept_packedint32array(from_rust), "ffi.accept_packedint32array(from_rust)")

	var from_gdscript: PackedInt32Array = PackedInt32Array()
	var mirrored: PackedInt32Array = ffi.mirror_packedint32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedint64array():
	var ffi := GenFfi.new()

	var from_rust: PackedInt64Array = ffi.return_packedint64array()
	assert_that(ffi.accept_packedint64array(from_rust), "ffi.accept_packedint64array(from_rust)")

	var from_gdscript: PackedInt64Array = PackedInt64Array()
	var mirrored: PackedInt64Array = ffi.mirror_packedint64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedfloat32array():
	var ffi := GenFfi.new()

	var from_rust: PackedFloat32Array = ffi.return_packedfloat32array()
	assert_that(ffi.accept_packedfloat32array(from_rust), "ffi.accept_packedfloat32array(from_rust)")

	var from_gdscript: PackedFloat32Array = PackedFloat32Array()
	var mirrored: PackedFloat32Array = ffi.mirror_packedfloat32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedfloat64array():
	var ffi := GenFfi.new()

	var from_rust: PackedFloat64Array = ffi.return_packedfloat64array()
	assert_that(ffi.accept_packedfloat64array(from_rust), "ffi.accept_packedfloat64array(from_rust)")

	var from_gdscript: PackedFloat64Array = PackedFloat64Array()
	var mirrored: PackedFloat64Array = ffi.mirror_packedfloat64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedstringarray():
	var ffi := GenFfi.new()

	var from_rust: PackedStringArray = ffi.return_packedstringarray()
	assert_that(ffi.accept_packedstringarray(from_rust), "ffi.accept_packedstringarray(from_rust)")

	var from_gdscript: PackedStringArray = PackedStringArray()
	var mirrored: PackedStringArray = ffi.mirror_packedstringarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedvector2array():
	var ffi := GenFfi.new()

	var from_rust: PackedVector2Array = ffi.return_packedvector2array()
	assert_that(ffi.accept_packedvector2array(from_rust), "ffi.accept_packedvector2array(from_rust)")

	var from_gdscript: PackedVector2Array = PackedVector2Array()
	var mirrored: PackedVector2Array = ffi.mirror_packedvector2array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedvector3array():
	var ffi := GenFfi.new()

	var from_rust: PackedVector3Array = ffi.return_packedvector3array()
	assert_that(ffi.accept_packedvector3array(from_rust), "ffi.accept_packedvector3array(from_rust)")

	var from_gdscript: PackedVector3Array = PackedVector3Array()
	var mirrored: PackedVector3Array = ffi.mirror_packedvector3array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_packedcolorarray():
	var ffi := GenFfi.new()

	var from_rust: PackedColorArray = ffi.return_packedcolorarray()
	assert_that(ffi.accept_packedcolorarray(from_rust), "ffi.accept_packedcolorarray(from_rust)")

	var from_gdscript: PackedColorArray = PackedColorArray()
	var mirrored: PackedColorArray = ffi.mirror_packedcolorarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newi64():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newi64()
	assert_that(ffi.accept_newi64(from_rust), "ffi.accept_newi64(from_rust)")

	var from_gdscript: int = -922337203685477580
	var mirrored: int = ffi.mirror_newi64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newi32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newi32()
	assert_that(ffi.accept_newi32(from_rust), "ffi.accept_newi32(from_rust)")

	var from_gdscript: int = -2147483648
	var mirrored: int = ffi.mirror_newi32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newu32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newu32()
	assert_that(ffi.accept_newu32(from_rust), "ffi.accept_newu32(from_rust)")

	var from_gdscript: int = 4294967295
	var mirrored: int = ffi.mirror_newu32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newi16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newi16()
	assert_that(ffi.accept_newi16(from_rust), "ffi.accept_newi16(from_rust)")

	var from_gdscript: int = -32767
	var mirrored: int = ffi.mirror_newi16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newu16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newu16()
	assert_that(ffi.accept_newu16(from_rust), "ffi.accept_newu16(from_rust)")

	var from_gdscript: int = 65535
	var mirrored: int = ffi.mirror_newu16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newi8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newi8()
	assert_that(ffi.accept_newi8(from_rust), "ffi.accept_newi8(from_rust)")

	var from_gdscript: int = -128
	var mirrored: int = ffi.mirror_newi8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newu8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_newu8()
	assert_that(ffi.accept_newu8(from_rust), "ffi.accept_newu8(from_rust)")

	var from_gdscript: int = 255
	var mirrored: int = ffi.mirror_newu8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newf32():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_newf32()
	assert_that(ffi.accept_newf32(from_rust), "ffi.accept_newf32(from_rust)")

	var from_gdscript: float = 12.5
	var mirrored: float = ffi.mirror_newf32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newf64():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_newf64()
	assert_that(ffi.accept_newf64(from_rust), "ffi.accept_newf64(from_rust)")

	var from_gdscript: float = 127.83156478
	var mirrored: float = ffi.mirror_newf64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newbool():
	var ffi := GenFfi.new()

	var from_rust: bool = ffi.return_newbool()
	assert_that(ffi.accept_newbool(from_rust), "ffi.accept_newbool(from_rust)")

	var from_gdscript: bool = true
	var mirrored: bool = ffi.mirror_newbool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newcolor():
	var ffi := GenFfi.new()

	var from_rust: Color = ffi.return_newcolor()
	assert_that(ffi.accept_newcolor(from_rust), "ffi.accept_newcolor(from_rust)")

	var from_gdscript: Color = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Color = ffi.mirror_newcolor(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newstring():
	var ffi := GenFfi.new()

	var from_rust: String = ffi.return_newstring()
	assert_that(ffi.accept_newstring(from_rust), "ffi.accept_newstring(from_rust)")

	var from_gdscript: String = "hello"
	var mirrored: String = ffi.mirror_newstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newstringname():
	var ffi := GenFfi.new()

	var from_rust: StringName = ffi.return_newstringname()
	assert_that(ffi.accept_newstringname(from_rust), "ffi.accept_newstringname(from_rust)")

	var from_gdscript: StringName = &"hello"
	var mirrored: StringName = ffi.mirror_newstringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newnodepath():
	var ffi := GenFfi.new()

	var from_rust: NodePath = ffi.return_newnodepath()
	assert_that(ffi.accept_newnodepath(from_rust), "ffi.accept_newnodepath(from_rust)")

	var from_gdscript: NodePath = ^"hello"
	var mirrored: NodePath = ffi.mirror_newnodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newvector2():
	var ffi := GenFfi.new()

	var from_rust: Vector2 = ffi.return_newvector2()
	assert_that(ffi.accept_newvector2(from_rust), "ffi.accept_newvector2(from_rust)")

	var from_gdscript: Vector2 = Vector2(12.5, -3.5)
	var mirrored: Vector2 = ffi.mirror_newvector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newvector3():
	var ffi := GenFfi.new()

	var from_rust: Vector3 = ffi.return_newvector3()
	assert_that(ffi.accept_newvector3(from_rust), "ffi.accept_newvector3(from_rust)")

	var from_gdscript: Vector3 = Vector3(117.5, 100.0, -323.25)
	var mirrored: Vector3 = ffi.mirror_newvector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newvector4():
	var ffi := GenFfi.new()

	var from_rust: Vector4 = ffi.return_newvector4()
	assert_that(ffi.accept_newvector4(from_rust), "ffi.accept_newvector4(from_rust)")

	var from_gdscript: Vector4 = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Vector4 = ffi.mirror_newvector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newvector2i():
	var ffi := GenFfi.new()

	var from_rust: Vector2i = ffi.return_newvector2i()
	assert_that(ffi.accept_newvector2i(from_rust), "ffi.accept_newvector2i(from_rust)")

	var from_gdscript: Vector2i = Vector2i(-2147483648, 2147483647)
	var mirrored: Vector2i = ffi.mirror_newvector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newvector3i():
	var ffi := GenFfi.new()

	var from_rust: Vector3i = ffi.return_newvector3i()
	assert_that(ffi.accept_newvector3i(from_rust), "ffi.accept_newvector3i(from_rust)")

	var from_gdscript: Vector3i = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Vector3i = ffi.mirror_newvector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_newcallable():
	var ffi := GenFfi.new()

	var from_rust: Callable = ffi.return_newcallable()
	assert_that(ffi.accept_newcallable(from_rust), "ffi.accept_newcallable(from_rust)")

	var from_gdscript: Callable = Callable()
	var mirrored: Callable = ffi.mirror_newcallable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_variantarray():
	var ffi := GenFfi.new()

	var from_rust: Array = ffi.return_variantarray()
	assert_that(ffi.accept_variantarray(from_rust), "ffi.accept_variantarray(from_rust)")

	var from_gdscript: Array = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Array = ffi.mirror_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_dictionary():
	var ffi := GenFfi.new()

	var from_rust: Dictionary = ffi.return_dictionary()
	assert_that(ffi.accept_dictionary(from_rust), "ffi.accept_dictionary(from_rust)")

	var from_gdscript: Dictionary = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Dictionary = ffi.mirror_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_instanceid():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_instanceid()
	assert_that(ffi.accept_instanceid(from_rust), "ffi.accept_instanceid(from_rust)")

	var from_gdscript: int = -1
	var mirrored: int = ffi.mirror_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_variant():
	var ffi := GenFfi.new()

	var from_rust: Variant = ffi.return_variant()
	assert_that(ffi.accept_variant(from_rust), "ffi.accept_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = ffi.mirror_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_error():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_error()
	assert_that(ffi.accept_error(from_rust), "ffi.accept_error(from_rust)")

	var from_gdscript: int = 0
	var mirrored: int = ffi.mirror_error(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")



func test_ptrcall_static_i64():
	var from_rust: int = GenFfi.return_static_i64()
	assert_that(GenFfi.accept_static_i64(from_rust), "ffi.accept_static_i64(from_rust)")

	var from_gdscript: int = -922337203685477580
	var mirrored: int = GenFfi.mirror_static_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_i32():
	var from_rust: int = GenFfi.return_static_i32()
	assert_that(GenFfi.accept_static_i32(from_rust), "ffi.accept_static_i32(from_rust)")

	var from_gdscript: int = -2147483648
	var mirrored: int = GenFfi.mirror_static_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_u32():
	var from_rust: int = GenFfi.return_static_u32()
	assert_that(GenFfi.accept_static_u32(from_rust), "ffi.accept_static_u32(from_rust)")

	var from_gdscript: int = 4294967295
	var mirrored: int = GenFfi.mirror_static_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_i16():
	var from_rust: int = GenFfi.return_static_i16()
	assert_that(GenFfi.accept_static_i16(from_rust), "ffi.accept_static_i16(from_rust)")

	var from_gdscript: int = -32767
	var mirrored: int = GenFfi.mirror_static_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_u16():
	var from_rust: int = GenFfi.return_static_u16()
	assert_that(GenFfi.accept_static_u16(from_rust), "ffi.accept_static_u16(from_rust)")

	var from_gdscript: int = 65535
	var mirrored: int = GenFfi.mirror_static_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_i8():
	var from_rust: int = GenFfi.return_static_i8()
	assert_that(GenFfi.accept_static_i8(from_rust), "ffi.accept_static_i8(from_rust)")

	var from_gdscript: int = -128
	var mirrored: int = GenFfi.mirror_static_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_u8():
	var from_rust: int = GenFfi.return_static_u8()
	assert_that(GenFfi.accept_static_u8(from_rust), "ffi.accept_static_u8(from_rust)")

	var from_gdscript: int = 255
	var mirrored: int = GenFfi.mirror_static_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_f32():
	var from_rust: float = GenFfi.return_static_f32()
	assert_that(GenFfi.accept_static_f32(from_rust), "ffi.accept_static_f32(from_rust)")

	var from_gdscript: float = 12.5
	var mirrored: float = GenFfi.mirror_static_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_f64():
	var from_rust: float = GenFfi.return_static_f64()
	assert_that(GenFfi.accept_static_f64(from_rust), "ffi.accept_static_f64(from_rust)")

	var from_gdscript: float = 127.83156478
	var mirrored: float = GenFfi.mirror_static_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_bool():
	var from_rust: bool = GenFfi.return_static_bool()
	assert_that(GenFfi.accept_static_bool(from_rust), "ffi.accept_static_bool(from_rust)")

	var from_gdscript: bool = true
	var mirrored: bool = GenFfi.mirror_static_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_color():
	var from_rust: Color = GenFfi.return_static_color()
	assert_that(GenFfi.accept_static_color(from_rust), "ffi.accept_static_color(from_rust)")

	var from_gdscript: Color = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Color = GenFfi.mirror_static_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_gstring():
	var from_rust: String = GenFfi.return_static_gstring()
	assert_that(GenFfi.accept_static_gstring(from_rust), "ffi.accept_static_gstring(from_rust)")

	var from_gdscript: String = "hello"
	var mirrored: String = GenFfi.mirror_static_gstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_stringname():
	var from_rust: StringName = GenFfi.return_static_stringname()
	assert_that(GenFfi.accept_static_stringname(from_rust), "ffi.accept_static_stringname(from_rust)")

	var from_gdscript: StringName = &"hello"
	var mirrored: StringName = GenFfi.mirror_static_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_nodepath():
	var from_rust: NodePath = GenFfi.return_static_nodepath()
	assert_that(GenFfi.accept_static_nodepath(from_rust), "ffi.accept_static_nodepath(from_rust)")

	var from_gdscript: NodePath = ^"hello"
	var mirrored: NodePath = GenFfi.mirror_static_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector2():
	var from_rust: Vector2 = GenFfi.return_static_vector2()
	assert_that(GenFfi.accept_static_vector2(from_rust), "ffi.accept_static_vector2(from_rust)")

	var from_gdscript: Vector2 = Vector2(12.5, -3.5)
	var mirrored: Vector2 = GenFfi.mirror_static_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector3():
	var from_rust: Vector3 = GenFfi.return_static_vector3()
	assert_that(GenFfi.accept_static_vector3(from_rust), "ffi.accept_static_vector3(from_rust)")

	var from_gdscript: Vector3 = Vector3(117.5, 100.0, -323.25)
	var mirrored: Vector3 = GenFfi.mirror_static_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector4():
	var from_rust: Vector4 = GenFfi.return_static_vector4()
	assert_that(GenFfi.accept_static_vector4(from_rust), "ffi.accept_static_vector4(from_rust)")

	var from_gdscript: Vector4 = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Vector4 = GenFfi.mirror_static_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector2i():
	var from_rust: Vector2i = GenFfi.return_static_vector2i()
	assert_that(GenFfi.accept_static_vector2i(from_rust), "ffi.accept_static_vector2i(from_rust)")

	var from_gdscript: Vector2i = Vector2i(-2147483648, 2147483647)
	var mirrored: Vector2i = GenFfi.mirror_static_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector3i():
	var from_rust: Vector3i = GenFfi.return_static_vector3i()
	assert_that(GenFfi.accept_static_vector3i(from_rust), "ffi.accept_static_vector3i(from_rust)")

	var from_gdscript: Vector3i = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Vector3i = GenFfi.mirror_static_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_vector4i():
	var from_rust: Vector4i = GenFfi.return_static_vector4i()
	assert_that(GenFfi.accept_static_vector4i(from_rust), "ffi.accept_static_vector4i(from_rust)")

	var from_gdscript: Vector4i = Vector4i(-1, -2147483648, 2147483647, 1000)
	var mirrored: Vector4i = GenFfi.mirror_static_vector4i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_callable():
	var from_rust: Callable = GenFfi.return_static_callable()
	assert_that(GenFfi.accept_static_callable(from_rust), "ffi.accept_static_callable(from_rust)")

	var from_gdscript: Callable = Callable()
	var mirrored: Callable = GenFfi.mirror_static_callable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_rect2():
	var from_rust: Rect2 = GenFfi.return_static_rect2()
	assert_that(GenFfi.accept_static_rect2(from_rust), "ffi.accept_static_rect2(from_rust)")

	var from_gdscript: Rect2 = Rect2()
	var mirrored: Rect2 = GenFfi.mirror_static_rect2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_rect2i():
	var from_rust: Rect2i = GenFfi.return_static_rect2i()
	assert_that(GenFfi.accept_static_rect2i(from_rust), "ffi.accept_static_rect2i(from_rust)")

	var from_gdscript: Rect2i = Rect2i()
	var mirrored: Rect2i = GenFfi.mirror_static_rect2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_transform2d():
	var from_rust: Transform2D = GenFfi.return_static_transform2d()
	assert_that(GenFfi.accept_static_transform2d(from_rust), "ffi.accept_static_transform2d(from_rust)")

	var from_gdscript: Transform2D = Transform2D()
	var mirrored: Transform2D = GenFfi.mirror_static_transform2d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_plane():
	var from_rust: Plane = GenFfi.return_static_plane()
	assert_that(GenFfi.accept_static_plane(from_rust), "ffi.accept_static_plane(from_rust)")

	var from_gdscript: Plane = Plane()
	var mirrored: Plane = GenFfi.mirror_static_plane(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_quaternion():
	var from_rust: Quaternion = GenFfi.return_static_quaternion()
	assert_that(GenFfi.accept_static_quaternion(from_rust), "ffi.accept_static_quaternion(from_rust)")

	var from_gdscript: Quaternion = Quaternion()
	var mirrored: Quaternion = GenFfi.mirror_static_quaternion(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_aabb():
	var from_rust: AABB = GenFfi.return_static_aabb()
	assert_that(GenFfi.accept_static_aabb(from_rust), "ffi.accept_static_aabb(from_rust)")

	var from_gdscript: AABB = AABB()
	var mirrored: AABB = GenFfi.mirror_static_aabb(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_basis():
	var from_rust: Basis = GenFfi.return_static_basis()
	assert_that(GenFfi.accept_static_basis(from_rust), "ffi.accept_static_basis(from_rust)")

	var from_gdscript: Basis = Basis()
	var mirrored: Basis = GenFfi.mirror_static_basis(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_transform3d():
	var from_rust: Transform3D = GenFfi.return_static_transform3d()
	assert_that(GenFfi.accept_static_transform3d(from_rust), "ffi.accept_static_transform3d(from_rust)")

	var from_gdscript: Transform3D = Transform3D()
	var mirrored: Transform3D = GenFfi.mirror_static_transform3d(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_projection():
	var from_rust: Projection = GenFfi.return_static_projection()
	assert_that(GenFfi.accept_static_projection(from_rust), "ffi.accept_static_projection(from_rust)")

	var from_gdscript: Projection = Projection()
	var mirrored: Projection = GenFfi.mirror_static_projection(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_rid():
	var from_rust: RID = GenFfi.return_static_rid()
	assert_that(GenFfi.accept_static_rid(from_rust), "ffi.accept_static_rid(from_rust)")

	var from_gdscript: RID = RID()
	var mirrored: RID = GenFfi.mirror_static_rid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_option_gd_node():
	var from_rust: Node = GenFfi.return_static_option_gd_node()
	assert_that(GenFfi.accept_static_option_gd_node(from_rust), "ffi.accept_static_option_gd_node(from_rust)")

	var from_gdscript: Node = null
	var mirrored: Node = GenFfi.mirror_static_option_gd_node(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_option_gd_resource():
	var from_rust: Resource = GenFfi.return_static_option_gd_resource()
	assert_that(GenFfi.accept_static_option_gd_resource(from_rust), "ffi.accept_static_option_gd_resource(from_rust)")

	var from_gdscript: Resource = null
	var mirrored: Resource = GenFfi.mirror_static_option_gd_resource(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedbytearray():
	var from_rust: PackedByteArray = GenFfi.return_static_packedbytearray()
	assert_that(GenFfi.accept_static_packedbytearray(from_rust), "ffi.accept_static_packedbytearray(from_rust)")

	var from_gdscript: PackedByteArray = PackedByteArray()
	var mirrored: PackedByteArray = GenFfi.mirror_static_packedbytearray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedint32array():
	var from_rust: PackedInt32Array = GenFfi.return_static_packedint32array()
	assert_that(GenFfi.accept_static_packedint32array(from_rust), "ffi.accept_static_packedint32array(from_rust)")

	var from_gdscript: PackedInt32Array = PackedInt32Array()
	var mirrored: PackedInt32Array = GenFfi.mirror_static_packedint32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedint64array():
	var from_rust: PackedInt64Array = GenFfi.return_static_packedint64array()
	assert_that(GenFfi.accept_static_packedint64array(from_rust), "ffi.accept_static_packedint64array(from_rust)")

	var from_gdscript: PackedInt64Array = PackedInt64Array()
	var mirrored: PackedInt64Array = GenFfi.mirror_static_packedint64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedfloat32array():
	var from_rust: PackedFloat32Array = GenFfi.return_static_packedfloat32array()
	assert_that(GenFfi.accept_static_packedfloat32array(from_rust), "ffi.accept_static_packedfloat32array(from_rust)")

	var from_gdscript: PackedFloat32Array = PackedFloat32Array()
	var mirrored: PackedFloat32Array = GenFfi.mirror_static_packedfloat32array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedfloat64array():
	var from_rust: PackedFloat64Array = GenFfi.return_static_packedfloat64array()
	assert_that(GenFfi.accept_static_packedfloat64array(from_rust), "ffi.accept_static_packedfloat64array(from_rust)")

	var from_gdscript: PackedFloat64Array = PackedFloat64Array()
	var mirrored: PackedFloat64Array = GenFfi.mirror_static_packedfloat64array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedstringarray():
	var from_rust: PackedStringArray = GenFfi.return_static_packedstringarray()
	assert_that(GenFfi.accept_static_packedstringarray(from_rust), "ffi.accept_static_packedstringarray(from_rust)")

	var from_gdscript: PackedStringArray = PackedStringArray()
	var mirrored: PackedStringArray = GenFfi.mirror_static_packedstringarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedvector2array():
	var from_rust: PackedVector2Array = GenFfi.return_static_packedvector2array()
	assert_that(GenFfi.accept_static_packedvector2array(from_rust), "ffi.accept_static_packedvector2array(from_rust)")

	var from_gdscript: PackedVector2Array = PackedVector2Array()
	var mirrored: PackedVector2Array = GenFfi.mirror_static_packedvector2array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedvector3array():
	var from_rust: PackedVector3Array = GenFfi.return_static_packedvector3array()
	assert_that(GenFfi.accept_static_packedvector3array(from_rust), "ffi.accept_static_packedvector3array(from_rust)")

	var from_gdscript: PackedVector3Array = PackedVector3Array()
	var mirrored: PackedVector3Array = GenFfi.mirror_static_packedvector3array(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_packedcolorarray():
	var from_rust: PackedColorArray = GenFfi.return_static_packedcolorarray()
	assert_that(GenFfi.accept_static_packedcolorarray(from_rust), "ffi.accept_static_packedcolorarray(from_rust)")

	var from_gdscript: PackedColorArray = PackedColorArray()
	var mirrored: PackedColorArray = GenFfi.mirror_static_packedcolorarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newi64():
	var from_rust: int = GenFfi.return_static_newi64()
	assert_that(GenFfi.accept_static_newi64(from_rust), "ffi.accept_static_newi64(from_rust)")

	var from_gdscript: int = -922337203685477580
	var mirrored: int = GenFfi.mirror_static_newi64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newi32():
	var from_rust: int = GenFfi.return_static_newi32()
	assert_that(GenFfi.accept_static_newi32(from_rust), "ffi.accept_static_newi32(from_rust)")

	var from_gdscript: int = -2147483648
	var mirrored: int = GenFfi.mirror_static_newi32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newu32():
	var from_rust: int = GenFfi.return_static_newu32()
	assert_that(GenFfi.accept_static_newu32(from_rust), "ffi.accept_static_newu32(from_rust)")

	var from_gdscript: int = 4294967295
	var mirrored: int = GenFfi.mirror_static_newu32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newi16():
	var from_rust: int = GenFfi.return_static_newi16()
	assert_that(GenFfi.accept_static_newi16(from_rust), "ffi.accept_static_newi16(from_rust)")

	var from_gdscript: int = -32767
	var mirrored: int = GenFfi.mirror_static_newi16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newu16():
	var from_rust: int = GenFfi.return_static_newu16()
	assert_that(GenFfi.accept_static_newu16(from_rust), "ffi.accept_static_newu16(from_rust)")

	var from_gdscript: int = 65535
	var mirrored: int = GenFfi.mirror_static_newu16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newi8():
	var from_rust: int = GenFfi.return_static_newi8()
	assert_that(GenFfi.accept_static_newi8(from_rust), "ffi.accept_static_newi8(from_rust)")

	var from_gdscript: int = -128
	var mirrored: int = GenFfi.mirror_static_newi8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newu8():
	var from_rust: int = GenFfi.return_static_newu8()
	assert_that(GenFfi.accept_static_newu8(from_rust), "ffi.accept_static_newu8(from_rust)")

	var from_gdscript: int = 255
	var mirrored: int = GenFfi.mirror_static_newu8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newf32():
	var from_rust: float = GenFfi.return_static_newf32()
	assert_that(GenFfi.accept_static_newf32(from_rust), "ffi.accept_static_newf32(from_rust)")

	var from_gdscript: float = 12.5
	var mirrored: float = GenFfi.mirror_static_newf32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newf64():
	var from_rust: float = GenFfi.return_static_newf64()
	assert_that(GenFfi.accept_static_newf64(from_rust), "ffi.accept_static_newf64(from_rust)")

	var from_gdscript: float = 127.83156478
	var mirrored: float = GenFfi.mirror_static_newf64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newbool():
	var from_rust: bool = GenFfi.return_static_newbool()
	assert_that(GenFfi.accept_static_newbool(from_rust), "ffi.accept_static_newbool(from_rust)")

	var from_gdscript: bool = true
	var mirrored: bool = GenFfi.mirror_static_newbool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newcolor():
	var from_rust: Color = GenFfi.return_static_newcolor()
	assert_that(GenFfi.accept_static_newcolor(from_rust), "ffi.accept_static_newcolor(from_rust)")

	var from_gdscript: Color = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Color = GenFfi.mirror_static_newcolor(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newstring():
	var from_rust: String = GenFfi.return_static_newstring()
	assert_that(GenFfi.accept_static_newstring(from_rust), "ffi.accept_static_newstring(from_rust)")

	var from_gdscript: String = "hello"
	var mirrored: String = GenFfi.mirror_static_newstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newstringname():
	var from_rust: StringName = GenFfi.return_static_newstringname()
	assert_that(GenFfi.accept_static_newstringname(from_rust), "ffi.accept_static_newstringname(from_rust)")

	var from_gdscript: StringName = &"hello"
	var mirrored: StringName = GenFfi.mirror_static_newstringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newnodepath():
	var from_rust: NodePath = GenFfi.return_static_newnodepath()
	assert_that(GenFfi.accept_static_newnodepath(from_rust), "ffi.accept_static_newnodepath(from_rust)")

	var from_gdscript: NodePath = ^"hello"
	var mirrored: NodePath = GenFfi.mirror_static_newnodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newvector2():
	var from_rust: Vector2 = GenFfi.return_static_newvector2()
	assert_that(GenFfi.accept_static_newvector2(from_rust), "ffi.accept_static_newvector2(from_rust)")

	var from_gdscript: Vector2 = Vector2(12.5, -3.5)
	var mirrored: Vector2 = GenFfi.mirror_static_newvector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newvector3():
	var from_rust: Vector3 = GenFfi.return_static_newvector3()
	assert_that(GenFfi.accept_static_newvector3(from_rust), "ffi.accept_static_newvector3(from_rust)")

	var from_gdscript: Vector3 = Vector3(117.5, 100.0, -323.25)
	var mirrored: Vector3 = GenFfi.mirror_static_newvector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newvector4():
	var from_rust: Vector4 = GenFfi.return_static_newvector4()
	assert_that(GenFfi.accept_static_newvector4(from_rust), "ffi.accept_static_newvector4(from_rust)")

	var from_gdscript: Vector4 = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Vector4 = GenFfi.mirror_static_newvector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newvector2i():
	var from_rust: Vector2i = GenFfi.return_static_newvector2i()
	assert_that(GenFfi.accept_static_newvector2i(from_rust), "ffi.accept_static_newvector2i(from_rust)")

	var from_gdscript: Vector2i = Vector2i(-2147483648, 2147483647)
	var mirrored: Vector2i = GenFfi.mirror_static_newvector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newvector3i():
	var from_rust: Vector3i = GenFfi.return_static_newvector3i()
	assert_that(GenFfi.accept_static_newvector3i(from_rust), "ffi.accept_static_newvector3i(from_rust)")

	var from_gdscript: Vector3i = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Vector3i = GenFfi.mirror_static_newvector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_newcallable():
	var from_rust: Callable = GenFfi.return_static_newcallable()
	assert_that(GenFfi.accept_static_newcallable(from_rust), "ffi.accept_static_newcallable(from_rust)")

	var from_gdscript: Callable = Callable()
	var mirrored: Callable = GenFfi.mirror_static_newcallable(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_variantarray():
	var from_rust: Array = GenFfi.return_static_variantarray()
	assert_that(GenFfi.accept_static_variantarray(from_rust), "ffi.accept_static_variantarray(from_rust)")

	var from_gdscript: Array = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Array = GenFfi.mirror_static_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_dictionary():
	var from_rust: Dictionary = GenFfi.return_static_dictionary()
	assert_that(GenFfi.accept_static_dictionary(from_rust), "ffi.accept_static_dictionary(from_rust)")

	var from_gdscript: Dictionary = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Dictionary = GenFfi.mirror_static_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_instanceid():
	var from_rust: int = GenFfi.return_static_instanceid()
	assert_that(GenFfi.accept_static_instanceid(from_rust), "ffi.accept_static_instanceid(from_rust)")

	var from_gdscript: int = -1
	var mirrored: int = GenFfi.mirror_static_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_variant():
	var from_rust: Variant = GenFfi.return_static_variant()
	assert_that(GenFfi.accept_static_variant(from_rust), "ffi.accept_static_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = GenFfi.mirror_static_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

func test_ptrcall_static_error():
	var from_rust: int = GenFfi.return_static_error()
	assert_that(GenFfi.accept_static_error(from_rust), "ffi.accept_static_error(from_rust)")

	var from_gdscript: int = 0
	var mirrored: int = GenFfi.mirror_static_error(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored_static == from_gdscript")

	