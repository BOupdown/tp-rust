# Documentation du Projet en Rust

## Introduction

Ce projet en Rust permet de générer des vecteurs aléatoires et de rechercher les plus similaires à un vecteur de requête, en utilisant la similarité cosinus et une base de données interne sous forme de `HashMap`.

## Structure du Code

- Génération d'embeddings aléatoires.
- Calcul de la similarité cosinus entre deux vecteurs.
- Structure `Db` pour stocker et rechercher les vecteurs.
- Fonction `main` pour exécuter le programme.

## Code Source

### Génération d'un embedding aléatoire

```
fn generer_embedding(dimension: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dimension).map(|_| rng.gen_range(0.0..1.0)).collect()
}
```


### Calcul de la similarité cosinus

fn calcul_similarite(vec1: &[f32], vec2: &[f32]) -> f32 {
    let produit: f32 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
    let vec1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let vec2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();

    if vec1 == 0.0 || vec2 == 0.0 {
        0.0
    } else {
        produit / (vec1 * vec2)
    }
}

### Structure Db pour la base de données

struct Db {
    inner: HashMap<Uuid, Vec<f32>>,
}

impl Db {
    fn new() -> Self { Db { inner: HashMap::new() } }

    fn insert(&mut self, uuid: Uuid, embedding: Vec<f32>) {
        self.inner.insert(uuid, embedding);
    }

    fn trouver_les_plus_similaires(&self, query: &[f32], n: usize) -> Vec<(Uuid, f32)> {
        let mut similarities: Vec<(Uuid, f32)> = self
            .inner
            .iter()
            .map(|(uuid, embedding)| (*uuid, calcul_similarite(query, embedding)))
            .collect();
        
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        similarities.into_iter().take(n).collect()
    }
}

### Fonction main

fn main() {
    let mut db = Db::new();
    let embedding_dimension = 768;

    for _ in 0..10 {
        let uuid = Uuid::new_v4();
        let embedding = generer_embedding(embedding_dimension);
        db.insert(uuid, embedding);
    }

    let query_embedding = generer_embedding(embedding_dimension);
    let plus_similaire = db.trouver_les_plus_similaires(&query_embedding, 3);

    println!("Les 3 vecteurs les plus similaires :");
    for (uuid, similarity) in plus_similaire {
        println!("UUID: {}, Similarité: {:.4}", uuid, similarity);
    }
}
