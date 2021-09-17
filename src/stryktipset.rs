use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DrawEvent {
    #[serde(default)]
    cancelled: bool,
    #[serde(deserialize_with = "parse_null")]
    event_comment: String,
    pub event_description: String,
    #[serde(deserialize_with = "parse_null")]
    extra_info: String,
    event_number: i32,
    event_type_id: i32,
    participant_type: String,
    #[serde(rename = "match")]
    game : Match,
    pub odds: Odds,
    #[serde(deserialize_with = "parse_null")]
    start_odds: String,
    #[serde(deserialize_with = "parse_null")]
    outcomes: String,
    pub svenska_folket: SvenskaFolket,
    tio_tidningars_tips: Option<TioTidningarsTips>,
    provider_ids: Option<Vec<ProviderIds>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    match_id: i32,
    #[serde(deserialize_with = "parse_null")]
    match_start: String,
    #[serde(deserialize_with = "parse_null")]
    status: String,
    status_id: i32,
    #[serde(deserialize_with = "parse_null")]
    status_time: String,
    coverage: i32,
    participants: Vec<Participants>,
    league: League,
    league_table: LeagueTable,
    result: Option<[Results; 0]>,
    media: Option<Vec<Media>>,
    mutuals: Mutuals,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Results {
    #[serde(deserialize_with = "parse_null")]
    empty: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Participants {
    id: i32,
    #[serde(rename = "type")]
    loc: String,
    name: String,
    latest: Latest,
    trend: i32,
    goal_avg: String,
    short_name: String,
    medium_name: String,
    code: String,
    country_id: i32,
    manager_id: i32,
    arena_id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Latest {
    #[serde(rename = "0")]
    zero: String,
    #[serde(rename = "1")]
    one: String,
    #[serde(rename = "2")]
    two: String,
    #[serde(rename = "3")]
    three: String,
    #[serde(rename = "4")]
    four: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct League {
    id: i32,
    unique_league_id: i32,
    unique_league_name: String,
    name: String,
    short_name: String,
    country: Country,
    code: String,
    print_abbreviation: String,
    season: Season,
    do_show: bool,
    is_home: bool,
    legacy_key: i32,
    num_teams: i32,
    popular: bool,
    rank: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Country {
    id: i32,
    name: String,
    iso_code: String,
    population: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Season {
    id: i32,
    name: String,
    start_date: String,
    end_date: String,
    legacy_key: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LeagueTable {
    home_team: TeamStats,
    away_team: TeamStats,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TeamStats {
    position: String,
    points: String,
    played: String,
    wins: String,
    draws: String,
    losses: String,
    goal_diff: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    channel_id: i32,
    #[serde(deserialize_with = "parse_null")]
    channel_name: String,
    #[serde(deserialize_with = "parse_null")]
    start_time: String,
    #[serde(deserialize_with = "parse_null")]
    end_time: String,
    competition_id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mutuals {
    home_wins: i32,
    draws: i32,
    away_wins: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Odds {
    #[serde(deserialize_with = "parse_null")]
    pub one: String,
    #[serde(deserialize_with = "parse_null")]
    pub x: String,
    #[serde(deserialize_with = "parse_null")]
    pub two: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SvenskaFolket {
    pub one: String,
    pub x: String,
    pub two: String,
    date: String,
    ref_one: String,
    ref_x: String,
    ref_two: String,
    ref_date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TioTidningarsTips {
    one: i32,
    x: i32,
    two: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProviderIds {
    #[serde(deserialize_with = "parse_null")]
    provider: String,
    #[serde(deserialize_with = "parse_null")]
    id: String,
}

fn parse_null<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("Null".to_string()))
}