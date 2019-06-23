use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
//    let mut new_map: BTreeMap<char, i32> = BTreeMap::new();
//    for key in h.keys(){
//        for values in h.get(key).unwrap() {
//            new_map.insert(values.to_ascii_lowercase(), key.clone());
//        }
//    }
//    new_map

    h
        .keys()
        .fold(BTreeMap::new(), |a, i| {
            h
                .get(i)
                .unwrap()
                .iter()
                .fold(a, |mut a, c| {
                    a.insert(c.to_ascii_lowercase(), i.clone());
                    a
                })
        })
}
