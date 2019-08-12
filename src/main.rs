#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]

#[macro_use]
extern crate pest_derive;

mod parser;
mod ast;
use parser::grammar::parse_code;
use ast::*;

mod resources;

fn main() {
    println!("{:?}", parse_code(
      "function f(x) {
         var y
         var a=1, b
         var u, v
         var c = \"x\"
         var d, e = 99.15

         if !(x > 2) == (k - 1)  {
           a = 2
           b = !x ? !p : !z
           c = !a + !b
           d = a + b * c + d
           e = a * b + c + d
           f = a + b + c * d
           g = a + b ? c + d : e + f
           h = a + (b ? c + d : e + f)
           i = (a + b) * c + d
           j = a - b + c
           k = a + b - c
           l = a - b - c

           with sprite::hello {
             return x[1,1] + f(1)
           }
         }
       }

       object hello {
         a: 10
         b[0]: 12

         function g(x,y) {
           return a + b[0]
         }
       }
       "
    ));

    let f = Expression::name("f");
    let x = Expression::name("x");
    let y = Expression::name("y");
    println!("{:?}", Expression::call(f, &[x, y]));
}
