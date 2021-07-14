// Your ferry can make it safely to a nearby port, but it won't get much further. When you call to
// book another ship, you discover that no ships embark from that port to your vacation island.
// You'll need to get from the port to the nearest airport.

// Fortunately, a shuttle bus service is available to bring you from the sea port to the airport!
// Each bus has an ID number that also indicates how often the bus leaves for the airport.

// Bus schedules are defined based on a timestamp that measures the number of minutes since some
// fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the
// sea port. After that, each bus travels to the airport, then various other locations, and finally
// returns to the sea port to repeat its journey forever.

// The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from
// the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33,
// and so on. If you are there when the bus departs, you can ride that bus to the airport!

// Your notes (your puzzle input) consist of two lines. The first line is your estimate of the
// earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in
// service according to the shuttle company; entries that show x must be out of service, so you
// decide to ignore them.

// To save time once you arrive, your goal is to figure out the earliest bus you can take to the
// airport. (There will be exactly one such bus.)

// For example, suppose you have the following notes:

// 939
// 7,13,x,x,59,x,31,19

// Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59,
// 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:

// time   bus 7   bus 13  bus 59  bus 31  bus 19
// 929      .       .       .       .       .
// 930      .       .       .       D       .
// 931      D       .       .       .       D
// 932      .       .       .       .       .
// 933      .       .       .       .       .
// 934      .       .       .       .       .
// 935      .       .       .       .       .
// 936      .       D       .       .       .
// 937      .       .       .       .       .
// 938      D       .       .       .       .
// 939      .       .       .       .       .
// 940      .       .       .       .       .
// 941      .       .       .       .       .
// 942      .       .       .       .       .
// 943      .       .       .       .       .
// 944      .       .       D       .       .
// 945      D       .       .       .       .
// 946      .       .       .       .       .
// 947      .       .       .       .       .
// 948      .       .       .       .       .
// 949      .       D       .       .       .

// The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you
// would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number
// of minutes you'd need to wait gives 295.

// What is the ID of the earliest bus you can take to the airport multiplied by the number of
// minutes you'll need to wait for that bus?

use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct BusAndWaitMinutes {
    bus_id: i64,
    minutes: i64,
}

impl BusAndWaitMinutes {
    fn new(bus_id: i64, minutes: i64) -> Self {
        BusAndWaitMinutes { bus_id, minutes }
    }
}

fn find_bus_id_and_wait_time(estimate: i64, bus_ids: &[i64]) -> BusAndWaitMinutes {
    let mut next_oportunities: Vec<i64> = Vec::new();
    for &bus_id in bus_ids {
        if estimate == bus_id || estimate % bus_id == 0 {
            return BusAndWaitMinutes::new(bus_id, 0);
        }
        next_oportunities.push((estimate / bus_id + 1) * bus_id);
    }
    let bus_and_wait_time = next_oportunities.iter().enumerate().fold(
        (0, i64::MAX),
        |(mut index, mut best_estimate), (i, &next_estimate)| {
            if next_estimate < best_estimate {
                best_estimate = next_estimate;
                index = i;
            }
            (index, best_estimate)
        },
    );
    BusAndWaitMinutes::new(bus_ids[bus_and_wait_time.0], bus_and_wait_time.1 - estimate)
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|x| x.ok()).collect();
    let estimate = lines[0]
        .parse::<i64>()
        .expect("First line is estimate and should be a number");
    let bus_ids: Vec<i64> = lines[1]
        .split(',')
        .map(|x| x.parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    let bus_wait_time = find_bus_id_and_wait_time(estimate, &bus_ids);
    println!("{}", bus_wait_time.bus_id * bus_wait_time.minutes);
}
