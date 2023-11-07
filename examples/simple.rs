use deinflect::Deinflections;

fn main() {
    let deinflections = Deinflections::from_word("聞かれました");

    // iterate over all possible deinflections
    for deinflection in deinflections.iter() {
        // get the deinflected word as a string
        let deinflected = deinflections.to_string(deinflection);
        println!("{}", deinflected);
    }
}
