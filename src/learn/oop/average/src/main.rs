use average::AveragedCollection;

fn main() {
    // let s = AveragedCollection {
    // list: vec![], // field `list` of struct `AveragedCollection` is private: private field
    // average: 0.0, // field `average` of struct `AveragedCollection` is private: private field
    // };

    // field `average` of struct `AveragedCollection` is private: private field
    // assert_eq!(s.average, 0.0);

    let mut s = AveragedCollection::new();
    assert_eq!(s.average(), 0.0);

    // s.list.push(3);
    //   ^^^^ private field

    s.add(1);
    s.add(2);
    assert_eq!(s.average(), 1.5);

    s.remove();
    assert_eq!(s.average(), 1.0);
}
