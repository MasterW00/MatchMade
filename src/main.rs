use std::io;
use std::io::Write;
fn main() {
  display_welcome();
  let mut page: fn() = menu_page;
  loop{
    page();
    let selection = fancy_io("Choose a page");
    page = change_page(selection);
  }
}

fn display_welcome(){
  println!("
  ___        __        _                                           
 \\\\  \\      / / ___  || |  ___  ___  _ __  ___    ___     
  \\\\  \\ /\\ / /   ,,\\ || |/  __ / _ \\||  _ ` _ \\  / ,, \\ 
   \\\\  V  V /    __/ || |  |__| (_) || ||| ||| |    __/ 
    \\\\__/\\_/  \\\\___| \\|_|\\\\___\\\\___/\\|_|\\|_|\\|_| \\\\___|   
                                      _         
                                    || |__  __   
                                    || __/  _ \\ 
                                    || |   (_) | 
                                    \\\\__\\ \\___/
        ------------------------------------
            Match Maker OS 
                        -Jared Farnbach
        ------------------------------------
");
}

fn menu_page(){
  println!("
  ====================================
        Find your match! 
                who could it be??
  ====================================
    Disclaimer: not a love potion
  ------------------------------------
  ");
  println!("type help for options");
  
}

fn help_page(){
  println!("

  Help page
  \"home\": access the homepage
  \"help\": the page you are on, noob. (respectfully)
  \"match\": start the basic match maker process
  \"quit\": Quit the program
  ");
}

fn make_a_match(){
  println!("
  
  ------------------------------------------------------------
                            Begin!
  ------------------------------------------------------------
  Lets make a new Match!
  Enter each persons Name, Birthday(Month, day, year), and Gender 
  and the program will do the rest!
  
  ");
  println!("----First person----");
  let person1 = Person::input_person();
  println!("----Second person----");
  let person2 = Person::input_person();
  let sign1 = person1.get_astrological_sign().unwrap();
  let sign2 = person2.get_astrological_sign().unwrap();
  let compatibility_score = Person::compatibility(
    &sign1,
    &sign2,
    person1.gender,
    person2.gender,
    &person1.name,
    &person2.name,
  );
  println!("
  =============================
    {}
          &         : {}
              {} 
  =============================
  ",
  person1.name,compatibility_score,person2.name)
}

fn change_page(input:String) -> fn(){
  match input.as_str(){
    "home" =>{menu_page},
     "help" =>{help_page},
     "match"=>{make_a_match}
     "quit" =>{std::process::exit(0)}
     &_ =>{println!("not an option");help_page}
  }
}

fn fancy_io(message:&str) -> String{
  println!("{}",message);
  print!(">");
  io::stdout().flush().unwrap();
  let mut input: String = String::new();
  match io::stdin()
    .read_line(&mut input) {
      Ok(_)=>{input.trim().to_string()},
      Err(_)=>{String::new()},
  }
}

struct Person {
  name: String,
  month: u32,
  day: u32,
  year: u32,
  gender: bool,
}

impl Person {
  fn new(name: String, birthday: String, gender: String) -> Result<Person, String> {
      let (year, month, day) = Person::parse_date(&birthday)?;
      Ok(Person {
          name,
          month,
          day,
          year,
          gender: Person::parse_gender(&gender),
      })
  }

  fn input_person() -> Person {
    loop{
      let name = fancy_io("Enter the name");
      let birthday = fancy_io("Enter the birthday");
      let gender = fancy_io("Enter the gender");
      match Person::new(name, birthday, gender){
        Ok(person) => break person,
        Err(message) => print!("{}",message),
      }
    }
  }

  fn parse_date(input: &str) -> Result<(u32, u32, u32), String> {
      let parts: Vec<&str> = if input.contains('-') {
          input.split('-').collect()
      } else if input.contains('/') {
          input.split('/').collect()
      } else {
          input.split(' ').collect()
      };

      if parts.len() == 3 {
          let year = parts[2].parse::<u32>().map_err(|_| "Invalid year".to_string())?;
          let month = match parts[0].parse::<u32>() {
              Ok(num) => num,
              Err(_) => Person::month_u32(parts[0])?,
          };
          let day = parts[1].parse::<u32>().map_err(|_| "Invalid day".to_string())?;
          Ok((year, month, day))
      } else {
          Err("Invalid date format: expected three parts".to_string())
      }
  }

  fn parse_gender(input: &str) -> bool {
      match input.to_lowercase().as_str() {
          "boy" | "male" | "man" => true,
          "girl" | "female" | "woman" => false,
          _ => true, // Default to true for unrecognized input
      }
  }

  fn month_u32(month: &str) -> Result<u32, String> {
      match month.to_lowercase().as_str() {
          "january" | "jan" => Ok(1),
          "february" | "feb" => Ok(2),
          "march" | "mar" => Ok(3),
          "april" | "apr" => Ok(4),
          "may" => Ok(5),
          "june" | "jun" => Ok(6),
          "july" | "jul" => Ok(7),
          "august" | "aug" => Ok(8),
          "september" | "sep" | "sept" => Ok(9),
          "october" | "oct" => Ok(10),
          "november" | "nov" => Ok(11),
          "december" | "dec" => Ok(12),
          _ => Err(format!("Unrecognized month: {}", month)),
      }
  }

  fn get_astrological_sign(&self) -> Result<String, String> {
    Person::astrological_sign(&self.month, &self.day)
  }

  fn astrological_sign(month: &u32, day: &u32) -> Result<String, String> {
    match (month, day) {
        (1, 1..=19) => Ok("Capricorn".to_string()),
        (1, 20..=31) => Ok("Aquarius".to_string()),
        (2, 1..=18) => Ok("Aquarius".to_string()),
        (2, 19..=29) => Ok("Pisces".to_string()),
        (3, 1..=20) => Ok("Pisces".to_string()),
        (3, 21..=31) => Ok("Aries".to_string()),
        (4, 1..=19) => Ok("Aries".to_string()),
        (4, 20..=30) => Ok("Taurus".to_string()),
        (5, 1..=20) => Ok("Taurus".to_string()),
        (5, 21..=31) => Ok("Gemini".to_string()),
        (6, 1..=20) => Ok("Gemini".to_string()),
        (6, 21..=30) => Ok("Cancer".to_string()),
        (7, 1..=22) => Ok("Cancer".to_string()),
        (7, 23..=31) => Ok("Leo".to_string()),
        (8, 1..=22) => Ok("Leo".to_string()),
        (8, 23..=31) => Ok("Virgo".to_string()),
        (9, 1..=22) => Ok("Virgo".to_string()),
        (9, 23..=30) => Ok("Libra".to_string()),
        (10, 1..=22) => Ok("Libra".to_string()),
        (10, 23..=31) => Ok("Scorpio".to_string()),
        (11, 1..=21) => Ok("Scorpio".to_string()),
        (11, 22..=30) => Ok("Sagittarius".to_string()),
        (12, 1..=21) => Ok("Sagittarius".to_string()),
        (12, 22..=31) => Ok("Capricorn".to_string()),
        _ => Err("Invalid date".to_string()), // Handle invalid dates
    }
  }

  fn compatibility(
      sign1: &str,
      sign2: &str,
      gender1: bool,
      gender2: bool,
      name1: &str,
      name2: &str,
  ) -> i32 {
      let mut score: i32 = 50; // Start with a base compatibility score of 50%

      // Rule 1: Astrological sign compatibility
      score += match (sign1, sign2) {
        // Highly compatible pairs
        ("Aries", "Leo") | ("Leo", "Aries") => 30,
        ("Taurus", "Virgo") | ("Virgo", "Taurus") => 30,
        ("Gemini", "Libra") | ("Libra", "Gemini") => 30,
        ("Cancer", "Scorpio") | ("Scorpio", "Cancer") => 30,
        ("Sagittarius", "Aries") | ("Aries", "Sagittarius") => 30,
        ("Pisces", "Cancer") | ("Cancer", "Pisces") => 30,

        // Moderately compatible pairs
        ("Aries", "Sagittarius") | ("Sagittarius", "Aries") => 15,
        ("Taurus", "Capricorn") | ("Capricorn", "Taurus") => 15,
        ("Gemini", "Aquarius") | ("Aquarius", "Gemini") => 15,
        ("Cancer", "Pisces") | ("Pisces", "Cancer") => 15,
        ("Leo", "Sagittarius") | ("Sagittarius", "Leo") => 15,
        ("Virgo", "Capricorn") | ("Capricorn", "Virgo") => 15,
        ("Libra", "Aquarius") | ("Aquarius", "Libra") => 15,
        ("Scorpio", "Pisces") | ("Pisces", "Scorpio") => 15,

        // Neutral pairs
        ("Aries", "Gemini") | ("Gemini", "Aries") => 10,
        ("Taurus", "Cancer") | ("Cancer", "Taurus") => 10,
        ("Leo", "Libra") | ("Libra", "Leo") => 10,
        ("Virgo", "Scorpio") | ("Scorpio", "Virgo") => 10,
        ("Sagittarius", "Aquarius") | ("Aquarius", "Sagittarius") => 10,
        ("Capricorn", "Pisces") | ("Pisces", "Capricorn") => 10,

        // Incompatible pairs
        ("Aries", "Cancer") | ("Cancer", "Aries") => -10,
        ("Taurus", "Leo") | ("Leo", "Taurus") => -10,
        ("Gemini", "Virgo") | ("Virgo", "Gemini") => -10,
        ("Libra", "Capricorn") | ("Capricorn", "Libra") => -10,
        ("Scorpio", "Aquarius") | ("Aquarius", "Scorpio") => -10,
        ("Sagittarius", "Pisces") | ("Pisces", "Sagittarius") => -10,

        // Default case: no bonus or penalty
        _ => 0,
      };

      // Rule 2: Gender compatibility
      if gender1 != gender2 {
          score += 10; // Add 10% if genders are different
      } else {
          score -= 5; // Subtract 5% if genders are the same
      }

      // Rule 3: First letter of names
      let first_letter1 = name1.chars().next().unwrap_or('A').to_ascii_lowercase();
      let first_letter2 = name2.chars().next().unwrap_or('A').to_ascii_lowercase();

      if first_letter1 == first_letter2 {
          score += 5; // Add 5% if the first letters match
      } else if (first_letter1 as i32 - first_letter2 as i32).abs() <= 2 {
          score += 3; // Add 3% if the first letters are close alphabetically
      } else {
          score -= 5; // Subtract 5% if the first letters are far apart
      }

      // Ensure the score is within the range of 0 to 100
      if score > 100 {
          score = 100;
      } else if score < 0 {
          score = 0;
      }

      score
  }
}