use serde::Deserialize;

pub mod stryktipset;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct StrykTipset {
    week: String,
    draws: Vec<stryktipset::DrawEvent>,
    revenue: String,
}

fn main() {
    let games = get_games();
    print_games(games);
}

#[tokio::main]
async fn get_games() -> StrykTipset {
    let url = format!("https://api.www.svenskaspel.se/draw/1/stryktipset/draws");
    let res = reqwest::get(url)
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

    let mut g: Vec<stryktipset::DrawEvent> = Vec::new();

    for game in 0..13 {
        g.push(<stryktipset::DrawEvent>::deserialize(&res["draws"][0]["drawEvents"][game]).unwrap());
    }

    StrykTipset {
        week: String::deserialize(&res["draws"][0]["drawComment"]).unwrap(),
        draws: g,
        revenue: String::deserialize(&res["draws"][0]["currentNetSale"]).unwrap(),
    }
}

fn print_games(s: StrykTipset) {
    println!("{}\nRevenue: {} SEK\n", s.week, &s.revenue);
    for game in 0..13 {
        println!("{}", s.draws[game].event_description);
        println!("1:{} x:{} 2:{}",
                parse_odds(&s.draws[game].odds.one),
                parse_odds(&s.draws[game].odds.x),
                parse_odds(&s.draws[game].odds.two));
        print_favorite(&s.draws[game]);
        print_goal_avg(&s.draws[game].game.participants[0]);
        print_goal_avg(&s.draws[game].game.participants[1]);
        println!("");
    }
}

fn print_favorite(o: &stryktipset::DrawEvent) {
    let one: f64 = parse_odds(&o.odds.one).trim().parse().unwrap();
    let x: f64 = parse_odds(&o.odds.x).trim().parse().unwrap();
    let two: f64 = parse_odds(&o.odds.two).trim().parse().unwrap();
    println!("Favorite: {}", get_lowest_odds(one, x, two));
}

fn print_goal_avg(t: &stryktipset::Participants) {
    println!("{} goal average: {}", t.name, t.goal_avg);
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