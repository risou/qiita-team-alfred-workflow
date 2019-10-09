extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize)]
struct QiitaItem {
    title: String,
    url: String,
    created_at: DateTime<Local>,
    user: QiitaUser,
}

#[derive(Deserialize)]
struct QiitaUser {
    id: String,
}

#[derive(Serialize)]
struct AlfredScriptFilterBody {
    title: String,
    subtitle: String,
    arg: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let team = &args[1];
    let token = &args[2];
    let query = &args[3..].join(" ");

    let client = reqwest::Client::new();
    let per_page = 20;
    let mut res = client.get(&format!("https://{}.qiita.com/api/v2/items", team))
        .query(&[("query", query), ("per_page", &per_page.to_string())])
        .bearer_auth(token)
        .send()
        .unwrap();

    let items: Vec<QiitaItem> = res.json().unwrap();
    let mut results = Vec::new();
    for item in &items {
        let result = AlfredScriptFilterBody {
            title: item.title.to_owned(),
            subtitle: format!("{} {}", item.user.id, item.created_at.format("%Y-%m-%d %H:%M:%S")),
            arg: item.url.to_owned(),
        };
        results.push(result);
    }

    println!("{{\"items\": {}}}", serde_json::to_string(&results).unwrap());
}
