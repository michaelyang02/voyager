use std::collections::HashSet;
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

pub struct Voyager {
    uuid: Uuid,
    nickname: String,
    birthday: NaiveDate,
    friends: HashSet<Uuid>,
}

impl Voyager {
    pub fn new(nickname: String) -> Voyager {
        Voyager {
            uuid: Uuid::new_v4(),
            nickname,
            birthday: Utc::now().date_naive(),
            friends: HashSet::new(),
        }
    }

    pub fn add_friend(&mut self, friend: Uuid) {
        self.friends.insert(friend);
    }
}