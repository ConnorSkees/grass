#![cfg(test)]

#[macro_use]
mod macros;

test!(
    clamp_in_the_middle,
    "@use 'sass:math';\na {\n  color: math.clamp(0, 1, 2);\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    clamp_first_is_bigger,
    "@use 'sass:math';\na {\n  color: math.clamp(2, 1, 0);\n}\n",
    "a {\n  color: 2;\n}\n"
);
test!(
    clamp_all_same_unit,
    "@use 'sass:math';\na {\n  color: math.clamp(0px, 1px, 2px);\n}\n",
    "a {\n  color: 1px;\n}\n"
);
test!(
    clamp_all_different_but_compatible_unit,
    "@use 'sass:math';\na {\n  color: math.clamp(0mm, 1cm, 2in);\n}\n",
    "a {\n  color: 1cm;\n}\n"
);
error!(
    clamp_only_min_has_no_unit,
    "@use 'sass:math';\na {\n  color: math.clamp(0, 1cm, 2in);\n}\n",
    "Error: $min is unitless but $number has unit cm. Arguments must all have units or all be unitless."
);
error!(
    clamp_only_number_has_no_unit,
    "@use 'sass:math';\na {\n  color: math.clamp(0mm, 1, 2in);\n}\n",
    "Error: $min has unit mm but $number is unitless. Arguments must all have units or all be unitless."
);
error!(
    clamp_only_max_has_no_unit,
    "@use 'sass:math';\na {\n  color: math.clamp(0mm, 1cm, 2);\n}\n",
    "Error: $min has unit mm but $max is unitless. Arguments must all have units or all be unitless."
);
test!(
    sqrt_zero,
    "@use 'sass:math';\na {\n  color: math.sqrt(0);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    sqrt_small_positive,
    "@use 'sass:math';\na {\n  color: math.sqrt(99);\n}\n",
    "a {\n  color: 9.9498743711;\n}\n"
);
test!(
    sqrt_small_negative,
    "@use 'sass:math';\na {\n  color: math.sqrt(-99);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    sqrt_big_positive,
    "@use 'sass:math';\na {\n  color: math.sqrt(9999999999999999999999999999999999999999999999999);\n}\n",
    "a {\n  color: 3162277660168379038695424;\n}\n"
);
test!(
    sqrt_big_negative,
    "@use 'sass:math';\na {\n  color: math.sqrt(-9999999999999999999999999999999999999999999999999);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    sqrt_irrational,
    "@use 'sass:math';\na {\n  color: math.sqrt(2);\n}\n",
    "a {\n  color: 1.4142135624;\n}\n"
);
test!(
    sqrt_of_nan,
    "@use 'sass:math';\na {\n  color: math.sqrt((0 / 0));\n}\n",
    "a {\n  color: NaN;\n}\n"
);
error!(
    sqrt_with_units,
    "@use 'sass:math';\na {\n  color: math.sqrt(1px);\n}\n",
    "Error: $number: Expected 1px to have no units."
);
error!(
    cos_non_angle,
    "@use 'sass:math';\na {\n  color: math.cos(1px);\n}\n",
    "Error: $number: Expected 1px to be an angle."
);
test!(
    cos_small_degree,
    "@use 'sass:math';\na {\n  color: math.cos(1deg);\n}\n",
    "a {\n  color: 0.9998476952;\n}\n"
);
test!(
    cos_small_radian,
    "@use 'sass:math';\na {\n  color: math.cos(1rad);\n}\n",
    "a {\n  color: 0.5403023059;\n}\n"
);
test!(
    cos_small_no_unit,
    "@use 'sass:math';\na {\n  color: math.cos(1);\n}\n",
    "a {\n  color: 0.5403023059;\n}\n"
);
test!(
    cos_small_negative_degree,
    "@use 'sass:math';\na {\n  color: math.cos(-1deg);\n}\n",
    "a {\n  color: 0.9998476952;\n}\n"
);
test!(
    cos_small_negative_radian,
    "@use 'sass:math';\na {\n  color: math.cos(-1rad);\n}\n",
    "a {\n  color: 0.5403023059;\n}\n"
);
test!(
    cos_small_negative_no_unit,
    "@use 'sass:math';\na {\n  color: math.cos(-1);\n}\n",
    "a {\n  color: 0.5403023059;\n}\n"
);
test!(
    cos_pi,
    "@use 'sass:math';\na {\n  color: math.cos(math.$pi);\n}\n",
    "a {\n  color: -1;\n}\n"
);
test!(
    cos_two_pi,
    "@use 'sass:math';\na {\n  color: math.cos(2 * math.$pi);\n}\n",
    "a {\n  color: 1;\n}\n"
);
error!(
    sin_non_angle,
    "@use 'sass:math';\na {\n  color: math.sin(1px);\n}\n",
    "Error: $number: Expected 1px to be an angle."
);
test!(
    sin_small_degree,
    "@use 'sass:math';\na {\n  color: math.sin(1deg);\n}\n",
    "a {\n  color: 0.0174524064;\n}\n"
);
test!(
    sin_small_radian,
    "@use 'sass:math';\na {\n  color: math.sin(1rad);\n}\n",
    "a {\n  color: 0.8414709848;\n}\n"
);
test!(
    sin_small_no_unit,
    "@use 'sass:math';\na {\n  color: math.sin(1);\n}\n",
    "a {\n  color: 0.8414709848;\n}\n"
);
test!(
    sin_small_negative_degree,
    "@use 'sass:math';\na {\n  color: math.sin(-1deg);\n}\n",
    "a {\n  color: -0.0174524064;\n}\n"
);
test!(
    sin_small_negative_radian,
    "@use 'sass:math';\na {\n  color: math.sin(-1rad);\n}\n",
    "a {\n  color: -0.8414709848;\n}\n"
);
test!(
    sin_small_negative_no_unit,
    "@use 'sass:math';\na {\n  color: math.sin(-1);\n}\n",
    "a {\n  color: -0.8414709848;\n}\n"
);
test!(
    sin_pi,
    "@use 'sass:math';\na {\n  color: math.sin(math.$pi);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    sin_two_pi,
    "@use 'sass:math';\na {\n  color: math.sin(2 * math.$pi);\n}\n",
    "a {\n  color: 0;\n}\n"
);
error!(
    tan_non_angle,
    "@use 'sass:math';\na {\n  color: math.tan(1px);\n}\n",
    "Error: $number: Expected 1px to be an angle."
);
test!(
    tan_small_degree,
    "@use 'sass:math';\na {\n  color: math.tan(1deg);\n}\n",
    "a {\n  color: 0.0174550649;\n}\n"
);
test!(
    tan_small_radian,
    "@use 'sass:math';\na {\n  color: math.tan(1rad);\n}\n",
    "a {\n  color: 1.5574077247;\n}\n"
);
test!(
    tan_small_no_unit,
    "@use 'sass:math';\na {\n  color: math.tan(1);\n}\n",
    "a {\n  color: 1.5574077247;\n}\n"
);
test!(
    tan_small_negative_degree,
    "@use 'sass:math';\na {\n  color: math.tan(-1deg);\n}\n",
    "a {\n  color: -0.0174550649;\n}\n"
);
test!(
    tan_small_negative_radian,
    "@use 'sass:math';\na {\n  color: math.tan(-1rad);\n}\n",
    "a {\n  color: -1.5574077247;\n}\n"
);
test!(
    tan_small_negative_no_unit,
    "@use 'sass:math';\na {\n  color: math.tan(-1);\n}\n",
    "a {\n  color: -1.5574077247;\n}\n"
);
test!(
    tan_pi,
    "@use 'sass:math';\na {\n  color: math.tan(math.$pi);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    tan_two_pi,
    "@use 'sass:math';\na {\n  color: math.tan(2 * math.$pi);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    acos_above_one,
    "@use 'sass:math';\na {\n  color: math.acos(2);\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    acos_below_negative_one,
    "@use 'sass:math';\na {\n  color: math.acos(-2);\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    acos_one,
    "@use 'sass:math';\na {\n  color: math.acos(1);\n}\n",
    "a {\n  color: 0deg;\n}\n"
);
test!(
    acos_negative_one,
    "@use 'sass:math';\na {\n  color: math.acos(-1);\n}\n",
    "a {\n  color: 180deg;\n}\n"
);
test!(
    acos_zero,
    "@use 'sass:math';\na {\n  color: math.acos(0);\n}\n",
    "a {\n  color: 90deg;\n}\n"
);
test!(
    acos_point_five,
    "@use 'sass:math';\na {\n  color: math.acos(.5);\n}\n",
    "a {\n  color: 60deg;\n}\n"
);
test!(
    acos_nan,
    "@use 'sass:math';\na {\n  color: math.acos((0 / 0));\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    asin_above_one,
    "@use 'sass:math';\na {\n  color: math.asin(2);\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    asin_below_negative_one,
    "@use 'sass:math';\na {\n  color: math.asin(-2);\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    asin_one,
    "@use 'sass:math';\na {\n  color: math.asin(1);\n}\n",
    "a {\n  color: 90deg;\n}\n"
);
test!(
    asin_negative_one,
    "@use 'sass:math';\na {\n  color: math.asin(-1);\n}\n",
    "a {\n  color: -90deg;\n}\n"
);
test!(
    asin_zero,
    "@use 'sass:math';\na {\n  color: math.asin(0);\n}\n",
    "a {\n  color: 0deg;\n}\n"
);
test!(
    asin_point_five,
    "@use 'sass:math';\na {\n  color: math.asin(.5);\n}\n",
    "a {\n  color: 30deg;\n}\n"
);
test!(
    asin_nan,
    "@use 'sass:math';\na {\n  color: math.asin((0 / 0));\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    atan_above_one,
    "@use 'sass:math';\na {\n  color: math.atan(2);\n}\n",
    "a {\n  color: 63.4349488229deg;\n}\n"
);
test!(
    atan_below_negative_one,
    "@use 'sass:math';\na {\n  color: math.atan(-2);\n}\n",
    "a {\n  color: -63.4349488229deg;\n}\n"
);
test!(
    atan_one,
    "@use 'sass:math';\na {\n  color: math.atan(1);\n}\n",
    "a {\n  color: 45deg;\n}\n"
);
test!(
    atan_negative_one,
    "@use 'sass:math';\na {\n  color: math.atan(-1);\n}\n",
    "a {\n  color: -45deg;\n}\n"
);
test!(
    atan_zero,
    "@use 'sass:math';\na {\n  color: math.atan(0);\n}\n",
    "a {\n  color: 0deg;\n}\n"
);
test!(
    atan_point_five,
    "@use 'sass:math';\na {\n  color: math.atan(.5);\n}\n",
    "a {\n  color: 26.5650511771deg;\n}\n"
);
test!(
    atan_nan,
    "@use 'sass:math';\na {\n  color: math.atan((0 / 0));\n}\n",
    "a {\n  color: NaNdeg;\n}\n"
);
test!(
    log_above_one,
    "@use 'sass:math';\na {\n  color: math.log(2);\n}\n",
    "a {\n  color: 0.6931471806;\n}\n"
);
test!(
    log_below_negative_one,
    "@use 'sass:math';\na {\n  color: math.log(-2);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    log_one,
    "@use 'sass:math';\na {\n  color: math.log(1);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    log_negative_one,
    "@use 'sass:math';\na {\n  color: math.log(-1);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    #[ignore = "we do not support Infinity"]
    log_zero,
    "@use 'sass:math';\na {\n  color: math.log(0);\n}\n",
    "a {\n  color: -Infinity;\n}\n"
);
test!(
    log_point_five,
    "@use 'sass:math';\na {\n  color: math.log(.5);\n}\n",
    "a {\n  color: -0.6931471806;\n}\n"
);
test!(
    log_nan,
    "@use 'sass:math';\na {\n  color: math.log((0 / 0));\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    log_base_nan,
    "@use 'sass:math';\na {\n  color: math.log(1, (0 / 0));\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    log_base_above_one,
    "@use 'sass:math';\na {\n  color: math.log(2, 2);\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    log_base_below_negative_one,
    "@use 'sass:math';\na {\n  color: math.log(2, -2);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    #[ignore = "we do not support Infinity"]
    log_base_one,
    "@use 'sass:math';\na {\n  color: math.log(2, 1);\n}\n",
    "a {\n  color: Infinity;\n}\n"
);
test!(
    log_base_negative_one,
    "@use 'sass:math';\na {\n  color: math.log(2, -1);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    log_base_zero,
    "@use 'sass:math';\na {\n  color: math.log(2, 0);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    log_base_point_five,
    "@use 'sass:math';\na {\n  color: math.log(2, .5);\n}\n",
    "a {\n  color: -1;\n}\n"
);

