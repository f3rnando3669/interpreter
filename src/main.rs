enum Primitive {
 Add, 
 Multiply, 
 Number(i32), 
} 
fn eval_prim(p: &Primitive) -> i32 {
     match p {
     Primitive::Add => 0, 
     Primitive::Multiply => 0, 
     Primitive::Number(val) => *val
      }
     }
fn evaluate(v: Vec<Primitive>) -> i32 {
    match v[0] { 
        Primitive::Add => {eval_prim(&v[1]) + eval_prim(&v[2]) }, 
        Primitive::Multiply => { eval_prim(&v[1]) * eval_prim(&v[2]) }, 
        _=> 0
    }
}
fn main() {
    let mut expression = Vec::<Primitive>::new(); 
    expression.push(Primitive::Add); 
    expression.push(Primitive::Number(3)); 
    expression.push(Primitive::Number(4)); 
    let sum = evaluate(expression); 

    let mut expression2 = Vec::<Primitive>::new();
    expression2.push(Primitive::Multiply);
    expression2.push(Primitive::Number(2));
    expression2.push(Primitive::Number(5));
    let sum2 = evaluate(expression2);
    println!("{sum} and {sum2}");
}
