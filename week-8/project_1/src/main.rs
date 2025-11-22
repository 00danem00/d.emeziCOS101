use std::io;

fn main() {

    let office_administrator = vec![
        ("Intern"), ("APS 1-2"),
        ("Administrator"), ("APS 3-5"),
        ("Senior Administrator"), ("APS 5-8"),
        ("Office Manager"), ("EL1 8-10"),
        ("Director"), ("EL2 10-13"),
        ("CEO"), ("SES")
    ];

    let academic = vec![
        ("-"), ("APS 1-2"),
        ("Research Assistant"), ("APS 3-5"),
        ("PhD Candidate"), ("APS 5-8"),
        ("Post-Doc Researcher"), ("EL1 8-10"),
        ("Senior Lecturer"), ("EL2 10-13"),
        ("Dean"), ("SES")
    ];

    let lawyer = vec![
        ("Paralegal"), ("APS 1-2"),
        ("Junior Associate"), ("APS 3-5"),
        ("Associate"), ("APS 5-8"),
        ("Senior Associate 1-2"), ("EL1 8-10"),
        ("Senior Associate 3-4"), ("EL2 10-13"),
        ("Partner"), ("SES")
    ];

    let teacher = vec![
        ("Placement"), ("APS 1-2"),
        ("Classroom Teacher"), ("APS 3-5"),
        ("Snr Teacher"), ("APS 5-8"),
        ("Leading Teacher"), ("EL1 8-10"),
        ("Deputy Principal"), ("EL2 10-13"),
        ("Principal"), ("SES")
    ];

    let mut profession = String::new();
    println!("\ninput your profession (office administrator, academic, lawyer, teacher):");
    io::stdin().read_line(&mut profession).expect("failed to read line");
    let profession = profession.trim();

    let mut years = String::new();
    println!("input your experience in years:");
    io::stdin().read_line(&mut years).expect("failed to read line");
    let years: u8 = years.trim().parse().expect("invalid input for experience");

    let mut job_title = String::new();
    println!("input your job title (CEO, Dean, Partner, Principal e.t.c) in all lowercase:");
    io::stdin().read_line(&mut job_title).expect("failed to read line");
    let job_title = job_title.trim();


                //for OFFICE ADMINISTRATOR
    if profession == "office administrator" && job_title == "intern" && years >= 1 && years <=2 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[0], office_administrator[1]);
    } else if profession == "office administrator" && job_title == "administrator" && years >= 3 && years <=5 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[2], office_administrator[3]);
    } else if profession == "office administrator" && job_title == "senior administrator" && years > 5 && years <=8 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[4], office_administrator[5]);
    } else if profession == "office administrator" && job_title == "office manager" && years > 8 && years <=10 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[6], office_administrator[7]);
    } else if profession == "office administrator" && job_title == "director" && years > 10 && years <=13 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[8], office_administrator[9]);
    } else if profession == "office administrator" && job_title == "ceo" && years > 13 && years <=15 {
        println!("your job title is: {:?}, and your position is: {:?}", office_administrator[10], office_administrator[11]);
    } 



                //for ACADEMIC
    if profession == "academic" && job_title == "research assistant" && years >= 3 && years <=5 {
        println!("your job title is: {:?}, and your position is: {:?}", academic[2], academic[3]);
    } else if profession == "academic" && job_title == "phd candidate" && years > 5 && years <=8 {
        println!("your job title is: {:?}, and your position is: {:?}", academic[4], academic[5]);
    } else if profession == "academic" && job_title == "post-doc researcher" && years > 8 && years <=10 {
        println!("your job title is: {:?}, and your position is: {:?}", academic[6], academic[7]);
    } else if profession == "academic" && job_title == "senior lecturer" && years > 10 && years <=13 {
        println!("your job title is: {:?}, and your position is: {:?}", academic[8], academic[9]);
    } else if profession == "academic" && job_title == "dean" && years > 13 && years <=15 {
        println!("your job title is: {:?}, and your position is: {:?}", academic[10], academic[11]);
    } 
  



                //for LAWYER
    if profession == "lawyer" && job_title == "paralegal" && years >= 1 && years <=2 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[0], lawyer[1]);
    } else if profession == "lawyer" && job_title == "junior associate" && years >= 3 && years <=5 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[2], lawyer[3]);
    } else if profession == "lawyer" && job_title == "associate" && years > 5 && years <=8 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[4], lawyer[5]);
    } else if profession == "lawyer" && job_title == "senior associate 1-2" && years > 8 && years <=10 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[6], lawyer[7]);
    } else if profession == "lawyer" && job_title == "senior associate 3-4" && years > 10 && years <=13 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[8], lawyer[9]);
    } else if profession == "lawyer" && job_title == "partner" && years > 13 && years <=15 {
        println!("your job title is: {:?}, and your position is: {:?}", lawyer[10], lawyer[11]);
    }
 



                //for TEACHER
    if profession == "teacher" && job_title == "placement" && years >= 1 && years <=2 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[0], teacher[1]);
    } else if profession == "teacher" && job_title == "classroom teacher" && years >= 3 && years <=5 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[2], teacher[3]);
    } else if profession == "teacher" && job_title == "snr teacher" && years > 5 && years <=8 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[4], teacher[5]);
    } else if profession == "teacher" && job_title == "leading teacher" && years > 8 && years <=10 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[6], teacher[7]);
    } else if profession == "teacher" && job_title == "deputy principal" && years > 10 && years <=13 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[8], teacher[9]);
    } else if profession == "teacher" && job_title == "principal" && years > 13 && years <=15 {
        println!("your job title is: {:?}, and your position is: {:?}", teacher[10], teacher[11]);
    }



}