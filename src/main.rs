use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

/// Génère un vecteur de taille fixe avec des valeurs aléatoires entre 0 et 1.
/// 
/// # Arguments
/// 
/// * `dimension` - La dimension (ou taille) du vecteur à générer.
/// 
/// # Retour
/// 
/// Retourne un vecteur de type `Vec<f32>` de la taille spécifiée avec des valeurs aléatoires.
/// 
/// # Exemple
/// 
/// ```
/// let embedding = generer_embedding(128);
/// ```
fn generer_embedding(dimension: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dimension).map(|_| rng.gen_range(0.0..1.0)).collect()
}

/// Calcule la similarité cosinus entre deux vecteurs.
/// 
/// La similarité cosinus est une mesure qui permet d'évaluer la similarité entre deux vecteurs. Elle est
/// calculée en prenant le produit scalaire des deux vecteurs et en le divisant par le produit de leurs normes.
/// 
/// # Arguments
/// 
/// * `vec1` - Le premier vecteur sous forme de slice de `f32`.
/// * `vec2` - Le deuxième vecteur sous forme de slice de `f32`.
/// 
/// # Retour
/// 
/// Retourne la similarité cosinus sous forme de `f32`. Si l'un des vecteurs a une norme nulle, la fonction
/// retourne `0.0` pour éviter une division par zéro.
/// 
/// # Exemple
/// 
/// ```
/// let similarity = calcul_similarite(&vec1, &vec2);
/// ```
fn calcul_similarite(vec1: &[f32], vec2: &[f32]) -> f32 {
    let produit: f32 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
    let vec1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let vec2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();

    if vec1 == 0.0 || vec2 == 0.0 {
        0.0 // Éviter la division par zéro
    } else {
        produit / (vec1 * vec2)
    }
}

/// Structure représentant une base de données simple avec des UUIDs associés à des vecteurs d'embarquement.
/// 
/// La structure permet d'insérer des vecteurs et de rechercher les vecteurs les plus similaires à une requête donnée.
/// 
/// # Champs
/// 
/// * `inner` - Un `HashMap` qui associe un identifiant unique (UUID) à un vecteur d'embarquement (`Vec<f32>`).
struct Db {
    inner: HashMap<Uuid, Vec<f32>>,
}

impl Db {
    /// Crée une nouvelle base de données vide.
    /// 
    /// # Retour
    /// 
    /// Retourne une instance de `Db` initialisée avec un `HashMap` vide.
    fn new() -> Self {
        Db {
            inner: HashMap::new(),
        }
    }

    /// Insère un vecteur d'embarquement dans la base de données associé à un UUID unique.
    /// 
    /// # Arguments
    /// 
    /// * `uuid` - L'UUID unique à associer au vecteur.
    /// * `embedding` - Le vecteur d'embarquement à insérer.
    fn insert(&mut self, uuid: Uuid, embedding: Vec<f32>) {
        self.inner.insert(uuid, embedding);
    }

    /// Trouve les `n` vecteurs les plus similaires à un vecteur de requête donné.
    /// 
    /// # Arguments
    /// 
    /// * `query` - Le vecteur de requête pour lequel on recherche les vecteurs les plus similaires.
    /// * `n` - Le nombre de résultats à retourner.
    /// 
    /// # Retour
    /// 
    /// Retourne un vecteur de tuples `(Uuid, f32)` où chaque tuple contient l'UUID d'un vecteur
    /// et la similarité cosinus entre ce vecteur et la requête.
    /// 
    /// # Exemple
    /// 
    /// ```
    /// let resultats = db.trouver_les_plus_similaires(&query_embedding, 3);
    /// ```
    fn trouver_les_plus_similaires(&self, query: &[f32], n: usize) -> Vec<(Uuid, f32)> {
        let mut similarities: Vec<(Uuid, f32)> = self
            .inner
            .iter()
            .map(|(uuid, embedding)| (*uuid, calcul_similarite(query, embedding)))
            .collect();

        // Trier par similarité décroissante
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Retourner les n premiers
        similarities.into_iter().take(n).collect()
    }
}



/// Fonction principale qui simule l'insertion de vecteurs et la recherche de similarités.
/// 
/// Dans cette fonction, plusieurs vecteurs sont insérés dans la base de données,
/// un vecteur de requête est généré et les `n` vecteurs les plus similaires à ce vecteur sont
/// recherchés et affichés.
/// 
/// # Exemple
/// 
/// ```
/// main();
/// ```
fn main() {
    let mut db = Db::new();
    let embedding_dimension = 768;
    let phrase = "Ceci est un exemple de phrase";

    // Insérer un vecteur d'embarquement pour chaque mot dans la base de données
    for _mot in phrase.split_whitespace() {
        let uuid = Uuid::new_v4();
        let embedding = generer_embedding(embedding_dimension);
        db.insert(uuid, embedding);  // On insère chaque mot avec son vecteur d'embarquement
    }

    // Générer un vecteur de requête
    let query_embedding = generer_embedding(embedding_dimension);

    // Trouver les 3 vecteurs les plus similaires
    let plus_similaire = db.trouver_les_plus_similaires(&query_embedding, 3);

    println!("Les 3 vecteurs les plus similaires :");
    for (uuid, similarity) in plus_similaire {
        println!("UUID: {}, Similarité: {:.4}", uuid, similarity);
    }
}

