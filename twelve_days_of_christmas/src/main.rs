fn main() {

    let christmas_days_and_presents = [("first","A partridge in a pear tree",),
                                        ("second","Two turtle doves"),
                                        ("third","Three French hens"),
                                        ("fourth","Four calling birds"),
                                        ("fifth","Five golden rings"),
                                        ("sixth","Six geese a-laying"),
                                        ("seventh","Seven swans a-swimming"),
                                        ("eighth","Eight maids a-milking"),
                                        ("ninth","Nine ladies dancing"),
                                        ("tenth","Ten lords a-leaping"),
                                        ("eleventh","Eleven pipers piping"),
                                        ("twelfth","Twelve drummers drumming")];

    let mut index = 0;

    for day in christmas_days_and_presents.iter() {

        println!("On the {} day of Christmas,",day.0);
        println!("my true love sent to me");
        if index == 0 {
            println!("{}.",day.1);
        } else {
            for present in (0..index).rev() {
                match present {
                    0 => println!("And {}\n\n",christmas_days_and_presents[present].1),
                    _ => println!("{},",christmas_days_and_presents[present+1].1),
                    };
                }
            }
            
        index += 1;
        }
    }
