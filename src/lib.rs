pub struct Game {
    players: Vec<i32>, //B(I)
    assets: Vec<f64>, //A(I)

    price_threshhold: i32, //P9
    advertisement_price: f64, //S3
    s2: i32, // idk??? 
    initial_assets: f64, //A2
    c9: f64, // wtf :cry
    c2: i32,  // helpme
}
impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(), 
            assets: Vec::new(),
    
            price_threshhold: 10, //P9
            advertisement_price: 0.15, //S3
            s2: 30,
            initial_assets: 2.00, //A2
            c9: 0.5, // wtf :cry
            c2: 1,  // helpme
        }
    }
}