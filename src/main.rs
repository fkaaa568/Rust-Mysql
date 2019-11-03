#[macro_use]
extern crate mysql;

use std::io;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Student {
    student_id: i32,
    student_name: Option<String>,
    student_email: Option<String>,
    student_age: i32,
    student_status: Option<String>,
}

fn main() {
    println!("Enter your Student_id");
    let mut student_id = String::new();
    io::stdin().read_line(&mut student_id);
    let student_id:i32 = student_id.trim().parse().unwrap();

    println!("Enter your Name");
    let mut student_name = String::new();
    io::stdin().read_line(&mut student_name);
    let student_name = student_name.trim().parse().unwrap();

    println!("Enter your email");
    let mut student_email = String::new();
    io::stdin().read_line(&mut student_email);
    let student_email = student_email.trim().parse().unwrap();
    
    println!("Enter your age");
    let mut student_age = String::new();
    io::stdin().read_line(&mut student_age);
    let student_age = student_age.trim().parse().unwrap();

    println!("Enter your Status");
    let mut student_status = String::new();
    io::stdin().read_line(&mut student_status);
    let student_status = student_status.trim().parse().unwrap();


    let student = Student{student_id:student_id,student_name:Some(student_name),student_email:Some(student_email),student_age:student_age,student_status:Some(student_status)};
    insert(student);
    fetch();
    
}

fn insert(student: Student){
    let pool = my::Pool::new("mysql://root:@localhost:3309/test").unwrap();    
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
        student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO tblStudent
                                       (student_id, student_name, student_email, student_age,student_status)
                                   VALUES
                                       (:student_id, :student_name, :student_email, :student_age, :student_status)").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "student_id" => s.student_id,
                "student_name" => &s.student_name,
                "student_email" => &s.student_email,
                "student_age" => &s.student_age,
                "student_status" => &s.student_status,


            }).unwrap();
        }
    }
}


fn update(student:Student){
    let pool = my::Pool::new("mysql://root:@localhost:3309/test").unwrap();    
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
       student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"Update tblStudent
                                    set student_name=:student_name,
                                    student_email=:student_email,
                                     student_age=:student_age,
                                     student_status=:student_status

                                    where student_id=:student_id
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "student_id" => s.student_id,
                "student_name" => &s.student_name,
                "student_email" => &s.student_email,
                "student_age" => &s.student_age,
                "student_status" => &s.student_status,

            }).unwrap();
        }
    }
}


fn delete(student: Student){
    let pool = my::Pool::new("mysql://root:@localhost:3309/test").unwrap();    
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
        student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"delete from tblStudent                                    
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
              "student_id" => s.student_id,
                "student_name" => &s.student_name,
                "student_email" => &s.student_email,
                "student_age" => &s.student_age,
                "student_status" => &s.student_status,

            }).unwrap();
        }
    }
}

fn fetch(){
    let pool = my::Pool::new("mysql://root:@localhost:3309/test").unwrap();
    // Let's select payments from database
    let selected_students: Vec<Student> =
    pool.prep_exec("SELECT student_id, student_name, student_email, student_age, student_status from tblStudent", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let (student_id, student_name, student_email, student_age, student_status) = my::from_row(row);
            Student {
                student_id: student_id,
                student_name: student_name,
                student_email: student_email,
                student_age: student_age,
                student_status: student_status,

            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    for s in 0..selected_students.len(){
        println!("ID: {} Name: {:?} Email: {:?} Age: {:?} Status: {:?}",selected_students[s].student_id,selected_students[s].student_name,
                        selected_students[s].student_email,selected_students[s].student_age,selected_students[s].student_status);

    }    
}