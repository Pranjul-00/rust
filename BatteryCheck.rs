fn main(){

    let system_name = "drogon";
    let mut battery_level = 100;

    println!("Booting up system: {}", system_name);
    println!("Initial Battery: {}%", battery_level);

    println!("Running IP scan....");
    battery_level -= 15;

    let remaining_uptime = calc_uptime(battery_level);

    println!("IP scan completed. Batter Remaining: {}%", battery_level);
    println!("Estimated uptime remaining: {} minutes.", remaining_uptime);

}

fn calc_uptime(battery: i32) -> i32 {
    
    battery*3

}
