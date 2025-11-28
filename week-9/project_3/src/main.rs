use std::fs::File;
use std::io::Write;

fn main() {

    let name = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministry = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zone = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South east"
    ];


    let mut file = File::create("convicted ministers.txt").expect("create failed");
    file.write_all(format!("1).NAME: {}, MINISTRY: {}, ZONE: {}\n",name[0],ministry[0],zone[0]).as_bytes()).unwrap();
    file.write_all(format!("2).NAME: {}, MINISTRY: {}, ZONE: {}\n",name[1],ministry[1],zone[1]).as_bytes()).unwrap();
    file.write_all(format!("3).NAME: {}, MINISTRY: {}, ZONE: {}\n",name[2],ministry[2],zone[2]).as_bytes()).unwrap();
    file.write_all(format!("4).NAME: {}, MINISTRY: {}, ZONE: {}\n",name[3],ministry[3],zone[3]).as_bytes()).unwrap();
    file.write_all(format!("5).NAME: {}, MINISTRY: {}, ZONE: {}\n",name[4],ministry[4],zone[4]).as_bytes()).unwrap();

    println!("\nfile created.");
}