test!(
    pow_exponent_and_base_one,
    "@use 'sass:math';\na {\n  color: math.pow(1, 1);\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    pow_exponent_and_base_ten,
    "@use 'sass:math';\na {\n  color: math.pow(10, 10);\n}\n",
    "a {\n  color: 10000000000;\n}\n"
);
test!(
    pow_base_negative_exponent_positive,
    "@use 'sass:math';\na {\n  color: math.pow(-2, 3);\n}\n",
    "a {\n  color: -8;\n}\n"
);
test!(
    pow_base_positive_exponent_negative,
    "@use 'sass:math';\na {\n  color: math.pow(2, -3);\n}\n",
    "a {\n  color: 0.125;\n}\n"
);
test!(
    pow_base_negative_exponent_negative,
    "@use 'sass:math';\na {\n  color: math.pow(-2, -3);\n}\n",
    "a {\n  color: -0.125;\n}\n"
);
test!(
    pow_base_decimal,
    "@use 'sass:math';\na {\n  color: math.pow(2.4, 3);\n}\n",
    "a {\n  color: 13.824;\n}\n"
);
test!(
    pow_exponent_decimal,
    "@use 'sass:math';\na {\n  color: math.pow(2, 3.5);\n}\n",
    "a {\n  color: 11.313708499;\n}\n"
);
test!(
    pow_base_nan,
    "@use 'sass:math';\na {\n  color: math.pow((0 / 0), 3);\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    pow_exponent_nan,
    "@use 'sass:math';\na {\n  color: math.pow(2, (0 / 0));\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    pow_base_and_exponent_nan,
    "@use 'sass:math';\na {\n  color: math.pow((0 / 0), (0 / 0));\n}\n",
    "a {\n  color: NaN;\n}\n"
);
test!(
    pow_exponent_zero,
    "@use 'sass:math';\na {\n  color: math.pow(2, 0);\n}\n",
    "a {\n  color: 1;\n}\n"
);