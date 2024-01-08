mod mem;
mod typ;
use mem::MemManager;
use typ::refine;

// La main fonction ne sert qu'aux tests !!!
fn main(){  
    let mut manager = MemManager::new();
    let mut p:*mut u8;
    {
        let string = String::from("Test 1 2 3 4 blablablablablablablabla type stréng de ouf&");
        let size = string.as_bytes().len();
        p = manager.add_st(string, size);
    }
    println!("Adresse : {:?}",p);
    let res = manager.take(p);
    if res.is_ok(){
        println!("{}",String::from_utf8_lossy(&res.unwrap()));
    }
    else{
        println!("Votre adresse est inconnue");
    }

    {
        let number:u16= 512;
        let size = 2;
        p = manager.add(&number.to_ne_bytes() as &[u8], size);
    }
    println!("Adresse : {:?}",p);
    let res = manager.take(p);
    if res.is_ok(){
        println!("{}",refine::<u16>(res.unwrap()));
    }
    else{
        println!("Votre adresse est inconnue");
    }
}


// Concevoir une fonction d'adaptation aux données