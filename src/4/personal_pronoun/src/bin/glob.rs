extern crate diesel;
extern crate personal_pronoun;

use self::diesel::prelude::*;
use self::personal_pronoun::models::*;
use self::personal_pronoun::*;
use rand::{distributions::{Distribution, Standard},Rng,seq::SliceRandom};

fn main() {
    use personal_pronoun::schema::FirstPersons::dsl::*;
    let connection = establish_connection();

    let sql = "select * from FirstPersons where (',' || comment || ',') glob '*,公的表現,*' or (',' || comment || ',') glob '*,私的表現,*' ;";
    let mut results = diesel::sql_query(sql)
        .load::<self::personal_pronoun::models::FirstPersonsRaw>(&connection);
    println!("{:?}", results);
    if let Ok(res_array) = results {
        for res in res_array {
            println!("{}: {}", res.id, res.value);
//            println!("{:?}", res);
    //        println!("{}: {}", res[0], res[1]);
        }
    }
    /*
    for res in results {
        println!("{:?}", res);
//        println!("{}: {}", res.id, res.value);
//        println!("{}: {}", res[0], res[1]);
    }
    */
}

