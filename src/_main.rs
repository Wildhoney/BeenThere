#[allow(dead_code)]

mod trip {
    use serde::Deserialize;
    #[derive(Deserialize, Debug)]
    pub struct Destination {
        name: DestinationName
    }

    #[derive(Deserialize, Debug)]
    pub struct DestinationName {
        common: String
    }

    #[derive(Debug)]
    pub struct Passenger {
        name: String,
        age: usize,
        discount: usize
    }

    impl Passenger {
        pub fn create(name: String, age: usize) -> Passenger {
            let discount = Self::discount_for_age(age);
            Passenger { name, age, discount }
        }

        fn discount_for_age(age: usize) -> usize {
            match age {
                _ if age < 2  => 100,
                _ if age < 12 => 10,
                _             => 0
            }
        }
    }

    impl Destination {
        fn price(destination: String) -> usize {
            0
        }
    }

    pub async fn price(destination: String, passengers: Vec<Passenger>) -> Option<usize> {
        let url = "https://restcountries.com/v3.1/all";
        let destinations = reqwest::get(url).await.unwrap().json::<Vec<Destination>>().await;
        println!("{:?}", destinations);

        let price_per_seat = Destination::price(destination);

        let price = passengers.into_iter().fold(0, |total, passenger| {
            let discount =  (price_per_seat / 100) * passenger.discount;
            total + (price_per_seat - discount)
        });

        Some(price)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adam   = trip::Passenger::create("Adam".to_string(), 36);
    let maria  = trip::Passenger::create("Maria".to_string(), 31);
    let imogen = trip::Passenger::create("Imogen".to_string(), 2);

    let passengers = vec![adam, maria, imogen];
    let price      = trip::price("Peru".to_string(), passengers).await.unwrap_or(0);

    println!("get_total_price: {:?}", price);
    println!("\\uD83C\\uDDE8\\uD83C\\uDDF1");
    Ok(())
}
