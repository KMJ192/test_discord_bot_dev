use std::env;

use serenity::prelude::*;

pub mod receive_event;
use receive_event::*;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let mut client = Client::new(token)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}

/*
  1. index 순서대로 정렬한다.
  2. 옆에 있는 원소를 value로 추가한다.
  3. 추가된 value도 dictionary에 추가한다.
  4. pair가 되지 않은 멤버에 한하여 중복하여 다른 멤버와 matching한다.
*/

// use std::collections::HashMap;
// use std::collections::HashSet;
// use rand::Rng;

// fn matching_pair(members_array: Vec<String>, pair_num: usize) {
//   let mut dictionary: HashMap<String, HashSet<String>> = HashMap::new();
//   let members_len = members_array.len();

//   for i in 0..members_len {
//     let mem: String = members_array[i].clone();
//     dictionary.entry(mem.clone()).or_insert(HashSet::new());

//     let pair = dictionary.get(&*mem);
//     if let Some(p) = pair {
//       if p.len() == pair_num {
//         continue;
//       }
//     }
//     let mut pair_idx = i;
//     for j in 0..pair_num {
//       if j + pair_idx == pair_num { 
//         pair_idx = 0;
//       }
//       println!("{}", pair_idx);
//       pair_idx += 1;
//     }
//   }

//   println!("{:?}", dictionary);
// }

// fn random_sort(members: String) -> Vec<String> {
//   let split: std::str::SplitWhitespace = members.split_whitespace();
//   let arr: Vec<&str> = split.collect::<Vec<&str>>();
//   let arr_len: usize = arr.len();

//   let mut random_sort: HashSet<String> = HashSet::new();

//   let mut limit = arr_len * 10;
//   while (limit > 0) {
//     let sort_num = rand::thread_rng().gen_range(0..arr_len);
//     random_sort.insert(String::from(arr[sort_num]));
//     if random_sort.len() == arr_len {
//       break;
//     }
//     limit -= 1;
//   }
//   let tmp = random_sort.iter();

//   let mut random_member = vec![String::new(); arr_len];
//   let mut idx = 0;
//   for c in tmp {
//     random_member[idx] = String::from(c);
//     idx += 1;
//   }

//   random_member
// }

// fn main() {
//   let members: String = String::from("a b c d e f g");
//   matching_pair(random_sort(members), 2);
// }