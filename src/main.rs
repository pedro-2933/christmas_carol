fn main() {
    println!("Hello, world!");
    let days = [
        "twelfth", "eleventh", "tenth", "ninth", "eighth", "seventh", "sixth", "fifth", "fourth",
        "third", "second", "first",
    ];

    let repeats = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french horns",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    for n in (0..12).rev() {
        let day = days[n];

        println!("On the {day} day of Christmas, my\ntrue love sent to me");

        for i in n..12 {
            let line = repeats[i];

            println!("{line}");
        }
        println!();
    }
}
