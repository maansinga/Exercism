pub fn is_armstrong_number(num: u32) -> bool {
    let mut mnum = num;
    let mut digits: Vec<u32> = Vec::new();
    while mnum > 0{
        digits.push(mnum % 10);
        mnum = mnum / 10;
    }
    let len = digits.len();
    num == digits.iter().map(|x| (*x).pow(len as u32)).sum()

}
