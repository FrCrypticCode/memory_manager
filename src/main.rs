mod mem;
use mem::MemoryManager;
mod typ;
fn main() {
    // Test pour vérifier la viabilité du process
    let mut ges = MemoryManager::new();
    println!("Capacité du MemoryManager : {}",ges.in_use.capacity());
    println!("Taille du MemoryManager : {}",ges.in_use.len());
    println!("Capacité du Garbage : {}",ges.empty.capacity());
    println!("Taille du Garbage : {}",ges.empty.len());
    {
        let x = String::from("Ceci est un test, espèce de connasse de machine de merde.");
        let size = x.len();
        unsafe{
            let _p = ges.add(x, size);
        }
        let y = 1519;
        let size = 16;
        unsafe{
            let _p = ges.add(y, size);
        }
    }
    for b in &mut ges.in_use{
        unsafe{
            println!("Adresse allouée ({}) : {:?} || Valeur : {:?}",b.size.size(),b.address,char::from(b.address.read()));  // Implémenter la lecture de donnée
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

// Modifier le MemoryManager pour compacter les emplacements mémoire vides et non vides