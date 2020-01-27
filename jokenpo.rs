#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Object {
    Pedra,
    Papel,
    Tesoura,
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    object: Object,
    defeats: Object,
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Player) -> Option<Ordering> {
        if self.object == other.object {
            Some(Ordering::Equal)
        } else if self.defeats == other.object {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

fn jokenpo<'a>(p1: &'a Player, p2: &'a Player) -> &'a Player {
    if p1 > p2 {
        &p1
    } else {
        &p2
    }
}

const PEDRA: Player = Player {
    object: Object::Pedra,
    defeats: Object::Tesoura,
};
const PAPEL: Player = Player {
    object: Object::Papel,
    defeats: Object::Pedra,
};
const TESOURA: Player = Player {
    object: Object::Tesoura,
    defeats: Object::Papel,
};

#[test]
fn test_pedra() {
    assert_eq!(jokenpo(&PEDRA, &PAPEL), &PAPEL);
    assert_eq!(jokenpo(&PEDRA, &TESOURA), &PEDRA);
    assert_eq!(jokenpo(&PEDRA, &PEDRA), &PEDRA);
}

#[test]
fn test_papel() {
    assert_eq!(jokenpo(&PAPEL, &TESOURA), &TESOURA);
    assert_eq!(jokenpo(&PAPEL, &PEDRA), &PAPEL);
    assert_eq!(jokenpo(&PAPEL, &PAPEL), &PAPEL);
}

#[test]
fn test_tesoura() {
    assert_eq!(jokenpo(&TESOURA, &PEDRA), &PEDRA);
    assert_eq!(jokenpo(&TESOURA, &PAPEL), &TESOURA);
    assert_eq!(jokenpo(&TESOURA, &TESOURA), &TESOURA);
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8c1067e2a9da4bb309d4337b83952dc2
// https://www.youtube.com/watch?v=PSUAJn153eY
