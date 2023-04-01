/// [TODO:description]
///
/// * `nums1`: [TODO:parameter]
/// * `nums2`: [TODO:parameter]
#[allow(dead_code)]
fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut r: i32 = -1;

    nums1.iter().for_each(|&v| {
        let o = nums2.iter().find(|&&p| p == v);
        if o.is_some() {
            r = v;
        }
    });
    r
}
