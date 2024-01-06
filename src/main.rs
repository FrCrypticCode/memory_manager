mod mem;
use mem::MemoryManager;
mod typ;
use typ::deftype;
fn main() {
    // Test pour vérifier la viabilité du process
    let mut ges = MemoryManager::new();
    println!("Capacité du MemoryManager : {}",ges.in_use.capacity());
    println!("Taille du MemoryManager : {}",ges.in_use.len());
    println!("Capacité du Garbage : {}",ges.empty.capacity());
    println!("Taille du Garbage : {}",ges.empty.len());
    {
        let x = String::from("Ceci est un test, espèce de connasse de machine de merde.");
        let x = (x.as_bytes(),deftype(x.clone()));
        unsafe{
            let _p = ges.add(x);
        }
        let y:u16 = 1519;
        let y:(&[u8],(&str,usize)) =  (&y.to_ne_bytes(),(deftype(y)));
        unsafe{
            let _p = ges.add(y);
        }
    }
    for b in &mut ges.in_use{
        unsafe{
            if b.typ == "String" || b.typ == "char"{
                println!("Adresse allouée ({}) : {:?} || Valeur : {:?}",b.size.size(),b.address,char::from(b.address.read()));  // Implémenter la lecture de donnée
            }
            else{
                println!("Adresse allouée ({}) : {:?} || Valeur : {:?}",b.size.size(),b.address,b.address.read());
            }
            b.allocated = false;
        }
    }
    ges.check_leaks();
    println!("Capacité du Garbage : {}",ges.empty.capacity());
    println!("Taille du Garbage : {}",ges.empty.len());
    ges.clear();
    println!("Capacité du MemoryManager : {}",ges.in_use.capacity());
    println!("Taille du MemoryManager : {}",ges.in_use.len());
    println!("Capacité du Garbage : {}",ges.empty.capacity());
    println!("Taille du Garbage : {}",ges.empty.len());
    
}

// Implémenter la fonction read pour extraire la donnée stockée de son bloc