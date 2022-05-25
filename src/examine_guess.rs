use std::iter::zip;
use std::rc::Rc;
use crate::utils::{CompFlag, InputType, WordSize};
use crate::word_info::TinyMap;

/// In this file I'll define functions and data structures dealing with the comparison
/// of a guess with the reference

/// The result of a comparison
enum ComparisonRes {
    Win,
    Fail(NewInfo)
}

/// The info collected from a non winning comparison
struct NewInfo {
    res: Vec<CompFlag>,
    correct_placements: Vec<(InputType, WordSize)>,
    wrong_pos: Vec<(InputType, WordSize)>,
    not_present: Vec<InputType>,
    min_times: TinyMap,
    exact_times: TinyMap,
}

/// Evaluate the word (don't analyze it)
fn evaluate_word(word: &Vec<InputType>, reference: Rc<Vec<InputType>>) -> Vec<CompFlag> {
    let mut map: TinyMap = TinyMap::from_vec(reference.as_ref());
    let mut res: Vec<Option<CompFlag>> = Vec::with_capacity(word.len());

    for (&w, &r) in zip(word, reference.as_ref()) {
        if w == r {
            res.push(Some(CompFlag::Match));
            map.pop_letter(w);
        } else if map.test_membership(w) {
            res.push(None);
        } else {
            res.push(Some(CompFlag::NotPresent))
        }
    }
    for (mut r_i, &w) in zip(res.iter_mut(), word.iter()) {
        match *r_i {
            None => {
                if map.pop_letter(w) {
                    *r_i = Some(CompFlag::WrongPosition);
                } else {
                    *r_i = Some(CompFlag::WrongPositionOverflow);
                }
            }
            _ => ()
        }
    }
    return res.into_iter().map(|x| x.unwrap()).collect();
}


#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::examine_guess::evaluate_word;
    use crate::utils::{CompFlag, InputType};

    fn vec_i32_to_input(inp: Vec<i32>) -> Vec<InputType> {
        inp.iter().map(|&x| {InputType::new(x as u8).unwrap()}).collect()
    }

    #[test]
    fn test_complex_1() {
        let targ: Rc<Vec<InputType>> =
            Rc::new(vec_i32_to_input(        vec![1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,4]));
        let guess = vec_i32_to_input(vec![2,2,1,1,2,3,3,2,3,3,2,3,1,2,3,5]);
        let expect = vec![
            CompFlag::WrongPositionOverflow,
            CompFlag::Match,
            CompFlag::WrongPosition,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::WrongPosition,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::WrongPositionOverflow,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::Match,
            CompFlag::NotPresent,
        ];

        let res = evaluate_word(&guess, targ);
        let r = CompFlag::vec_to_res(&res);
        let e = CompFlag::vec_to_res(&expect);

        let eq = res.iter().zip(expect.iter())
            .map(|(x, y)| *x == *y)
            .fold(true, |acc, v| v && acc);
        assert!(eq);
    }
}