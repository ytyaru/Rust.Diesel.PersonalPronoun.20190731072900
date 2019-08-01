extern crate diesel;
extern crate personal_pronoun;

use self::diesel::prelude::*;
use self::personal_pronoun::models::*;
use self::personal_pronoun::*;
use rand::{distributions::{Distribution, Standard},Rng,seq::SliceRandom};

fn main() {
    use personal_pronoun::schema::FirstPersons::dsl::*;
    let connection = establish_connection();

    let count: i64 = personal_pronoun::schema::FirstPersons::dsl::FirstPersons.count().get_result(&connection).unwrap();
    println!("count: {}", count);
    let mut results = personal_pronoun::schema::FirstPersons::dsl::FirstPersons
        .filter(comment.eq("私的表現")
            .or(comment.like("私的表現,%"))
            .or(comment.like("%,私的表現"))
            .or(comment.like("%,私的表現,%")))
//        .limit(1)
        .load::<self::personal_pronoun::models::FirstPersons>(&connection)
        .expect("Error loading table.");

    results.shuffle(&mut rand::thread_rng());
    println!("{}: {}", results[0].id, results[0].value);
}

