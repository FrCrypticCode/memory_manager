use std::alloc::{alloc,dealloc,Layout};

struct Bloc{
    address : *mut u8,
    size : Layout,
    allocated : bool
}
impl Bloc{
    unsafe fn new<T>(typ:T)->Bloc{
        let lay = Layout::new::<T>();
        return Bloc { address: alloc(lay), size: lay, allocated: true }
    }

    unsafe fn deallocate(&self){
        dealloc(self.address, self.size)
    }
}

struct MemoryManager{
    blocks : Vec<Bloc>
}
impl MemoryManager{
    fn new()->MemoryManager{
        return MemoryManager { blocks: vec![] }
    }
    unsafe fn add<Data>(&mut self,data:Data, size:usize)->*mut u8{  // Création/Sélection d'un bloc mémoire puis renvoit d'une adresse exploitable pour stockage
        for b in self.blocks.iter_mut(){
            if b.size.size() == size && b.allocated == false{
                b.allocated = true;
                return b.address
            }
        }
        let bloc = Bloc::new(data);
        let addr = bloc.address;
        self.blocks.push(bloc);
        return addr
    }

    fn rem<Data>(&mut self, data:*mut u8)->bool{    // Le Booléen sert à assurer que le bloc à supprimer est bien existant
        for b in self.blocks.iter_mut(){
            if b.address == data{
                b.allocated = false;
                return true
            }
        }
        return false
    }
}
fn main() {
}
