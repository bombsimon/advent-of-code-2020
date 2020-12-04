use regex::Regex;

use crate::input;

#[derive(Clone, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

pub fn solve() {
    let x = input::file_for_day(4);
    let l = parse_file(x);

    println!("Solution part 1: {}", part_one(l.clone()));
    println!("Solution part 2: {}", part_two(l.clone()));
}

fn parse_file(vec: Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    // Make each block of text separated by double newline a single vector element no matter how
    // many lines it spans over.
    let groups: Vec<String> = vec
        .join("\n")
        .split("\n\n")
        .map(|x| x.trim().replace("\n", " ").to_string())
        .collect();

    for group in groups {
        let mut passport = Passport::new();

        for pair in group.split(" ") {
            let kv: Vec<&str> = pair.split(":").collect();
            let (k, v) = (kv[0], kv[1]);

            match k {
                "byr" => passport.byr = Some(v.to_string()),
                "iyr" => passport.iyr = Some(v.to_string()),
                "eyr" => passport.eyr = Some(v.to_string()),
                "hgt" => passport.hgt = Some(v.to_string()),
                "hcl" => passport.hcl = Some(v.to_string()),
                "ecl" => passport.ecl = Some(v.to_string()),
                "pid" => passport.pid = Some(v.to_string()),
                "cid" => passport.cid = Some(v.to_string()),
                x => panic!(format!("Unexpected key: {}", x)),
            }
        }

        passports.push(passport);
    }

    passports
}

/// With strict mode each segment of the passport must have a valid value. Without strict mode the
/// password is to be seen as valid as long as all required fields are present.
fn valid_passport(p: Passport, strict: bool) -> bool {
    if let Passport {
        byr: Some(ref byr),
        iyr: Some(ref iyr),
        eyr: Some(ref eyr),
        hgt: Some(ref hgt),
        hcl: Some(ref hcl),
        ecl: Some(ref ecl),
        pid: Some(ref pid),
        ..
    } = p
    {
        if !strict {
            return true;
        }

        // Check if the integer value of a string is able to parse into i32 and between a given
        // range.
        let between = |num: &String, low: i32, high: i32| -> bool {
            match num.parse::<i32>() {
                Ok(x) => {
                    if x < low || x > high {
                        return false;
                    }
                }
                _ => return false,
            }
            true
        };

        if !between(byr, 1920, 2002) {
            return false;
        }

        if !between(iyr, 2010, 2020) {
            return false;
        }

        if !between(eyr, 2020, 2030) {
            return false;
        }

        let hgt_re = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
        let captures = hgt_re.captures(hgt.as_str()).unwrap();
        if captures.len() != 3 {
            return false;
        }

        let height = &captures[1];
        let unit = &captures[2];

        match unit {
            "cm" => {
                if !between(&height.to_string(), 150, 193) {
                    return false;
                }
            }
            "in" => {
                if !between(&height.to_string(), 59, 76) {
                    return false;
                }
            }
            x => panic!(format!("Unexpected unit: {}", x)),
        }

        let hcl_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        if !hcl_re.is_match(hcl.as_str()) {
            return false;
        }

        let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !eye_colors.contains(&ecl.as_str()) {
            return false;
        }

        if pid.len() != 9 || !pid.chars().all(char::is_numeric) {
            return false;
        }

        return true;
    }

    false
}

fn part_one(vec: Vec<Passport>) -> i64 {
    let mut valid_passports = 0i64;

    for passport in vec {
        if valid_passport(passport, false) {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn part_two(vec: Vec<Passport>) -> i64 {
    let mut valid_passports = 0i64;

    for passport in vec {
        if valid_passport(passport, true) {
            valid_passports += 1;
        }
    }

    valid_passports
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_ONE: &str = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
    static TEST_INPUT_TWO: &str = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
    static SOLUTION_ONE: i64 = 2;
    static SOLUTION_TWO: i64 = 4;

    #[test]
    fn part_one() {
        let x = input::string_to_vec(TEST_INPUT_ONE, true);
        let map = parse_file(x);

        assert_eq!(super::part_one(map), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::string_to_vec(TEST_INPUT_TWO, true);
        let map = parse_file(x);

        assert_eq!(super::part_two(map), SOLUTION_TWO);
    }
}
