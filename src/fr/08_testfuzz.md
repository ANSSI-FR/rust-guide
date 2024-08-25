# Test et fuzzing

## Test

Rust propose deux types de test intégrés par défaut : les tests internes ainsi que les tests d'intégration. 
Dans cette section, nous discuterons de ces deux types de tests ainsi que d'un type de test un peu particulier qui est le test d'implémentation de trait.

> ### Recommandation {{#check TEST-DRIVEN-DEV | Adoptez la méthode de développement par les tests}}
>
> Une des bonnes habitudes de développement est de commencer à développer en écrivant l'ensemble des tests auquel doit répondre la fonctionnalité. 
### Interne

Les tests dits internes définissent l'ensemble des tests étant présent dans le dossier `src/` d'un projet Rust. Ceux-ci présentent le grand avantage de pouvoir tester l'ensemble des fonctions (même privées) si placé dans le même fichier que celui-ci.

> ### Recommandation {{#check TEST-INTERNE | Tester aux maximum les différentes fonctionnalités}}
>
> Il est conseillé l'ensemble des fonctions de votre programme même celle qui peuvent sembler le plus trivial.
> 
> Ainsi, dans le cas où une modification future provoquerait un effet de bord modifiant le comportement d'une autre fonction, vous le remarquerez bien plus rapidement.
> Cela permet également de limiter le nombre de bugs au plus tôt. 

```rust
// fonction privée
fn my_function(){
	... // Votre code ici
}

#[cfg(test)]
mod tests{
	#[test]
	fn test_my_function(){
		... // Vos tests ici
	}
}
```

Il est également possible d'ignorer certains tests dans le cas où ceux-ci prendraient trop de temps à être effectué. Cela peut être fait en rajouter `#[ignore]` au-dessus de la fonction de test.

```rust
#[cfg(test)]
mod tests{
	#[test]
	fn test_non_ignore(){ ... }

	#[ignore]
	#[test]
	fn test_ignore() { ... }
}
```

> ### Important
>
> Dans le cas où un test est marqué comme étant a ignorer, il ne sera plus possible de l’exécuter même si vous précisez son nom en utilisant la commande `cargo test <test_name>`. 
> 
> Le seul moyen qu'il puisse être lancé et de lui enlever le `#[ignore]` au-dessus de lui.

> ### Recommandation {{#check TEST-IGNORE | Limiter au maximum le nombre de tests ignoré}}
>
> Il est recommandé de limiter au maximum le nombre de tests qui seront ignorée 
 
Rust possède un système d'attribue permettant de ne compiler une partie du code que si c'est nécessaire. 
Cela permet notamment de définir du code qui ne sera compilé que lorsqu'une feature particulière est demandé. 

L'une des caractéristiques présente de base dans tout projet est `test`. Celle-ci permet de décrire du code qui ne sera présent que lorsque le code est compilé pour être testé (via la commande `cargo test`).

Pour ce faire, il faut ajouter l'attribue `#[cfg(test)]` sur la ligne au dessus de la fonction ou du module concerné : 
```rust
#[cfg(test)]
mod test{

	#[test]
	fn test_1(){}
}
```

> ### Règles {{#check TEST-CFG | Encadrer les tests dans un sous-module ayant l'attribue `#[cfg(test)]`}}
>
> L'ensemble des tests internes doivent être englobé dans un sous-module ayant l'attribue `#[cfg(test)]`.
> 
> De même, les potentielles fonctions que vous pouvant être développé pour aider ces tests doivent avoir également avoir l'attribue `#[cfg(test)]`.

### Intégration

> ### Attention
>
> Ce type de test n'est disponible que pour les crates qui sont des librairies.

Les tests d'intégrations sont l'ensemble de tests présents dans le dossier `tests/` à la racine de la crate. 
Dans ce dossier, chaque fichier `*.rs` sera compilé comme étant une crate différentes et la bibliothèque testée sera à utiliser comme si un projet externe l'utilisait.  

Par exemple, dans si nous développions une librairie nommé `exemple`, nous pourrions faire le test d'intégration suivant : 
```rust
use exemple::method_name;

#[test]
fn test_api(){
	method_name();
}
```

Ces tests sont lancés en même temps que l'ensemble des autres tests via la commande suivante : 
```bash
cargo test
```

> ### Règle {{#check TEST-IMPL | Vérifier que le comportement publique de l'API est bien celui attendus}}
>
> Les tests d'intégration doivent permettre de s'assurer que le comportement de la librairie est celui attendu. Ces tests doivent couvrir l'ensemble des fonctionnalités publiques de la solution (y compris l'import de type, fonction, enum, etc.).
> 
> Cela permets également de s'assurer de la bonne ergonomie de l'API.
### Implémentation de trait

L'exemple ci-dessous permet de créer un test permettant de s'assurer qu'une struct ou qu'une enum implémente bien un trait.

Ces tests sont un peu particuliers. En effet, si positionné dans un projet, alors ils peuvent empêcher le projet de se compiler s'il ne sont pas valide.

Voici un exemple de code permettant de s'assurer qu'une enum possède bien les Traits Send et Sync : 
```rust
enum Exemple {}

#[cfg(test)]
mod test{
	use super::*;
	
	fn send_sync_trait<T : Sendc + Sync>(){}
	
	#[test]
	fn test_traits_impl(){
		send_sync_trait::<Exemple>();
	}
}
```

> ### Recommandation {{#check TEST-TRAIT | Créer des tests permettant de s'assurer que certains traits sont bien implémenter pour des structures/enums}}
>
> Dans certains contextes, il est nécessaires que certaines struct ou enum implémente certains traits particuliers. 
> Dans ce genre de cas, il est vivement conseillé d'implémenter ce genre de test.
 
<!-- ## <mark>TODO</mark> : Fuzzing

### cargo-fuzz

<mark>TODO</mark> : bonnes pratiques pour fuzzer des programmes ou des parties
de programmes.
-->