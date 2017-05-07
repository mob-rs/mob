table! {
    teams {
        id -> Integer,
        driver_id -> Integer,
        time -> Double,
    }
}

table! {
    members {
        id -> Integer,
        name -> VarChar,
    }
}
