use std::hash::Hash;

fn main() {
    let mut students = vec![Student {
        name: "Ryan".to_string(),
    }];
    students.push(Student {
        name: "Lisa".to_string(),
    });

    assert!(
        &students[0]
            == &Student {
                name: "Ryan".to_string()
            }
    );

    assert!(
        students.get(0)
            == Some(&Student {
                name: "Ryan".to_string()
            })
    );

    assert!(students.get(1000) == None);

    for student in students.iter() {
        println!("Student name: {}", student.name);
    }

    use std::collections::HashMap;

    let mut enrollment = HashMap::new();
    enrollment.insert("Biology".to_string(), students);

    let bio_students = enrollment.get("Biology");
    let students = enrollment.remove("Biology");
}

#[derive(PartialEq, Eq)]
struct Student {
    name: String,
}
