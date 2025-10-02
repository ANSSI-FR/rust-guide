---
references:
  - DOI: 10.1145/3418898
    ISSN: 2471-2566
    url: https://doi.org/10.1145/3418898
    author:
      - family: Papaevripides
        given: Michalis
      - family: Athanasopoulos
        given: Elias
    container-title: ACM Trans. Priv. Secur.
    id: mixed-bins
    issue: '2'
    issued:
      date-parts:
        - - 2021
          - 1
    keyword: CFI, Go, Memory safety, Rust, SafeStack
    publisher: Association for Computing Machinery
    publisher-place: New York, NY, USA
    title: Exploiting Mixed Binaries
    type: article-journal
    volume: '24'
---

# Compilation

## Durcissement et binaires mixtes

Les _durcissements_ sont des mécanismes mis en place pendant la compilation
permettant de réduire l'impact ou l'exploitabilité d'un certain nombre de défaut
de sûreté mémoire. Dans le cas de Rust, ces durcissements n'ont pas beaucoup
d'intérêt (hors code _unsafe_). Toutefois, la question se pose de nouveau dans
le cas de logiciel mixte, c'est-à-dire contenant des composants écrits en Rust
et des composants écrits dans un ou les langages n'assurant pas la sûreté
mémoire. En effet, il a été montré (par exemple dans [@mixed-bins]) que du code Rust peut être utilisé pour
contourner des durcissements d'un code C vulnérable.

<div class="reco" id="COMP-MIXED" type="Recommandation" title="Activer les durcissements pour tous les langages d'un logiciel mixte">

Dans le cadre du développement d'une application sécurisée comportant des
composants dans plusieurs langages, les compilations des composants (y compris
Rust) devraient appliquer des durcissements de manière à limiter
l'exploitabilité des vulnérabilités présents dans les composants dont le
langage n'assure pas la sûreté mémoire.

</div>

### Références

- _Exploiting Mixed Binaries_, Michalis Papaevripides, Elias Athanasopoulos, <https://dl.acm.org/doi/10.1145/3418898>