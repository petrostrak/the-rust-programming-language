// Implement IntoIterator on a struct with an iterable collection

struct Friends {
    names: Vec<String>,
}
// Mutable iteretor that borrows
impl<'a> IntoIterator for &'a mut Friends {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

// Iteretor that borrows
// impl<'a> IntoIterator for &'a Friends {
//     type Item = &'a String;
//     type IntoIter = std::slice::Iter<'a, String>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.names.iter()
//     }
// }

// impl IntoIterator for Friends {
//     type Item = String;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.names.into_iter()
//     }
// }

fn main() {
    let mut friends = Friends {
        names: vec![
            "pit".to_owned(),
            "maggie".to_owned(),
            "john".to_owned(),
            "greg".to_owned(),
        ],
    };
    for f in &mut friends {
        *f = f.to_uppercase();
        println!("{}", f);
    }
}
