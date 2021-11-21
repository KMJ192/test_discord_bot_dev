use std::env;

use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

pub mod random_matching;
pub mod receive_event;
use receive_event::*;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "A simple test bot").await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}


#[group]
#[commands(about, ping)]
struct General;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = StandardFramework::new()
                  .configure(|c| c.prefix("~"))
                  .group(&GENERAL_GROUP);

  let mut client = Client::builder(token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}

// use std::collections::{HashMap, HashSet};
// use rand::Rng;

// fn matching(members_array: Vec<String>, pair_num: usize) -> HashMap<String, HashSet<String>> {
//     if members_array.len() < pair_num { return HashMap::new(); }
//     let mut dictionary: HashMap<String, HashSet<String>> = HashMap::new();
//     let members_len = members_array.len();

//     for i in 0..members_len {
//         let mem: String = members_array[i].clone();
//         dictionary.insert(mem, HashSet::new());
//     }

//     let mut complete = false;

//     let (mut idx, limit) = (0, members_len * 10000);

//     while complete == false {
//         idx += 1;
//         let ran_num1 = rand::thread_rng().gen_range(0..members_len);
//         let ran_num2 = rand::thread_rng().gen_range(0..members_len);
//         if ran_num1 == ran_num2 { continue; }

//         let mem1 = members_array[ran_num1].clone();
//         let mem2 = members_array[ran_num2].clone();

//         let mut set1 = HashSet::new();
//         let mut set2 = HashSet::new();

//         if let Some(p1) = dictionary.get(&*mem1) {
//             if p1.len() >= pair_num {
//                 continue;
//             }
//             set1 = p1.clone();
//         }
//         if let Some(p2) = dictionary.get(&*mem2) {
//             if p2.len() >= pair_num {
//                 continue;
//             }
//             set2 = p2.clone();
//         }

//         set1.insert(mem2.clone());
//         set2.insert(mem1.clone());
//         dictionary.insert(mem1, set1);
//         dictionary.insert(mem2, set2);

//         let mut cnt = 0;
//         for d in &dictionary {
//             if d.1.len() < pair_num {
//                 cnt += 1;
//             }
//         }
//         if cnt <= 1 {
//             complete = true;
//         }
//         if idx > limit {
//             break;
//         }
//     }
//     let tmp = dictionary.clone();
//     for d in tmp {
//         if d.1.len() < pair_num {
//             let mut l = d.1.len();
//             let (mut idx, limit) = (0, 10000);
//             while l < pair_num {
//                 idx += 1;
//                 if let Some(p) = dictionary.get(&*d.0) {
//                     let mut set1 = p.clone();
//                     let mut set2 = HashSet::new();

//                     let ran_num = rand::thread_rng().gen_range(0..members_len);
//                     let mem = members_array[ran_num].clone();
//                     if mem != *d.0 {
//                         l += 1;
//                         if let Some(pp) = dictionary.get(&*mem) {
//                             set2 = pp.clone();
//                         }

//                         if let Some(_) = set1.get(&*mem){}
//                         else {
//                             set1.insert(mem.clone());
//                             set2.insert(d.0.clone());
//                             dictionary.insert(d.0.clone(), set1);
//                             dictionary.insert(mem, set2);
//                         }
//                     }
//                 }
//                 if idx > limit {
//                     break;
//                 }
//             }
//         }
//     }
//     dictionary
// }

// fn make_array(members: String) -> Vec<String> {
//     let split: std::str::SplitWhitespace = members.split_whitespace();
//     let arr = split.collect::<Vec<&str>>();
//     let mut re = vec![];
//     for s in arr {
//         re.push(String::from(s));
//     }
//     re
// }

// fn output_str(result: HashMap<String, HashSet<String>>) -> String {
//   let mut make = String::new();
//   let ar = String::from(" => ");

//   for d in result {
//     make.push_str(&*d.0);
//     make.push_str(&ar);
//     for e in d.1 {
//       make.push_str(&*e);
//       make.push_str(&*String::from(" "));
//     }
//     make.push('\n');
//   }
//   make
// }

// fn main() {
//     let members: String = String::from("member1 member2 member3 member4 member5 member6 member7 member8");
//     let result = matching(make_array(members), 2);
//     let str = output_str(result);
//     println!("{}", str);
// }