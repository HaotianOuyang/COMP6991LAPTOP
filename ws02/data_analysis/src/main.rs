use csv::ReaderBuilder;
use std::{collections::{HashMap, HashSet}, error::Error};
use serde::Deserialize;



//how many unique students are there?
//what is the most commmon course(with how many students)?
//what is the least common course(with how many students)?
//what is the average WAM of all students in the file( to 2 decimal places)?
#[derive(Debug, Deserialize)]
struct Enrolment{
    course_code : String,
    student_num : String,
    name: String,
    program_num: String,
    plan : String,
    wam: String,
    session: String,
    birthdate: String,
    sex : String,

}
const ENROLMENTS_PATH: &str = "enrolments.psv";
fn main() -> Result<(), Box<dyn Error>>{
    

    let mut csv_reader = ReaderBuilder::new().delimiter(b'|').has_headers(false).from_path(ENROLMENTS_PATH)?;

    let mut unique_students = HashSet::new();
    let mut courses= HashMap::new();
    let mut total_wam = 0.0;
    let mut wam_count = 0;
    for result in csv_reader.deserialize::<Enrolment>(){
        let record = result?;

        unique_students.insert(record.student_num.clone());
        courses.entry(record.course_code.clone()).and_modify(|counter| *counter += 1).or_insert(1);

        if let Ok(wam_value) = record.wam.parse::<f64>() {
            total_wam += wam_value;
            wam_count += 1;
        }
    }

    let most_common_course = courses.iter().max_by_key(|x| x.1);
    let least_common_course = courses.iter().min_by_key(|x| x.1);

    let average_wam = if wam_count > 0 {
        total_wam / wam_count as f64
    } else {
        0.0
    };

    println!("Number of students: {}", unique_students.len());
    if let Some((course, count)) = most_common_course {
        println!("Most common course: {} with {} students", course, count);
    }
    if let Some((course, count)) = least_common_course {
        println!("Least common course: {} with {} students", course, count);
    }
    println!("Average WAM: {:.2}", average_wam);
    Ok(())
}
