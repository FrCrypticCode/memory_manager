use std::alloc::{alloc,dealloc,Layout};
use crate::typ::deftype;
#[derive(Copy,Clone)]
pub struct Bloc<'t>{
    pub address : *mut u8,
    pub size : Layout,
    pub allocated : bool,
    pub typ : &'t str
}
impl Bloc<'_>{
    unsafe fn new<T>(typ:T)->Bloc<'static>{
        let lay = Layout::new::<T>();
        return Bloc { address: alloc(lay), size: lay, allocated: true, typ:deftype(typ).0 }
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
    pub in_use : Vec<Bloc<'static>>,
    pub empty : Vec<Bloc<'static>>
}
impl MemoryManager{
    pub fn new()->MemoryManager{
        return MemoryManager { in_use: vec![], empty: vec![] }
    }
    pub unsafe fn add(&mut self,data:(&[u8],(&str,usize)))->*mut u8{  // Création/Sélection d'un bloc mémoire puis renvoit d'une adresse exploitable pour stockage
        let (data,(typ,size)) = data;
        match typ{  // Gérer le type Integer et Char
            "u8"=>{
                let typ:u8 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            "u16"=>{
                let typ:u16 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            "u32"=>{
                let typ:u32 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            }
            "i8"=>{
                let typ:u8 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            "i16"=>{
                let typ:u16 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            "i32"=>{
                let typ:u32 = 0;
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            "char"=>{
                let typ:char = '0';
                let addr = self.seek_mem(data, typ, size);
                return addr
            }
            "String"=>{
                let typ:String = String::new();
                let addr = self.seek_mem(data, typ, size);
                return addr
            },
            _=>{panic!("Type non pris en charge")}
        }
    }

    unsafe fn seek_mem<T>(&mut self,data:&[u8],typ:T,size:usize)->*mut u8{
        for b in self.empty.iter_mut(){
            if b.size.size() == size && b.allocated == false{
                b.allocated = true;
                b.write(data);  // Gérer le Data
                self.in_use.push(*b);
                return b.address
            }
        }
        let bloc = Bloc::new(typ);
        bloc.write(data);   // Gérer la Data
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