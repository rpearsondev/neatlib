use crate::common::NeatFloat;

pub fn relu(v: NeatFloat, _m: NeatFloat) -> NeatFloat {
    if v > 0.0 {
        return v;
    }
    0.0
}
pub fn leaky_relu(v: NeatFloat, _m: NeatFloat) -> NeatFloat {
    if v > 0.0 {
        return v;
    }
    v * 0.1
}
pub fn sigmoid(v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    return 1.0 / (1.0 + NeatFloat::exp(v*5.0))
}
pub fn tanh(v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    return NeatFloat::tanh(v * 5.0)
}
pub fn binary(v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    if v > 0.0 
    { 
        return 1.0; 
    }
    -1.0
}
pub fn linear_clip(v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    if v < -1.0 {
        return -1.0;
    }
    if v > 1.0 {
        return 1.0;
    }
    v
}
pub fn linear(v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    v
}
pub fn sin(v: NeatFloat, m: NeatFloat) -> NeatFloat{
    ((v * m) * 2.0).sin()
}
pub fn bipolar_sigmoid(v: NeatFloat, _m: NeatFloat) -> NeatFloat {
    (2.0 / (1.0 + NeatFloat::exp(-4.9 * v))) - 1.0
}
pub fn gaussian(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    NeatFloat::exp(-NeatFloat::powf((v * m) * 2.5, 2.0))
}
pub fn binary_sin(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    binary(sin(v, m), m)
}
pub fn binary_gaussian(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    binary(gaussian(v, m), m)
}
pub fn linear_clip_gaussian(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    linear_clip(gaussian(v, m), m)
}
pub fn invert(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    -v * m
}
pub fn band(v: NeatFloat, m: NeatFloat) -> NeatFloat {
    if v.abs() > (m.abs() * 1.25){
        return v;
    }
    return 0.0;
}

pub fn not_mapped(_v: NeatFloat, _m: NeatFloat) -> NeatFloat{
    panic!("activation function not mapped");
}

#[ignore = "dev only"]
#[test]
fn sigmoid_test() {
    let range: NeatFloat = 4.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v = ((range / steps) * i as NeatFloat) - (range / 2.0);
        let res = sigmoid(v, 1.0);
        println!("val:{:.2} sig:{:.2}",v, res);
    }
}

#[ignore = "dev only"]
#[test]
fn tanh_test() {
    let range: NeatFloat = 4.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v = ((range / steps) * i as NeatFloat) - (range / 2.0);
        let res = tanh(v, 1.0);
        println!("val:{:.2} tanh:{:.2}",v, res);
    }
}

#[ignore = "dev only"]
#[test]
fn leaky_relu_test() {
    let range: NeatFloat = 4.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v = ((range / steps) * i as NeatFloat) - (range / 2.0);
        let res = leaky_relu(v, 1.0);
        println!("val:{:.2} lu:{:.2}",v, res);
    }
}

#[ignore = "dev only"]
#[test]
fn sin_test() {
    let range: NeatFloat = 2.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v: NeatFloat = ((range / steps) * i as NeatFloat) - (range / 2.0);
        let res = sin(v, 1.0);
        println!("val:{:.2} sin:{:.2}",v, res);
    }
}

#[ignore = "dev only"]
#[test]
fn bipolar_sigmoid_test() {
    let range: NeatFloat = 4.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v = ((range / steps) * i as NeatFloat) - (range / 2.0);
        println!("val:{:.2} b_sig:{:.2}, sig:{:.2}",v, bipolar_sigmoid(v, 1.0), sigmoid(v, 1.0));
    }
}

#[ignore = "dev only"]
#[test]
fn gaussian_test() {
    let range: NeatFloat = 4.0;
    let steps: NeatFloat = 100.0;
    for i in 1..steps as i32{
        let v: NeatFloat = ((range / steps) * i as NeatFloat) - (range / 2.0);
        let res: NeatFloat = gaussian(v, 1.0);
        println!("val:{:.2} gaussian:{:.2}",v, res);
    }
}
