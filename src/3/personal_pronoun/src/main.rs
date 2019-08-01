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
    /*
    let mut rng = rand::thread_rng();
    let rnd_id = rng.gen_range(0, count-1);
    println!("rnd_id: {}", rnd_id);
    */
    // 条件に一致するレコードのうち1件をランダムで返す
    /*
    let mut results = personal_pronoun::schema::FirstPersons::dsl::FirstPersons
//        .filter(id.eq(rnd_id as i32))
        .filter(id.eq(rnd_id as i32)
//            .and(comment.eq("私的表現").or(like("私的表現,%")).or(like("%,私的表現")).or(like("%,私的表現,%")))
            .and(comment.eq("私的表現").or(comment.like("私的表現,%")).or(comment.like("%,私的表現")).or(comment.like("%,私的表現,%")))
        )
//        .limit(1)
        .load::<self::personal_pronoun::models::FirstPersons>(&connection)
        .expect("Error loading table.");
    */
    let mut results = personal_pronoun::schema::FirstPersons::dsl::FirstPersons
//        .filter(id.eq(rnd_id as i32))
        .filter(comment.eq("私的表現")
            .or(comment.like("私的表現,%"))
            .or(comment.like("%,私的表現"))
            .or(comment.like("%,私的表現,%")))
//        .limit(1)
        .load::<self::personal_pronoun::models::FirstPersons>(&connection)
        .expect("Error loading table.");



//    let rnd_res = results.collect().shuffle(&mut rand::thread_rng());
//    println!("{}", rnd_res.next());
//    let rnd_res = results.shuffle(&mut rand::thread_rng());
//    println!("{}", rnd_res[0]);
    results.shuffle(&mut rand::thread_rng());
    println!("{}: {}", results[0].id, results[0].value);
    /*
    for first in results {
        println!("{}", first.value);
    }
    */
}
