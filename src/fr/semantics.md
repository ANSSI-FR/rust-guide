# Sémantique

Les choix sémantiques de Rust, motivés en partie pour des besoins de sûreté,
doivent être bien compris pour savoir 

* s'appuyer dessus pour sécuriser son code
* ne pas introduire d'effet indésirables

## Déplacement des valeurs

Par défaut, les valeurs de Rust sont considérées comme *déplaçable* à loisir en mémoire.
On donne dans la suite quelques exemples de déplacements en mémoire.

* Le passage *par valeur* lors d'un appel de fonction peut déplacer en mémoire l'objet.
  Par exemple, la fonction `Box::new<T>(t : T) -> Box<T>` *déplace* la valeur `t`
  depuis la pile vers le tas.
* Lors du redimensionnement d'un tableau dynamique de type `Vec<T>`, les éléments du
  tableau sont tous déplacés depuis l'ancien vers le nouveau tableau.
* Lors du déréférencement d'une variable (par exemple `let y = *x;` où `x` est de type `Box<T>`),
  le contenu du tas est déplacé vers la pile.

Les déplacements sont de simples copies bit à bit depuis l'emplacement initial vers l'emplacement
final, suivi d'une libération mémoire simple (sans `drop`) lors que l'emplacement initial se trouve
sur le tas.


Cette particularité complexifie notamment les [effacements sécurisés](erasure.md#fixer-les-valeurs-en-mémoire) de variables sensibles.