mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table() {}
    }

    mod serving {
        // fn take_order() {}

        // fn serve_order() {}

        // fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house::hosting;

pub fn take_at_restaurant() {
    // 関数は親のパスまで、関数がローカルで定義されていないことを明らかにできます
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 構造体やenumその他の要素をuseで持ち込むときは、フルパスを書くのが慣例的
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }


// use std::cmp::Ordering;
// use std::io;


// use std::{cmp::Ordering, io};