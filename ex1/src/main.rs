fn main() {
    let mut v = Vector::new();

    v.add(5);
    v.add(7);
    v.add(2);
    v.add(0);
    v.add(3);
    v.add(9);

    v.display();
    println!();

    v.remove(3);
    v.remove(0);
    v.remove(9);

    v.display();

}

struct Vector {
    tabel: Vec<i32>,
}

impl Vector {
    fn new() -> Vector {
        Vector { tabel: vec![] }
    }

    fn add(&mut self, value: i32) {
        let mut pozitie:usize = 0;

        for index in 0..self.tabel.len() {
            if self.tabel[index]<value {
                pozitie+=1;
            }
        }

        self.tabel.insert(pozitie, value)
    }

    fn remove(&mut self, value: i32) {
        let mut pozitie = None;
        for index in 0..self.tabel.len() {
            if self.tabel[index] == value {
                pozitie = Some(index);
            }
        }

        match pozitie {
            Some(pozitie) => { self.tabel.remove(pozitie); },
            None =>  println!("Valoarea introdusa nu exista in vector!")
        }
        
        
    }

    fn display(&self) {
        for index in 0..self.tabel.len() {
            print!("{} ",self.tabel[index]);
        }
    }
}
