#![allow(dead_code)]
pub mod fun_plots {
    fn mand_op(a: f64, b: f64, n: i64) -> Option<(f64, f64)> {
        let c_re = a;
        let c_im = b;
        let mut zp_re: f64 = 0.0;
        let mut zp_im: f64 = 0.0;

        for _i in 0..n {
            let zn_re = zp_re.powf(2.0) - zp_im.powf(2.0) + c_re;
            let zn_im = 2.0 * zp_re * zp_im + c_im;
            if zn_re.powf(2.0) + zn_im.powf(2.0) >= 4.0 {
                return None;
            }
            zp_re = zn_re;
            zp_im = zn_im;
        }
        return Some((a, b));
    }
    fn gen(n: i64, g1: f64, g2: f64, p1: f64, p2: f64) -> (Vec<f64>, Vec<f64>) {
        let mut a = Vec::new();
        let mut b = Vec::new();

        for i in 0..n {
            let i2 = i as f64;
            let n2 = n as f64;
            a.push((i2 + p1 / g1 * n2) / (n2 / g1));
            b.push((i2 + p2 / g2 * n2) / (n2 / g2));
        }
        (a, b)
    }
    pub fn mand(n: i64, g1: f64, g2: f64, p1: f64, p2: f64) -> Vec<(f64, f64)> {
        let mut buffer = Vec::new();
        let (var1, var2) = gen(n, g1, g2, p1, p2);
        for &i in &var2 {
            for &j in &var1 {
                if let Some(thing) = mand_op(i, j, 250) {
                    buffer.push(thing);
                }
            }
        }
        buffer
    }
    pub fn wisteria(a: usize) -> Vec<(f64, f64)> {
        let mut x = Vec::new();
        let mut y = Vec::new();

        for i in 1..a {
            let i_list = format!("{}", i as f64);
            let mut prod = 1 as f64;
            for j in (&i_list).chars() {
                if (j as i64) != 0 {
                    prod *= format!("{}", j).parse::<f64>().unwrap();
                }
            }

            let xval = i as f64 - prod;
            y.push(i as f64);
            x.push(xval as f64);
        }
        y.into_iter().zip(x).collect()
    }
    pub fn perk_sq_new() -> Vec<(f64, f64)> {
        use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
        const M: usize = 50;
        #[derive(Copy, Clone, Debug)]
        struct Forest {
            parent: usize,
            x: usize,
            rank: usize,
        }
        impl Forest {
            fn new(xn: usize) -> Forest {
                Forest {
                    parent: xn,
                    x: xn,
                    rank: 0,
                }
            }
            fn find(mut self, tree: &mut [Forest; M.pow(2)]) -> Forest {
                // println!(temp_parent, temp_x)
                while self.x != self.parent {
                    self.parent = tree[self.parent].parent;
                    self.x = tree[self.x].parent;
                    // println!(temp_parent, temp_x)
                }
                // Zwraca "najstarszego przodka" instancji
                tree[self.x]
            }

            fn union(x: Forest, y: Forest, tree: &mut [Forest; M.pow(2)]) {
                let mut x = x.find(tree);
                let mut y = y.find(tree);

                // x i y mają tą samą wartość, czyli prawdopodobnie są nawet tą samą instancją. Nie robimy nic
                if x.x == y.x {}
                // sortowanie według rangi - czy pierwszy, czy drugi argument zostanie rodzicem
                if x.rank < y.rank {
                    (x, y) = (y, x)
                }
                // nie wiem, czy to potrzebne, ale nie chcę popsuć kodu. Jeśli instancja została wywołana funkcją union,
                // to rośnie jej ranga, edit: niepotrzebne
                //x.rank += 1;
                //y.rank += 1;
                // po możliwej zamianie opisanej dwa komentarze wyżej ygrek przyjmuje iksa za rodzica
                y.parent = x.x;
                // funkcja jest jednokierunkowa, tu wymuszamy ten kierunek. Ma to większy sens w połączeniu
                // z kodem wywołującym funkcję union
                if x.rank == y.rank {
                    x.rank = x.rank + 1;
                }
                tree[x.x] = x;
                tree[y.x] = y;
            }
        }
        fn pair_to_index(i: usize, j: usize) -> usize {
            j + i * M
        }
        fn index_to_pair(id: usize) -> (usize, usize) {
            let j = id % M;
            let i = id / M;
            (i, j)
        }
        fn pick_from(l: &mut Vec<usize>) -> usize {
            let sample = l.pop();
            //println!("{}", sample.unwrap());
            sample.unwrap()
        }
        fn sasiadv2(id: usize) -> Vec<usize> {
            let (i, j) = index_to_pair(id);
            let mut do_zwrotu = Vec::new();
            if i + 1 < M {
                do_zwrotu.push(pair_to_index(i + 1, j));
            }
            if i > 0 {
                do_zwrotu.push(pair_to_index(i - 1, j));
            }
            if j + 1 < M {
                do_zwrotu.push(pair_to_index(i, j + 1));
            }
            if j > 0 {
                do_zwrotu.push(pair_to_index(i, j - 1));
            }
            do_zwrotu
        }
        let mut blist = [0; M.pow(2)];
        for j in 0..M.pow(2) {
            blist[j] = j;
        }
        fn perk_it(list: [usize; M.pow(2)]) -> usize {
            let mut blist = list.to_vec();
            blist.shuffle(&mut SmallRng::from_entropy());
            let mut trees = [Forest::new(M.pow(3)); M.pow(2) as usize];
            let mut first_row = Vec::new();
            let mut last_row = Vec::new();
            let mut counter: usize = 0;
            let mut breaker = false;
            let mut_trees = &mut trees;
            loop {
                let sample = pick_from(&mut blist);
                mut_trees[sample] = Forest::new(sample);
                if sample < M {
                    first_row.push(sample);
                    // bez tego symulacja się partoli. Jest tu po to, żeby przypadkiem nie wyszło, że jakaś
                    // instancja klasy odpowiadająca elementowi z rzędu i = 1 (czyli drugiego) nie miała
                    // wyższej rangi niż instancja odpow. elementowi z rzędu i = 0 (pierwszego) i nie
                    // "przyciągała" do siebie innych instancji jako "dzieci", zanim może to zrobić wartość z góry
                    mut_trees[sample].rank += M.pow(2);
                } else if sample >= M * (M - 1) {
                    last_row.push(sample)
                }
                for &id in &sasiadv2(sample) {
                    if mut_trees[id].x <= M.pow(2) {
                        Forest::union(
                            mut_trees[sample],
                            mut_trees[id],
                            mut_trees,
                        );
                        //println!("{:?} {:?}", trees[sample], trees[id])
                    }
                }
                for &id2 in &last_row {
                    for &id in &first_row {
                        if id == mut_trees[id2].find(mut_trees).x {
                            breaker = true;
                            // println!("Perkolacja!")
                            break;
                        }
                    }
                    if breaker {
                        break;
                    }
                }
                if breaker {
                    break;
                }
                counter += 1;
            }
            counter
        }
        let mut sampler = [0f64; M.pow(2)];
        let mut new_counter = [0f64; M.pow(2)];
        (0..M.pow(2)).for_each(|j| {
            new_counter[j] = 100.0 * j as f64 / (M.pow(2) as f64);
        });
        for _i in 0..10000 {
            let temp = perk_it(blist);
            (0..M.pow(2)).for_each(|j| {
                if j >= temp {
                    sampler[j] += 1f64;
                }
            })
        }
        new_counter
            .to_vec()
            .into_iter()
            .zip(sampler.to_vec())
            .collect()
    }
}
