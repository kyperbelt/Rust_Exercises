fn main() {
    let days : [&str;12] = ["first","second","third","fourth","fifth","sixth","seventh","eigth","ninth","tenth","eleventh","twelfth"];
    let items : [&str;12] = [
        "a Partridge and a Pear Tree",
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Lords a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming"
    ];
    
    println!("Concierto Kyper Presents 🎶 Twelve days of Chirstmas 🎶 \n");
    for i in 0..12{
        println!("On the {0} day of Christmas",days[i]);
        println!("my true love game to me");
        for j in (0..i+1).rev(){
            
            if i+1 == 1 && j == 0{ 
                let mut c = items[j].chars();
                match c.next(){
                    Some(f) => println!("{}",f.to_uppercase().collect::<String>() + c.as_str()),
                    _=> panic!("this was not supposed to happen")
                }
            }else if j == 0{
                println!("and {}",items[j])
            }else{
                println!("{}",items[j])
            }

        }
        println!("")
    }
}
