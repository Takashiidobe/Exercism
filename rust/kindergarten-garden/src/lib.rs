use std::collections::HashMap;
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let map: HashMap<u8, &str> = HashMap::from([
        (b'G', "grass"),
        (b'C', "clover"),
        (b'R', "radishes"),
        (b'V', "violets"),
    ]);
    let students = HashMap::from([
        ("Alice", 0),
        ("Bob", 2),
        ("Charlie", 4),
        ("David", 6),
        ("Eve", 8),
        ("Fred", 10),
        ("Ginny", 12),
        ("Harriet", 14),
        ("Ileana", 16),
        ("Joseph", 18),
        ("Kincaid", 20),
        ("Larry", 22),
    ]);
    let index = *students.get(student).unwrap();
    let diagram: Vec<_> = diagram
        .lines()
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect();

    vec![
        map.get(&diagram[0][index]).unwrap(),
        map.get(&diagram[0][index + 1]).unwrap(),
        map.get(&diagram[1][index]).unwrap(),
        map.get(&diagram[1][index + 1]).unwrap(),
    ]
}
