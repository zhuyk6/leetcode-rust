use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    movie_price_of_shop: Vec<HashMap<i32, i32>>,
    unrented_movies: HashMap<i32, BTreeSet<(i32, i32)>>,
    rented_movies: BTreeSet<(i32, usize, i32)>,
}

impl MovieRentingSystem {
    pub fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut movie_price_of_shop = vec![HashMap::new(); n];
        let mut unrented_movies: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        for entry in entries {
            let shop = entry[0] as usize;
            let movie = entry[1];
            let price = entry[2];
            movie_price_of_shop[shop].insert(movie, price);
            unrented_movies
                .entry(movie)
                .or_default()
                .insert((price, shop as i32));
        }
        Self {
            movie_price_of_shop,
            unrented_movies,
            rented_movies: BTreeSet::new(),
        }
    }

    /// Given `movie`, search the 5 shops which have unrented `movie` and lowest price.
    pub fn search(&self, movie: i32) -> Vec<i32> {
        if let Some(btree) = self.unrented_movies.get(&movie) {
            btree.iter().take(5).map(|&(_, shop)| shop).collect()
        } else {
            vec![]
        }
    }

    /// Rent `movie` from `shop`.
    pub fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.movie_price_of_shop[shop as usize][&movie];
        self.unrented_movies
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
        self.rented_movies.insert((price, shop as usize, movie));
    }

    /// Return `movie` to `shop`.
    pub fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.movie_price_of_shop[shop as usize][&movie];
        self.rented_movies.remove(&(price, shop as usize, movie));
        self.unrented_movies
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));
    }

    /// Return the 5 rented movies with lowest price.
    pub fn report(&self) -> Vec<Vec<i32>> {
        self.rented_movies
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop as i32, movie])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nested_vec;

    #[test]
    fn sample1() {
        let mut shop = MovieRentingSystem::new(
            3,
            nested_vec![
                [0, 1, 5],
                [0, 2, 6],
                [0, 3, 7],
                [1, 1, 4],
                [1, 2, 7],
                [2, 1, 5],
            ],
        );
        assert_eq!(shop.search(1), vec![1, 0, 2]);
        shop.rent(0, 1);
        shop.rent(1, 2);
        assert_eq!(shop.report(), nested_vec![[0, 1], [1, 2]]);
        shop.drop(1, 2);
        assert_eq!(shop.search(2), vec![0, 1]);
    }
}
