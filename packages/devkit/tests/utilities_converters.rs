//! XLM 转换工具测试：往返精度、负数、边界极值
use devkit::utilities::converters::{
    xlm_to_stroops, stroops_to_xlm, ConvertError, MAX_STROOPS, MIN_STROOPS, STROOPS_PER_XLM,
};
use std::f64::consts::EPSILON;

#[test]
fn test_convert_round_trip_precision() {
    let test_cases = [0.0, 1.0, 123.456789, 999999.999999, 0.000001];
    for xlm in test_cases {
        let stroops = xlm_to_stroops(xlm).unwrap();
        let restored_xlm = stroops_to_xlm(stroops);
        assert!(
            (xlm - restored_xlm).abs() < EPSILON,
            "往返精度丢失：原值={}, 还原={}",
            xlm,
            restored_xlm
        );
    }
}

#[test]
fn test_negative_xlm_input() {
    let negative_cases = [-0.000001, -1.0, -123.456789, -999999.999999];
    for xlm in negative_cases {
        let stroops = xlm_to_stroops(xlm).unwrap();
        let restored = stroops_to_xlm(stroops);
        assert!(
            (xlm - restored).abs() < EPSILON,
            "负数转换误差：输入={}, 转回={}",
            xlm,
            restored
        );
        assert_eq!(stroops.is_negative(), xlm.is_sign_negative());
    }
}

#[test]
fn test_boundary_edge_values() {
    // 最小单位 0.000001 XLM = 1 stroop
    let min_unit_xlm = 0.000001;
    let min_stroop = xlm_to_stroops(min_unit_xlm).unwrap();
    assert_eq!(min_stroop, 1i64);
    assert!((stroops_to_xlm(1) - min_unit_xlm).abs() < EPSILON);

    // 负最小单位
    let neg_min_unit = -0.000001;
    let neg_min_stroop = xlm_to_stroops(neg_min_unit).unwrap();
    assert_eq!(neg_min_stroop, -1i64);

    // 最大合法stroops
    let max_xlm = MAX_STROOPS as f64 / STROOPS_PER_XLM as f64;
    let max_stroops = xlm_to_stroops(max_xlm).unwrap();
    assert_eq!(max_stroops, MAX_STROOPS);
    assert!((stroops_to_xlm(max_stroops) - max_xlm).abs() < EPSILON);

    // 最小合法stroops
    let min_xlm = MIN_STROOPS as f64 / STROOPS_PER_XLM as f64;
    let min_stroops = xlm_to_stroops(min_xlm).unwrap();
    assert_eq!(min_stroops, MIN_STROOPS);
    assert!((stroops_to_xlm(min_stroops) - min_xlm).abs() < EPSILON);

    // 零值边界
    let zero_stroops = xlm_to_stroops(0.0).unwrap();
    assert_eq!(zero_stroops, 0i64);
    assert_eq!(stroops_to_xlm(0), 0.0);
}

#[test]
fn test_out_of_bound_values_error() {
    // 超过最大stroops
    let over_max_xlm = (MAX_STROOPS + 1) as f64 / STROOPS_PER_XLM as f64;
    assert_eq!(xlm_to_stroops(over_max_xlm), Err(ConvertError::OverflowMaxStroops));

    // 低于最小stroops
    let under_min_xlm = (MIN_STROOPS - 1) as f64 / STROOPS_PER_XLM as f64;
    assert_eq!(xlm_to_stroops(under_min_xlm), Err(ConvertError::UnderflowMinStroops));

    // 超过6位小数，非法精度
    let extra_decimal = 1.1234567;
    assert_eq!(xlm_to_stroops(extra_decimal), Err(ConvertError::ExceedSixDecimalPlaces));
}
