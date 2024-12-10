
fn dev1(){
    use regex::Regex;

    // We use 'unwrap()' here because it would be a bug in our program if the
    // pattern failed to compile to a regex. Panicking in the presence of a bug
    // is okay.
    let re = Regex::new(r"Homer (.)\. Simpson").unwrap();
    let hay = "Homer J. Simpson";
    let Some(caps) = re.captures(hay) else { return };

    let a = &caps[1];
    assert_eq!("J", a);



}

fn dev2() {
    use regex::Regex;
    let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
    let uno = re.find(hay).unwrap().as_str();
    let iterablethingy  = re.find_iter(hay);
    let dates: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
}

pub fn dev() {
    dev1();
    dev2();
}
