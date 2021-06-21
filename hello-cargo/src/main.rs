fn main() {
    let did = 9;
    let di = -3;
    let ret = is_divided(did, di);
    println!("did:{}, di:{}, ret:{}", did, di, ret)
}

fn is_divided(did: i64, di: i64) -> bool {
    if di == 0 {
        return false;
    }
    return did % di == 0;
}
