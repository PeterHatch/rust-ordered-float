extern crate ordered_float;
extern crate num;

use ordered_float::*;
use num::Float;
use std::cmp::Ordering::*;

#[test]
fn compare_regular_ordered_float32() {
    assert_eq!(OrderedFloat(7.0f32).cmp(&OrderedFloat(7.0)), Equal);
    assert_eq!(OrderedFloat(8.0f32).cmp(&OrderedFloat(7.0)), Greater);
    assert_eq!(OrderedFloat(4.0f32).cmp(&OrderedFloat(7.0)), Less);
}

#[test]
fn compare_nan_ordered_float32() {
    let f32_nan: f32 = Float::nan();
    assert_eq!(OrderedFloat(f32_nan).cmp(&OrderedFloat(Float::nan())), Equal);
    assert_eq!(OrderedFloat(f32_nan).cmp(&OrderedFloat(-100000.0f32)), Greater);
    assert_eq!(OrderedFloat(-100.0f32).cmp(&OrderedFloat(Float::nan())), Less);
}

#[test]
fn compare_regular_ordered_float64() {
    assert_eq!(OrderedFloat(7.0f64).cmp(&OrderedFloat(7.0)), Equal);
    assert_eq!(OrderedFloat(8.0f64).cmp(&OrderedFloat(7.0)), Greater);
    assert_eq!(OrderedFloat(4.0f64).cmp(&OrderedFloat(7.0)), Less);
}

#[test]
fn compare_nan_ordered_float64() {
    let f64_nan: f64 = Float::nan();
    assert_eq!(OrderedFloat(f64_nan).cmp(&OrderedFloat(Float::nan())), Equal);
    assert_eq!(OrderedFloat(f64_nan).cmp(&OrderedFloat(-100000.0f64)), Greater);
    assert_eq!(OrderedFloat(-100.0f64).cmp(&OrderedFloat(Float::nan())), Less);
}

#[test]
fn compare_regular_notnan32() {
    assert_eq!(NotNaN::from(7.0f32).cmp(&NotNaN::from(7.0)), Equal);
    assert_eq!(NotNaN::from(8.0f32).cmp(&NotNaN::from(7.0)), Greater);
    assert_eq!(NotNaN::from(4.0f32).cmp(&NotNaN::from(7.0)), Less);
}

#[test]
fn error_when_creating_notnan32_from_nan() {
    let f32_nan: f32 = Float::nan();
    assert!(NotNaN::new(f32_nan).is_err());
}

#[test]
fn compare_regular_notnan64() {
    assert_eq!(NotNaN::from(7.0f64).cmp(&NotNaN::from(7.0)), Equal);
    assert_eq!(NotNaN::from(8.0f64).cmp(&NotNaN::from(7.0)), Greater);
    assert_eq!(NotNaN::from(4.0f64).cmp(&NotNaN::from(7.0)), Less);
}

#[test]
fn error_when_creating_notnan64_from_nan() {
    let f64_nan: f64 = Float::nan();
    assert!(NotNaN::new(f64_nan).is_err());
}
