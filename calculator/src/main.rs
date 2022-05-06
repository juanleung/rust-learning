use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let primero: String = args.nth(1).unwrap();
  let operador: char = args.nth(0).unwrap().chars().next().unwrap();
  let segundo: String = args.nth(0).unwrap();

  let primer_numero: f32 = primero.parse::<f32>().unwrap();
  let segundo_numero: f32 = segundo.parse::<f32>().unwrap();
  let resultado: f32 = operate(operador, primer_numero, segundo_numero);
  println!("{}", output(primer_numero, segundo_numero, operador, resultado));
}

fn operate(operador: char, primer_numero: f32, segundo_numero: f32) -> f32 {
  match operador {
    '+'=> primer_numero + segundo_numero,
    '-' => primer_numero - segundo_numero,
    '*' | 'x' | 'X' => primer_numero * segundo_numero,
    '/' => primer_numero / segundo_numero,
    _ => panic!("(Operador inválido => {}", operador)
  }
}

fn output(
  primer_numero: f32, segundo_numero: f32, operador: char, 
  result: f32) -> String {
  let op: char = match operador {
    'x' | 'X' => '*',
    _ => operador
  };
  format!("{} {} {} = {}", primer_numero, op, segundo_numero, result)
} 
