extern crate diesel;
extern crate personal_pronoun;

use self::diesel::prelude::*;
use self::personal_pronoun::models::*;
use self::personal_pronoun::*;
use rand::{distributions::{Distribution, Standard},Rng};

fn main() {
    use personal_pronoun::schema::FirstPersons::dsl::*;
    let connection = establish_connection();

    // i32だと怒られる
//    let count: i64 = FirstPersons.count().get_result(&connection).unwrap();
//    let count: i32 = personal_pronoun::schema::FirstPersons::dsl::FirstPersons.count().get_result(&connection).unwrap() as i32;
    let count: i64 = personal_pronoun::schema::FirstPersons::dsl::FirstPersons.count().get_result(&connection).unwrap();
    println!("count: {}", count);
    let mut rng = rand::thread_rng();
    let rnd_id = rng.gen_range(0, count-1);
    println!("rnd_id: {}", rnd_id);

    let results = personal_pronoun::schema::FirstPersons::dsl::FirstPersons
        .filter(id.eq(rnd_id as i32))
//        .limit(1)
        .load::<self::personal_pronoun::models::FirstPersons>(&connection)
        .expect("Error loading table.");

    for first in results {
        println!("{}", first.value);
    }
}
