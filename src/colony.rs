

extern crate chrono;
extern crate rustc_serialize;
use self::chrono::DateTime;
use self::chrono::UTC;
use self::chrono::offset::TimeZone;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Object;

#[derive(Copy,Clone,Debug)]
pub struct Colony {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub visited: bool,
    pub active: bool,
    pub updated: DateTime<UTC>,
}

impl Colony {
    pub fn from_json(json: Json) -> Result<Colony, &'static str> {
        if let Json::Object(obj) = json {
            let id: u32;
            let x: f64;
            let y: f64;
            let visited: bool;
            let active: bool;
            let updated: DateTime<UTC>;
            if let Some(id_obj) = obj.get("id") {
                match *id_obj {
                    Json::I64(id_i64) => id = id_i64 as u32,
                    Json::U64(id_u64) => id = id_u64 as u32,
                    _ => return Err("ID not an integer"),
                }
            }
            else {
                return Err("No ID")
            }
            if let Some(x_obj) = obj.get("x") {
                match x_obj.as_f64() {
                    Some(x_f64) => x = x_f64,
                    None => return Err("x not a number"),
                }
            }
            else {
                return Err("No x");
            }
            if let Some(y_obj) = obj.get("y") {
                match y_obj.as_f64() {
                    Some(y_f64) => y = y_f64,
                    None => return Err("y not a number"),
                }
            }
            else {
                return Err("No y");
            }
            if let Some(visited_obj) = obj.get("visited") {
                match visited_obj.as_boolean() {
                    Some(visited_bool) => visited = visited_bool,
                    None => return Err("visited not boolean"),
                }
            }
            else {
                return Err("No visited");
            }
            if let Some(active_obj) = obj.get("active") {
                match active_obj.as_boolean() {
                    Some(active_bool) => active = active_bool,
                    None => return Err("active not boolean"),
                }
            }
            else {
                return Err("No active");
            }
            if let Some(updated_obj) = obj.get("modified") {
                match *updated_obj {
                    Json::String(ref updated_str) => {
                        // Try to parse the date/time
                        match DateTime::parse_from_rfc3339(updated_str) {
                            Ok(updated_time) => updated = DateTime::from_utc(updated_time.naive_utc(), UTC),
                            Err(_) => return Err("Malformed update time"),
                        }
                    },
                    // Assign a date in the past
                    _ => updated = UTC.ymd(1900, 1, 1).and_hms(0, 0, 0),
                }
            }
            else {
                return Err("No updated");
            }

            Ok(Colony {
                id: id,
                x: x,
                y: y,
                visited: visited,
                active: active,
                updated: updated,
            })
        }
        else {
            Err("Provided JSON not an object")
        }
    }
}

impl ToJson for Colony {
    fn to_json(&self) -> Json {
        let mut json = Object::new();
        json.insert("id".to_string(), Json::U64(self.id as u64));
        json.insert("x".to_string(), Json::U64(self.x as u64));
        json.insert("y".to_string(), Json::U64(self.y as u64));
        json.insert("visited".to_string(), Json::Boolean(self.visited));
        json.insert("active".to_string(), Json::Boolean(self.active));

        json.insert("modified".to_string(), Json::String(self.updated.to_rfc3339()));

        Json::Object(json)
    }
}
