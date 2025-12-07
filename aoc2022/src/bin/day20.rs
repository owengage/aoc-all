use aoc::{fetch_input, lines};

fn decrypt(cypher: &mut Vec<isize>, positions: &mut Vec<usize>) {
    for i in 0..cypher.len() {
        fast_shift(cypher, positions, i);
    }
}

fn fast_shift(plaintext: &mut Vec<isize>, positions: &mut Vec<usize>, i: usize) {
    let original_index = positions.iter().position(|&n| n == i).unwrap();
    let shift = plaintext[original_index];

    let dir = shift.signum();
    let full_shifts = shift.unsigned_abs() / plaintext.len();
    let full_shifts = full_shifts % (plaintext.len() - 1);
    let remaining_shifts = shift % plaintext.len() as isize;

    if full_shifts > 0 {
        plaintext.remove(original_index);
        positions.remove(original_index);
        match dir {
            1 => {
                plaintext.rotate_left(full_shifts);
                positions.rotate_left(full_shifts);
            }
            -1 => {
                plaintext.rotate_right(full_shifts);
                positions.rotate_right(full_shifts);
            }
            _ => {}
        }

        plaintext.insert(original_index, shift);
        positions.insert(original_index, i);
    }

    slow_shift_helper(plaintext, positions, original_index, remaining_shifts);
}

fn slow_shift_helper(plaintext: &mut [isize], positions: &mut [usize], index: usize, shift: isize) {
    let plen = plaintext.len();

    if shift > 0 {
        // Shift element index shift times.
        let it: Vec<_> = (0..plen)
            .into_iter()
            .cycle()
            .skip(index)
            .take(shift as usize + 1)
            .collect();

        for win in it.windows(2) {
            plaintext.swap(win[0], win[1]);
            positions.swap(win[0], win[1]);
        }
    }

    if shift < 0 {
        // Shift element index shift times.
        let it: Vec<_> = (0..plen)
            .into_iter()
            .rev()
            .cycle()
            .skip(plen - index - 1)
            .take(-shift as usize + 1)
            .collect();

        for win in it.windows(2) {
            plaintext.swap(win[0], win[1]);
            positions.swap(win[0], win[1]);
        }
    }

    // if shift < 0 {
    //     let start = index as isize + shift;

    //     if start >= 0 {
    //         plaintext[start as usize..(index + 1)].rotate_right(1);
    //         positions[start as usize..(index + 1)].rotate_right(1);
    //     } else {
    //         let remains = start.unsigned_abs() % plen;

    //         plaintext[0..index + 1].rotate_right(1);
    //         if remains == 0 {
    //             plaintext[1..plen].rotate_right(1);
    //         } else {
    //             plaintext.swap(0, plen - 1);
    //             plaintext[plen - remains..plen].rotate_right(1);
    //         }

    //         positions[0..index + 1].rotate_right(1);
    //         if remains == 0 {
    //             positions[1..plen].rotate_right(1);
    //         } else {
    //             positions.swap(0, plen - 1);
    //             positions[plen - remains..plen].rotate_right(1);
    //         }
    //     }
    // }
}

const DECRYPTION_KEY: isize = 811589153;

fn main() {
    // Not all of the numbers are unique.
    let input: Vec<_> = lines(fetch_input(2022, 20))
        .into_iter()
        .flat_map(|n| n.parse::<isize>())
        .map(|n| n * DECRYPTION_KEY)
        .collect();

    let mut positions: Vec<usize> = (0..input.len()).into_iter().collect();
    let mut output = input;

    for _ in 0..10 {
        decrypt(&mut output, &mut positions);
    }

    let zero_index = output.iter().position(|&n| n == 0).unwrap();
    let one = output[(zero_index + 1000) % output.len()];
    let two = output[(zero_index + 2000) % output.len()];
    let three = output[(zero_index + 3000) % output.len()];

    // wrong: -21663

    // part 2: too high 12040736673908
    //                   6420481789383
    //         too low:  3466297272463

    println!(
        "part 1: {} + {} + {} = {}",
        one,
        two,
        three,
        one + two + three
    );
}

#[cfg(test)]
mod test {
    use crate::{fast_shift, slow_shift_helper};

    fn slow_shift(plaintext: &mut [isize], positions: &mut [usize], i: usize) {
        let index = positions.iter().position(|&n| n == i).unwrap();
        let shift = plaintext[index];
        slow_shift_helper(plaintext, positions, index, shift);
    }
    #[test]
    fn golden() {
        let input: Vec<_> = "todo"
            .lines()
            .flat_map(|n| n.parse::<isize>())
            .map(|n| n * 34) // make slightly harder
            .collect();

        let mut pos_slow: Vec<usize> = (0..input.len()).into_iter().collect();
        let mut plain_slow = input.to_vec();

        let mut pos_fast: Vec<usize> = (0..input.len()).into_iter().collect();
        let mut plain_fast = input.to_vec();

        for i in 0..input.len() {
            slow_shift(&mut plain_slow, &mut pos_slow, i);
            fast_shift(&mut plain_fast, &mut pos_fast, i);
            assert_eq!(plain_slow, plain_fast);
        }
    }

    #[test]
    fn simple() {
        let mut pln = [0, 0, 2, 0, 0];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [0, 0, 0, 0, 2]);
        assert_eq!(pos, [0, 1, 3, 4, 2]);
    }

    #[test]
    fn simple_neg() {
        let mut pln = [0, 1, -2, 3, 4];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [-2, 0, 1, 3, 4]);
        assert_eq!(pos, [2, 0, 1, 3, 4]);
    }

    #[test]
    fn rollover() {
        let mut pln = [0, 0, 3, 0, 0];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [3, 0, 0, 0, 0]);
        assert_eq!(pos, [2, 1, 3, 4, 0]);
    }

    #[test]
    fn rollover_neg() {
        let mut pln = [0, 0, -3, 0, 0];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [0, 0, 0, 0, -3]);
        assert_eq!(pos, [4, 0, 1, 3, 2]);
    }

    #[test]
    fn rollover_beyond_original_position() {
        let mut pln = [0, 0, 5, 0, 0];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [0, 0, 5, 0, 0]);
        assert_eq!(pos, [1, 3, 2, 4, 0]);
    }

    #[test]
    fn rollover_to_end() {
        let mut pln = [1, 2, 7, 3, 4];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [2, 3, 4, 1, 7]);
        assert_eq!(pos, [1, 3, 4, 0, 2]);
    }

    #[test]
    fn rollover_to_end_neg() {
        let mut pln = [1, 2, -7, 3, 4];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [-7, 4, 1, 2, 3]);
    }

    #[test]
    fn max_roll() {
        let mut pln = [1, 2, -8, 3, 4];
        let mut pos = [0, 1, 2, 3, 4];
        slow_shift(&mut pln, &mut pos, 2);
        assert_eq!(pln, [3, 4, 1, 2, -8]);
    }
}
