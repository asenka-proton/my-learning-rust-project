## La pile et le tas
 
En Rust, contrairement à d'autres langages, il est nécessaire de se préoccuper de la zone mémoire où est stockée
une donnée. Elle peut être dans 2 grandes zones :

- la pile (`stack`)
- le tas (`heap`)

Ces 2 zones sont à disposition du programme lors de son exécution mais ne fonctionnent pas de la même façon et n'ont
pas les mêmes contraintes. 

---

### La pile (stack)

La pile enregistre les valeurs dans l'**ordre** dans lequel elle les reçoit et les enlève dans l'autre sens (_dernier 
entré, premier sorti_ ou LIFO). On peut le voir comme une pile d'assiettes : on ajoute une assiette au sommet de la pile,
et on prend l'assiette qui se trouve également au sommet. Cela signifie qu'il est peu aisé d'accéder à une donnée qui se
trouve au milieu ou en dessous !

Les opérations principales de la pile sont donc les suivantes :

- Empiler (`push`)
- Dépiler (`pop`)

> **IMPORTANT**
> 
> Toutes les données dans la pile doivent avoir une taille **connue** et **fixe** !

Empiler une donnée sur la pile est une **opération très rapide**, car le gestionnaire de mémoire n'a aucune opération à effectuer
pour choisir où placer la donnée et quelle espace lui attribuer : la donnée ira toujours **au sommet de la pile**.

---

### Le tas (heap)


Le tas est ine zone mémoire moins "organisée" que la pile. Lors d'un ajout, on demande une certaine quantité d'espace
mémoire. C'est le **gestionnaire de mémoire** qui se charge de trouver un emplacement dans le tas qui a une **taille suffisante**
et le marquer comme étant "_en cours d'utilisation_". Une fois cela fait, le gestionnaire de mémoire renvoie un **pointeur** 
vers cet emplacement. Le pointeur représente l'**adresse** de cet emplacement. On appelle cette procédure l'**allocation 
sur le tas**.

Le pointeur est une donnée de **taille fixe**, il est stockée sur la **pile**. Mais il ne s'agit que de l'adresse ; la 
donnée référencée se situe bien dans le tas.

Allouer de la place sur le tas demande **plus de travail** que d'empiler une donnée sur la pile ; le gestionnaire doit trouver
un espace assez grand et se mettre à jour pour préparer la prochaine allocation mémoire.

---

### Comparaison

| Caractéristique        | Pile (Stack)                    | Tas (Heap)                               |
|:-----------------------|:--------------------------------|:-----------------------------------------|
| **Vitesse**            | Très rapide (O(1))              | Plus lent (allocation dynamique)         |
| **Taille des données** | Fixe, connue à la compilation   | Dynamique ou inconnue à la compilation   |
| **Gestion**            | Automatique (LIFO, scope)       | Via Ownership (`Box`, `Vec`, `Rc`)       |
| **Durée de vie**       | Fin du scope (stack frame)      | Tant qu'il existe un propriétaire valide |
| **Types courants**     | `i32`, `bool`, `[T; N]`, tuples | `String`, `Vec<T>`, `Box<T>`, `HashMap`  |
| **Fragmentation**      | Non                             | Possible                                 |
| **Coût de copie**      | Copie complète des données      | Copie du pointeur seulement              |
| **Limite**             | Taille fixe (ex: 1-8 Mo)        | Mémoire RAM disponible                   |







