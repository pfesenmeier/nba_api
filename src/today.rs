use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Today {
    #[serde(rename = "_internal")]
    internal: Internal,
    links: Links,
    season_schedule_year: i64,
    show_playoffs_clinch: bool,
    team_sites_only: TeamSitesOnly,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Internal {
    consolidated_dom_key: String,
    end_to_end_time_millis: String,
    igor_path: String,
    pub_date_time: String,
    xslt: String,
    xslt_compile_time_millis: String,
    xslt_force_recompile: String,
    xslt_in_cache: String,
    xslt_transform_time_millis: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Links {
    allstar_roster: String,
    anchor_date: String,
    boxscore: String,
    calendar: String,
    current_date: String,
    current_scoreboard: String,
    game_book_pdf: String,
    lead_tracker: String,
    league_conf_standings: String,
    league_div_standings: String,
    league_last_five_game_team_stats: String,
    league_mini_standings: String,
    league_roster_coaches: String,
    league_roster_players: String,
    league_schedule: String,
    league_team_stats_leaders: String,
    league_ungrouped_standings: String,
    mini_boxscore: String,
    pbp: String,
    player_game_log: String,
    player_profile: String,
    player_uber_stats: String,
    playoff_series_leaders: String,
    playoffs_bracket: String,
    preview_article: String,
    recap_article: String,
    scoreboard: String,
    #[serde(rename = "teamICS2")]
    team_ics2: String,
    #[serde(rename = "teamICS")]
    team_ics: String,
    team_leaders2: String,
    team_leaders: String,
    team_roster: String,
    team_schedule: String,
    team_schedule_year2: String,
    team_schedule_year: String,
    teams: String,
    teams_config: String,
    teams_config_year: String,
    ticket_link: String,
    today_scoreboard: String,
    universal_link_mapping: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamSitesOnly {
    all_play_by_play: String,
    display_year: String,
    last_play_by_play: String,
    player_matchup: String,
    roster_year: i64,
    season_stage: i64,
    season_year: i64,
    series: String,
    stats_stage: i64,
    stats_year: i64,
}
