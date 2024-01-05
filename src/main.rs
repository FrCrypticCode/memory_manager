use std::alloc::{alloc,dealloc,Layout};
#[derive(Copy,Clone)]
struct Bloc{
    address : *mut u8,
    size : Layout,
    allocated : bool
}
impl Bloc{
    unsafe fn new<T>(_typ:T)->Bloc{
        let lay = Layout::new::<T>();
        return Bloc { address: alloc(lay), size: lay, allocated: true }
    }

    unsafe fn write(&self, data:&[u8]){
        let mut point = self.address.clone();
        for d in data{
            point.write(*d);
            point = point.add(1);
        }
    }

    unsafe fn deallocate(&mut self){
        self.allocated = false;
        dealloc(self.address, self.size)
    }
}

#[derive(Clone)]
struct MemoryManager{
    in_use : Vec<Bloc>,
    empty : Vec<Bloc>
}
impl MemoryManager{
    fn new()->MemoryManager{
        return MemoryManager { in_use: vec![], empty: vec![] }
    }
    unsafe fn add(&mut self,data:&[u8], size:usize)->*mut u8{  // Création/Sélection d'un bloc mémoire puis renvoit d'une adresse exploitable pour stockage
        for b in self.empty.iter_mut(){
            if b.size.size() == size && b.allocated == false{
                b.allocated = true;
                b.write(data);
                self.in_use.push(*b);
                return b.address
            }
        }
        let bloc = Bloc::new(data);
        let addr = bloc.address;
        self.in_use.push(bloc);
        return addr
    }

    

    fn rem<Data>(&mut self, data:*mut u8){    // Passage du bloc en libre via le allocated = false
        for b in self.in_use.iter_mut(){
            if b.address == data{
                b.allocated = false;
            }
        }
    }

    fn check_leaks(&mut self){  // Vérification des blocs non exploités et transfert vers le empty
        for b in self.in_use.iter_mut(){
            if !b.allocated{
                println!("{:?} n'est pas utilisée.",b.address);
                self.empty.push(*b);
            }
        }
    }

    fn clear(&mut self){    // Vidage des blocs non exploités dans le empty + Vérification du in_use
        self.in_use.retain(|i| i.allocated==true);
        for b in self.empty.iter_mut(){
            unsafe{
                b.deallocate();
            }
        }
        self.empty.clear();
    }
}
fn main() {
    // Test pour vérifier la viabilité du process
    let mut ges = MemoryManager::new();
    println!("Capacité du MemoryManager : {}",ges.in_use.capacity());
    println!("Taille du MemoryManager : {}",ges.in_use.len());
    println!("Capacité du Garbage : {}",ges.empty.capacity());
    println!("Taille du Garbage : {}",ges.empty.len());
    {
        let x = String::from("Test");
        let size = x.len();
        unsafe{
            let p = ges.add(x.as_bytes(), size);
            *p = b'C';
        }
        let y:u16 = 1519;
        let size = 16;
        unsafe{
            ges.add(&y.to_be_bytes(), size);
        }
    }
    for b in &mut ges.in_use{
        unsafe{
            println!("Adresse allouée ({}) : {:?} || Valeur : {:?}",b.size.size(),b.address,b.address.read());  // Implémenter la lecture de donnée
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


// Implémenter le transfert entre Vec
// Modifier le MemoryManager pour compacter les emplacements mémoire vides et non vides