
use manipulating::helper::longest;

#[test]
fn longest_test() {
    let sentence = "the quick brown fox jumps over the superlazy dog";
    assert_eq!(longest(sentence), "superlazy");
}