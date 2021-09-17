use serde::Deserialize;

pub mod stryktipset;

fn main() {
    let games = get_games();
    print_games(games);
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct StrykTipset {
    week: String,
    draws: Vec<stryktipset::DrawEvent>,
    revenue: String,
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
    println!("Week: {}\nRevenue: {}", s.week, s.revenue);
    
    for game in 1..13 {
        println!("{}", s.draws[game].event_description);
    }
}