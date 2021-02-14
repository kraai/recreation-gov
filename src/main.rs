use chrono::{DateTime, FixedOffset, TimeZone};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
struct Campsites {
    campsites: BTreeMap<String, Campsite>,
}

#[derive(Debug, Deserialize)]
struct Campsite {
    availabilities: BTreeMap<DateTime<FixedOffset>, String>,
    campsite_id: String,
    campsite_reserve_type: String,
    campsite_type: String,
    capacity_rating: String,
    // loop: String,
    max_num_people: usize,
    min_num_people: usize,
    quantities: Option<()>,
    site: String,
    type_of_use: String,
}

fn main() {
    let client = Client::new();
    let campsites = client
        .get("https://www.recreation.gov/api/camps/availability/campground/272300/month")
        .query(&[("start_date", "2021-04-01T00:00:00.000Z")])
        .send()
        .unwrap()
        .json::<Campsites>()
        .unwrap();
    let mut first = true;

    for day in &[23, 24, 25] {
        let mut print_header = true;

        for campsite in campsites.campsites.values() {
            if campsite.availabilities[&FixedOffset::east(0).ymd(2021, 4, *day).and_hms(0, 0, 0)]
                == "Available"
            {
                if print_header {
                    if first {
                        first = false;
                    } else {
                        println!();
                    }

                    println!("# 2021-04-{}", day);
                    println!("");
                    print_header = false;
                }

                println!("* {}", campsite.site);
            }
        }
    }
}
