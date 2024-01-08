use std::alloc::{alloc,dealloc,Layout};

#[derive(Clone)]
struct Bloc{    // Structure de Bloc regroupant : Adresse | Taille réelle + Layout de structure | Paramètre indiquant si la donnée est sollicitée
    addr: *mut u8,
    size : (usize,Layout),
    alloc : bool
}
impl Bloc{
    // Fonction d'appel pour construire un bloc mémoire, elle accepte un paramètre générique T
    unsafe fn new<T>(data:&T,size:usize)->Bloc{
        let lay = Layout::for_value(data);
        return Bloc{
            addr : alloc(lay),
            size : (size,lay),
            alloc : false
        }
    }

    // Fonction d'appel pour récupérer les binaires contenus dans le bloc
    fn read(&self)->Vec<u8>{
        let mut point = self.addr.clone();
        let s = self.size.0;
        let mut d = vec![0;s];
        for i in 0..self.size.0{
            unsafe{
                d[i] = point.read();
                point = point.add(1);
            }
        }
        return d
    }

    // Fonction d'appel pour écriture des binaires dans le bloc
    fn write(&mut self,data:&[u8]){
        let mut point = self.addr.clone();
        for i in 0..self.size.0{
            unsafe{
                point.write(data[i]);
                point = point.add(1);
            }
        }
        self.alloc = true;
    }

    // Fonction de libération du bloc, ce dernier sera transféré en cas d'appel de check_leaks depuis le MemManager
    fn rem(&mut self){
        self.alloc = false;
    }
}

// Struct du Gestionnaire de Mémoire, le Trait Clone est là pour assurer les échanges entre les Vec
#[derive(Clone)]
pub struct MemManager{  // Deux Vec d'usage 
    in_use : Vec<Bloc>, // => in_use contient tous les blocs utiles et les regroupe selon le principe dynamique du Vec => Pas de fragmentation théorique
    free : Vec<Bloc>    // => free récupère les blocs donc le paramètre est passé alloc en false(donc bloc libre), ils sont stockés et regroupés dans ce vec, disponibles au besoin
}impl MemManager{
    pub fn new()->MemManager{   // Conception d'un MemManager 
        return MemManager{
            in_use : vec![],
            free : vec![]
        }
    }

    // Fonction d'appel pour allocation d'une donnée primitive
    pub fn add(&mut self,data:&[u8],size:usize)->*mut u8{   
        let mut index = 0;
        let check = false;
        for bloc in self.free.iter_mut(){   // Vérification d'un bloc mémoire de taille identique disponible
            if size == bloc.size.0{
                self.in_use.push(bloc.clone());
                bloc.write(data.as_ref());
                break;
            }
            index +=1;
        }
        if check{   // Transfert du bloc mémoire libre et compatible si existant
            let addr = self.in_use.get(index).unwrap().addr;
            self.free.remove(index);
            return addr
        }
        unsafe{ // Conception du bloc mémoire si non existant
            let mut b = Bloc::new(&data, size);
            let addr = b.addr;
            b.write(data.as_ref());
            self.in_use.push(b);
            return addr
        }
    }

    // Fonction d'appel pour allocation d'une donnée String
    pub fn add_st<T:AsRef<[u8]>>(&mut self,data:T,size:usize)->*mut u8{  // 
        let mut index = 0;
        let check = false;
        for bloc in self.free.iter_mut(){   // Vérification d'un bloc mémoire de taille identique disponible
            if size == bloc.size.0{
                self.in_use.push(bloc.clone());
                bloc.write(data.as_ref());
                break;
            }
            index +=1;
        }
        if check{   // Transfert du bloc mémoire libre et compatible si existant
            let addr = self.in_use.get(index).unwrap().addr;
            self.free.remove(index);
            return addr
        }
        unsafe{ // Conception du bloc mémoire si non existant
            let mut b = Bloc::new(&data, size);
            let addr = b.addr;
            b.write(data.as_ref());
            self.in_use.push(b);
            return addr
        } 
    }

    // Fonction d'appel pour une récupération d'une donnée en mémoire
    pub fn take(&self,mark:*mut u8)->Result<Vec<u8>,()>{
        for bloc in self.in_use.iter(){
            if bloc.addr == mark{
                return Ok(bloc.read())
            }
        }
        return Err(())
    }

    // Fonction d'appel pour une mise en libération d'un bloc mémoire
    pub fn del(&mut self,mark:*mut u8){
        for bloc in self.in_use.iter_mut(){
            if bloc.addr == mark{
                bloc.rem();
            }
        }
    }

    // Fonction d'appel pour vérifier les blocs mémoire libérées dans le Vec in_use
    // Elle s'assure également si un transfert entre les deux vec est nécessaire (Gestion en cas de doublon)
    pub fn check_leaks(&mut self){
        let mut leaks:Vec<(usize,bool)> = vec![];
        let mut ind:u32 = 0;
        for bloc in self.in_use.iter_mut(){ // Recherche des blocs mis en libre
            if bloc.alloc == false{
                let status = check_garb(&self.free,bloc.addr);
                leaks.push((ind as usize,status));
            }
            ind += 1;
        }
        for v in leaks{ // Déplacement et suppression des blocs mémoire dans la vec d'usage
            if v.1{
                self.in_use.remove(v.0);
            }
            else{
                self.free.push(self.in_use.get(v.0).unwrap().clone());
                self.in_use.remove(v.0);
            }
        }
    }

    // Fonction d'appel pour vider les blocs mémoire non exploités
    pub  fn clear(&mut self){
        let mut clean:Vec<usize> = vec![];
        for bloc in self.free.iter(){
            if !bloc.alloc{ // Vérification en cas d'anomalie sur l'usage d'un bloc mémoire => Par sécurité le bloc classé en toujours utilisé ne sera pas vidé
                unsafe{
                    dealloc(bloc.addr, bloc.size.1);
                }
                
            }
        }
        clean.clear();
    }

    
}

// Fonction extérieur au MemManager permettant la vérification de doublon entre les deux vec
// Stocké en dehors pour des raisons d'ownership et d'un emprunt mutable qui ne peut excéder 1
fn check_garb(garb:&Vec<Bloc>,mark:*mut u8)->(bool){
    let mut ind:i32 = 0;
    for bloc in garb{
        if bloc.addr == mark{
            return true
        }
        ind+=1;
    }
    return false
}