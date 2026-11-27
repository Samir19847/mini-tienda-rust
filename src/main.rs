mod Tienda{
    use std::string;

use crate::pausa;

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
            println!("Producto agregado!"); pausa();
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
        pub fn cambiar_nombre(&mut self, nombre_actual: String, nombre_nuevo:String)->Result<String,ErrorTienda>{
            match self.productos.iter().position(|x|x.nombre==nombre_actual){
                Some(i)=>
                {
                    self.productos[i].cambiar_nombre_producto(nombre_nuevo)?;
                    Ok(self.productos[i].obtener_nombre_producto())
                },
                None=>{return Err(ErrorTienda::NoExiste);},
            }
        }
        pub fn vender_dos(&mut self, a: &str, ca: i32, b: &str, cb: i32) -> Result<i32, ErrorTienda>{
            self.vender(a, ca)?;
            self.vender(b, cb)?;
            Ok(ca+cb)
        }
    }
}

use std::collections::VecDeque;
use Tienda::{Inventario, Producto};
use std::io::{self,Write};

use crate::Tienda::ErrorTienda::{self, CantidadInvalida, NombreVacio};
#[derive(Debug)]
enum Comando {
    Vender(String,i32),
    Descontinuar(String),
    Deshacer,
}
fn entrada_nombres()->String{
    print!("Por favor, ingrese el nombre del producto:");
    io::stdout().flush().unwrap();
    let mut entrada:String=String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    println!();
    println!("Nombre guardado!");
    entrada.trim().to_string()
}
fn entrada_numeros(mensaje:String)->i32{
    let stock=loop{
    print!("Por favor, ingrese {}:",mensaje);
    io::stdout().flush().unwrap();
    let mut entrada:String=String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    match entrada.trim().parse::<i32>(){
        Ok(v)=> break v,
        Err(or)=>println!("Error: Tipo de dato incorrecto. Por favor, vuelve a intentarlo...\n"),
    }
    };
    stock
}
fn pausa(){
    let mut entrada:String=String::new();
    io::stdin().read_line(&mut entrada);
    entrada.clear();
}
fn main() {
    let mut inventario_lista:Vec<Producto>=Vec::new();
    let mut Inventario=Inventario::new(inventario_lista);
    let mut cola: VecDeque<Comando> = VecDeque::new();
    let mut historial: Vec<(String, i32)> = Vec::new();
    let mut binario=0;
    let mut nombre_producto_nuevo:String=String::from("");
    loop{
        println!("+================================+");
        println!("|    SISTEMAS DE VENTAS RUST     |");
        println!("+================================+");
        println!();
        println!("+================================+");
        println!("| 1. Registrar productos.        |");
        println!("| 2. Vender productos.           |");
        println!("| 3. Descontinuar productos.     |");
        println!("| 4. Deshacer cambios.           |");
        println!("| 5. Ver lista de pedidos.       |");
        println!("| 6. Ver inventario actual.      |");
        println!("| 7. Salir.                      |");
        println!("+================================+");
        println!();
        let mut opcion=entrada_numeros(String::from("una opción del menú"));
        match opcion{
            1=>{
                while opcion!=3 {
                    println!("|==================================|");
                    println!("| 1. Registrar productos.          |");
                    println!("| 2. Cambiar nombre algun producto |");
                    println!("| 3. Salir.                        |");
                    println!("|==================================|");
                    opcion=entrada_numeros(String::from("ingrese una opción"));
                    match opcion{
                        1=>{
                            let nombre_producto:String=entrada_nombres();
                            let stock=entrada_numeros(String::from("el stock del producto"));
                            match Producto::new(nombre_producto, stock) {
                                Ok(p) => {Inventario.agregar_productos(p);}
                                Err(ErrorTienda::NombreVacio) => {println!("Error: El nombre no puede ir vacío."); pausa();},
                                Err(ErrorTienda::StockNegativo(s)) => {println!("Error: El stock no puede ser negativo ({}).", s); pausa();},
                                Err(e) => {println!("Error inesperado: {:?}", e); pausa();},
                            }
                        },
                        2=>{
                            if Inventario.obtener_productos().is_empty(){
                                println!("Error: no se ha registrado ningun producto al inventario.");
                            }
                            let nombre_producto_actual:String=entrada_nombres(); pausa();
                            nombre_producto_nuevo=entrada_nombres(); pausa();
                            match Inventario.cambiar_nombre(nombre_producto_actual.clone(), nombre_producto_nuevo.clone()){
                                Ok(k)=>{println!("El nombre nuevo del producto es: {k}\nEl cambio ha sido un exito!"); pausa(); 
                                historial.push((nombre_producto_actual.clone(),0)); binario=3;},
                                Err(ErrorTienda::NoExiste)=>println!("Error: El producto ingresado, no existe"),
                                Err(ErrorTienda::NombreVacio) => { println!("Error: El nuevo nombre no puede estar vacío"); pausa(); },
                                Err(e) => { println!("Error inesperado: {:?}", e); pausa();}
                            }
                        },
                        3=> {break;},
                        _=>{ println!("Error: Por favor seleccione una opción correcta.")},
                    }
                }
            },
            2=>{
                let nombre = entrada_nombres();
                let cantidad = entrada_numeros(String::from("la cantidad a vender"));
                cola.push_back(Comando::Vender(nombre, cantidad));
                println!("Pedido de venta agregado a la cola."); pausa();
            },
            3=>{
                let nombre=entrada_nombres();
                cola.push_back(Comando::Descontinuar(nombre));
                println!("Orden de descontinuar agregada a la cola."); pausa();
            }
            4=>{
                cola.push_back(Comando::Deshacer);
                println!("Orden de deshacer agregada a la cola."); pausa();
            }
            5=>{
                if cola.is_empty() {
                    println!("No hay pedidos en el sistema.");
                    pausa();
                }
                else{
                    while !cola.is_empty(){
                        match cola.pop_front(){
                            Some(comando)=>{
                                 match comando{
                                Comando::Vender(_,0)=>{
                                    println!("Error: Venta ignorada, no se puede vender esa cantidad"); pausa();
                                },
                                Comando::Vender(nombre, stock)=>{
                                    match Inventario.vender(&nombre, stock){
                                        Ok(k)=>{
                                            println!("Venta realizada correctamente.\nEl producto: {}, tiene {} unidades",&nombre, k);
                                            historial.push((nombre,stock)); binario=1;
                                            pausa();
                                        },
                                        Err(or)=>{println!("Error: El producto que ha querido vender no existe"); pausa();},
                                    }
                                },
                                Comando::Descontinuar(nombre)=>{
                                    match Inventario.descontinuar(&nombre){
                                        Ok(k)=>{
                                            println!("El producto {}, se ha descontinuado de la tienda", k.obtener_nombre_producto());
                                            historial.push((k.obtener_nombre_producto(),k.obtener_stock())); binario=2; pausa();
                                        },
                                        Err(or)=>{println!("Error: El producto que ha querido descontinuar no existe"); pausa();},    
                                    }
                                }
                                Comando::Deshacer=>{
                                 match historial.pop(){
                                    Some(cambio)=>{
                                        if binario==1{
                                            let nombre_producto1:&str=&cambio.0;
                                            let stock_producto1=cambio.1;
                                            Inventario.reponer(nombre_producto1, stock_producto1);
                                            println!("Se ha repuesto el stock al producto: {}, ahora tiene {} unidades.",nombre_producto1,stock_producto1);
                                            pausa();
                                        }
                                        if binario==2{
                                            let nombre_producto2:&str=&cambio.0;
                                            nombre_producto2.to_string();
                                            let stock_producto2=cambio.1;
                                            if let Ok(p)=Producto::new(nombre_producto2.to_string(), stock_producto2){
                                                Inventario.agregar_productos(p);
                                                println!("El producto: {nombre_producto2}, ha sido implementado nuevamente al inventario.");
                                                pausa();
                                            }
                                        }
                                        if binario==3{
                                            let nombre_antiguo=&cambio.0;
                                            match Inventario.cambiar_nombre(nombre_producto_nuevo.clone(), nombre_antiguo.to_string()){
                                                Ok(k)=>{println!("El nombre nuevo del producto es: {k}\nEl cambio ha sido revertido!"); pausa(); },
                                                Err(ErrorTienda::NoExiste)=>println!("Error: El producto ingresado, no existe"),
                                                Err(ErrorTienda::NombreVacio) => { println!("Error: El nuevo nombre no puede estar vacío"); pausa(); },
                                                Err(e) => { println!("Error inesperado: {:?}", e); pausa();}
                                            }
                                        }
                                    },
                                    None=>{
                                        println!("No se ha hecho ningún cambio en el inventario del sistema.");
                                    },
                                 }   
                                }
                            }
                        }                            
                            None=>println!(),   
                        }
                        }
                    }
                },
            6=>{
                if Inventario.obtener_productos().is_empty(){
                    println!("Error: no se ha registrado ningún producto al iventario"); pausa();
                }
                let mut inventario_lista=Inventario.obtener_productos();
                for p in inventario_lista{
                    println!("| Nombre: {} - Stock: {}", p.obtener_nombre_producto(),p.obtener_stock());
                }
                pausa();
            }
            7=>{println!("Saliendo del programa..."); return;},
            _=> println!("Opción inválida, por favor, ingrese una opción correcta."),
            
        }
    }
}

