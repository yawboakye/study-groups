use std::cmp::Ordering;

// the implicit assumption made here is that corpus is sorted.
// we still take the pain to assert that this is in fact the
// case whenever we pull values from corpus. panic in case of
// a violation.
pub fn find_in_sorted<T: Ord + Copy>(corpus: &Vec<T>, el: T) -> Option<usize> {
    // if corpus is empty, we definitely can't find `el` in there.
    // return None, early.
    let corpus_size = corpus.len();
    if corpus_size == 0 {
        return None;
    }

    let mut lower = 0;
    let mut upper = corpus_size - 1;
    while lower <= upper {
        let middle_index = (lower + upper) / 2;
        let mid_el = corpus[middle_index];
        match mid_el.cmp(&el) {
            Ordering::Equal => return Some(middle_index),
            // if the element in the middle is greater than our target,
            // it means if it were to be found in the corpus, it will
            // be found between the middle and the end (since it's assumed
            // that the ordering is in an ascending order).
            // TODO(yaw, maybe): take ordering as argument.
            Ordering::Greater => upper = middle_index - 1,
            Ordering::Less => lower = middle_index + 1,
        }
    }
    None
}

pub fn find_in_sorted_recursive<T: Ord + Copy>(corpus: &Vec<T>, el: T) -> Option<usize> {
    fn inner<T: Ord + Copy>(corpus: &Vec<T>, el: &T, lower: usize, higher: usize) -> Option<usize> {
        let middle_index = (lower + higher) / 2;

        #[rustfmt::skip]
        if corpus.len() == 0 { return None; }
        if corpus.len() == 1 {
            match corpus[0].cmp(&el) {
                Ordering::Equal => return Some(middle_index),
                _ => return None,
            }
        }

        match corpus[corpus.len() / 2].cmp(&el) {
            Ordering::Equal => return Some(middle_index),
            Ordering::Greater => inner(&corpus[..middle_index].to_vec(), &el, lower, middle_index),
            Ordering::Less => inner(&corpus[middle_index..].to_vec(), el, middle_index, higher),
        }
    }

    inner(corpus, &el, 0, corpus.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_in_sorted_element_found() {
        let corpus = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected_member = 4;
        match find_in_sorted(&corpus, expected_member) {
            Some(found) => assert_eq!(3, found),
            None => panic!("expected to find element"),
        };

        match find_in_sorted_recursive(&corpus, expected_member) {
            Some(found) => assert_eq!(3, found),
            None => panic!("expected to find element"),
        };
    }

    #[test]
    fn find_in_sorted_elemnt_not_found() {
        let corpus = vec![1, 2];
        let non_member_element = 3;
        match find_in_sorted(&corpus, non_member_element) {
            Some(v) => panic!("found {v} while looking for {non_member_element}"),
            None => (),
        }

        match find_in_sorted_recursive(&corpus, non_member_element) {
            Some(v) => panic!("found {v} while looking for {non_member_element}"),
            None => (),
        }
    }

    #[test]
    fn find_in_sorted_corpus_empty() {
        let corpus = vec![];
        let non_member_element = 3;
        assert_eq!(0, corpus.len());
        match find_in_sorted(&corpus, non_member_element) {
            Some(v) => panic!("found {v} while looking for {non_member_element} in empty corpus"),
            None => (),
        }

        match find_in_sorted_recursive(&corpus, non_member_element) {
            Some(v) => panic!("found {v} while looking for {non_member_element} in empty corpus"),
            None => (),
        }
    }
}
