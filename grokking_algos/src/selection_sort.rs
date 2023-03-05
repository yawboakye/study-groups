use std::cmp::Ordering;

pub enum Order {
    Desc,
    Asc,
}

pub fn sort<T: Ord + Copy>(vector: &mut Vec<T>, order: Order) {
    let v_len = vector.len();
    for index_of_interest in 0..v_len {
        let mut lesser_index = index_of_interest;
        for secondary_index in (index_of_interest + 1)..v_len {
            match vector[lesser_index].cmp(&vector[secondary_index]) {
                Ordering::Greater => lesser_index = secondary_index,
                _ => continue,
            }
        }

        let swap_tmp = vector[index_of_interest];
        vector[index_of_interest] = vector[lesser_index];
        vector[lesser_index] = swap_tmp;
    }

    // after the above, we have the elements in an ascending
    // order. reverse, if the descending order was desired.
    // wasteful but not the end of the world.
    match order {
        Order::Desc => vector.reverse(),
        Order::Asc => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{seq::SliceRandom, thread_rng};

    #[test]
    fn sorts_empty_vector() {
        let mut empty_vector: Vec<()> = vec![];
        sort(&mut empty_vector, Order::Desc);
        assert_eq!(vec![] as Vec<()>, empty_vector);
    }

    #[test]
    fn sorts_unit_vector() {
        let mut unit_vector = vec![0];
        sort(&mut unit_vector, Order::Desc);
        assert_eq!(1, unit_vector.len());
    }

    #[test]
    fn sorts_in_ascending_order() {
        let mut vector = vec![1, 2, 3, 4, 5];
        vector.shuffle(&mut thread_rng());
        sort(&mut vector, Order::Asc);
        assert_eq!(&vector, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_in_descending_order() {
        let mut vector = vec![1, 2, 3, 4, 5, 6];
        vector.shuffle(&mut thread_rng());
        sort(&mut vector, Order::Desc);
        assert_eq!(&vector, &[6, 5, 4, 3, 2, 1])
    }

    #[test]
    fn sorts_in_ascending_order_neg_to_pos() {
        let mut vector = vec![-2, -1, 0, 1, 2];
        vector.shuffle(&mut thread_rng());
        sort(&mut vector, Order::Asc);
        assert_eq!(&vector, &[-2, -1, 0, 1, 2])
    }
}
