fn sum_number(v:&[u32]) -> Option<u32> {
    let mut r:u32 = 0;

    for n in v {
        r = r + n;
    }
    if r > u32::MAX {
        return None;
    }
    Some(r)

}

fn main() {
    let num = &[2, 7, 9, 323, 59863900000];
    let res = sum_number(num);
    println!("答案：{:?}", res);
}