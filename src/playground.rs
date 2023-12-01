struct ByCol<'a, T> {
    rows: Vec<std::slice::Iter<'a, T>>,
}
trait Columnizable<T> {
    fn by_col(&self) -> ByCol<T>;
}

impl<'a, T> Iterator for ByCol<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.iter_mut().map(|iter| iter.next()).collect()
    }
}

impl<T> Columnizable<T> for Vec<Vec<T>> {
    fn by_col(&self) -> ByCol<T> {
        let rows: Vec<_> = self.into_iter().map(|v| v.into_iter()).collect();
        return ByCol { rows };
    }
}

pub fn main() {
    dbg!((-1 as i32).rem_euclid(10));
}

#[allow(dead_code)]
const INPUT: &str = "";
