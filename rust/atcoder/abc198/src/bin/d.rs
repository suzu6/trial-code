use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    proconio::input! {
        s1: String,
        s2: String,
        s3: String
    }

    let cs1_org: Vec<char> = s1.chars().rev().collect();
    let cs2_org: Vec<char> = s2.chars().rev().collect();
    let cs3_org: Vec<char> = s3.chars().rev().collect();
    let cs1: Vec<char> = s1.chars().rev().collect();
    let cs2: Vec<char> = s2.chars().rev().collect();
    let cs3: Vec<char> = s3.chars().rev().collect();

    let mut array: Vec<char> = cs1.clone();
    array.extend(cs2.clone()); 
    array.extend(cs3.clone()); 
    
    let unique: HashSet<char> = array.clone().into_iter().collect();
    let mut map = HashMap::new();
    for x in unique {
        map.entry(x).or_insert(-1 as i32);
    }

    let keta:usize = 0;

    for i in 0..9 {
        let Some(c1) = map.get_mut(&cs1[keta]);
        if *c1 == -1 {
            *c1 = i as i32;
        }

        for j in 0..9 {
            let Some(c2) = map.get_mut(&cs2[keta]);
            if *c2 == -1 {
                *c2 = j as i32;
            }
        }
    }

}