trait Monster {
    fn strength(&self)     -> int;
    fn dexterity(&self)    -> int;
    fn charisma(&self)     -> int;
    fn intelligence(&self) -> int;
    fn constitution(&self) -> int;
    fn wisdom(&self)       -> int;
}

struct MindFlayer {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for MindFlayer {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

struct Beholder {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for Beholder {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

struct Orc {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for Orc {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

struct Tiefling {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for Tiefling {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

struct Cow {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for Cow {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

struct GelatinousCube {
    strength: int,
    dexterity: int,
    charisma: int,
    intelligence: int,
    constitution: int,
    wisdom: int,
}

impl Monster for GelatinousCube {
    fn strength(&self) -> int {self.strength }
    fn dexterity(&self) -> int {self.dexterity}
    fn charisma(&self) -> int {self.charisma}
    fn intelligence(&self) -> int {self.intelligence}
    fn constitution(&self) -> int {self.constitution}
    fn wisdom(&self) -> int {self.wisdom}
}

fn show_monster_stats(monsters: &[&Monster]) {
    for monster in monsters.iter() {
        println!("|  STR  |  DEX  |  CHA  |  INT  |  CON  |  WIS  |")
        println!("+-------+-------+-------+-------+-------+-------+")
        println!("|  {:d}   |  {:d}   |  {:d}   |  {:d}   |  {:d}   |  {:d}   |",
                    monster.strength(),
                    monster.dexterity(),
                    monster.charisma(),
                    monster.intelligence(),
                    monster.constitution(),
                    monster.wisdom()
                )
    }
}

fn main() {
    let monsters = [
        &GelatinousCube {strength: 2, dexterity: 0, charisma: 0, intelligence: 2, constitution: 15, wisdom: 2} as &Monster,
        &Tiefling {strength: 8, dexterity: 16, charisma: 16, intelligence: 10, constitution: 12, wisdom: 10} as &Monster,
        &MindFlayer {strength: 8, dexterity: 10, charisma: 12, intelligence: 20, constitution: 15, wisdom: 14} as &Monster,
        &Beholder {strength: 6, dexterity: 10, charisma: 10, intelligence: 16, constitution: 15, wisdom: 16} as &Monster,
        &Orc {strength: 16, dexterity: 10, charisma: 8, intelligence: 7, constitution: 16, wisdom: 10} as &Monster,
        &Cow {strength: 2, dexterity: 2, charisma: 2, intelligence: 2, constitution: 2, wisdom: 2} as &Monster,
    ];

    show_monster_stats(monsters)
}
