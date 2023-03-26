struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
        return announcement;
    }
}
fn main() {
    let string1 = String::from("long string is long");

    let ex = ImportantExcerpt { part: &*string1 };
    let result;
    {
        let string2 = String::from("xyz");
        result = ex.announce_and_return_part(&string2) // error !
    }
    println!("The longest string is {}", result);
}
