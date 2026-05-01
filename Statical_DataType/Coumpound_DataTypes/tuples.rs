fn main(){
    let person : (i32,f64,&str) = (21,175.5,"Nara");
    println!("person : {:?}",person);
    // default is (i32,f64,&str)
    let a = (21,175.5,"Nara");
    println!("a : {:?}",a);
    let mix_tuple = (1,"Nara",true,3.14,[10,20,30]);
    println!("mix_tuple : {:?}",mix_tuple);
    println!("mix_tuple : {}",mix_tuple.0);
    println!("mix_tuple : {}",mix_tuple.1);
    println!("mix_tuple : {}",mix_tuple.2);
    println!("mix_tuple : {}",mix_tuple.3);
    println!("mix_tuple : {:?}",mix_tuple.4);
    let (age,height,name) = person;
    println!("age : {}",age);
    println!("height : {}",height);
    println!("name : {}",name);
}