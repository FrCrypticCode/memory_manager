use std::alloc::{alloc,dealloc,Layout};
use crate::typ::deftype;

#[derive(Copy,Clone)]
pub struct Bloc{
    pub address : *mut u8,
    pub size : Layout,
    pub allocated : bool
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
pub struct MemoryManager{
    pub in_use : Vec<Bloc>,
    pub empty : Vec<Bloc>
}
impl MemoryManager{
    pub fn new()->MemoryManager{
        return MemoryManager { in_use: vec![], empty: vec![] }
    }
    pub unsafe fn add<T:AsRef<[u8]>+Clone>(&mut self,data:T, size:usize)->*mut u8{  // Création/Sélection d'un bloc mémoire puis renvoit d'une adresse exploitable pour stockage
        let (t,s) = deftype(data.to_owned());
        let bytes:&[u8];
        {
            let clone = data.to_owned();   // Extraction temp des données de data sans altérer data
            bytes = clone.as_ref();
        }

        println!("{} : {}",t,s);
        for b in self.empty.iter_mut(){
            if b.size.size() == size && b.allocated == false{
                b.allocated = true;
                b.write(bytes);  // Gérer le Data
                self.in_use.push(*b);
                return b.address
            }
        }
        let bloc = Bloc::new(data);
        bloc.write(bytes);   // Gérer la Data
        let addr = bloc.address;
        self.in_use.push(bloc);
        return addr
    }

    pub fn rem<Data>(&mut self, data:*mut u8){    // Passage du bloc en libre via le allocated = false
        for b in self.in_use.iter_mut(){
            if b.address == data{
                b.allocated = false;
            }
        }
    }

    pub fn check_leaks(&mut self){  // Vérification des blocs non exploités et transfert vers le empty
        for b in self.in_use.iter_mut(){
            if !b.allocated{
                println!("{:?} n'est pas utilisée.",b.address);
                self.empty.push(*b);
            }
        }
    }

    pub fn clear(&mut self){    // Vidage des blocs non exploités dans le empty + Vérification du in_use
        self.in_use.retain(|i| i.allocated==true);
        for b in self.empty.iter_mut(){
            unsafe{
                b.deallocate();
            }
        }
        self.empty.clear();
    }
}