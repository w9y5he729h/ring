use crate::{
    aead::{chacha::Key, iv::Iv},
    arithmetic::bigint::N0,
    c,
    limb::Limb,
};

#[no_mangle]
pub extern "C" fn GFp_bn_mul_mont(
    r: *mut Limb,
    a: *const Limb,
    b: *const Limb,
    n: *const Limb,
    n0: &N0,
    num_limbs: c::size_t,
) {
    todo!();
}

#[no_mangle]
pub extern "C" fn GFp_nistz256_add(
    r: *mut Limb,   // [COMMON_OPS.num_limbs]
    a: *const Limb, // [COMMON_OPS.num_limbs]
    b: *const Limb, // [COMMON_OPS.num_limbs]
) {
    todo!();
}

#[no_mangle]
pub extern "C" fn GFp_nistz256_mul_mont(
    r: *mut Limb,   // [COMMON_OPS.num_limbs]
    a: *const Limb, // [COMMON_OPS.num_limbs]
    b: *const Limb, // [COMMON_OPS.num_limbs]
) {
    todo!();
}

#[no_mangle]
pub extern "C" fn GFp_nistz256_neg(r: *mut Limb, a: *const Limb) {
    todo!();
}

#[no_mangle]
pub extern "C" fn GFp_nistz256_point_double(
    r: *mut Limb,   // [p256::COMMON_OPS.num_limbs*3]
    a: *const Limb, // [p256::COMMON_OPS.num_limbs*3]
) {
    todo!();
}

#[no_mangle]
pub extern "C" fn GFp_nistz256_sqr_mont(
    r: *mut Limb,   // [COMMON_OPS.num_limbs]
    a: *const Limb, // [COMMON_OPS.num_limbs]
) {
    todo!();
}
