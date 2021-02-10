use bluearch_recruitment::banner::BannerBuilder;
use bluearch_recruitment::gacha::{GachaBuilder, Recruitment};
use bluearch_recruitment::i18n::Language;
use bluearch_recruitment::student::Student;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./examples/students.json").unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let students: Vec<Student> = serde_json::from_str(&json).unwrap();

    // This particular banner consists of everyone BUT Nozomi.
    let banner_students: Vec<Student> = students
        .iter()
        .filter(|student| student.name != "ノゾミ")
        .map(|student| student.clone())
        .collect();

    // Both Hoshino and Shiroko have an increased chance of being pulled.
    let hoshino = find_student(&students, "ホシノ").unwrap();
    let shiroko = find_student(&students, "シロコ").unwrap();
    let rate_up_students = vec![shiroko, hoshino];

    let gacha = GachaBuilder::new(79.0, 18.5, 2.5)
        .with_pool(banner_students)
        .with_priority(&rate_up_students, 0.7)
        .finish()
        .unwrap();

    let banner = BannerBuilder::new("ピックアップ募集")
        .with_name_translation(Language::English, "Rate-Up Registration")
        .with_sparkable_students(&rate_up_students)
        .with_gacha(&gacha)
        .finish()
        .unwrap();

    let students = banner.roll10();

    println!("{} 10-pull: \n", banner.name);
    for student in students.iter() {
        println!(
            "{} {}",
            student.name.get(Language::English).unwrap(),
            student.rarity
        );
    }
}

pub fn find_student(students: &[Student], jpn_name: &str) -> Option<Student> {
    students
        .iter()
        .find(|student| student.name == jpn_name)
        .map(|student| student.clone())
}