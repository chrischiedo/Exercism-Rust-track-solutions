#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len1 = first_list.len();
    let len2 = second_list.len();

    if len1 < len2 {
        match len1 {
            0 => Comparison::Sublist,
            _ => {
                for i in 0 ..=(len2 - len1) {
                    if first_list[..] == second_list[i..i + len1] {
                        return Comparison::Sublist;
                    }
                }
                Comparison::Unequal
            },
        }
    } else if len2 < len1 {
        for i in 0..=(len1 - len2) {
            if second_list[..] == first_list[i..i + len2] {
                return Comparison::Superlist;
            }
        }
        Comparison::Unequal
    } else {
        if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    }
}
