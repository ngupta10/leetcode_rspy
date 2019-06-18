/*
 * Suppose you have a random list of people standing in a queue.
 * Each person is described by a pair of integers (h, k), where h is the height of the person and k is the number of people
 * in front of this person who have a height greater than or equal to h. Write an algorithm to reconstruct the queue.
 *
 * Note: The number of people is less than 1,100.
 *
 * Example
 * -----------
 * Input: [[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]]
 * Output: [[5,0], [7,0], [5,2], [6,1], [4,4], [7,1]]
*/


// XXX: Could not figure out on my own because I'm a dumb idiot
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people.clone();
    let mut res = vec![];
    people.sort_by_key(|k| (-k[0], k[1]));

    for x in people {
        res.insert(x[1] as usize, vec![x[0], x[1]])
    }

    return res
}

#[cfg(test)]
mod test {
    use super::reconstruct_queue;

    #[test]
    fn example1() {
        let input = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
        let output = vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]];
        assert_eq!(reconstruct_queue(input), output);
    }
}
