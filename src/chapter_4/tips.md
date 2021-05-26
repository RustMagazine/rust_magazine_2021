---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: tips

---

# 语言技巧

编辑：张汉东

---

## 返回多态类型

```rust
use rand::{thread_rng, Rng};

/// This is the trait that every die needs to implement to be... well... "rollable", right?
pub trait Rollable {
    /// Roll the die
    fn roll() -> Self;
    /// Get the value from the latest roll
    fn val(&self) -> u8;
}

/// A generic function to roll a given die.
pub fn roll<T: Rollable>() -> T {
    Rollable::roll() // <- Note that here `Rollable` is the current type for a given call!
}

/// A D6 die (6 faces): a roll will give you a `u8` in the `1..=6` range.
#[derive(Debug)]
pub struct D6(u8);

impl Rollable for D6 {
    fn roll() -> D6 {
        D6 {
            0: thread_rng().gen_range(1..=6),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D8 die (8 faces): a roll will give you a `u8` in the `1..=8` range.
#[derive(Debug)]
pub struct D8(u8);

impl Rollable for D8 {
    fn roll() -> D8 {
        D8 {
            0: thread_rng().gen_range(1..=8),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

#[derive(Debug)]
struct Fake100(u8);

impl Rollable for Fake100 {
    fn roll() -> Fake100 {
        Fake100 { 0: 100 } // <- forces it to roll 100
    }
    fn val(&self) -> u8 {
        self.0
    }
}

fn main() {
    // let's roll a D6
    let r: D6 = roll();
    println!("{:?}", r); // D6(3)

    // let's roll a D8
    let r: D8 = roll();
    println!("{:?}", r); // D8(3)

    println!("I bet I'll get a 100 this time!");
    let d: Fake100 = roll();
    println!("Look what I got: {}!", d.val()) // <- yeah this will always be 100
}
```

也支持类型推断：

```rust
fn try_dodge_attack(d6: D6, d8: D8) -> bool {
    d6.val() + d8.val() > 10
}

fn main() {
    let escaped = try_dodge_attack(roll(), roll());
    println!(
        "{}",
        match escaped {
            true => "You dogded!",
            false => "Ouch! The attack hit you!",
        }
    );
}
```

[来源](https://loige.co/rust-shenanigans-return-type-polymorphism/)

## 一个零开销链表的实现

下面代码实现了一个 持久性/不变性（Persistent / Immutable ）的单向链表（Singly-linked）。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum List<'a, T> {
    Node { data: T, next: &'a List<'a, T> },
    Tail,
}

impl<T> Default for List<'_, T> {
    fn default() -> Self {
        List::Tail
    }
}

impl<'a, T> List<'a, T> {
    pub fn add(&'a self, data: T) -> Self {
        List::Node { data, next: self }
    }

    pub fn rev_iter(&'a self, f: impl Fn(&'a T)) {
        if let List::Node { data, next } = self {
            next.rev_iter(&f);
            f(data);
        }
    }

    pub fn try_rev_iter<E, F>(&'a self, f: F) -> Result<(), E>
    where
        F: Fn(&'a T) -> Result<(), E>,
    {
        if let List::Node { data, next } = self {
            next.try_rev_iter(&f)?;
            f(data)?;
        }
        Ok(())
    }
}

pub struct ListIter<'a, T>(&'a List<'a, T>);

impl<'a, T> IntoIterator for &'a List<'a, T> {
    type Item = &'a T;

    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            List::Node { data, next } => {
                self.0 = next;
                Some(data)
            }
            List::Tail => None,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Num(f64),
    Bool(bool),
    String(String),
}

#[derive(PartialEq)]
pub enum ValueKind {
    Num,
    Bool,
    String,
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Num(_) => ValueKind::Num,
            Value::Bool(_) => ValueKind::Bool,
            Value::String(_) => ValueKind::String,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Variable(String),
    UnExpr(UnExprKind, Box<Expr>),
    BinExpr(BinExprKind, Box<(Expr, Expr)>),
    Define(String, Box<(Expr, Expr)>),
    IfThenElse(Box<(Expr, Expr, Expr)>),
}

