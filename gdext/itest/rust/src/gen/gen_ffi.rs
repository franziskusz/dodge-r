#![allow(clippy::partialeq_to_none)]
use godot::builtin::meta::*;
use godot::builtin::*;
use godot::engine::global::Error;
use godot::engine::{Node, Resource};
use godot::obj::{Gd, InstanceId};
#[derive(godot :: register :: GodotClass)]
#[class(init)]
struct GenFfi {}
#[allow(clippy::bool_comparison)]
#[godot::register::godot_api]
impl GenFfi {
    #[func]
    fn return_i64(&self) -> i64 {
        -922337203685477580
    }
    #[func]
    fn accept_i64(&self, i: i64) -> bool {
        i == -922337203685477580
    }
    #[func]
    fn mirror_i64(&self, i: i64) -> i64 {
        i
    }
    #[func]
    fn return_static_i64() -> i64 {
        -922337203685477580
    }
    #[func]
    fn accept_static_i64(i: i64) -> bool {
        i == -922337203685477580
    }
    #[func]
    fn mirror_static_i64(i: i64) -> i64 {
        i
    }
    #[func]
    fn return_i32(&self) -> i32 {
        -2147483648
    }
    #[func]
    fn accept_i32(&self, i: i32) -> bool {
        i == -2147483648
    }
    #[func]
    fn mirror_i32(&self, i: i32) -> i32 {
        i
    }
    #[func]
    fn return_static_i32() -> i32 {
        -2147483648
    }
    #[func]
    fn accept_static_i32(i: i32) -> bool {
        i == -2147483648
    }
    #[func]
    fn mirror_static_i32(i: i32) -> i32 {
        i
    }
    #[func]
    fn return_u32(&self) -> u32 {
        4294967295
    }
    #[func]
    fn accept_u32(&self, i: u32) -> bool {
        i == 4294967295
    }
    #[func]
    fn mirror_u32(&self, i: u32) -> u32 {
        i
    }
    #[func]
    fn return_static_u32() -> u32 {
        4294967295
    }
    #[func]
    fn accept_static_u32(i: u32) -> bool {
        i == 4294967295
    }
    #[func]
    fn mirror_static_u32(i: u32) -> u32 {
        i
    }
    #[func]
    fn return_i16(&self) -> i16 {
        -32767
    }
    #[func]
    fn accept_i16(&self, i: i16) -> bool {
        i == -32767
    }
    #[func]
    fn mirror_i16(&self, i: i16) -> i16 {
        i
    }
    #[func]
    fn return_static_i16() -> i16 {
        -32767
    }
    #[func]
    fn accept_static_i16(i: i16) -> bool {
        i == -32767
    }
    #[func]
    fn mirror_static_i16(i: i16) -> i16 {
        i
    }
    #[func]
    fn return_u16(&self) -> u16 {
        65535
    }
    #[func]
    fn accept_u16(&self, i: u16) -> bool {
        i == 65535
    }
    #[func]
    fn mirror_u16(&self, i: u16) -> u16 {
        i
    }
    #[func]
    fn return_static_u16() -> u16 {
        65535
    }
    #[func]
    fn accept_static_u16(i: u16) -> bool {
        i == 65535
    }
    #[func]
    fn mirror_static_u16(i: u16) -> u16 {
        i
    }
    #[func]
    fn return_i8(&self) -> i8 {
        -128
    }
    #[func]
    fn accept_i8(&self, i: i8) -> bool {
        i == -128
    }
    #[func]
    fn mirror_i8(&self, i: i8) -> i8 {
        i
    }
    #[func]
    fn return_static_i8() -> i8 {
        -128
    }
    #[func]
    fn accept_static_i8(i: i8) -> bool {
        i == -128
    }
    #[func]
    fn mirror_static_i8(i: i8) -> i8 {
        i
    }
    #[func]
    fn return_u8(&self) -> u8 {
        255
    }
    #[func]
    fn accept_u8(&self, i: u8) -> bool {
        i == 255
    }
    #[func]
    fn mirror_u8(&self, i: u8) -> u8 {
        i
    }
    #[func]
    fn return_static_u8() -> u8 {
        255
    }
    #[func]
    fn accept_static_u8(i: u8) -> bool {
        i == 255
    }
    #[func]
    fn mirror_static_u8(i: u8) -> u8 {
        i
    }
    #[func]
    fn return_f32(&self) -> f32 {
        12.5
    }
    #[func]
    fn accept_f32(&self, i: f32) -> bool {
        i == 12.5
    }
    #[func]
    fn mirror_f32(&self, i: f32) -> f32 {
        i
    }
    #[func]
    fn return_static_f32() -> f32 {
        12.5
    }
    #[func]
    fn accept_static_f32(i: f32) -> bool {
        i == 12.5
    }
    #[func]
    fn mirror_static_f32(i: f32) -> f32 {
        i
    }
    #[func]
    fn return_f64(&self) -> f64 {
        127.83156478
    }
    #[func]
    fn accept_f64(&self, i: f64) -> bool {
        i == 127.83156478
    }
    #[func]
    fn mirror_f64(&self, i: f64) -> f64 {
        i
    }
    #[func]
    fn return_static_f64() -> f64 {
        127.83156478
    }
    #[func]
    fn accept_static_f64(i: f64) -> bool {
        i == 127.83156478
    }
    #[func]
    fn mirror_static_f64(i: f64) -> f64 {
        i
    }
    #[func]
    fn return_bool(&self) -> bool {
        true
    }
    #[func]
    fn accept_bool(&self, i: bool) -> bool {
        i == true
    }
    #[func]
    fn mirror_bool(&self, i: bool) -> bool {
        i
    }
    #[func]
    fn return_static_bool() -> bool {
        true
    }
    #[func]
    fn accept_static_bool(i: bool) -> bool {
        i == true
    }
    #[func]
    fn mirror_static_bool(i: bool) -> bool {
        i
    }
    #[func]
    fn return_color(&self) -> Color {
        Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn accept_color(&self, i: Color) -> bool {
        i == Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn mirror_color(&self, i: Color) -> Color {
        i
    }
    #[func]
    fn return_static_color() -> Color {
        Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn accept_static_color(i: Color) -> bool {
        i == Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn mirror_static_color(i: Color) -> Color {
        i
    }
    #[func]
    fn return_gstring(&self) -> GString {
        "hello".into()
    }
    #[func]
    fn accept_gstring(&self, i: GString) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_gstring(&self, i: GString) -> GString {
        i
    }
    #[func]
    fn return_static_gstring() -> GString {
        "hello".into()
    }
    #[func]
    fn accept_static_gstring(i: GString) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_static_gstring(i: GString) -> GString {
        i
    }
    #[func]
    fn return_stringname(&self) -> StringName {
        "hello".into()
    }
    #[func]
    fn accept_stringname(&self, i: StringName) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_stringname(&self, i: StringName) -> StringName {
        i
    }
    #[func]
    fn return_static_stringname() -> StringName {
        "hello".into()
    }
    #[func]
    fn accept_static_stringname(i: StringName) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_static_stringname(i: StringName) -> StringName {
        i
    }
    #[func]
    fn return_nodepath(&self) -> NodePath {
        "hello".into()
    }
    #[func]
    fn accept_nodepath(&self, i: NodePath) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_nodepath(&self, i: NodePath) -> NodePath {
        i
    }
    #[func]
    fn return_static_nodepath() -> NodePath {
        "hello".into()
    }
    #[func]
    fn accept_static_nodepath(i: NodePath) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_static_nodepath(i: NodePath) -> NodePath {
        i
    }
    #[func]
    fn return_vector2(&self) -> Vector2 {
        Vector2::new(12.5, -3.5)
    }
    #[func]
    fn accept_vector2(&self, i: Vector2) -> bool {
        i == Vector2::new(12.5, -3.5)
    }
    #[func]
    fn mirror_vector2(&self, i: Vector2) -> Vector2 {
        i
    }
    #[func]
    fn return_static_vector2() -> Vector2 {
        Vector2::new(12.5, -3.5)
    }
    #[func]
    fn accept_static_vector2(i: Vector2) -> bool {
        i == Vector2::new(12.5, -3.5)
    }
    #[func]
    fn mirror_static_vector2(i: Vector2) -> Vector2 {
        i
    }
    #[func]
    fn return_vector3(&self) -> Vector3 {
        Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn accept_vector3(&self, i: Vector3) -> bool {
        i == Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn mirror_vector3(&self, i: Vector3) -> Vector3 {
        i
    }
    #[func]
    fn return_static_vector3() -> Vector3 {
        Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn accept_static_vector3(i: Vector3) -> bool {
        i == Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn mirror_static_vector3(i: Vector3) -> Vector3 {
        i
    }
    #[func]
    fn return_vector4(&self) -> Vector4 {
        Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn accept_vector4(&self, i: Vector4) -> bool {
        i == Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn mirror_vector4(&self, i: Vector4) -> Vector4 {
        i
    }
    #[func]
    fn return_static_vector4() -> Vector4 {
        Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn accept_static_vector4(i: Vector4) -> bool {
        i == Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn mirror_static_vector4(i: Vector4) -> Vector4 {
        i
    }
    #[func]
    fn return_vector2i(&self) -> Vector2i {
        Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn accept_vector2i(&self, i: Vector2i) -> bool {
        i == Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn mirror_vector2i(&self, i: Vector2i) -> Vector2i {
        i
    }
    #[func]
    fn return_static_vector2i() -> Vector2i {
        Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn accept_static_vector2i(i: Vector2i) -> bool {
        i == Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn mirror_static_vector2i(i: Vector2i) -> Vector2i {
        i
    }
    #[func]
    fn return_vector3i(&self) -> Vector3i {
        Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn accept_vector3i(&self, i: Vector3i) -> bool {
        i == Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn mirror_vector3i(&self, i: Vector3i) -> Vector3i {
        i
    }
    #[func]
    fn return_static_vector3i() -> Vector3i {
        Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn accept_static_vector3i(i: Vector3i) -> bool {
        i == Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn mirror_static_vector3i(i: Vector3i) -> Vector3i {
        i
    }
    #[func]
    fn return_vector4i(&self) -> Vector4i {
        Vector4i::new(-1, -2147483648, 2147483647, 100)
    }
    #[func]
    fn accept_vector4i(&self, i: Vector4i) -> bool {
        i == Vector4i::new(-1, -2147483648, 2147483647, 100)
    }
    #[func]
    fn mirror_vector4i(&self, i: Vector4i) -> Vector4i {
        i
    }
    #[func]
    fn return_static_vector4i() -> Vector4i {
        Vector4i::new(-1, -2147483648, 2147483647, 100)
    }
    #[func]
    fn accept_static_vector4i(i: Vector4i) -> bool {
        i == Vector4i::new(-1, -2147483648, 2147483647, 100)
    }
    #[func]
    fn mirror_static_vector4i(i: Vector4i) -> Vector4i {
        i
    }
    #[func]
    fn return_callable(&self) -> Callable {
        Callable::invalid()
    }
    #[func]
    fn accept_callable(&self, i: Callable) -> bool {
        i == Callable::invalid()
    }
    #[func]
    fn mirror_callable(&self, i: Callable) -> Callable {
        i
    }
    #[func]
    fn return_static_callable() -> Callable {
        Callable::invalid()
    }
    #[func]
    fn accept_static_callable(i: Callable) -> bool {
        i == Callable::invalid()
    }
    #[func]
    fn mirror_static_callable(i: Callable) -> Callable {
        i
    }
    #[func]
    fn return_rect2(&self) -> Rect2 {
        Rect2::default()
    }
    #[func]
    fn accept_rect2(&self, i: Rect2) -> bool {
        i == Rect2::default()
    }
    #[func]
    fn mirror_rect2(&self, i: Rect2) -> Rect2 {
        i
    }
    #[func]
    fn return_static_rect2() -> Rect2 {
        Rect2::default()
    }
    #[func]
    fn accept_static_rect2(i: Rect2) -> bool {
        i == Rect2::default()
    }
    #[func]
    fn mirror_static_rect2(i: Rect2) -> Rect2 {
        i
    }
    #[func]
    fn return_rect2i(&self) -> Rect2i {
        Rect2i::default()
    }
    #[func]
    fn accept_rect2i(&self, i: Rect2i) -> bool {
        i == Rect2i::default()
    }
    #[func]
    fn mirror_rect2i(&self, i: Rect2i) -> Rect2i {
        i
    }
    #[func]
    fn return_static_rect2i() -> Rect2i {
        Rect2i::default()
    }
    #[func]
    fn accept_static_rect2i(i: Rect2i) -> bool {
        i == Rect2i::default()
    }
    #[func]
    fn mirror_static_rect2i(i: Rect2i) -> Rect2i {
        i
    }
    #[func]
    fn return_transform2d(&self) -> Transform2D {
        Transform2D::default()
    }
    #[func]
    fn accept_transform2d(&self, i: Transform2D) -> bool {
        i == Transform2D::default()
    }
    #[func]
    fn mirror_transform2d(&self, i: Transform2D) -> Transform2D {
        i
    }
    #[func]
    fn return_static_transform2d() -> Transform2D {
        Transform2D::default()
    }
    #[func]
    fn accept_static_transform2d(i: Transform2D) -> bool {
        i == Transform2D::default()
    }
    #[func]
    fn mirror_static_transform2d(i: Transform2D) -> Transform2D {
        i
    }
    #[func]
    fn return_plane(&self) -> Plane {
        Plane::new(Vector3::new(1.0, 0.0, 0.0), 0.0)
    }
    #[func]
    fn accept_plane(&self, i: Plane) -> bool {
        i == Plane::new(Vector3::new(1.0, 0.0, 0.0), 0.0)
    }
    #[func]
    fn mirror_plane(&self, i: Plane) -> Plane {
        i
    }
    #[func]
    fn return_static_plane() -> Plane {
        Plane::new(Vector3::new(1.0, 0.0, 0.0), 0.0)
    }
    #[func]
    fn accept_static_plane(i: Plane) -> bool {
        i == Plane::new(Vector3::new(1.0, 0.0, 0.0), 0.0)
    }
    #[func]
    fn mirror_static_plane(i: Plane) -> Plane {
        i
    }
    #[func]
    fn return_quaternion(&self) -> Quaternion {
        Quaternion::default()
    }
    #[func]
    fn accept_quaternion(&self, i: Quaternion) -> bool {
        i == Quaternion::default()
    }
    #[func]
    fn mirror_quaternion(&self, i: Quaternion) -> Quaternion {
        i
    }
    #[func]
    fn return_static_quaternion() -> Quaternion {
        Quaternion::default()
    }
    #[func]
    fn accept_static_quaternion(i: Quaternion) -> bool {
        i == Quaternion::default()
    }
    #[func]
    fn mirror_static_quaternion(i: Quaternion) -> Quaternion {
        i
    }
    #[func]
    fn return_aabb(&self) -> Aabb {
        Aabb::default()
    }
    #[func]
    fn accept_aabb(&self, i: Aabb) -> bool {
        i == Aabb::default()
    }
    #[func]
    fn mirror_aabb(&self, i: Aabb) -> Aabb {
        i
    }
    #[func]
    fn return_static_aabb() -> Aabb {
        Aabb::default()
    }
    #[func]
    fn accept_static_aabb(i: Aabb) -> bool {
        i == Aabb::default()
    }
    #[func]
    fn mirror_static_aabb(i: Aabb) -> Aabb {
        i
    }
    #[func]
    fn return_basis(&self) -> Basis {
        Basis::default()
    }
    #[func]
    fn accept_basis(&self, i: Basis) -> bool {
        i == Basis::default()
    }
    #[func]
    fn mirror_basis(&self, i: Basis) -> Basis {
        i
    }
    #[func]
    fn return_static_basis() -> Basis {
        Basis::default()
    }
    #[func]
    fn accept_static_basis(i: Basis) -> bool {
        i == Basis::default()
    }
    #[func]
    fn mirror_static_basis(i: Basis) -> Basis {
        i
    }
    #[func]
    fn return_transform3d(&self) -> Transform3D {
        Transform3D::default()
    }
    #[func]
    fn accept_transform3d(&self, i: Transform3D) -> bool {
        i == Transform3D::default()
    }
    #[func]
    fn mirror_transform3d(&self, i: Transform3D) -> Transform3D {
        i
    }
    #[func]
    fn return_static_transform3d() -> Transform3D {
        Transform3D::default()
    }
    #[func]
    fn accept_static_transform3d(i: Transform3D) -> bool {
        i == Transform3D::default()
    }
    #[func]
    fn mirror_static_transform3d(i: Transform3D) -> Transform3D {
        i
    }
    #[func]
    fn return_projection(&self) -> Projection {
        Projection::default()
    }
    #[func]
    fn accept_projection(&self, i: Projection) -> bool {
        i == Projection::default()
    }
    #[func]
    fn mirror_projection(&self, i: Projection) -> Projection {
        i
    }
    #[func]
    fn return_static_projection() -> Projection {
        Projection::default()
    }
    #[func]
    fn accept_static_projection(i: Projection) -> bool {
        i == Projection::default()
    }
    #[func]
    fn mirror_static_projection(i: Projection) -> Projection {
        i
    }
    #[func]
    fn return_rid(&self) -> Rid {
        Rid::Invalid
    }
    #[func]
    fn accept_rid(&self, i: Rid) -> bool {
        i == Rid::Invalid
    }
    #[func]
    fn mirror_rid(&self, i: Rid) -> Rid {
        i
    }
    #[func]
    fn return_static_rid() -> Rid {
        Rid::Invalid
    }
    #[func]
    fn accept_static_rid(i: Rid) -> bool {
        i == Rid::Invalid
    }
    #[func]
    fn mirror_static_rid(i: Rid) -> Rid {
        i
    }
    #[func]
    fn return_option_gd_node(&self) -> Option<Gd<Node>> {
        None
    }
    #[func]
    fn accept_option_gd_node(&self, i: Option<Gd<Node>>) -> bool {
        i == None
    }
    #[func]
    fn mirror_option_gd_node(&self, i: Option<Gd<Node>>) -> Option<Gd<Node>> {
        i
    }
    #[func]
    fn return_static_option_gd_node() -> Option<Gd<Node>> {
        None
    }
    #[func]
    fn accept_static_option_gd_node(i: Option<Gd<Node>>) -> bool {
        i == None
    }
    #[func]
    fn mirror_static_option_gd_node(i: Option<Gd<Node>>) -> Option<Gd<Node>> {
        i
    }
    #[func]
    fn return_option_gd_resource(&self) -> Option<Gd<Resource>> {
        None
    }
    #[func]
    fn accept_option_gd_resource(&self, i: Option<Gd<Resource>>) -> bool {
        i == None
    }
    #[func]
    fn mirror_option_gd_resource(&self, i: Option<Gd<Resource>>) -> Option<Gd<Resource>> {
        i
    }
    #[func]
    fn return_static_option_gd_resource() -> Option<Gd<Resource>> {
        None
    }
    #[func]
    fn accept_static_option_gd_resource(i: Option<Gd<Resource>>) -> bool {
        i == None
    }
    #[func]
    fn mirror_static_option_gd_resource(i: Option<Gd<Resource>>) -> Option<Gd<Resource>> {
        i
    }
    #[func]
    fn return_packedbytearray(&self) -> PackedByteArray {
        PackedByteArray::new()
    }
    #[func]
    fn accept_packedbytearray(&self, i: PackedByteArray) -> bool {
        i == PackedByteArray::new()
    }
    #[func]
    fn mirror_packedbytearray(&self, i: PackedByteArray) -> PackedByteArray {
        i
    }
    #[func]
    fn return_static_packedbytearray() -> PackedByteArray {
        PackedByteArray::new()
    }
    #[func]
    fn accept_static_packedbytearray(i: PackedByteArray) -> bool {
        i == PackedByteArray::new()
    }
    #[func]
    fn mirror_static_packedbytearray(i: PackedByteArray) -> PackedByteArray {
        i
    }
    #[func]
    fn return_packedint32array(&self) -> PackedInt32Array {
        PackedInt32Array::new()
    }
    #[func]
    fn accept_packedint32array(&self, i: PackedInt32Array) -> bool {
        i == PackedInt32Array::new()
    }
    #[func]
    fn mirror_packedint32array(&self, i: PackedInt32Array) -> PackedInt32Array {
        i
    }
    #[func]
    fn return_static_packedint32array() -> PackedInt32Array {
        PackedInt32Array::new()
    }
    #[func]
    fn accept_static_packedint32array(i: PackedInt32Array) -> bool {
        i == PackedInt32Array::new()
    }
    #[func]
    fn mirror_static_packedint32array(i: PackedInt32Array) -> PackedInt32Array {
        i
    }
    #[func]
    fn return_packedint64array(&self) -> PackedInt64Array {
        PackedInt64Array::new()
    }
    #[func]
    fn accept_packedint64array(&self, i: PackedInt64Array) -> bool {
        i == PackedInt64Array::new()
    }
    #[func]
    fn mirror_packedint64array(&self, i: PackedInt64Array) -> PackedInt64Array {
        i
    }
    #[func]
    fn return_static_packedint64array() -> PackedInt64Array {
        PackedInt64Array::new()
    }
    #[func]
    fn accept_static_packedint64array(i: PackedInt64Array) -> bool {
        i == PackedInt64Array::new()
    }
    #[func]
    fn mirror_static_packedint64array(i: PackedInt64Array) -> PackedInt64Array {
        i
    }
    #[func]
    fn return_packedfloat32array(&self) -> PackedFloat32Array {
        PackedFloat32Array::new()
    }
    #[func]
    fn accept_packedfloat32array(&self, i: PackedFloat32Array) -> bool {
        i == PackedFloat32Array::new()
    }
    #[func]
    fn mirror_packedfloat32array(&self, i: PackedFloat32Array) -> PackedFloat32Array {
        i
    }
    #[func]
    fn return_static_packedfloat32array() -> PackedFloat32Array {
        PackedFloat32Array::new()
    }
    #[func]
    fn accept_static_packedfloat32array(i: PackedFloat32Array) -> bool {
        i == PackedFloat32Array::new()
    }
    #[func]
    fn mirror_static_packedfloat32array(i: PackedFloat32Array) -> PackedFloat32Array {
        i
    }
    #[func]
    fn return_packedfloat64array(&self) -> PackedFloat64Array {
        PackedFloat64Array::new()
    }
    #[func]
    fn accept_packedfloat64array(&self, i: PackedFloat64Array) -> bool {
        i == PackedFloat64Array::new()
    }
    #[func]
    fn mirror_packedfloat64array(&self, i: PackedFloat64Array) -> PackedFloat64Array {
        i
    }
    #[func]
    fn return_static_packedfloat64array() -> PackedFloat64Array {
        PackedFloat64Array::new()
    }
    #[func]
    fn accept_static_packedfloat64array(i: PackedFloat64Array) -> bool {
        i == PackedFloat64Array::new()
    }
    #[func]
    fn mirror_static_packedfloat64array(i: PackedFloat64Array) -> PackedFloat64Array {
        i
    }
    #[func]
    fn return_packedstringarray(&self) -> PackedStringArray {
        PackedStringArray::new()
    }
    #[func]
    fn accept_packedstringarray(&self, i: PackedStringArray) -> bool {
        i == PackedStringArray::new()
    }
    #[func]
    fn mirror_packedstringarray(&self, i: PackedStringArray) -> PackedStringArray {
        i
    }
    #[func]
    fn return_static_packedstringarray() -> PackedStringArray {
        PackedStringArray::new()
    }
    #[func]
    fn accept_static_packedstringarray(i: PackedStringArray) -> bool {
        i == PackedStringArray::new()
    }
    #[func]
    fn mirror_static_packedstringarray(i: PackedStringArray) -> PackedStringArray {
        i
    }
    #[func]
    fn return_packedvector2array(&self) -> PackedVector2Array {
        PackedVector2Array::new()
    }
    #[func]
    fn accept_packedvector2array(&self, i: PackedVector2Array) -> bool {
        i == PackedVector2Array::new()
    }
    #[func]
    fn mirror_packedvector2array(&self, i: PackedVector2Array) -> PackedVector2Array {
        i
    }
    #[func]
    fn return_static_packedvector2array() -> PackedVector2Array {
        PackedVector2Array::new()
    }
    #[func]
    fn accept_static_packedvector2array(i: PackedVector2Array) -> bool {
        i == PackedVector2Array::new()
    }
    #[func]
    fn mirror_static_packedvector2array(i: PackedVector2Array) -> PackedVector2Array {
        i
    }
    #[func]
    fn return_packedvector3array(&self) -> PackedVector3Array {
        PackedVector3Array::new()
    }
    #[func]
    fn accept_packedvector3array(&self, i: PackedVector3Array) -> bool {
        i == PackedVector3Array::new()
    }
    #[func]
    fn mirror_packedvector3array(&self, i: PackedVector3Array) -> PackedVector3Array {
        i
    }
    #[func]
    fn return_static_packedvector3array() -> PackedVector3Array {
        PackedVector3Array::new()
    }
    #[func]
    fn accept_static_packedvector3array(i: PackedVector3Array) -> bool {
        i == PackedVector3Array::new()
    }
    #[func]
    fn mirror_static_packedvector3array(i: PackedVector3Array) -> PackedVector3Array {
        i
    }
    #[func]
    fn return_packedcolorarray(&self) -> PackedColorArray {
        PackedColorArray::new()
    }
    #[func]
    fn accept_packedcolorarray(&self, i: PackedColorArray) -> bool {
        i == PackedColorArray::new()
    }
    #[func]
    fn mirror_packedcolorarray(&self, i: PackedColorArray) -> PackedColorArray {
        i
    }
    #[func]
    fn return_static_packedcolorarray() -> PackedColorArray {
        PackedColorArray::new()
    }
    #[func]
    fn accept_static_packedcolorarray(i: PackedColorArray) -> bool {
        i == PackedColorArray::new()
    }
    #[func]
    fn mirror_static_packedcolorarray(i: PackedColorArray) -> PackedColorArray {
        i
    }
    #[func]
    fn return_newi64(&self) -> NewI64 {
        NewI64(-922337203685477580)
    }
    #[func]
    fn accept_newi64(&self, i: NewI64) -> bool {
        i == NewI64(-922337203685477580)
    }
    #[func]
    fn mirror_newi64(&self, i: NewI64) -> NewI64 {
        i
    }
    #[func]
    fn return_static_newi64() -> NewI64 {
        NewI64(-922337203685477580)
    }
    #[func]
    fn accept_static_newi64(i: NewI64) -> bool {
        i == NewI64(-922337203685477580)
    }
    #[func]
    fn mirror_static_newi64(i: NewI64) -> NewI64 {
        i
    }
    #[func]
    fn return_newi32(&self) -> NewI32 {
        NewI32(-2147483648)
    }
    #[func]
    fn accept_newi32(&self, i: NewI32) -> bool {
        i == NewI32(-2147483648)
    }
    #[func]
    fn mirror_newi32(&self, i: NewI32) -> NewI32 {
        i
    }
    #[func]
    fn return_static_newi32() -> NewI32 {
        NewI32(-2147483648)
    }
    #[func]
    fn accept_static_newi32(i: NewI32) -> bool {
        i == NewI32(-2147483648)
    }
    #[func]
    fn mirror_static_newi32(i: NewI32) -> NewI32 {
        i
    }
    #[func]
    fn return_newu32(&self) -> NewU32 {
        NewU32(4294967295)
    }
    #[func]
    fn accept_newu32(&self, i: NewU32) -> bool {
        i == NewU32(4294967295)
    }
    #[func]
    fn mirror_newu32(&self, i: NewU32) -> NewU32 {
        i
    }
    #[func]
    fn return_static_newu32() -> NewU32 {
        NewU32(4294967295)
    }
    #[func]
    fn accept_static_newu32(i: NewU32) -> bool {
        i == NewU32(4294967295)
    }
    #[func]
    fn mirror_static_newu32(i: NewU32) -> NewU32 {
        i
    }
    #[func]
    fn return_newi16(&self) -> NewI16 {
        NewI16(-32767)
    }
    #[func]
    fn accept_newi16(&self, i: NewI16) -> bool {
        i == NewI16(-32767)
    }
    #[func]
    fn mirror_newi16(&self, i: NewI16) -> NewI16 {
        i
    }
    #[func]
    fn return_static_newi16() -> NewI16 {
        NewI16(-32767)
    }
    #[func]
    fn accept_static_newi16(i: NewI16) -> bool {
        i == NewI16(-32767)
    }
    #[func]
    fn mirror_static_newi16(i: NewI16) -> NewI16 {
        i
    }
    #[func]
    fn return_newu16(&self) -> NewU16 {
        NewU16(65535)
    }
    #[func]
    fn accept_newu16(&self, i: NewU16) -> bool {
        i == NewU16(65535)
    }
    #[func]
    fn mirror_newu16(&self, i: NewU16) -> NewU16 {
        i
    }
    #[func]
    fn return_static_newu16() -> NewU16 {
        NewU16(65535)
    }
    #[func]
    fn accept_static_newu16(i: NewU16) -> bool {
        i == NewU16(65535)
    }
    #[func]
    fn mirror_static_newu16(i: NewU16) -> NewU16 {
        i
    }
    #[func]
    fn return_newi8(&self) -> NewI8 {
        NewI8(-128)
    }
    #[func]
    fn accept_newi8(&self, i: NewI8) -> bool {
        i == NewI8(-128)
    }
    #[func]
    fn mirror_newi8(&self, i: NewI8) -> NewI8 {
        i
    }
    #[func]
    fn return_static_newi8() -> NewI8 {
        NewI8(-128)
    }
    #[func]
    fn accept_static_newi8(i: NewI8) -> bool {
        i == NewI8(-128)
    }
    #[func]
    fn mirror_static_newi8(i: NewI8) -> NewI8 {
        i
    }
    #[func]
    fn return_newu8(&self) -> NewU8 {
        NewU8(255)
    }
    #[func]
    fn accept_newu8(&self, i: NewU8) -> bool {
        i == NewU8(255)
    }
    #[func]
    fn mirror_newu8(&self, i: NewU8) -> NewU8 {
        i
    }
    #[func]
    fn return_static_newu8() -> NewU8 {
        NewU8(255)
    }
    #[func]
    fn accept_static_newu8(i: NewU8) -> bool {
        i == NewU8(255)
    }
    #[func]
    fn mirror_static_newu8(i: NewU8) -> NewU8 {
        i
    }
    #[func]
    fn return_newf32(&self) -> NewF32 {
        NewF32(12.5)
    }
    #[func]
    fn accept_newf32(&self, i: NewF32) -> bool {
        i == NewF32(12.5)
    }
    #[func]
    fn mirror_newf32(&self, i: NewF32) -> NewF32 {
        i
    }
    #[func]
    fn return_static_newf32() -> NewF32 {
        NewF32(12.5)
    }
    #[func]
    fn accept_static_newf32(i: NewF32) -> bool {
        i == NewF32(12.5)
    }
    #[func]
    fn mirror_static_newf32(i: NewF32) -> NewF32 {
        i
    }
    #[func]
    fn return_newf64(&self) -> NewF64 {
        NewF64(127.83156478)
    }
    #[func]
    fn accept_newf64(&self, i: NewF64) -> bool {
        i == NewF64(127.83156478)
    }
    #[func]
    fn mirror_newf64(&self, i: NewF64) -> NewF64 {
        i
    }
    #[func]
    fn return_static_newf64() -> NewF64 {
        NewF64(127.83156478)
    }
    #[func]
    fn accept_static_newf64(i: NewF64) -> bool {
        i == NewF64(127.83156478)
    }
    #[func]
    fn mirror_static_newf64(i: NewF64) -> NewF64 {
        i
    }
    #[func]
    fn return_newbool(&self) -> NewBool {
        NewBool(true)
    }
    #[func]
    fn accept_newbool(&self, i: NewBool) -> bool {
        i == NewBool(true)
    }
    #[func]
    fn mirror_newbool(&self, i: NewBool) -> NewBool {
        i
    }
    #[func]
    fn return_static_newbool() -> NewBool {
        NewBool(true)
    }
    #[func]
    fn accept_static_newbool(i: NewBool) -> bool {
        i == NewBool(true)
    }
    #[func]
    fn mirror_static_newbool(i: NewBool) -> NewBool {
        i
    }
    #[func]
    fn return_newcolor(&self) -> NewColor {
        NewColor(Color::from_rgba(0.7, 0.5, 0.3, 0.2))
    }
    #[func]
    fn accept_newcolor(&self, i: NewColor) -> bool {
        i == NewColor(Color::from_rgba(0.7, 0.5, 0.3, 0.2))
    }
    #[func]
    fn mirror_newcolor(&self, i: NewColor) -> NewColor {
        i
    }
    #[func]
    fn return_static_newcolor() -> NewColor {
        NewColor(Color::from_rgba(0.7, 0.5, 0.3, 0.2))
    }
    #[func]
    fn accept_static_newcolor(i: NewColor) -> bool {
        i == NewColor(Color::from_rgba(0.7, 0.5, 0.3, 0.2))
    }
    #[func]
    fn mirror_static_newcolor(i: NewColor) -> NewColor {
        i
    }
    #[func]
    fn return_newstring(&self) -> NewString {
        NewString("hello".into())
    }
    #[func]
    fn accept_newstring(&self, i: NewString) -> bool {
        i == NewString("hello".into())
    }
    #[func]
    fn mirror_newstring(&self, i: NewString) -> NewString {
        i
    }
    #[func]
    fn return_static_newstring() -> NewString {
        NewString("hello".into())
    }
    #[func]
    fn accept_static_newstring(i: NewString) -> bool {
        i == NewString("hello".into())
    }
    #[func]
    fn mirror_static_newstring(i: NewString) -> NewString {
        i
    }
    #[func]
    fn return_newstringname(&self) -> NewStringName {
        NewStringName("hello".into())
    }
    #[func]
    fn accept_newstringname(&self, i: NewStringName) -> bool {
        i == NewStringName("hello".into())
    }
    #[func]
    fn mirror_newstringname(&self, i: NewStringName) -> NewStringName {
        i
    }
    #[func]
    fn return_static_newstringname() -> NewStringName {
        NewStringName("hello".into())
    }
    #[func]
    fn accept_static_newstringname(i: NewStringName) -> bool {
        i == NewStringName("hello".into())
    }
    #[func]
    fn mirror_static_newstringname(i: NewStringName) -> NewStringName {
        i
    }
    #[func]
    fn return_newnodepath(&self) -> NewNodePath {
        NewNodePath("hello".into())
    }
    #[func]
    fn accept_newnodepath(&self, i: NewNodePath) -> bool {
        i == NewNodePath("hello".into())
    }
    #[func]
    fn mirror_newnodepath(&self, i: NewNodePath) -> NewNodePath {
        i
    }
    #[func]
    fn return_static_newnodepath() -> NewNodePath {
        NewNodePath("hello".into())
    }
    #[func]
    fn accept_static_newnodepath(i: NewNodePath) -> bool {
        i == NewNodePath("hello".into())
    }
    #[func]
    fn mirror_static_newnodepath(i: NewNodePath) -> NewNodePath {
        i
    }
    #[func]
    fn return_newvector2(&self) -> NewVector2 {
        NewVector2(Vector2::new(12.5, -3.5))
    }
    #[func]
    fn accept_newvector2(&self, i: NewVector2) -> bool {
        i == NewVector2(Vector2::new(12.5, -3.5))
    }
    #[func]
    fn mirror_newvector2(&self, i: NewVector2) -> NewVector2 {
        i
    }
    #[func]
    fn return_static_newvector2() -> NewVector2 {
        NewVector2(Vector2::new(12.5, -3.5))
    }
    #[func]
    fn accept_static_newvector2(i: NewVector2) -> bool {
        i == NewVector2(Vector2::new(12.5, -3.5))
    }
    #[func]
    fn mirror_static_newvector2(i: NewVector2) -> NewVector2 {
        i
    }
    #[func]
    fn return_newvector3(&self) -> NewVector3 {
        NewVector3(Vector3::new(117.5, 100.0, -323.25))
    }
    #[func]
    fn accept_newvector3(&self, i: NewVector3) -> bool {
        i == NewVector3(Vector3::new(117.5, 100.0, -323.25))
    }
    #[func]
    fn mirror_newvector3(&self, i: NewVector3) -> NewVector3 {
        i
    }
    #[func]
    fn return_static_newvector3() -> NewVector3 {
        NewVector3(Vector3::new(117.5, 100.0, -323.25))
    }
    #[func]
    fn accept_static_newvector3(i: NewVector3) -> bool {
        i == NewVector3(Vector3::new(117.5, 100.0, -323.25))
    }
    #[func]
    fn mirror_static_newvector3(i: NewVector3) -> NewVector3 {
        i
    }
    #[func]
    fn return_newvector4(&self) -> NewVector4 {
        NewVector4(Vector4::new(-18.5, 24.75, -1.25, 777.875))
    }
    #[func]
    fn accept_newvector4(&self, i: NewVector4) -> bool {
        i == NewVector4(Vector4::new(-18.5, 24.75, -1.25, 777.875))
    }
    #[func]
    fn mirror_newvector4(&self, i: NewVector4) -> NewVector4 {
        i
    }
    #[func]
    fn return_static_newvector4() -> NewVector4 {
        NewVector4(Vector4::new(-18.5, 24.75, -1.25, 777.875))
    }
    #[func]
    fn accept_static_newvector4(i: NewVector4) -> bool {
        i == NewVector4(Vector4::new(-18.5, 24.75, -1.25, 777.875))
    }
    #[func]
    fn mirror_static_newvector4(i: NewVector4) -> NewVector4 {
        i
    }
    #[func]
    fn return_newvector2i(&self) -> NewVector2i {
        NewVector2i(Vector2i::new(-2147483648, 2147483647))
    }
    #[func]
    fn accept_newvector2i(&self, i: NewVector2i) -> bool {
        i == NewVector2i(Vector2i::new(-2147483648, 2147483647))
    }
    #[func]
    fn mirror_newvector2i(&self, i: NewVector2i) -> NewVector2i {
        i
    }
    #[func]
    fn return_static_newvector2i() -> NewVector2i {
        NewVector2i(Vector2i::new(-2147483648, 2147483647))
    }
    #[func]
    fn accept_static_newvector2i(i: NewVector2i) -> bool {
        i == NewVector2i(Vector2i::new(-2147483648, 2147483647))
    }
    #[func]
    fn mirror_static_newvector2i(i: NewVector2i) -> NewVector2i {
        i
    }
    #[func]
    fn return_newvector3i(&self) -> NewVector3i {
        NewVector3i(Vector3i::new(-1, -2147483648, 2147483647))
    }
    #[func]
    fn accept_newvector3i(&self, i: NewVector3i) -> bool {
        i == NewVector3i(Vector3i::new(-1, -2147483648, 2147483647))
    }
    #[func]
    fn mirror_newvector3i(&self, i: NewVector3i) -> NewVector3i {
        i
    }
    #[func]
    fn return_static_newvector3i() -> NewVector3i {
        NewVector3i(Vector3i::new(-1, -2147483648, 2147483647))
    }
    #[func]
    fn accept_static_newvector3i(i: NewVector3i) -> bool {
        i == NewVector3i(Vector3i::new(-1, -2147483648, 2147483647))
    }
    #[func]
    fn mirror_static_newvector3i(i: NewVector3i) -> NewVector3i {
        i
    }
    #[func]
    fn return_newcallable(&self) -> NewCallable {
        NewCallable(Callable::invalid())
    }
    #[func]
    fn accept_newcallable(&self, i: NewCallable) -> bool {
        i == NewCallable(Callable::invalid())
    }
    #[func]
    fn mirror_newcallable(&self, i: NewCallable) -> NewCallable {
        i
    }
    #[func]
    fn return_static_newcallable() -> NewCallable {
        NewCallable(Callable::invalid())
    }
    #[func]
    fn accept_static_newcallable(i: NewCallable) -> bool {
        i == NewCallable(Callable::invalid())
    }
    #[func]
    fn mirror_static_newcallable(i: NewCallable) -> NewCallable {
        i
    }
    #[func]
    fn return_variantarray(&self) -> VariantArray {
        varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn accept_variantarray(&self, i: VariantArray) -> bool {
        i == varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn mirror_variantarray(&self, i: VariantArray) -> VariantArray {
        i
    }
    #[func]
    fn return_static_variantarray() -> VariantArray {
        varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn accept_static_variantarray(i: VariantArray) -> bool {
        i == varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn mirror_static_variantarray(i: VariantArray) -> VariantArray {
        i
    }
    #[func]
    fn return_dictionary(&self) -> Dictionary {
        dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn accept_dictionary(&self, i: Dictionary) -> bool {
        i == dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn mirror_dictionary(&self, i: Dictionary) -> Dictionary {
        i
    }
    #[func]
    fn return_static_dictionary() -> Dictionary {
        dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn accept_static_dictionary(i: Dictionary) -> bool {
        i == dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn mirror_static_dictionary(i: Dictionary) -> Dictionary {
        i
    }
    #[func]
    fn return_instanceid(&self) -> InstanceId {
        InstanceId::from_i64(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn accept_instanceid(&self, i: InstanceId) -> bool {
        i == InstanceId::from_i64(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn mirror_instanceid(&self, i: InstanceId) -> InstanceId {
        i
    }
    #[func]
    fn return_static_instanceid() -> InstanceId {
        InstanceId::from_i64(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn accept_static_instanceid(i: InstanceId) -> bool {
        i == InstanceId::from_i64(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn mirror_static_instanceid(i: InstanceId) -> InstanceId {
        i
    }
    #[func]
    fn return_variant(&self) -> Variant {
        123i64.to_variant()
    }
    #[func]
    fn accept_variant(&self, i: Variant) -> bool {
        i == 123i64.to_variant()
    }
    #[func]
    fn mirror_variant(&self, i: Variant) -> Variant {
        i
    }
    #[func]
    fn return_static_variant() -> Variant {
        123i64.to_variant()
    }
    #[func]
    fn accept_static_variant(i: Variant) -> bool {
        i == 123i64.to_variant()
    }
    #[func]
    fn mirror_static_variant(i: Variant) -> Variant {
        i
    }
    #[func]
    fn return_error(&self) -> Error {
        Error::OK
    }
    #[func]
    fn accept_error(&self, i: Error) -> bool {
        i == Error::OK
    }
    #[func]
    fn mirror_error(&self, i: Error) -> Error {
        i
    }
    #[func]
    fn return_static_error() -> Error {
        Error::OK
    }
    #[func]
    fn accept_static_error(i: Error) -> bool {
        i == Error::OK
    }
    #[func]
    fn mirror_static_error(i: Error) -> Error {
        i
    }
}
mod property_tests {
    use godot::prelude::*;
    #[derive(GodotClass)]
    # [class (base = Node , init)]
    pub struct PropertyTestsRust {
        #[var]
        property_i64: i64,
        #[var]
        property_array_i64: Array<i64>,
        #[export]
        export_i64: i64,
        #[export]
        export_array_i64: Array<i64>,
        #[var]
        property_i32: i32,
        #[var]
        property_array_i32: Array<i32>,
        #[export]
        export_i32: i32,
        #[export]
        export_array_i32: Array<i32>,
        #[var]
        property_u32: u32,
        #[var]
        property_array_u32: Array<u32>,
        #[export]
        export_u32: u32,
        #[export]
        export_array_u32: Array<u32>,
        #[var]
        property_i16: i16,
        #[var]
        property_array_i16: Array<i16>,
        #[export]
        export_i16: i16,
        #[export]
        export_array_i16: Array<i16>,
        #[var]
        property_u16: u16,
        #[var]
        property_array_u16: Array<u16>,
        #[export]
        export_u16: u16,
        #[export]
        export_array_u16: Array<u16>,
        #[var]
        property_i8: i8,
        #[var]
        property_array_i8: Array<i8>,
        #[export]
        export_i8: i8,
        #[export]
        export_array_i8: Array<i8>,
        #[var]
        property_u8: u8,
        #[var]
        property_array_u8: Array<u8>,
        #[export]
        export_u8: u8,
        #[export]
        export_array_u8: Array<u8>,
        #[var]
        property_f32: f32,
        #[var]
        property_array_f32: Array<f32>,
        #[export]
        export_f32: f32,
        #[export]
        export_array_f32: Array<f32>,
        #[var]
        property_f64: f64,
        #[var]
        property_array_f64: Array<f64>,
        #[export]
        export_f64: f64,
        #[export]
        export_array_f64: Array<f64>,
        #[var]
        property_bool: bool,
        #[var]
        property_array_bool: Array<bool>,
        #[export]
        export_bool: bool,
        #[export]
        export_array_bool: Array<bool>,
        #[var]
        property_color: Color,
        #[var]
        property_array_color: Array<Color>,
        #[export]
        export_color: Color,
        #[export]
        export_array_color: Array<Color>,
        #[var]
        property_gstring: GString,
        #[var]
        property_array_gstring: Array<GString>,
        #[export]
        export_gstring: GString,
        #[export]
        export_array_gstring: Array<GString>,
        #[var]
        property_stringname: StringName,
        #[var]
        property_array_stringname: Array<StringName>,
        #[export]
        export_stringname: StringName,
        #[export]
        export_array_stringname: Array<StringName>,
        #[var]
        property_nodepath: NodePath,
        #[var]
        property_array_nodepath: Array<NodePath>,
        #[export]
        export_nodepath: NodePath,
        #[export]
        export_array_nodepath: Array<NodePath>,
        #[var]
        property_vector2: Vector2,
        #[var]
        property_array_vector2: Array<Vector2>,
        #[export]
        export_vector2: Vector2,
        #[export]
        export_array_vector2: Array<Vector2>,
        #[var]
        property_vector3: Vector3,
        #[var]
        property_array_vector3: Array<Vector3>,
        #[export]
        export_vector3: Vector3,
        #[export]
        export_array_vector3: Array<Vector3>,
        #[var]
        property_vector4: Vector4,
        #[var]
        property_array_vector4: Array<Vector4>,
        #[export]
        export_vector4: Vector4,
        #[export]
        export_array_vector4: Array<Vector4>,
        #[var]
        property_vector2i: Vector2i,
        #[var]
        property_array_vector2i: Array<Vector2i>,
        #[export]
        export_vector2i: Vector2i,
        #[export]
        export_array_vector2i: Array<Vector2i>,
        #[var]
        property_vector3i: Vector3i,
        #[var]
        property_array_vector3i: Array<Vector3i>,
        #[export]
        export_vector3i: Vector3i,
        #[export]
        export_array_vector3i: Array<Vector3i>,
        #[var]
        property_vector4i: Vector4i,
        #[var]
        property_array_vector4i: Array<Vector4i>,
        #[export]
        export_vector4i: Vector4i,
        #[export]
        export_array_vector4i: Array<Vector4i>,
        #[var]
        # [init (default = Callable :: invalid ())]
        property_callable: Callable,
        #[var]
        property_array_callable: Array<Callable>,
        #[var]
        property_rect2: Rect2,
        #[var]
        property_array_rect2: Array<Rect2>,
        #[export]
        export_rect2: Rect2,
        #[export]
        export_array_rect2: Array<Rect2>,
        #[var]
        property_rect2i: Rect2i,
        #[var]
        property_array_rect2i: Array<Rect2i>,
        #[export]
        export_rect2i: Rect2i,
        #[export]
        export_array_rect2i: Array<Rect2i>,
        #[var]
        property_transform2d: Transform2D,
        #[var]
        property_array_transform2d: Array<Transform2D>,
        #[export]
        export_transform2d: Transform2D,
        #[export]
        export_array_transform2d: Array<Transform2D>,
        #[var]
        # [init (default = Plane :: new (Vector3 :: new (1.0 , 0.0 , 0.0) , 0.0))]
        property_plane: Plane,
        #[var]
        property_array_plane: Array<Plane>,
        #[export]
        # [init (default = Plane :: new (Vector3 :: new (1.0 , 0.0 , 0.0) , 0.0))]
        export_plane: Plane,
        #[export]
        export_array_plane: Array<Plane>,
        #[var]
        property_quaternion: Quaternion,
        #[var]
        property_array_quaternion: Array<Quaternion>,
        #[export]
        export_quaternion: Quaternion,
        #[export]
        export_array_quaternion: Array<Quaternion>,
        #[var]
        property_aabb: Aabb,
        #[var]
        property_array_aabb: Array<Aabb>,
        #[export]
        export_aabb: Aabb,
        #[export]
        export_array_aabb: Array<Aabb>,
        #[var]
        property_basis: Basis,
        #[var]
        property_array_basis: Array<Basis>,
        #[export]
        export_basis: Basis,
        #[export]
        export_array_basis: Array<Basis>,
        #[var]
        property_transform3d: Transform3D,
        #[var]
        property_array_transform3d: Array<Transform3D>,
        #[export]
        export_transform3d: Transform3D,
        #[export]
        export_array_transform3d: Array<Transform3D>,
        #[var]
        property_projection: Projection,
        #[var]
        property_array_projection: Array<Projection>,
        #[export]
        export_projection: Projection,
        #[export]
        export_array_projection: Array<Projection>,
        #[var]
        # [init (default = Rid :: Invalid)]
        property_rid: Rid,
        #[var]
        property_array_rid: Array<Rid>,
        #[var]
        property_option_gd_node: Option<Gd<Node>>,
        #[var]
        property_array_option_gd_node: Array<Option<Gd<Node>>>,
        #[export]
        export_option_gd_node: Option<Gd<Node>>,
        #[export]
        export_array_option_gd_node: Array<Option<Gd<Node>>>,
        #[var]
        property_option_gd_resource: Option<Gd<Resource>>,
        #[var]
        property_array_option_gd_resource: Array<Option<Gd<Resource>>>,
        #[export]
        export_option_gd_resource: Option<Gd<Resource>>,
        #[export]
        export_array_option_gd_resource: Array<Option<Gd<Resource>>>,
        #[var]
        property_packedbytearray: PackedByteArray,
        #[var]
        property_array_packedbytearray: Array<PackedByteArray>,
        #[export]
        export_packedbytearray: PackedByteArray,
        #[export]
        export_array_packedbytearray: Array<PackedByteArray>,
        #[var]
        property_packedint32array: PackedInt32Array,
        #[var]
        property_array_packedint32array: Array<PackedInt32Array>,
        #[export]
        export_packedint32array: PackedInt32Array,
        #[export]
        export_array_packedint32array: Array<PackedInt32Array>,
        #[var]
        property_packedint64array: PackedInt64Array,
        #[var]
        property_array_packedint64array: Array<PackedInt64Array>,
        #[export]
        export_packedint64array: PackedInt64Array,
        #[export]
        export_array_packedint64array: Array<PackedInt64Array>,
        #[var]
        property_packedfloat32array: PackedFloat32Array,
        #[var]
        property_array_packedfloat32array: Array<PackedFloat32Array>,
        #[export]
        export_packedfloat32array: PackedFloat32Array,
        #[export]
        export_array_packedfloat32array: Array<PackedFloat32Array>,
        #[var]
        property_packedfloat64array: PackedFloat64Array,
        #[var]
        property_array_packedfloat64array: Array<PackedFloat64Array>,
        #[export]
        export_packedfloat64array: PackedFloat64Array,
        #[export]
        export_array_packedfloat64array: Array<PackedFloat64Array>,
        #[var]
        property_packedstringarray: PackedStringArray,
        #[var]
        property_array_packedstringarray: Array<PackedStringArray>,
        #[export]
        export_packedstringarray: PackedStringArray,
        #[export]
        export_array_packedstringarray: Array<PackedStringArray>,
        #[var]
        property_packedvector2array: PackedVector2Array,
        #[var]
        property_array_packedvector2array: Array<PackedVector2Array>,
        #[export]
        export_packedvector2array: PackedVector2Array,
        #[export]
        export_array_packedvector2array: Array<PackedVector2Array>,
        #[var]
        property_packedvector3array: PackedVector3Array,
        #[var]
        property_array_packedvector3array: Array<PackedVector3Array>,
        #[export]
        export_packedvector3array: PackedVector3Array,
        #[export]
        export_array_packedvector3array: Array<PackedVector3Array>,
        #[var]
        property_packedcolorarray: PackedColorArray,
        #[var]
        property_array_packedcolorarray: Array<PackedColorArray>,
        #[export]
        export_packedcolorarray: PackedColorArray,
        #[export]
        export_array_packedcolorarray: Array<PackedColorArray>,
        #[var]
        property_variantarray: VariantArray,
        #[var]
        property_array_variantarray: Array<VariantArray>,
        #[export]
        export_variantarray: VariantArray,
        #[export]
        export_array_variantarray: Array<VariantArray>,
        #[var]
        property_dictionary: Dictionary,
        #[var]
        property_array_dictionary: Array<Dictionary>,
        #[export]
        export_dictionary: Dictionary,
        #[export]
        export_array_dictionary: Array<Dictionary>,
        #[export(file)]
        export_file: GString,
        #[export(file = "*.txt")]
        export_file_wildcard_txt: GString,
        #[export(global_file)]
        export_global_file: GString,
        #[export(global_file = "*.png")]
        export_global_file_wildcard_png: GString,
        #[export(dir)]
        export_dir: GString,
        #[export(global_dir)]
        export_global_dir: GString,
        #[export(multiline)]
        export_multiline: GString,
        # [export (range = (0.0 , 20.0))]
        export_range_float_0_20: f64,
        # [export (range = (- 10.0 , 20.0 , 0.2))]
        export_range_float_neg10_20_02: f64,
        #[export(exp_easing)]
        export_exp_easing: f64,
        #[export(color_no_alpha)]
        export_color_no_alpha: Color,
        # [export (flags = (Fire , Water , Earth , Wind))]
        export_flags_fire_water_earth_wind: i64,
        # [export (flags = (Self = 4 , Allies = 8 , Foes = 16))]
        export_flags_self_4_allies_8_foes_16: i64,
        #[export(flags_2d_physics)]
        export_flags_2d_physics: i64,
        #[export(flags_2d_render)]
        export_flags_2d_render: i64,
        #[export(flags_2d_navigation)]
        export_flags_2d_navigation: i64,
        #[export(flags_3d_physics)]
        export_flags_3d_physics: i64,
        #[export(flags_3d_render)]
        export_flags_3d_render: i64,
        #[export(flags_3d_navigation)]
        export_flags_3d_navigation: i64,
        # [export (enum = (Warrior , Magician , Thief))]
        export_enum_int_warrior_magician_thief: i64,
        # [export (enum = (Slow = 30 , Average = 60 , VeryFast = 200))]
        export_enum_int_slow_30_average_60_very_fast_200: i64,
        # [export (enum = (Rebecca , Mary , Leah))]
        export_enum_string_rebecca_mary_leah: GString,
    }
}
pub use property_tests::*;
#[derive(Clone, PartialEq, Debug)]
pub struct NewI64(i64);
impl godot::builtin::meta::GodotConvert for NewI64 {
    type Via = i64;
}
impl godot::builtin::meta::ToGodot for NewI64 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewI64 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewI32(i32);
impl godot::builtin::meta::GodotConvert for NewI32 {
    type Via = i32;
}
impl godot::builtin::meta::ToGodot for NewI32 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewI32 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewU32(u32);
impl godot::builtin::meta::GodotConvert for NewU32 {
    type Via = u32;
}
impl godot::builtin::meta::ToGodot for NewU32 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewU32 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewI16(i16);
impl godot::builtin::meta::GodotConvert for NewI16 {
    type Via = i16;
}
impl godot::builtin::meta::ToGodot for NewI16 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewI16 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewU16(u16);
impl godot::builtin::meta::GodotConvert for NewU16 {
    type Via = u16;
}
impl godot::builtin::meta::ToGodot for NewU16 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewU16 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewI8(i8);
impl godot::builtin::meta::GodotConvert for NewI8 {
    type Via = i8;
}
impl godot::builtin::meta::ToGodot for NewI8 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewI8 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewU8(u8);
impl godot::builtin::meta::GodotConvert for NewU8 {
    type Via = u8;
}
impl godot::builtin::meta::ToGodot for NewU8 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewU8 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewF32(f32);
impl godot::builtin::meta::GodotConvert for NewF32 {
    type Via = f32;
}
impl godot::builtin::meta::ToGodot for NewF32 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewF32 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewF64(f64);
impl godot::builtin::meta::GodotConvert for NewF64 {
    type Via = f64;
}
impl godot::builtin::meta::ToGodot for NewF64 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewF64 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewBool(bool);
impl godot::builtin::meta::GodotConvert for NewBool {
    type Via = bool;
}
impl godot::builtin::meta::ToGodot for NewBool {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewBool {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewColor(Color);
impl godot::builtin::meta::GodotConvert for NewColor {
    type Via = Color;
}
impl godot::builtin::meta::ToGodot for NewColor {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewColor {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewString(GString);
impl godot::builtin::meta::GodotConvert for NewString {
    type Via = GString;
}
impl godot::builtin::meta::ToGodot for NewString {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewString {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewStringName(StringName);
impl godot::builtin::meta::GodotConvert for NewStringName {
    type Via = StringName;
}
impl godot::builtin::meta::ToGodot for NewStringName {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewStringName {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewNodePath(NodePath);
impl godot::builtin::meta::GodotConvert for NewNodePath {
    type Via = NodePath;
}
impl godot::builtin::meta::ToGodot for NewNodePath {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewNodePath {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewVector2(Vector2);
impl godot::builtin::meta::GodotConvert for NewVector2 {
    type Via = Vector2;
}
impl godot::builtin::meta::ToGodot for NewVector2 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewVector2 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewVector3(Vector3);
impl godot::builtin::meta::GodotConvert for NewVector3 {
    type Via = Vector3;
}
impl godot::builtin::meta::ToGodot for NewVector3 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewVector3 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewVector4(Vector4);
impl godot::builtin::meta::GodotConvert for NewVector4 {
    type Via = Vector4;
}
impl godot::builtin::meta::ToGodot for NewVector4 {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewVector4 {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewVector2i(Vector2i);
impl godot::builtin::meta::GodotConvert for NewVector2i {
    type Via = Vector2i;
}
impl godot::builtin::meta::ToGodot for NewVector2i {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewVector2i {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewVector3i(Vector3i);
impl godot::builtin::meta::GodotConvert for NewVector3i {
    type Via = Vector3i;
}
impl godot::builtin::meta::ToGodot for NewVector3i {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewVector3i {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct NewCallable(Callable);
impl godot::builtin::meta::GodotConvert for NewCallable {
    type Via = Callable;
}
impl godot::builtin::meta::ToGodot for NewCallable {
    #[allow(clippy::clone_on_copy)]
    fn to_godot(&self) -> Self::Via {
        self.0.clone()
    }
}
impl godot::builtin::meta::FromGodot for NewCallable {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self(via))
    }
}
