#[allow(dead_code)]
pub fn display<T: ToString>(t: &Vec<Vec<T>>, offset: usize) {
    println!("");
    println!("");
    print!(
        "{}",
        t.iter()
            .map(|l| {
                l.iter()
                    .skip(offset)
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("");
}
