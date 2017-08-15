table! {
    teams {
        id -> Integer,
        time -> Double,
    }
}

table! {
    members {
        id -> Integer,
        name -> Text,
        team_id -> Integer,
        active -> Bool,
        driver -> Bool,
        position -> Integer,
    }
}
