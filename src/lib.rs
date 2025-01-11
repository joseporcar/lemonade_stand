use std::{io::stdin, thread, time::Duration};
pub struct Game {
    buff: String, // not from source code

    players: [i32;30], //B(I)
    assets: [f64;30], //A(I)

    price_threshhold: i32, //P9
    advertisement_price: f64, //S3
    s2: i32, // idk??? 
    initial_assets: f64, //A2
    c9: f64, // wtf :cry
    c2: i32,  // helpme
    day:i32, //D
    weather: Weather,
}
impl Game {
    pub fn new() -> Game {
        Game {
            buff: String::new(),
            
            players: [-1; 30], 
            assets: [-1.; 30],
    
            price_threshhold: 10, 
            advertisement_price: 0.15, 
            s2: 30,
            initial_assets: 2.00, 
            c9: 0.5, 
            c2: 1,  
            day: 0,
            weather: Weather::Sunny,
        }
    }
    
    pub fn start(&mut self) {

        Self::print_intro();
        // here would go the resume mechanic prompt as defined per gosub 12000
        let people_playing = Self::get_people_playing(&mut self.buff); //N
    
        for i in 0..people_playing {
            self.players[i] = 0;
            self.assets[i] = self.initial_assets;
        }
        // here would go resume mechanic logic 14000

        Self::new_business();
    }

    pub fn weather_report(&mut self) {
        self.get_weather();
        self.weather.print();
    }

    fn get_weather(&mut self) {
        let random_number = rand::random::<f32>();
        if random_number < 0.6 || self.day < 3 {
            self.weather = Weather::Sunny;
        } else if random_number < 0.8 {
            self.weather = Weather::Cloudy;
        } else {
            self.weather = Weather::Dry;
        }
    }

    fn get_people_playing(buff: &mut String) -> usize {
        println!("HOW MANY PEOPLE WILL BE PLAYING?");
        stdin().read_line(buff).unwrap();
        
        buff.trim().parse().unwrap()
    }
    
    fn print_intro() {
        println!("
        HI!  WELCOME TO LEMONSVILLE, CALIFORNIA!
        IN THIS SMALL TOWN, YOU ARE IN CHARGE OF
        RUNNING YOUR OWN LEMONADE STAND. YOU CAN
        COMPETE WITH AS MANY OTHER PEOPLE AS YOU
        WISH, BUT HOW MUCH PROFIT YOU MAKE IS UP
        TO YOU (THE OTHER STANDS' SALES WILL NOT
        AFFECT YOUR BUSINESS IN ANY WAY). IF YOU
        MAKE THE MOST MONEY, YOU'RE THE WINNER!!\n");
        thread::sleep(Duration::from_secs(2))
    }
    
    fn new_business() {
        println!("
            TO MANAGE YOUR LEMONADE STAND, YOU WILL 
            NEED TO MAKE THESE DECISIONS EVERY DAY: 
            1. HOW MANY GLASSES OF LEMONADE TO MAKE    (ONLY ONE BATCH IS MADE EACH MORNING)
            2. HOW MANY ADVERTISING SIGNS TO MAKE      (THE SIGNS COST FIFTEEN CENTS EACH)  
            3. WHAT PRICE TO CHARGE FOR EACH GLASS  
            YOU WILL BEGIN WITH $2.00 CASH (ASSETS).
            BECAUSE YOUR MOTHER GAVE YOU SOME SUGAR,
            YOUR COST TO MAKE LEMONADE IS TWO CENTS 
            A GLASS (THIS MAY CHANGE IN THE FUTURE).\n"); 
        
        println!("press enter to continue");
        stdin().read_line(&mut String::new()).unwrap();
        println!("
            YOUR EXPENSES ARE THE SUM OF THE COST OF
            THE LEMONADE AND THE COST OF THE SIGNS. 
            YOUR PROFITS ARE THE DIFFERENCE BETWEEN 
            THE INCOME FROM SALES AND YOUR EXPENSES.
            THE NUMBER OF GLASSES YOU SELL EACH DAY 
            DEPENDS ON THE PRICE YOU CHARGE, AND ON 
            THE NUMBER OF ADVERTISING SIGNS YOU USE. 
            KEEP TRACK OF YOUR ASSETS, BECAUSE YOU  
            CAN'T SPEND MORE MONEY THAN YOU HAVE!\n\n\n");
        println!("press enter to continue");
        stdin().read_line(&mut String::new()).unwrap();
    }
    
}

enum Weather {
    Sunny, //SC=2
    Dry, //SC=7
    Cloudy, //SC=10
    Thunderstorm, //SC=5
}
impl Weather {
    fn print(&self) {
        println!("LEMONSVILLE WEATHER REPORT ");
        match self {
            Weather::Sunny => println!("SUNNY"),
            Weather::Dry => println!("HOT AND DRY"),
            Weather::Cloudy => println!("CLOUDY"),
            Weather::Thunderstorm => println!("THUNDERSTORMS!"),
        }
        println!("\n\n");
        thread::sleep(Duration::from_secs(1));
    }
}