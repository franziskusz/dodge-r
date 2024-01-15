#[allow(non_snake_case)]
pub struct UtilityFunctionTable {
    pub sin: crate::UtilityFunctionBind, pub cos: crate::UtilityFunctionBind, pub tan: crate::UtilityFunctionBind, pub sinh: crate::UtilityFunctionBind, pub cosh: crate::UtilityFunctionBind, pub tanh: crate::UtilityFunctionBind, pub asin: crate::UtilityFunctionBind, pub acos: crate::UtilityFunctionBind, pub atan: crate::UtilityFunctionBind, pub atan2: crate::UtilityFunctionBind, pub asinh: crate::UtilityFunctionBind, pub acosh: crate::UtilityFunctionBind, pub atanh: crate::UtilityFunctionBind, pub sqrt: crate::UtilityFunctionBind, pub fmod: crate::UtilityFunctionBind, pub fposmod: crate::UtilityFunctionBind, pub posmod: crate::UtilityFunctionBind, pub floor: crate::UtilityFunctionBind, pub floorf: crate::UtilityFunctionBind, pub floori: crate::UtilityFunctionBind, pub ceil: crate::UtilityFunctionBind, pub ceilf: crate::UtilityFunctionBind, pub ceili: crate::UtilityFunctionBind, pub round: crate::UtilityFunctionBind, pub roundf: crate::UtilityFunctionBind, pub roundi: crate::UtilityFunctionBind, pub abs: crate::UtilityFunctionBind, pub absf: crate::UtilityFunctionBind, pub absi: crate::UtilityFunctionBind, pub sign: crate::UtilityFunctionBind, pub signf: crate::UtilityFunctionBind, pub signi: crate::UtilityFunctionBind, pub snapped: crate::UtilityFunctionBind, pub snappedf: crate::UtilityFunctionBind, pub snappedi: crate::UtilityFunctionBind, pub pow: crate::UtilityFunctionBind, pub log: crate::UtilityFunctionBind, pub exp: crate::UtilityFunctionBind, pub is_nan: crate::UtilityFunctionBind, pub is_inf: crate::UtilityFunctionBind, pub is_equal_approx: crate::UtilityFunctionBind, pub is_zero_approx: crate::UtilityFunctionBind, pub is_finite: crate::UtilityFunctionBind, pub ease: crate::UtilityFunctionBind, pub step_decimals: crate::UtilityFunctionBind, pub lerp: crate::UtilityFunctionBind, pub lerpf: crate::UtilityFunctionBind, pub cubic_interpolate: crate::UtilityFunctionBind, pub cubic_interpolate_angle: crate::UtilityFunctionBind, pub cubic_interpolate_in_time: crate::UtilityFunctionBind, pub cubic_interpolate_angle_in_time: crate::UtilityFunctionBind, pub bezier_interpolate: crate::UtilityFunctionBind, pub bezier_derivative: crate::UtilityFunctionBind, pub angle_difference: crate::UtilityFunctionBind, pub lerp_angle: crate::UtilityFunctionBind, pub inverse_lerp: crate::UtilityFunctionBind, pub remap: crate::UtilityFunctionBind, pub smoothstep: crate::UtilityFunctionBind, pub move_toward: crate::UtilityFunctionBind, pub rotate_toward: crate::UtilityFunctionBind, pub deg_to_rad: crate::UtilityFunctionBind, pub rad_to_deg: crate::UtilityFunctionBind, pub linear_to_db: crate::UtilityFunctionBind, pub db_to_linear: crate::UtilityFunctionBind, pub wrap: crate::UtilityFunctionBind, pub wrapi: crate::UtilityFunctionBind, pub wrapf: crate::UtilityFunctionBind, pub max: crate::UtilityFunctionBind, pub maxi: crate::UtilityFunctionBind, pub maxf: crate::UtilityFunctionBind, pub min: crate::UtilityFunctionBind, pub mini: crate::UtilityFunctionBind, pub minf: crate::UtilityFunctionBind, pub clamp: crate::UtilityFunctionBind, pub clampi: crate::UtilityFunctionBind, pub clampf: crate::UtilityFunctionBind, pub nearest_po2: crate::UtilityFunctionBind, pub pingpong: crate::UtilityFunctionBind, pub randomize: crate::UtilityFunctionBind, pub randi: crate::UtilityFunctionBind, pub randf: crate::UtilityFunctionBind, pub randi_range: crate::UtilityFunctionBind, pub randf_range: crate::UtilityFunctionBind, pub randfn: crate::UtilityFunctionBind, pub seed: crate::UtilityFunctionBind, pub rand_from_seed: crate::UtilityFunctionBind, pub weakref: crate::UtilityFunctionBind, pub typeof_: crate::UtilityFunctionBind, pub type_convert: crate::UtilityFunctionBind, pub str: crate::UtilityFunctionBind, pub error_string: crate::UtilityFunctionBind, pub type_string: crate::UtilityFunctionBind, pub print: crate::UtilityFunctionBind, pub print_rich: crate::UtilityFunctionBind, pub printerr: crate::UtilityFunctionBind, pub printt: crate::UtilityFunctionBind, pub prints: crate::UtilityFunctionBind, pub printraw: crate::UtilityFunctionBind, pub print_verbose: crate::UtilityFunctionBind, pub push_error: crate::UtilityFunctionBind, pub push_warning: crate::UtilityFunctionBind, pub var_to_str: crate::UtilityFunctionBind, pub str_to_var: crate::UtilityFunctionBind, pub var_to_bytes: crate::UtilityFunctionBind, pub bytes_to_var: crate::UtilityFunctionBind, pub var_to_bytes_with_objects: crate::UtilityFunctionBind, pub bytes_to_var_with_objects: crate::UtilityFunctionBind, pub hash: crate::UtilityFunctionBind, pub instance_from_id: crate::UtilityFunctionBind, pub is_instance_id_valid: crate::UtilityFunctionBind, pub is_instance_valid: crate::UtilityFunctionBind, pub rid_allocate_id: crate::UtilityFunctionBind, pub rid_from_int64: crate::UtilityFunctionBind, pub is_same: crate::UtilityFunctionBind,
}
impl UtilityFunctionTable {
    pub const CLASS_COUNT: usize = 0usize;
    pub const METHOD_COUNT: usize = 114usize;
    pub fn load(interface: &crate::GDExtensionInterface, string_names: &mut crate::StringCache,) -> Self {
        let get_utility_fn = interface.variant_get_ptr_utility_function.expect("variant_get_ptr_utility_function absent");
        Self {
            sin: crate::load_utility_function(get_utility_fn, string_names, "sin", 2140049587i64), cos: crate::load_utility_function(get_utility_fn, string_names, "cos", 2140049587i64), tan: crate::load_utility_function(get_utility_fn, string_names, "tan", 2140049587i64), sinh: crate::load_utility_function(get_utility_fn, string_names, "sinh", 2140049587i64), cosh: crate::load_utility_function(get_utility_fn, string_names, "cosh", 2140049587i64), tanh: crate::load_utility_function(get_utility_fn, string_names, "tanh", 2140049587i64), asin: crate::load_utility_function(get_utility_fn, string_names, "asin", 2140049587i64), acos: crate::load_utility_function(get_utility_fn, string_names, "acos", 2140049587i64), atan: crate::load_utility_function(get_utility_fn, string_names, "atan", 2140049587i64), atan2: crate::load_utility_function(get_utility_fn, string_names, "atan2", 92296394i64), asinh: crate::load_utility_function(get_utility_fn, string_names, "asinh", 2140049587i64), acosh: crate::load_utility_function(get_utility_fn, string_names, "acosh", 2140049587i64), atanh: crate::load_utility_function(get_utility_fn, string_names, "atanh", 2140049587i64), sqrt: crate::load_utility_function(get_utility_fn, string_names, "sqrt", 2140049587i64), fmod: crate::load_utility_function(get_utility_fn, string_names, "fmod", 92296394i64), fposmod: crate::load_utility_function(get_utility_fn, string_names, "fposmod", 92296394i64), posmod: crate::load_utility_function(get_utility_fn, string_names, "posmod", 3133453818i64), floor: crate::load_utility_function(get_utility_fn, string_names, "floor", 4776452i64), floorf: crate::load_utility_function(get_utility_fn, string_names, "floorf", 2140049587i64), floori: crate::load_utility_function(get_utility_fn, string_names, "floori", 2780425386i64), ceil: crate::load_utility_function(get_utility_fn, string_names, "ceil", 4776452i64), ceilf: crate::load_utility_function(get_utility_fn, string_names, "ceilf", 2140049587i64), ceili: crate::load_utility_function(get_utility_fn, string_names, "ceili", 2780425386i64), round: crate::load_utility_function(get_utility_fn, string_names, "round", 4776452i64), roundf: crate::load_utility_function(get_utility_fn, string_names, "roundf", 2140049587i64), roundi: crate::load_utility_function(get_utility_fn, string_names, "roundi", 2780425386i64), abs: crate::load_utility_function(get_utility_fn, string_names, "abs", 4776452i64), absf: crate::load_utility_function(get_utility_fn, string_names, "absf", 2140049587i64), absi: crate::load_utility_function(get_utility_fn, string_names, "absi", 2157319888i64), sign: crate::load_utility_function(get_utility_fn, string_names, "sign", 4776452i64), signf: crate::load_utility_function(get_utility_fn, string_names, "signf", 2140049587i64), signi: crate::load_utility_function(get_utility_fn, string_names, "signi", 2157319888i64), snapped: crate::load_utility_function(get_utility_fn, string_names, "snapped", 459914704i64), snappedf: crate::load_utility_function(get_utility_fn, string_names, "snappedf", 92296394i64), snappedi: crate::load_utility_function(get_utility_fn, string_names, "snappedi", 3570758393i64), pow: crate::load_utility_function(get_utility_fn, string_names, "pow", 92296394i64), log: crate::load_utility_function(get_utility_fn, string_names, "log", 2140049587i64), exp: crate::load_utility_function(get_utility_fn, string_names, "exp", 2140049587i64), is_nan: crate::load_utility_function(get_utility_fn, string_names, "is_nan", 3569215213i64), is_inf: crate::load_utility_function(get_utility_fn, string_names, "is_inf", 3569215213i64), is_equal_approx: crate::load_utility_function(get_utility_fn, string_names, "is_equal_approx", 1400789633i64), is_zero_approx: crate::load_utility_function(get_utility_fn, string_names, "is_zero_approx", 3569215213i64), is_finite: crate::load_utility_function(get_utility_fn, string_names, "is_finite", 3569215213i64), ease: crate::load_utility_function(get_utility_fn, string_names, "ease", 92296394i64), step_decimals: crate::load_utility_function(get_utility_fn, string_names, "step_decimals", 2780425386i64), lerp: crate::load_utility_function(get_utility_fn, string_names, "lerp", 3389874542i64), lerpf: crate::load_utility_function(get_utility_fn, string_names, "lerpf", 998901048i64), cubic_interpolate: crate::load_utility_function(get_utility_fn, string_names, "cubic_interpolate", 1090965791i64), cubic_interpolate_angle: crate::load_utility_function(get_utility_fn, string_names, "cubic_interpolate_angle", 1090965791i64), cubic_interpolate_in_time: crate::load_utility_function(get_utility_fn, string_names, "cubic_interpolate_in_time", 388121036i64), cubic_interpolate_angle_in_time: crate::load_utility_function(get_utility_fn, string_names, "cubic_interpolate_angle_in_time", 388121036i64), bezier_interpolate: crate::load_utility_function(get_utility_fn, string_names, "bezier_interpolate", 1090965791i64), bezier_derivative: crate::load_utility_function(get_utility_fn, string_names, "bezier_derivative", 1090965791i64), angle_difference: crate::load_utility_function(get_utility_fn, string_names, "angle_difference", 92296394i64), lerp_angle: crate::load_utility_function(get_utility_fn, string_names, "lerp_angle", 998901048i64), inverse_lerp: crate::load_utility_function(get_utility_fn, string_names, "inverse_lerp", 998901048i64), remap: crate::load_utility_function(get_utility_fn, string_names, "remap", 1090965791i64), smoothstep: crate::load_utility_function(get_utility_fn, string_names, "smoothstep", 998901048i64), move_toward: crate::load_utility_function(get_utility_fn, string_names, "move_toward", 998901048i64), rotate_toward: crate::load_utility_function(get_utility_fn, string_names, "rotate_toward", 998901048i64), deg_to_rad: crate::load_utility_function(get_utility_fn, string_names, "deg_to_rad", 2140049587i64), rad_to_deg: crate::load_utility_function(get_utility_fn, string_names, "rad_to_deg", 2140049587i64), linear_to_db: crate::load_utility_function(get_utility_fn, string_names, "linear_to_db", 2140049587i64), db_to_linear: crate::load_utility_function(get_utility_fn, string_names, "db_to_linear", 2140049587i64), wrap: crate::load_utility_function(get_utility_fn, string_names, "wrap", 3389874542i64), wrapi: crate::load_utility_function(get_utility_fn, string_names, "wrapi", 650295447i64), wrapf: crate::load_utility_function(get_utility_fn, string_names, "wrapf", 998901048i64), max: crate::load_utility_function(get_utility_fn, string_names, "max", 3896050336i64), maxi: crate::load_utility_function(get_utility_fn, string_names, "maxi", 3133453818i64), maxf: crate::load_utility_function(get_utility_fn, string_names, "maxf", 92296394i64), min: crate::load_utility_function(get_utility_fn, string_names, "min", 3896050336i64), mini: crate::load_utility_function(get_utility_fn, string_names, "mini", 3133453818i64), minf: crate::load_utility_function(get_utility_fn, string_names, "minf", 92296394i64), clamp: crate::load_utility_function(get_utility_fn, string_names, "clamp", 3389874542i64), clampi: crate::load_utility_function(get_utility_fn, string_names, "clampi", 650295447i64), clampf: crate::load_utility_function(get_utility_fn, string_names, "clampf", 998901048i64), nearest_po2: crate::load_utility_function(get_utility_fn, string_names, "nearest_po2", 2157319888i64), pingpong: crate::load_utility_function(get_utility_fn, string_names, "pingpong", 92296394i64), randomize: crate::load_utility_function(get_utility_fn, string_names, "randomize", 1691721052i64), randi: crate::load_utility_function(get_utility_fn, string_names, "randi", 701202648i64), randf: crate::load_utility_function(get_utility_fn, string_names, "randf", 2086227845i64), randi_range: crate::load_utility_function(get_utility_fn, string_names, "randi_range", 3133453818i64), randf_range: crate::load_utility_function(get_utility_fn, string_names, "randf_range", 92296394i64), randfn: crate::load_utility_function(get_utility_fn, string_names, "randfn", 92296394i64), seed: crate::load_utility_function(get_utility_fn, string_names, "seed", 382931173i64), rand_from_seed: crate::load_utility_function(get_utility_fn, string_names, "rand_from_seed", 1391063685i64), weakref: crate::load_utility_function(get_utility_fn, string_names, "weakref", 4776452i64), typeof_: crate::load_utility_function(get_utility_fn, string_names, "typeof", 326422594i64), type_convert: crate::load_utility_function(get_utility_fn, string_names, "type_convert", 2453062746i64), str: crate::load_utility_function(get_utility_fn, string_names, "str", 32569176i64), error_string: crate::load_utility_function(get_utility_fn, string_names, "error_string", 942708242i64), type_string: crate::load_utility_function(get_utility_fn, string_names, "type_string", 942708242i64), print: crate::load_utility_function(get_utility_fn, string_names, "print", 2648703342i64), print_rich: crate::load_utility_function(get_utility_fn, string_names, "print_rich", 2648703342i64), printerr: crate::load_utility_function(get_utility_fn, string_names, "printerr", 2648703342i64), printt: crate::load_utility_function(get_utility_fn, string_names, "printt", 2648703342i64), prints: crate::load_utility_function(get_utility_fn, string_names, "prints", 2648703342i64), printraw: crate::load_utility_function(get_utility_fn, string_names, "printraw", 2648703342i64), print_verbose: crate::load_utility_function(get_utility_fn, string_names, "print_verbose", 2648703342i64), push_error: crate::load_utility_function(get_utility_fn, string_names, "push_error", 2648703342i64), push_warning: crate::load_utility_function(get_utility_fn, string_names, "push_warning", 2648703342i64), var_to_str: crate::load_utility_function(get_utility_fn, string_names, "var_to_str", 866625479i64), str_to_var: crate::load_utility_function(get_utility_fn, string_names, "str_to_var", 1891498491i64), var_to_bytes: crate::load_utility_function(get_utility_fn, string_names, "var_to_bytes", 2947269930i64), bytes_to_var: crate::load_utility_function(get_utility_fn, string_names, "bytes_to_var", 4249819452i64), var_to_bytes_with_objects: crate::load_utility_function(get_utility_fn, string_names, "var_to_bytes_with_objects", 2947269930i64), bytes_to_var_with_objects: crate::load_utility_function(get_utility_fn, string_names, "bytes_to_var_with_objects", 4249819452i64), hash: crate::load_utility_function(get_utility_fn, string_names, "hash", 326422594i64), instance_from_id: crate::load_utility_function(get_utility_fn, string_names, "instance_from_id", 1156694636i64), is_instance_id_valid: crate::load_utility_function(get_utility_fn, string_names, "is_instance_id_valid", 2232439758i64), is_instance_valid: crate::load_utility_function(get_utility_fn, string_names, "is_instance_valid", 996128841i64), rid_allocate_id: crate::load_utility_function(get_utility_fn, string_names, "rid_allocate_id", 701202648i64), rid_from_int64: crate::load_utility_function(get_utility_fn, string_names, "rid_from_int64", 3426892196i64), is_same: crate::load_utility_function(get_utility_fn, string_names, "is_same", 1409423524i64),
        }
    }
}