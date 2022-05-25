use std::iter::zip;
use std::num::NonZeroU8;
use std::os::linux::raw::stat;
use std::panic::resume_unwind;
use std::rc::Rc;
use test::RunIgnored::No;
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

    for (w, r) in zip(word, reference.as_ref()) {
        if w == r {
            res.push(Some(CompFlag::Match));
            map.pop_letter(*w)
        } else if map.test_membership(*w) {
            res.push(None);
        } else {
            res.push(Some(CompFlag::NotPresent))
        }
    }
    for (mut r_i, w) in zip(res.iter_mut(), word.iter()) {
        match *r_i {
            None => {
                if map.pop_letter(*w) {
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