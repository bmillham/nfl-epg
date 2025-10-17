use std::fs;
use std::string::ToString;
use quick_xml::se::to_string_with_root;
use xmltv::*;
use chrono::{Utc, Duration, Datelike, Timelike};
use clap::Parser;
use xtream_lib::xtream_connection::server;
use xtream_lib::xtream_connection::valueextensions::ValueExtensions;
use xtream_lib::xtream_info::account::{Account, XboolExtensions, XdateExtensions};
use nfl_epg;

const SPORTS_URL: &str = "https://epgshare01.online/epgshare01/epg_ripper_US_SPORTS1.xml.gz";
const LOCALS_URL: &str = "https://epgshare01.online/epgshare01/epg_ripper_US_LOCALS1.xml.gz";
const GENERATOR_INFO: &str = "NFL-EPG";
const GENERATOR_URL: &str = "https://github.com/bmillham/nfl-epg";

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    server: String,
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    password: String,
    #[arg(short, long, default_value = "false")]
    debugging: bool,
    #[arg(short, long, default_value = "false")]
    local_info: bool,
    #[arg(short, long, default_value = "false")]
    next_game: bool,
    #[arg(short, long, default_value = "nfl_epg.xml")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let server = server::new(&args.server, &args.username, &args.password);
    let account_info: Account = server.get_account_info().await;
    println!("Account Information");
    println!(" Created: {}", account_info.created_at.to_date());
    println!(" Expires: {}", account_info.exp_date.to_date());
    println!(" Status: {}", account_info.status);
    println!(" Active Connections: {}", account_info.active_cons);
    println!(" Max Connections: {}", account_info.max_connections);
    println!(" Trial: {}", account_info.is_trial.to_bool());
    let cats = server.get_live_categories().await;

    // Get NLF PPV channel names
    println!("Looking for US: NFL PPV");
    let mut teams = vec![];
    for cat in cats {
        if cat.get_category_name() == "US| NFL PPV" {
            let live = server.get_live_streams(Some(cat.get_category_id().try_into().unwrap())).await;
            for l in live {
                //println!("  {:?}", l.get_name());
                let chan_name = l.get_name();
                let s = chan_name.split(" at ").collect::<Vec<&str>>();
                if s.len() > 1 {
                    let s1 = s[0].split_whitespace().collect::<Vec<&str>>();
                    let away = s1.last().unwrap().to_string();
                    teams.push((s1[2].to_owned(), away.to_owned(), s[1].to_owned()));
                }
            }
        }
    }
    if teams.is_empty() {
        println!("No games found");
        return;
    } else {
        println!("Found {} games", teams.len());
    }

    let now = Utc::now();
    let end = now + Duration::days(7);
    let guide_start = format!("{:4}{:02}{:02}{:02}{:02}{:02}",
                        now.year(),
                        now.month(),
                        now.day(),
                        now.hour(),
                        now.minute(),
                        now.second());
    let guide_end = format!("{:4}{:02}{:02}{:02}{:02}{:02}",
                        end.year(),
                        end.month(),
                        end.day(),
                        end.hour(),
                        end.minute(),
                        end.second());

    let mut item = nfl_epg::get_and_unzip(SPORTS_URL).await;

    let mut sports_epg = Tv {
        source_info_url: item.source_info_url,
        source_info_name: item.source_info_name,
        generator_info_name: Some(GENERATOR_INFO.to_owned()),
        source_data_url: item.source_data_url,
        generator_info_url: Some(GENERATOR_URL.to_owned()),
        channels: vec![],
        programmes: vec![],
    };
    let mut local_epg: Tv = Tv{
        source_info_url: Some("".to_string()),
        source_info_name: Some("".to_string()),
        generator_info_name: Some("".to_string()),
        source_data_url: Some("".to_string()),
        generator_info_url: Some("".to_string()),
        channels: vec![],
        programmes: vec![],
    };
    if args.local_info {
        println!("Reading locals");
        local_epg = nfl_epg::get_and_unzip(LOCALS_URL).await;
    }
    println!("Read epg info");
    for team in teams.iter() {
        //println!("Team: {:?}", team);
        let mut chanid = "".to_string();
        let mut found_id: String = "".to_string();
        for c in &mut item.channels {
            if c.display_names[0].name.starts_with("NFL") {
                if c.display_names[0].name.to_lowercase().contains(team.2.to_lowercase().as_str()) {
                    chanid = format!("NFL{:02}.us", team.0).to_string();
                    found_id = c.id.clone();
                    c.id = chanid.clone();
                    sports_epg.channels.push(c.clone());
                }
            }
        }

        let mut events: u16 = 0;
        for p in &mut item.programmes {
            if p.channel == found_id {
                let mut z = p.start.split_whitespace();
                let day = z.next().unwrap().to_string();

                if day < guide_start || day > guide_end {
                    continue // Skip anything older than today and more than 6 days in the future
                }
                if !p.titles[0].value.to_lowercase().contains(team.2.to_lowercase().as_str()) ||
                    !p.titles[0].value.to_lowercase().contains(team.1.to_lowercase().as_str()) {
                    continue
                }
                p.channel = chanid.clone(); // Change the chanid
                if p.titles[0].value.to_lowercase().starts_with("next game:") {
                    // Skip if --next-game isn't selected
                    if !args.next_game {
                        continue;
                    }
                    // Remove categories so the guide doesn't highlight as sport
                    p.categories = vec![];
                } else {
                    if args.local_info {
                        for local_p in &mut local_epg.programmes {
                            if local_p.titles[0].value.to_string() == "Live: NFL Football" {
                                if local_p.descriptions[0].value.to_lowercase().contains(team.2.to_lowercase().as_str()) {
                                    if local_p.descriptions[0].value.to_lowercase().contains(team.1.to_lowercase().as_str()) {
                                        p.descriptions.push(local_p.descriptions[0].clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                events += 1;
                sports_epg.programmes.push(p.clone());
                if p.titles[0].value.to_lowercase().contains("overtime") {
                    // Once the overtime entry is found, done with this week
                    break;
                }
            }
        }
        println!("{} @ {}: Added {} guide entries", team.1, team.2 , events);
    }
    fs::write(args.output,
              "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n".to_owned() +
                  &to_string_with_root("tv", &sports_epg).unwrap())
        .expect("couldn't write to file");

}
