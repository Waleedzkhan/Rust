use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game");
    println!("Please Enter a Number");
    loop{
         let mut guess = String::new();

         io::stdin().read_line(&mut guess).expect("Nothing to read");
         let secret_number = rand::thread_rng().gen_range(1..101);
         let  guess:u32 = match guess.trim().parse(){
             Ok(num) => num,
             Err(_) => continue,
         };

         match guess.cmp(&secret_number){
             Ordering::Equal => println!("Wah!!, phate chak tay jay"),
             Ordering::Greater => println!("Yaar aap bay zyada bol diya"),
             Ordering::Less => println!("Aaap ghalat thay!, secret is say zyada hai")

     }
     println!("You guessed {}",guess);
     println!("Secret Number was {}",secret_number);
    }

     

}
