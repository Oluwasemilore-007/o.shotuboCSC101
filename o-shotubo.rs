use std::io;

fn main(){
   
    let mut counter = 0;

    while counter < 100 {

    let mut name = String::new();
    let mut day = String::new();
    let mut month = String::new();
    let mut year = String::new();
    let mut email = String::new();
    let mut phone = String::new();
    let mut siblings = String::new();
    let mut children = String::new();
    let mut medical = String::new();
    let mut residence = String::new();

    println!("ENTER YOUR INFORMATION AND ENSURE TO ANSWER EVERY QUESTION ");

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    println!("Enter your day of birth: ");
    io::stdin().read_line(&mut day).expect("Failed to read line");
    let day:u32 = day.trim().parse().expect("Not a valid year");

    println!("Enter your  month of birth:  ");
    io::stdin().read_line(&mut month).expect("Failed to read line");
    let month:u32 = month.trim().parse().expect("Not a valid year");

    println!("Enter your year of birth: ");
    io::stdin().read_line(&mut year).expect("Failed to read line");
    let year:u32 = year.trim().parse().expect("Not a valid year");

    println!("Enter your email: ");
    io::stdin().read_line(&mut email).expect("Failed to read line");
    let email = email.trim();

    println!("Enter your phone number: ");
    io::stdin().read_line(&mut phone).expect("Failed to read line");
    let phone:u64 = phone.trim().parse().expect("Not a valid phone number");

    println!("Enter the number of siblings you have: ");
    io::stdin().read_line(&mut siblings).expect("Failed to read line");
    let siblings:u32 = siblings.trim().parse().expect("Not a valid number");

    println!("Enter the number of children you have: ");
    io::stdin().read_line(&mut children).expect("Failed to read line");
    let children:u32 = children.trim().parse().expect("Not a valid number");
    
    println!("Please enter one of the following:\n(A)Alzheimer,\n(B)Arrythmia,\n(C)CKD,\n(D)Diabetes,\n(E)Arthritis");
    println!("Enter the number of medical conditions you have: ");
    io::stdin().read_line(&mut medical).expect("Failed to read line");
    let medical = medical.trim();

    println!("Please Select one of the following:\n(A)Akpabom,\n(B)Ngbauji,\n(C)Atabrikang,\n(D)Okorobilom,\n(E)Emeremen");
    println!("What is your village of residence");
    io::stdin().read_line(&mut residence).expect("Failed to read line");
    let residence = residence.trim();

    let current_year:u32 = 2024;
    let age = current_year - year;



     if medical == "A" && age > 50 && children >= 4 && residence == "A" {
        println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
        println!("The TOTAL Amount of your treatment plus our medical discount is: ₦960,000");
    }
    else if medical == "B" && age == 30 && siblings >= 4 && residence == "B" {
        println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
        println!("The TOTAL Amount of your treatment plus our medical discount is:₦527,250 ");
    }
   else if medical == "C" && age >= 40 && children >= 3 &&siblings >= 3 && residence == "C" {
    println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
    println!("The TOTAL Amount of your treatment plus our medical discount is : ₦1,275,000");
    
   }
   else if medical == "D" && age > 28 && age < 45 && children >= 2 && children <= 4 && residence == "D" {
    println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
    println!(" The Total Amount of your treatment plus our medical discount is :₦720,000");
    
   }
   else if medical == "E" && age > 28 && age < 45 && children >= 2 && children <= 4 && residence == "E" {
    println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
    println!("The Total Amount of your treatment plus our medical discount is :₦405,000");
    
   }else{
    println!(" This is your Medical Information: \nName:{},\nDate of birth:{}/{}/{},\nEmail Address:{},\nPhone number:{}\nno of siblings:{},\nno of children:{},\nMedical diagnosis:{},\nVillage of residence:{}", name, day, month, year, email, phone, siblings, children, medical, residence);
    println!("Since you your are not looking entitled to the Medical discount. The Total Amount for your treatment will be listed below.\nAlzheimer: ₦1,200,000. \nArrhythmia:₦550,000. \nChronic Kidney Diseases:₦1,500,000. \nDiabetes:₦800,000. \nArthritis:₦450,000.\n Sorry for the inconvenience");
}
   
   counter += 1;

}
println!("The first 100 people of today have been reached discount benefits don't apply again");
}
    