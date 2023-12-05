// #[allow(dead_code)]
// pub fn display<T, F>(t: &Vec<Vec<T>>, map: F)
// where
//     F: Fn(&T) -> String,
// {
//     println!("");
//     println!("");
//     print!(
//         "{}",
//         t.iter()
//             .map(|l| { l.iter().map(|a| map(a)).collect::<Vec<String>>().join("") })
//             .collect::<Vec<String>>()
//             .join("\n")
//     );
//     println!("");
// }

// pub fn display_bool(t: &Vec<Vec<bool>>, offset: usize) {
//     println!("");
//     println!("");
//     print!(
//         "{}",
//         t.iter()
//             .map(|l| {
//                 l.iter()
//                     .skip(offset)
//                     .map(|a| if *a { "#" } else { "." })
//                     .collect::<Vec<&str>>()
//                     .join("")
//             })
//             .collect::<Vec<String>>()
//             .join("\n")
//     );
//     println!("");
// }
