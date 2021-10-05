use clap::{App, Arg};
use serde::Deserialize;
use std::fs::File;

pub mod stryktipset;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct StrykTipset {
    week: String,
    draws: Vec<stryktipset::DrawEvent>,
    revenue: String,
}

fn main() {
    let matches = App::new("octorust")
        .version("1.0")
        .author("80-am <adam@flonko.com>")
        .about("Predicts Stryktipset")
        .arg(Arg::with_name("test")
            .short("t")
            .long("testdata")
            .help("Run testdata from testdata.json")
            .takes_value(false))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Verbosity level") 
            .takes_value(false))
        .get_matches();
    print_games(matches.is_present("verbose"), get_games(matches.is_present("test")));
}

#[tokio::main]
async fn get_games(test: bool) -> StrykTipset {
    let file = File::open("testdata.json")
            .expect("file should open read only");
    let mut json: serde_json::Value = serde_json::from_reader(file)
            .expect("file should be proper JSON");

    if !test {
        let url = format!("https://api.www.svenskaspel.se/draw/1/stryktipset/draws");
        json = reqwest::get(url)
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
    }

    let mut g: Vec<stryktipset::DrawEvent> = Vec::new();

    for game in 0..13 {
        g.push(<stryktipset::DrawEvent>::deserialize(&json["draws"][0]["drawEvents"][game]).unwrap());
    }

    StrykTipset {
        week: String::deserialize(&json["draws"][0]["drawComment"]).unwrap(),
        draws: g,
        revenue: String::deserialize(&json["draws"][0]["currentNetSale"]).unwrap(),
    }
}

fn print_games(v: bool, s: StrykTipset) {
    println!("{}\nRevenue: {} SEK\n", s.week, &s.revenue);
    for game in 0..13 {
        println!("\n############################\n{}", s.draws[game].event_description);
        if v {
            print_odds(&s.draws[game]);
            print_folket(&s.draws[game]);
            print_media(&s.draws[game]);
            print_favorite(&s.draws[game]);
            print_goal_avg(&s.draws[game].game.participants[0]);
            print_trend(&s.draws[game].game.participants[0]);
            print_table_position(0, &s.draws[game]);
            print_goal_avg(&s.draws[game].game.participants[1]);
            print_trend(&s.draws[game].game.participants[1]);
            print_table_position(1, &s.draws[game]);
        }
    }
}

fn print_odds(t: &stryktipset::DrawEvent) {
    println!("1: {} x: {} 2:{}",
        parse_odds(&t.odds.one),
        parse_odds(&t.odds.x),
        parse_odds(&t.odds.two));
}

fn print_folket(t: &stryktipset::DrawEvent) {
    println!("Folket: 1: {}% x: {}% 2: {}%",
        &t.svenska_folket.one,
        &t.svenska_folket.x,
        &t.svenska_folket.two);
}

fn print_media(t: &stryktipset::DrawEvent) {
    println!("10 tidningar: 1: {}0% x: {}0% 2: {}0%",
        &t.tio_tidningars_tips.one,
        &t.tio_tidningars_tips.x,
        &t.tio_tidningars_tips.two);
}

fn print_favorite(o: &stryktipset::DrawEvent) {
    let one: f64 = parse_odds(&o.odds.one).trim().parse().unwrap();
    let x: f64 = parse_odds(&o.odds.x).trim().parse().unwrap();
    let two: f64 = parse_odds(&o.odds.two).trim().parse().unwrap();
    println!("Favorite: {}", get_lowest_odds(one, x, two));
}

fn print_goal_avg(t: &stryktipset::Participants) {
    println!("{}\nGoal average: {}", t.name, t.goal_avg);
}

fn print_trend(t: &stryktipset::Participants) {
    print!("Trend:");
    for (_i, game) in t.latest.as_array().iter().enumerate() {
        print!(" {}", game)
    }
    print!("\n");
}

fn print_table_position(p: usize, t: &stryktipset::DrawEvent) {
    if p == 0 {
        println!("Position: {}", &t.game.league_table.home_team.position);
    } else {
        println!("Position: {}", &t.game.league_table.away_team.position);
    }
}

fn parse_odds(o: &str) -> String {
    let s: &str = &o[..];
    let period = s.replace(",", ".");
    String::from(period)
}

fn get_lowest_odds(one: f64, x: f64, two: f64) -> f64 {
    if one < x {
        if one < two {
            one
        } else {
            two
        }

    } else {
        if x < two {
            x
        } else {
            two
        }
    }
}