mod Tienda{
    #[derive(Debug)]
    pub enum ErrorTienda{
        NombreVacio,
        StockNegativo(i32),
        CantidadInvalida(i32),
        StockInsuficiente(i32,i32),
        NoExiste,
    }
    pub struct Producto{
        nombre:String,
        stock:i32,
    }
    impl Producto{
        pub fn new(nombre:String, stock:i32)->Result<Producto, ErrorTienda>{
            if nombre.trim().is_empty(){
                return Err(ErrorTienda::NombreVacio);
            }
            if stock<0{
                return Err(ErrorTienda::StockNegativo(stock));
            }
            Ok(Producto { nombre, stock })
        }

        pub fn obtener_nombre_producto(&self)->String{
            self.nombre.to_string()
        }
        pub fn cambiar_nombre_producto(&mut self, nombre:String)->Result<String, ErrorTienda>{
            if nombre.trim().is_empty(){
                return Err(ErrorTienda::NombreVacio);
            }
            self.nombre=nombre;
            Ok(self.nombre.to_string())
        }

        pub fn obtener_stock(&self)->i32{
            self.stock
        }
        pub fn vender (&mut self, cantidad:i32)->Result<i32, ErrorTienda>{
            if cantidad<=0{
                return  Err(ErrorTienda::CantidadInvalida(cantidad));
            }
            if cantidad>self.stock{
                return Err(ErrorTienda::StockInsuficiente(cantidad, self.stock));
            }
            self.stock-=cantidad;
            Ok(self.stock)            
        }
        pub fn reponer(&mut self, cantidad:i32)->Result<i32,ErrorTienda>{
            if cantidad<=0{
                return Err(ErrorTienda::CantidadInvalida(cantidad));
            }
            self.stock+=cantidad;
            Ok(self.stock)
        }
    }

    pub struct Inventario{
        productos:Vec<Producto>,
    }
    impl Inventario {
        pub fn new(productos:Vec<Producto>)->Inventario{
            Inventario { productos }
        }
        pub fn agregar_productos(&mut self, p:Producto){
            self.productos.push(p);
            println!("Producto agregado!");
        }
        pub fn obtener_productos(&self)->&Vec<Producto>{
            &self.productos
        }
        pub fn vender(&mut self, nombre: &str, cantidad: i32)->Result<i32, ErrorTienda>{
            match self.productos.iter().position(|x|x.nombre==nombre){
                Some(producto)=>{ self.productos[producto].vender(cantidad)
                },
                None=> {return Err(ErrorTienda::NoExiste);},
            }
        }
        pub fn reponer(&mut self, nombre:&str, cantidad: i32)->Result<i32, ErrorTienda>{
            match self.productos.iter().position(|x|x.nombre==nombre){
                Some(i)=>{self.productos[i].reponer(cantidad)}
                None=> {return Err(ErrorTienda::NoExiste);}
            }
        }
        pub fn descontinuar(&mut self, nombre: &str) -> Result<Producto, ErrorTienda>{
            match self.productos.iter().position(|x|x.nombre==nombre){
                Some(i)=>{
                    let producto:Producto=self.productos.remove(i);
                    Ok(producto)
                    },
                None=>{return Err(ErrorTienda::NoExiste);}
            }
        }
        pub fn vender_dos(&mut self, a: &str, ca: i32, b: &str, cb: i32) -> Result<i32, ErrorTienda>{
            self.vender(a, ca)?;
            self.vender(b, cb)?;
            Ok(ca+cb) //Representa las unidades vendidas en total de la operacion.
        }
    }
}

fn main() {
    println!("Hello, world!");
}
