# Introduction

[Rust](https://www.rust-lang.org) est un langage multiparadigmes orienté vers
la sûreté mémoire.

Il est entre autres orienté programmation système, en permettant une gestion
fine de la mémoire sans ramasse-miettes, mais également sans nécessiter
d'allocations et de désallocations manuelles, souvent sources d'erreurs et de
confusions. Le langage atteint ce but en introduisant un système
d'*ownership* (fortement lié à l'*aliasing* des variables). À tout point d'un
programme Rust, le compilateur recense les variables qui se réfèrent à une
même donnée, et applique un ensemble de règles qui permettent la récupération
automatique de la mémoire, la sûreté mémoire et l'absence de problèmes d'accès
concurrents.

Le langage est également axé sur la performance, avec des constructions
permettant des abstractions à coût nul et un compilateur proposant de puissantes
optimisations.

Enfin, le langage Rust fournit des fonctionnalités de programmation de
haut niveau. Grâce aux fonctions d'ordre supérieur, aux fermetures, aux
itérateurs, etc., il permet d'écrire tout ou parties des programmes dans un
style proche des langages de programmation fonctionnelle.
En outre, le typage statique, l'inférence de types et le polymorphisme
*ad hoc* (sous la forme de *traits*) sont d'autres moyens que Rust propose pour
construire des bibliothèques et des programmes de façon sûre.

Néanmoins, du fait de sa polyvalence, le langage offre des constructions et
fonctionnalités qui, si elles ne sont pas utilisées correctement, peuvent
potentiellement introduire des problèmes de sécurité, soit par construction,
soit en permettant d'écrire du code qui serait mal interprété par un développeur
ou un relecteur. De plus, comme pour la plupart des outils dans le domaine de la
compilation et de la vérification logicielles, les outils utilisés pour
développer, mettre au point, compiler et exécuter des programmes proposent des
options et des possibilités de configuration qui, si mal utilisées, peuvent
introduire des vulnérabilités.

L'objet de ce document est ainsi de rassembler une collection de conseils et de
recommandations pour rester autant que possible dans une zone sûre pour le
développement d'applications sécurisées, tout en profitant de la variété de
possibilités que le langage Rust peut offrir.

## Public visé

Ce guide compile une liste de recommandations qui doivent être observées pour
le développement d'applications aux besoins de sécurité élevés. Il peut
toutefois être suivi par quiconque souhaite s'assurer que les garanties offertes
par la plateforme Rust ne sont pas invalidées par l'utilisation d'une
fonctionnalité non sûre, trompeuse ou peu claire.

Il ne vise pas à constituer un cours sur la manière d'écrire des programmes en
Rust, il existe déjà une grande quantité de ressources de qualité sur le sujet
(par exemple, la [page principale de documentation de Rust](https://doc.rust-lang.org)).
L'intention est plutôt de guider le développeur et de l'informer à propos de
certains pièges. Ces recommandations forment un complément au bon niveau de
confiance que le langage Rust apporte déjà. Ceci étant dit, des rappels peuvent
parfois être nécessaires pour la clarté du discours, et le développeur Rust
expérimenté peut s'appuyer principalement sur le contenu des encarts (*Règle*,
*Recommandation*, *Avertissement*, etc.).

## Contributions

Ce guide est rédigé de manière collaborative et ouverte, via la plateforme
GitHub
([https://github.com/ANSSI-FR/rust-guide](https://github.com/ANSSI-FR/rust-guide)).
Toutes les contributions pour de futures versions sont les bienvenues, que ce
soit directement sous la forme de propositions (*pull requests*) ou bien de
suggestions et discussions (*issues*).

## Structure du document

La structure de ce document vise à discuter successivement des différentes
phases typiques (et simplifiées) d'un processus de développement. Tout d'abord,
nous proposons des recommandations concernant l'utilisation des outils de
l'écosystème Rust dans un cadre sécurisé. Ensuite, nous détaillons les
précautions à prendre durant le choix et l'utilisation de bibliothèques
externes. Ensuite, les recommandations à propos des constructions du langage
sont présentées. <!-- TODO: Enfin, nous discutons de la bonne utilisation des outils de
test et de *fuzzing* pour un projet réalisé en Rust.--> Un résumé des règles et
recommandations est disponible à la fin de ce guide.
