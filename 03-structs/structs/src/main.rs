
#[derive(Debug)]
struct Gun{
    ammo_in_clip: u32,
    clip_size: u32,
    total_ammo: u32,
    damage_per_hit: u32,
}

impl Gun{
    fn new() -> Gun{
        Gun{ammo_in_clip: 12, clip_size: 12, total_ammo: 120, damage_per_hit: 10}
    }

    fn reload(&mut self){
        println!("RELOADING");
        if self.total_ammo > self.clip_size{
            self.ammo_in_clip = self.clip_size;
            self.total_ammo -= self.clip_size;
        }else{
            self.ammo_in_clip = self.total_ammo;
            self.total_ammo = 0;
        }
    }

    fn pew(&mut self){
        if self.must_reload(){
            self.reload();
        }
        if self.ammo_in_clip > 0{
            self.ammo_in_clip -= 1;
            println!("PEW {} damage", self.damage_per_hit);
        }
    }

    fn must_reload(&self) -> bool{
        self.ammo_in_clip == 0
    }
}


struct Pooper();

impl Pooper{
    fn poop(&self) {
        println!("💩")
    }
}

struct Meower();

impl Meower{
    fn meow(&self) {
        println!("MEOW")
    }
}

struct Cat{
    pooper: Pooper,
    meower: Meower
}

impl Cat{
    fn new() -> Cat{
        Cat{pooper: Pooper{}, meower: Meower{}}
    }
}

fn main() {
    let cat = Cat::new();
    cat.pooper.poop();
    cat.meower.meow();    

    let mut gun = Gun::new();
    for _ in 0..20{
        gun.pew();
    }
    println!("{:?}",gun);
}
