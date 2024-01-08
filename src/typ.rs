// Fonction d'appel requise lors de la lecture d'un bloc de données, il est essentiel de préciser le type en paramètre
pub fn refine<T>(data:Vec<u8>)->T where T:FromBytes{
    let value = T::from_bytes(data);
    return value
}
pub trait FromBytes{
    fn from_bytes(bytes:Vec<u8>)->Self;
}
impl FromBytes for u8{
    fn from_bytes(bytes:Vec<u8>)->Self {
        return bytes[0]
    }
}
impl FromBytes for i8{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let d:[u8;1] = [bytes[0];1];
        return i8::from_ne_bytes(d);
    }
}
impl FromBytes for u16{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let mut b:[u8;2] = [0;2];
        b[0] = bytes[0];
        b[1] = bytes[1];
        return u16::from_ne_bytes(b);
    }
}
impl FromBytes for i16{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let mut b:[u8;2] = [0;2];
        b[0] = bytes[0];
        b[1] = bytes[1];
        return i16::from_ne_bytes(b);
    }
}
impl FromBytes for u32{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let mut b:[u8;4] = [0;4];
        b[0] = bytes[0];
        b[1] = bytes[1];
        b[2] = bytes[2];
        b[3] = bytes[3];
        return u32::from_ne_bytes(b);
    }
}
impl FromBytes for i32{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let mut b:[u8;4] = [0;4];
        b[0] = bytes[0];
        b[1] = bytes[1];
        b[2] = bytes[2];
        b[3] = bytes[3];
        return i32::from_ne_bytes(b);
    }
}
impl FromBytes for char{
    fn from_bytes(bytes:Vec<u8>)->Self {
        let mut d:[u8;4] = [0;4];
        for ind in 0..3{
            d[ind] = bytes[ind];
        }
        return char::from_u32(u32::from_ne_bytes(d)).unwrap();
    }
}
