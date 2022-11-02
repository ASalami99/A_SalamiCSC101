fn main() {

    //find and replace
    let name1 = "Ayomide Adesokan";
    println!("His name is {}",name1);
    let name2 = name1.replace("Ayomide","Adebare");
    println!("You can also call him {}",name2);

    //find and replace  
    let faculty ="Faculty of Science and Technology";
    let school = faculty.replace("Faculty","School");
    println!("He is a student of the {}",school);
}
