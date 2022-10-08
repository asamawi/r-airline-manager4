use std::env;

mod auth;
mod banking;
mod co2;
mod departure;
mod fuel;
mod hanger;
mod maintenance;
mod marketing;
mod operations;
mod profits;
mod utilities;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let username: String = env::var("AM_USER").unwrap_or_default();
    let password: String = env::var("AM_PASS").unwrap_or_default();

    let cookies: String = auth::login(&username, &password);

    // information
    print_bank_details(&cookies);
    print_co2_details(&cookies);
    print_fuel_details(&cookies);
    print_hanger_details(&cookies);
    print_profit_details(&cookies);
    print_marketing_details(&cookies);

    // actions
    operations::perform_routine_operations(&cookies);
    // maintenance::maintain_planes(&cookies);

    auth::logout(cookies);

    let elapsed = now.elapsed();
    println!("Application took {:.2?}", elapsed);
}

fn print_profit_details(cookies: &str) {
    let (check_profit_a339, check_profit_a388, check_profit_a388f) = (false, true, false);

    if check_profit_a339 {
        profits::get_aircraft_wise(&cookies, &308);
        println!();
    }

    if check_profit_a388 {
        profits::get_aircraft_wise(&cookies, &2);
        println!();
    }

    // This will not work properly.
    // Pax and Cargo have different structures.
    if check_profit_a388f {
        profits::get_aircraft_wise(&cookies, &358);
        println!();
    }
}

fn print_bank_details(cookies: &str) {
    let balance: i64 = banking::get_balance(&cookies);
    println!("Current Bank Balance is {}", balance);
    println!();
}

fn print_fuel_details(cookies: &str) {
    let (fuel_price, fuel_capacity, fuel_holding, fuel_to_buy): (i16, i32, i32, i32) =
        fuel::get_status(&cookies);
    println!("Current Fuel Price is {}", fuel_price);
    println!(
        "Currently holding {}/{} capacity",
        fuel_holding, fuel_capacity
    );
    println!("Can buy a maximum of {} lbs of fuel", fuel_to_buy);
    println!();
}

fn print_co2_details(cookies: &str) {
    let (co2_price, co2_capacity, co2_holding, co2_to_buy, airline_status): (
        i16,
        i32,
        i32,
        i32,
        String,
    ) = co2::get_status(&cookies);
    println!("Current co2 Price is {}", co2_price);
    println!(
        "Currently holding {}/{} capacity",
        co2_holding, co2_capacity
    );
    println!("Can buy a maximum of {} lbs of co2", co2_to_buy);
    println!("Airline status is {}", airline_status);
    println!();
}

fn print_hanger_details(cookies: &str) {
    let (pax_hanger_capacity, pax_hanger_in_use, pax_hanger_free): (i16, i16, i16) =
        hanger::get_pax_status(cookies);
    println!(
        "Current pax hanger usage is {}/{}",
        pax_hanger_in_use, pax_hanger_capacity
    );
    println!("Can buy a maximum of {} pax aircraft", pax_hanger_free);
    println!();

    let (cargo_hanger_capacity, cargo_hanger_in_use, cargo_hanger_free): (i16, i16, i16) =
        hanger::get_cargo_status(cookies);
    println!(
        "Current cargo hanger usage is {}/{}",
        cargo_hanger_in_use, cargo_hanger_capacity
    );
    println!("Can buy a maximum of {} cargo aircraft", cargo_hanger_free);
    println!();
}

fn print_marketing_details(cookies: &str) {
    let active_campaigns: Vec<String> = marketing::get_active_campaigns(cookies);
    let (airline_reputation, cargo_reputation) = marketing::get_reputation(cookies);

    println!("Airline reputation is {airline_reputation}");
    println!("Cargo reputation is {cargo_reputation}");
    println!();
    println!("Following campaigns are active:");
    for campaign in active_campaigns {
        println!("  - {}", campaign);
    }
    println!();
}