#[derive(Debug, Copy, Clone)]
pub enum UnExprKind {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinExprKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Logic
    And,
    Or,
    Equals,
    NotEquals,
}

type Variables<'a> = List<'a, (String, Value)>;

pub fn eval(vars: &Variables<'_>, expr: Expr) -> Option<Value> {
    match expr {
        Expr::Value(val) => Some(val),

        Expr::Variable(var) => vars
            .into_iter()
            .find(|&(v, _)| *v == var)
            .map(|(_, val)| val.clone()),

        Expr::UnExpr(kind, expr) => {
            eval_unary(kind, vars, *expr)
        }

        Expr::BinExpr(kind, exprs) => {
            eval_binary(kind, vars, exprs.0, exprs.1)
        }

        Expr::Define(name, exprs) => {
            let value = eval(vars, exprs.0)?;
            let vars = vars.add((name, value));
            eval(&vars, exprs.1)
        }

        Expr::IfThenElse(exprs) => {
            if let Value::Bool(b) = eval(vars, exprs.0)? {
                eval(vars, if b { exprs.1 } else { exprs.2 })
            } else {
                None
            }
        }
    }
}

fn eval_unary(
    kind: UnExprKind,
    vars: &Variables<'_>,
    expr: Expr,
) -> Option<Value> {
    let val = eval(vars, expr)?;
    match (kind, val) {
        (UnExprKind::Not, Value::Bool(b)) => {
            Some(Value::Bool(!b))
        }
        (UnExprKind::Neg, Value::Num(n)) => Some(Value::Num(-n)),
        _ => None,
    }
}

fn eval_binary(
    kind: BinExprKind,
    vars: &Variables<'_>,
    lhs: Expr,
    rhs: Expr,
) -> Option<Value> {
    let lhs = eval(vars, lhs)?;

    match kind {
        BinExprKind::Add => {
            if let Value::Num(lhs) = lhs {
                if let Value::Num(rhs) = eval(vars, rhs)? {
                    return Some(Value::Num(lhs + rhs));
                }
            }
            None
        }
        BinExprKind::Sub => {
            if let Value::Num(lhs) = lhs {
                if let Value::Num(rhs) = eval(vars, rhs)? {
                    return Some(Value::Num(lhs - rhs));
                }
            }
            None
        }
        BinExprKind::Mul => {
            if let Value::Num(lhs) = lhs {
                if let Value::Num(rhs) = eval(vars, rhs)? {
                    return Some(Value::Num(lhs * rhs));
                }
            }
            None
        }
        BinExprKind::Div => {
            if let Value::Num(lhs) = lhs {
                if let Value::Num(rhs) = eval(vars, rhs)? {
                    return Some(Value::Num(lhs / rhs));
                }
            }
            None
        }

        BinExprKind::And => {
            if let Value::Bool(lhs) = lhs {
                if !lhs {
                    return Some(Value::Bool(false));
                }
                if let Value::Bool(rhs) = eval(vars, rhs)? {
                    return Some(Value::Bool(rhs));
                }
            }
            None
        }
        BinExprKind::Or => {
            if let Value::Bool(lhs) = lhs {
                if lhs {
                    return Some(Value::Bool(true));
                }
                if let Value::Bool(rhs) = eval(vars, rhs)? {
                    return Some(Value::Bool(rhs));
                }
            }
            None
        }
        BinExprKind::Equals => {
            let rhs = eval(vars, rhs)?;
            if lhs.kind() == rhs.kind() {
                Some(Value::Bool(lhs == rhs))
            } else {
                None
            }
        }
        BinExprKind::NotEquals => {
            let rhs = eval(vars, rhs)?;
            if lhs.kind() == rhs.kind() {
                Some(Value::Bool(lhs != rhs))
            } else {
                None
            }
        }
    }
}
```

[来源](https://aloso.github.io/2021/04/12/linked-list.html)