# Interfaçage avec des fonctions externes (FFI)

L'approche de Rust en ce qui concerne l'interfaçage avec des fonctions d'autres
langages repose sur une compatibilité forte avec le C. Toutefois, cette
frontière est par nature **non sûre** (voir [Rust Book: Unsafe Rust]).

[Rust Book: Unsafe Rust]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

Les fonctions marquées comme externes (mot-clé `extern`) sont rendues
compatibles avec du code C à la compilation. Elles peuvent être appelées depuis
un code C avec n'importe quelle valeur en argument. La syntaxe complète est
`extern "<ABI>"` où `"<ABI>"` décrit la convention d'appel et dépend de la
plateforme d'exécution visée. Par défaut, elle vaut `"C"`, ce qui correspond à
la manière standard en C d'appeler des fonctions.

```rust
// exportation d'une fonction compatible avec le C
#[no_mangle]
unsafe extern "C" fn mylib_f(param: u32) -> i32 {
    if param == 0xCAFEBABE { 0 } else { -1 }
}
```

Pour que la fonction `mylib_f` soit accessible avec le même nom, la fonction
doit être annotée avec l'attribut `#[no_mangle]`).

À l'inverse, il est possible d'appeler des fonctions écrites en C depuis du code
Rust si celles-ci sont déclarées dans un bloc `extern` :

```rust
use std::os::raw::c_int;
// importation d'une fonction externe de la libc
extern "C" {
    fn abs(args: c_int) -> c_int;
}

fn main() {
    let x = -1;
    println!("{} {}\n", x, unsafe { abs(x) });
}
```

> ### Note
>
> Toute fonction écrite dans un autre langage et importée dans Rust par l'usage
> d'un bloc `extern` est **automatiquement *unsafe***. C'est pourquoi tout
> appel à une telle fonction doit être fait dans un contexte `unsafe`.

Les blocs `extern` peuvent également contenir des déclarations de variables
globales externes, préfixées alors par le mot-clé `static` :

```rust
//! Un accès direct aux variables d'environnement (sur Unix).
//! Ne doit pas être utilisé ! Non *thread-safe*, voir `std::env` !

extern {
    // Variable globale de la libc
    #[link_name = "environ"]
    static libc_environ: *const *const std::os::raw::c_char;
}

fn main() {
    let mut next = unsafe { libc_environ };
    while !next.is_null() && !unsafe { *next }.is_null() {
        let env = unsafe { std::ffi::CStr::from_ptr(*next) }
            .to_str()
            .unwrap_or("<invalid>");
        println!("{}", env);
        next = unsafe { next.offset(1) };
    }
}
```

## Typage

Le typage est le moyen qu'utilise Rust pour assurer la sûreté mémoire. Lors de
l'interfaçage avec d'autres langages, qui n'offrent peut-être pas les mêmes
garanties, le choix des types lors du *binding* est essentiel pour maintenir
au mieux cette sûreté mémoire.

### Agencement des données

Rust ne fournit aucune garantie, que ce soit sur un court ou un long terme,
vis-à-vis de la façon dont sont agencées les données en mémoire. La seule
manière de rendre les données compatibles avec d'autres langages est
la déclaration explicite de la compatibilité avec le C, avec l'attribut `repr`
(voir [Rust Reference: Type Layout]). Par exemple, les types Rust suivants :

```rust
#[repr(C)]
struct Data {
    a: u32,
    b: u16,
    c: u64,
}
#[repr(C, packed)]
struct PackedData {
    a: u32,
    b: u16,
    c: u64,
}
```

sont compatibles avec les types C suivants :

```c
struct Data {
    uint32_t a;
    uint16_t b;
    uint64_t c;
};
__attribute__((packed))
struct PackedData {
    uint32_t a;
    uint16_t b;
    uint64_t c;
}
```

> ### Règle {{#check FFI-CTYPE | Utilisation exclusive de types compatibles avec le C dans les FFI}}
>
> Dans un développement sécurisé, seuls les types compatibles avec le C peuvent
> être utilisés comme argument ou type de retour des fonctions importées ou
> exportées et comme type de variables globales importées ou exportées.
>
> La seule exception à cette règle est l'utilisation de types considérés
> comme **opaques** du côté du langage externe.

Les types suivants sont considérés comme compatibles avec le C :

- les types primitifs entiers et à virgule flottante ;
- les `struct`s annotées avec `repr(C)` ;
- les `enum`s annotées avec `repr(C)` ou `repr(Int)` (où `Int` est un type
  primitif entier), contenant au moins un variant et dont tous les variants ne
  comportent pas de champ ;
- les pointeurs.

Les types suivants ne sont pas compatibles avec le C :

- les types à taille variable ;
- les `trait object`s ;
- les `enum`s dont les variants comportent des champs ;
- les n-uplets (sauf les `struct`s à n-uplet annotées avec `repr(C)`).

Certains types sont compatibles, mais avec certaines limitations :

- les types à taille nulle, qui ne sont pas spécifiés pour le C et mènent à des
  contradictions dans les spécifications du C++ ;
- les `enum`s avec champs annotés avec `repr(C)`, `repr(C, Int)` ou `repr(Int)`
  (voir [RFC 2195]).

[RFC 2195]: https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
[Rust Reference: Type Layout]: https://doc.rust-lang.org/reference/type-layout.html

### Cohérence du typage

> ### Règle {{#check FFI-TCONS | Utilisation de types cohérents pour les FFI}}
>
> Les types doivent être cohérents entre les deux côtés des frontières des FFI.
>
> Bien que certains détails peuvent être masqués de la part d'un côté envers
> l'autre (typiquement, pour rendre un type opaque), les types des deux parties
> doivent avoir la même taille et respecter le même alignement.

En ce qui concerne les `enum`s avec des champs en particulier, les types
correspondant en C (ou en C++) ne sont pas évidents ([RFC 2195]).

Les outils permettant de générer automatiquement des *bindings*, comme
[rust-bindgen] ou [cbindgen], peuvent aider à assurer la cohérence entre les
types du côté C et ceux du côté Rust.

> ### Recommandation {{#check FFI-AUTOMATE | Utilisation des outils de génération automatique de *bindings*}}
>
> Dans un développement sécurisé en Rust, les outils de génération automatique
> de *bindings* doivent être utilisés lorsque cela est possible, et ce en
> continu.

<!-- -->

> ### Attention
>
> Pour les *bindings* C/C++ vers Rust, [rust-bindgen] est capable de générer
> automatiquement des *bindings* de bas niveau. L'écriture d'un *binding* de
> plus haut niveau est fortement recommandée (voir Recommandation
> [FFI-SAFEWRAPPING](#FFI-SAFEWRAPPING)). Attention également à certaines
> options dangereuses de `rust-bindgen`, en particulier `rustified_enum`.

[rust-bindgen]: https://crates.io/crates/bindgen
[cbindgen]: https://crates.io/crates/cbindgen

### Types dépendants de la plateforme d'exécution

Lors de l'interfaçage avec un langage externe, comme C ou C++, il est souvent
nécessaire d'utiliser des types dépendants de la plateforme d'exécution, comme
les `int`s C, les `long`s, etc.

En plus du type `c_void` de `std::ffi` (ou `core::ffi`) pour le type C `void`,
la bibliothèque standard offre des alias de types portables dans `std::os::raw`
(or `core::os::raw`) :

- `c_char` pour `char` (soit `i8` ou bien `u8`) ;
- `c_schar` pour `signed char` (toujours `i8`) ;
- `c_uchar` pour `unsigned char` (toujours `u8`) ;
- `c_short` pour `short` ;
- `c_ushort` pour `unsigned short` ;
- `c_int` pour `int` ;
- `c_uint` pour `unsigned int` ;
- `c_long` pour `long` ;
- `c_ulong` pour `unsigned long` ;
- `c_longlong` pour `long long` ;
- `c_ulonglong` pour `unsigned long long` ;
- `c_float` pour `float` (toujours `f32`) ;
- `c_double` pour `double` (toujours `f64`).

La crate [libc] offre des types supplémentaires compatibles avec le C qui
couvrent la quasi-entièreté de la bibliothèque standard du C.

> ### Règle {{#check FFI-PFTYPE | Utilisation des alias portables `c_*` pour faire correspondre les types dépendants de la plateforme d'exécution}}
>
> Dans un développement sécurisé en Rust, lors de l'interfaçage avec du code
> faisant usage de types dépendants de la plateforme d'exécution, comme les
> `int`s et les `long`s du C, le code Rust doit utiliser les alias portables de
> types, comme ceux fournis dans la bibliothèque standard ou dans la crate
> [libc], au lieu des types spécifiques à la plateforme, à l'exception du cas
> où les *bindings* sont générés automatiquement pour chaque plateforme (voir
> la note ci-dessous).

<!-- -->

> ### Note
>
> Les outils de génération automatiques de *bindings* (par exemple [cbindgen] ou
> [rust-bindgen]) sont capables d'assurer la cohérence des types dépendants de
> la plateforme. Ils doivent être utilisés durant le processus de compilation
> pour chaque cible afin d'assurer que la génération est cohérente pour la
> plateforme visée.

[libc]: https://crates.io/crates/libc

### Types non-robustes : références, pointeurs de fonction, énumérations

Une *représentation piégeuse* d'un type particulier est une représentation
(motif d'octets) qui respecte les contraintes de représentation du type (telles
que sa taille et son alignement), mais qui ne représente pas une valeur valide
de ce type et mène à des comportements indéfinis.

En d'autres termes, si une telle valeur invalide est affectée à une variable
Rust, tout peut arriver ensuite, d'un simple *crash* à une exécution de code
arbitraire. Quand on écrit du code Rust sûr, ce genre de comportement ne peut
arriver (à moins d'un *bug* dans le compilateur Rust). Toutefois, en écrivant
du code Rust non sûr, et en particulier dans des FFI, cela peut facilement
avoir lieu.

Dans la suite, on appelle des **types non-robustes** les types dont les valeurs
peuvent avoir ces représentations piégeuses (au moins une). Beaucoup de types
Rust sont non-robustes, même parmi les types compatibles avec le C :

- `bool` (1 octet, 256 représentations, seules deux d'entre elles valides) ;
- les références ;
- les pointeurs de fonction ;
- les énumérations ;
- les flottants (même si de nombreux langages ont la même compréhension de ce
  qu'est un flottant valide) ;
- les types composés qui contiennent au moins un champ ayant pour type un type
  non-robuste.

De l'autre côté, les types entiers (`u*`/`i*`), les types composés *packés* qui
ne contiennent pas de champs de type non-robuste, sont par exemple des
*types robustes*.

Les types non-robustes engendrent des difficultés lors de l'interfaçage entre
deux langages. Cela revient à décider **quel langage des deux est le plus
responsable pour assurer la validité des valeurs hors bornes** et comment
mettre cela en place.

> ### Règle {{#check FFI-CKNONROBUST | Non-vérification des valeurs de types non-robustes}}
>
> Dans un développement sécurisé en Rust, toute valeur externe de type non-
> robuste doit être vérifiée.
>
> Plus précisément, soit une conversion (en Rust) est effectuée depuis des types
> robustes vers des types non-robustes à l'aide de vérifications explicites,
> soit le langage externe offre des garanties fortes quant à la validité des
> valeurs en question.

<!-- -->

> ### Recommandation {{#check FFI-CKINRUST | Vérification des valeurs externes en Rust}}
> 
> Dans un développement Rust sécurisé, la vérification des valeurs provenant
> d'un langage externe doit être effectuée du côté Rust lorsque cela est
> possible.

Ces règles génériques peuvent être adaptées à un langage externe spécifique ou
selon les risques associés. En ce qui concerne les langages, le C est
particulièrement inapte à offrir des garanties de validité. Toutefois, Rust
n'est pas le seul langage à offrir de telles possibilités. Par exemple, un
certain sous-ensemble de C++ (sans la réinterprétation) permet au développeur
de faire beaucoup dans ce domaine à l'aide du typage. Parce que Rust sépare
nativement les segments sûrs des segments non-sûrs, la recommandation est de
toujours utiliser Rust pour les vérifications lorsque c'est possible. En ce qui
concerne les risques, les types présentant le plus de dangers sont les
références, les références de fonction et les énumérations, qui sont discutées
ci-dessous.

> ### Attention
>
> Le type `bool` de Rust a été rendu équivalent au type `_Bool` (renommé `bool`
> dans `<stdbool.h>`) de C99 et au type `bool` de C++. Toutefois, charger une
> valeur différente de 0 ou 1 en tant que `_Bool`/`bool` est un comportement
> indéfini *des deux côtés*. La partie sûre de Rust assure ce fait. Les
> compilateurs C et C++ assurent qu'aucune autre valeur que 0 et 1 ne peut être
> *stockée* dans un `_Bool`/`bool` mais ne peuvent garantir l'absence d'une
> *réinterprétation incorrecte* (par exemple dans un type union ou *via* un
> *cast* de pointeur). Pour détecter une telle réinterprétation, un
> *sanitizer* tel que l'option `-fsanitize=bool` de LLVM peut être utilisé.

#### Références et pointeurs

Bien qu'autorisée par le compilateur Rust, l'utilisation des références Rust dans
une FFI peut casser la sûreté mémoire. Parce que leur côté non sûr est plus
explicite, les pointeurs sont préférés aux références Rust pour un interfaçage
avec un autre langage.

D'un autre côté, les types des références ne sont pas robustes : ils permettent
seulement de pointer vers des objets valides en mémoire. Toute déviation mène à
des comportements indéfinis.

> ### Règle {{#check FFI-CKREF | Vérification des références provenant d'un langage externe}}
>
> Dans un développement sécurisé en Rust, les références externes transmises au
> côté Rust par le biais d'une FFI doivent être **vérifiées du côté du langage
> externe**, que ce soit de manière automatique (par exemple, par un
> compilateur) ou de manière manuelle.
>
> Les exceptions comprennent les références Rust *wrappées* de façon opaque et
> manipulées uniquement du côté Rust, et les références *wrappées* dans un type
> `Option` (voir note ci-dessous).

Lors d'un *binding* depuis et vers le C, le problème peut être particulièrement
sévère, parce que le langage C n'offre pas de références (dans le sens de
pointeurs valides) et le compilateur n'offre pas de garantie de sûreté.

Lors d'un *binding* avec le C++, les références Rust peuvent en pratique être
liées aux références C++ bien que l'ABI d'une fonction `extern "C"` en C++ avec
des références soit défini par l'implémentation. Enfin, le code C++ doit être
vérifié pour éviter toute confusion de pointeurs et de références.

Les références Rust peuvent être raisonnablement utilisées avec d'autres
langages compatibles avec le C, incluants les variantes de C qui mettent en
oeuvre la vérification que les pointeurs sont non nuls, comme du code annoté à
l'aide Microsoft SAL par exemple.

> ### Recommandation {{#check FFI-NOREF | Non-utilisation des types références et utilisation des types pointeurs}}
>
> Dans un développement sécurisé en Rust, le code Rust ne doit pas utiliser de
> types références, mais des types pointeurs.
>
> Les exceptions sont :
>
> - les références qui sont opaques dans le langage externe et qui sont
>   seulement manipulées du côté Rust ;
> - les références *wrappées* dans un type `Option` (voir note ci-dessous) ;
> - les références liées à des références sûres dans le langage externe, par
>   exemple dans des variantes du C ou dans du code compilé en C++ dans un
>   environnement où les références de fonctions `extern "C"` sont encodées
>   comme des pointeurs.

D'un autre côté, les *types pointeur* Rust peuvent aussi mener à des
comportements indéfinis, mais sont plus aisément vérifiables, principalement
par la comparaison avec `std/code::ptr::null()` (`(void*)0` en C), mais aussi
dans certains contextes par la vérification de l'appartenance à une plage
d'adresses mémoire (en particulier dans des systèmes embarqués ou pour un
développement au niveau noyau). Un autre avantage à utiliser les pointeurs Rust
dans des FFI est que tout chargement de valeur pointée est clairement marqué
comme appartenant à un bloc ou à une fonction `unsafe`.

> ### Règle {{#check FFI-CKPTR | Vérification des pointeurs externes}}
>
> Dans un développement sécurisé en Rust, tout code Rust qui déréférence un
> pointeur externe doit vérifier sa validité au préalable.
> En particulier, les pointeurs doivent être vérifiés comme étant non nuls avant
> toute utilisation.
>
> Des approches plus strictes sont recommandées lorsque cela est possible. Elles
> comprennent la vérification des pointeurs comme appartenant à une plage
> d'adresses mémoire valides ou comme étant des pointeurs avérés (étiquetés ou
> signés). Cette approche est particulièrement applicable si la valeur pointée
> est seulement manipulée depuis le code Rust.

Le code suivant est un simple exemple d'utilisation de pointeur externe dans une
fonction Rust exportée :

```rust,noplaypen
/// Ajout en place
#[no_mangle]
pub unsafe extern fn add_in_place(a: *mut u32, b: u32) {
    // Vérification du caractère non nul de `a`
    // et manipulation comme une référence mutable
    if let Some(a) = a.as_mut() {
        *a += b
    }
}
```

Il faut noter que les méthodes `as_ref` et `as_mut` (pour les pointeurs
mutables) permettent d'accéder facilement à la référence tout en assurant une
vérification du caractère non nul de manière très idiomatique en Rust. Du côté
du C, la fonction peut alors être utilisée comme suit :

```c
#include <stdint.h>
#include <inttypes.h>

//! Ajout en place
void add_in_place(uint32_t *a, uint32_t b);

int main() {
    uint32_t x = 25;
    add_in_place(&x, 17);
    printf("%" PRIu32 " == 42", x);
    return 0;
}
```

> ### Note
>
> Les valeurs de type `Option<&T>` ou `Option<&mut T>`, pour tout T tel que
> `T: Sized`, sont admissibles dans un FFI à la place de pointeurs avec
> comparaison explicite avec la valeur nulle. En raison de la garantie de Rust
> vis-à-vis des optimisations de pointeurs pouvant être nuls, un pointeur nul
> est acceptable du côté C. La valeur C `NULL` est comprise par Rust comme la
> valeur `None`, tandis qu'un pointeur non nul est encapsulé dans le
> constructeur `Some`. Bien qu'ergonomique, cette fonctionnalité ne permet par
> contre pas des validations fortes des valeurs de pointeurs comme
> l'appartenance à une plage d'adresses mémoire valides.

#### Pointeurs de fonction

Les pointeurs de fonction qui traversent les frontières d'une FFI peuvent mener
à de l'exécution de code arbitraire et impliquent donc des risques réels de
sécurité.

> ### Règle {{#check FFI-MARKEDFUNPTR | Marquage des types de pointeurs de fonction dans les FFI comme `extern` et `unsafe`}}
>
> Dans un développement sécurisé en Rust, tout type de pointeur de fonction dont
> les valeurs sont amenées à traverser les frontières d'une FFI doit être
> marqué comme `extern` (si possible avec l'ABI spécifiée) et comme `unsafe`.

Les pointeurs de fonction en Rust ressemblent bien plus aux références qu'aux
pointeurs simples. En particulier, la validité des pointeurs de fonction ne peut
pas être vérifiée directement du côté Rust. Toutefois, Rust offre deux
alternatives possibles :

- l'utilisation de pointeurs de fonctions *wrappé* dans une valeur de type
  `Option`, accompagnée d'un test contre la valeur nulle :

  ```rust,noplaypen
  #[no_mangle]
  pub unsafe extern "C" fn repeat(start: u32, n: u32, f: Option<unsafe extern "C" fn(u32) -> u32>) -> u32 {
      if let Some(f) = f {
          let mut value = start;
          for _ in 0..n {
              value = f(value);
          }
          value
      } else {
          start
      }
  }
  ```

  Du côté C :

  ```c
  uint32_t repeat(uint32_t start, uint32_t n, uint32_t (*f)(uint32_t));
  ```

- l'utilisation de pointeurs *bruts* avec une transformation `unsafe` vers un
  type pointeur de fonction, permettant des tests plus poussés au prix de
  l'ergonomie.

> ### Règle {{#check FFI-CKFUNPTR | Vérification des pointeurs de fonction provenant d'une FFI}}
>
> Dans un développement sécurisé en Rust, tout pointeur de fonction provenant de
> l'extérieur de l'écosystème Rust doit être vérifié à la frontière des FFI.

Lors d'un *binding* avec le C ou encore le C++, il n'est pas simple de garantir
la validité d'un pointeur de fonction. Les foncteurs C++ ne sont pas compatibles
avec le C.

#### Enumérations

Les valeurs (motifs de bits) valides pour une énumération donnée sont en général
assez peu nombreuses par rapport à l'ensemble des valeurs qu'il est possible
d'exprimer avec le même nombre de bits. Ne pas traiter correctement une valeur
d'`enum` fournie par un code externe peut mener à une confusion de types et
avoir de sérieuses conséquences sur la sécurité d'un programme. Malheureusement,
vérifier la valeur d'une énumération aux bornes d'une FFI n'est pas une tâche
triviale des deux côtés.

Du côté Rust, cette vérification consiste à utiliser un type entier dans la
déclaration du bloc `extern`, un type *robuste* donc, et d'effectuer une
conversion contrôlée vers le type `enum`.

Du côté externe, cela est possible uniquement si l'autre langage permet la mise
en place de tests plus stricts que ceux proposés en C. C'est par exemple
possible en C++ avec les `enum class`. Notons toutefois pour référence que
l'ABI `extern "C"` d'une `enum class` est définie par l'implémentation et doit
être vérifiée pour chaque environnement d'exécution.

> ### Recommandation {{#check FFI-NOENUM | Non-utilisation d'`enum`s Rust provenant de l'extérieur par une FFI}}
>
> Dans un développement sécurisé en Rust, lors de l'interfaçage avec un
> langage externe, le code Rust ne doit pas accepter de valeurs provenant de
> l'extérieur en tant que valeur d'un type `enum`.
>
> Les exceptions incluant des types `enum` Rust sont :
>
> - les types opaques du langage externe dont les valeurs sont uniquement
>   manipulées du côté Rust ;
> - les types liés à des types d'énumération sûrs du côté du langage externe,
>   comme les `enum class` de C++ par exemple.

Concernant les énumérations ne contenant aucun champ, des *crates* comme
[`num_derive`] ou [`num_enum`] permettent au développeur de fournir facilement
des opérations de conversions sûres depuis une valeur entière vers une
énumération et peuvent être utilisées pour convertir de manière contrôlée un
entier (fourni par une énumération C) vers une énumération C.

[num_derive]: https://crates.io/crates/num_derive
[num_enum]: https://crates.io/crates/num_enum

### Types opaques

Rendre opaques des types est une bonne méthode pour augmenter la modularité dans
un développement logiciel. C'est notamment une pratique assez courante dans un
développement impliquant plusieurs langages de programmation.

> ### Recommandation {{#check FFI-R-OPAQUE | Utilisation de types Rust dédiés pour les types opaques externes}}
>
> Dans un développement sécurisé en Rust, lors d'un *binding* avec des types
> opaques externes, des pointeurs vers des types opaques dédiés doivent être
> utilisés au lieu de pointeurs `c_void`.

La pratique recommandée pour récupérer des valeurs externes de type opaque est
illustrée comme suit :

```rust,unsafe,noplaypen
#[repr(C)]
pub struct Foo {_private: [u8; 0]}
extern "C" {
    fn foo(arg: *mut Foo);
}
```
La proposition [RFC 1861], non implémentée à la rédaction de ce guide, propose
de faciliter cette situation en permettant de déclarer des types opaques dans
des blocs `extern`.

[RFC 1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html

> ### Recommandation {{#check FFI-C-OPAQUE | Utilisation de pointeurs vers des `struct`s C/C++ pour rendre des types opaques}}
>
> Dans un développement sécurisé en Rust, lors de l'interfaçage avec du C ou du
> C++, les valeurs de types Rust considérés comme opaques dans la partie C/C++
> doivent être transformées en valeurs de type `struct` incomplet (c'est-à-dire
> déclaré sans définition) et être fournies avec un constructeur et un
> destructeur dédiés.

Un exemple d'utilisation de type opaque Rust :

```rust,unsafe,noplaypen
# use std::panic::catch_unwind;
#
struct Opaque {
    // (...) détails à cacher
}

#[no_mangle]
pub unsafe extern "C" fn new_opaque() -> *mut Opaque {
    catch_unwind(|| // Catch panics, see below
        Box::into_raw(Box::new(Opaque {
            // (...) construction
        }))
    ).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_opaque(o: *mut Opaque) {
    catch_unwind(||
        if !o.is_null() {
            drop(Box::from_raw(o))
        }
    ); // nécessaire seulement si `Opaque` ou un de ses champs est `Drop`
}
```

## Mémoire et gestion des ressources

Les langages de programmation ont de nombreuses façons de gérer la mémoire. En
résultat, il est important de savoir quel langage est responsable de la
réclamation de l'espace mémoire d'une donnée lorsqu'elle est échangée entre Rust
et un autre langage. Il en va de même pour d'autres types de ressources comme
les descripteurs de fichiers ou les *sockets*.

Rust piste le responsable ainsi que la durée de vie des variables pour
déterminer à la compilation si et quand la mémoire associée doit être libérée.
Grâce au trait `Drop`, il est possible d'exploiter ce système pour récupérer
toutes sortes de ressources comme des fichiers ou des accès au réseau.
*Déplacer* une donnée depuis Rust vers un langage signifie également abandonner
de possibles réclamations de la mémoire qui lui est associée.

> ### Règle {{#check FFI-MEM-NODROP | Non-utilisation de types qui implémentent `Drop` dans des FFI}}
>
> Dans un développement sécurisé en Rust, le code Rust ne doit pas implémenter
> `Drop` pour les valeurs de types qui sont directement transmis à du code
> externe (c'est-à-dire ni par pointeur, ni par référence).

En fait, il est même recommandé de n'utiliser que des types qui implémentent
`Copy`. Il faut noter que `*const T` est `Copy` même si `T` ne l'est pas.

Si ne pas récupérer la mémoire et les ressources est une mauvaise pratique, en
termes de sécurité, utiliser de la mémoire récupérée plus d'une fois ou libérer
deux fois certaines ressources peut être pire. Afin de libérer correctement une
ressource une seule et unique fois, il faut savoir quel langage est responsable
de la gestion de son allocation et de sa libération.

> ### Règle {{#check FFI-MEM-OWNER | Identification du langage responsable de la libération des données dans les FFI}}
>
> Dans un développement sécurisé en Rust, lorsqu'une donnée, quel que soit son
> type, est échangée par une FFI, il est nécessaire de s'assurer que :
>
> - un seul langage est responsable de l'allocation et de la libération d'une
>   donnée ;
> - l'autre langage ne doit ni allouer, ni libérer la donnée directement, mais
>   peut utiliser une fonction externe dédiée fournie par le langage responsable
>   choisie.

L'identification d'un langage responsable de la gestion des données en mémoire
ne suffit pas. Il reste à s'assurer de la durée de vie correcte de ces données,
principalement qu'elles ne sont plus utilisées après leur libération. C'est une
étape bien plus difficile. Lorsque le langage externe est responsable de la
mémoire, la même approche est de fournir un *wrapper* sûr autour du type
externe.

> ### Recommandation {{#check FFI-MEM-WRAPPING | Encapsulation des données externes dans un type `Drop`}}
>
> Dans un développement sécurisé en Rust, toute donnée à caractère non sensible
> allouée et libérée du côté du langage externe doit être encapsulée dans un
> type implémentant `Drop`, de telle sorte que Rust fournisse l'appel
> automatique au destructeur Rust.

Voici un simple exemple d'encapsulation autour d'un type opaque externe :


```rust,ignore,noplaypen
# use std::ops::Drop;
#
/// Type Foo privé, "raw", opaque, externe
#[repr(C)]
struct RawFoo {
    _private: [u8; 0],
}

/// API C privée "raw"
extern "C" {
    fn foo_create() -> *mut RawFoo;
    fn foo_do_something(this: *const RawFoo);
    fn foo_destroy(this: *mut RawFoo);
}

/// Foo
pub struct Foo(*mut RawFoo);
#
impl Foo {
    /// Création d'une valeur Foo
    pub fn new() -> Option<Foo> {
        let raw_ptr = unsafe { foo_create() };
        if raw_ptr.is_null() {
            None
        } else {
            Some(Foo(raw_ptr))
        }
    }
#
    /// Utilisation de Foo
    pub fn do_something(&self) {
        unsafe { foo_do_something(self.0) }
    }
}
#
impl Drop for Foo {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { foo_destroy(self.0) }
        }
    }
}

# fn main() {
#     let foo = Foo::new().expect("cannot create Foo");
#     foo.do_something();
# }
```

> ### Attention
>
> Parce que des `panic`s peuvent mener à ne pas exécuter la méthode
> `Drop::drop`, cette solution n'est pas satisfaisante pour le cas de la
> libération de ressources sensibles (pour effacer les données sensibles par
> exemple), à moins que le code soit garanti exempt de `panic` potentiel.
>
> Pour le cas de l'effacement des données sensibles, le problème peut être géré
> par l'utilisation d'un *handler* de `panic`.

Lorsque le langage externe exploite des ressources allouées depuis le côté Rust,
il est encore plus difficile d'offrir quelque garantie qui soit.

En C par exemple, il n'y a pas de moyen simple qui permette de vérifier que le
destructeur correspondant est appelé. Il est possible d'utiliser des *callbacks*
pour assurer que la libération est effectivement faite.

Le code Rust suivant est un exemple ***unsafe* du point de vue des threads**
d'une API compatible avec le C qui fournit une *callback* pour assurer la
libération d'une ressource :

```rust,noplaypen
# use std::ops::Drop;
#
pub struct XtraResource { /* champs */ }

impl XtraResource {
    pub fn new() -> Self {
        XtraResource { /* ... */ }
    }
    pub fn dosthg(&mut self) {
        /* ... */
    }
}

impl Drop for XtraResource {
    fn drop(&mut self) {
        println!("xtra drop");
    }
}

pub mod c_api {
    use super::XtraResource;
    use std::panic::catch_unwind;

    const INVALID_TAG: u32 = 0;
    const VALID_TAG: u32 = 0xDEAD_BEEF;
    const ERR_TAG: u32 = 0xDEAF_CAFE;

    static mut COUNTER: u32 = 0;

    pub struct CXtraResource {
        tag: u32, // pour prévenir d'une réutilisation accidentelle
        id: u32,
        inner: XtraResource,
    }

    #[no_mangle]
    pub unsafe extern "C" fn xtra_with(cb: extern "C" fn(*mut CXtraResource) -> ()) {
        let inner = if let Ok(res) = catch_unwind(XtraResource::new) {
            res
        } else {
#             println!("impossible d'allouer la ressource");
            return;
        };
        let id = COUNTER;
        let tag = VALID_TAG;

        COUNTER = COUNTER.wrapping_add(1);
        // Utilisation de la mémoire du tas pour ne pas fournir de pointeur de
        // pile au code C!
        let mut boxed = Box::new(CXtraResource { tag, id, inner });

#         println!("running the callback on {:p}", boxed.as_ref());
        cb(boxed.as_mut() as *mut CXtraResource);

        if boxed.id == id && (boxed.tag == VALID_TAG || boxed.tag == ERR_TAG) {
#             println!("freeing {:p}", boxed.as_ref());
            boxed.tag = INVALID_TAG; // prévention d'une réutilisation accidentelle
                                 // drop implicite de la `box`
        } else {
#             println!("oubli de {:p}", boxed.as_ref());
            // (...) gestion des erreurs (partie critique)
            boxed.tag = INVALID_TAG; // prévention d'une réutilisation
            std::mem::forget(boxed); // boxed is corrupted it should not be
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn xtra_dosthg(cxtra: *mut CXtraResource) {
        let do_it = || {
            if let Some(cxtra) = cxtra.as_mut() {
                if cxtra.tag == VALID_TAG {
#                     println!("doing something with {:p}", cxtra);
                    cxtra.inner.dosthg();
                    return;
                }
            }
            println!("ne fait rien avec {:p}", cxtra);
        };
        if catch_unwind(do_it).is_err() {
            if let Some(cxtra) = cxtra.as_mut() {
#                 println!("panic avec {:p}", cxtra);
                cxtra.tag = ERR_TAG;
            }
        };
    }
}
#
# fn main() {}
```

Un appel C compatible :

```c
struct XtraResource;
void xtra_with(void (*cb)(XtraResource* xtra));
void xtra_sthg(XtraResource* xtra);

void cb(XtraResource* xtra) {
    // ()...) do anything with the proposed C API for XtraResource
    xtra_sthg(xtra);
}

int main() {
    xtra_with(cb);
}
```

## `Panic`s et code externe

Lors de l'appel à du code Rust depuis un autre langage (par exemple, du C), le
code Rust ne doit pas provoquer de `panic`. Dérouler (*unwinding*) depuis le
code Rust dans du code externe résulte en un **comportement indéfini**.

> ### Règle {{#check FFI-NOPANIC | Gestion correcte des `panic`s dans les FFI}}
>
> Le code Rust appelé depuis un langage externe doit soit s'assurer que la
> fonction ne peut pas provoquer de `panic`, soit utiliser un mécanisme de
> récupération de `panic` (comme `std::panic::catch_unwind`,
> `std::panic::set_hook`, `#[panic_handler]`), afin d'assurer que la fonction
> Rust ne peut pas quitter ou retourner dans un état instable.

Il faut noter que `catch_unwind` rattrapera seulement les *unwinding `panic`s*
mais pas ceux provoquant un arrêt du processus.

```rust,unsafe,noplaypen,ignore
use std::panic::catch_unwind;
# use rand;

fn may_panic() {
    if rand::random() {
        panic!("panic happens");
    }
}

#[no_mangle]
pub unsafe extern "C" fn no_panic() -> i32 {
    let result = catch_unwind(may_panic);
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
```

### `no_std`

Dans le cas des programmes n'utilisant pas la bibliothèque standard
Rust (`#[no_std]`), un gestionnaire de `panic` (̀`#[panic_handler]`) doit être
défini pour la sécurité du programme. Le gestionnaire de `panic` doit être écrit
avec la plus grande précaution pour garantir non seulement la sécurité, mais
aussi la sûreté du programme.

Un approche alternative est de simplement s'assurer qu'il n'y a aucune
utilisation de `panic!` avec la *crate* [`panic-never`]. Comme [`no-panic`],
[`panic-never`] repose sur une astuce au moment de l'édition de liens : le
programme d'édition de liens échoue si une branche non trivialement
inaccessible mène à un appel à `panic!`.

[`panic-never`]: https://crates.io/crates/panic-never
[`no-panic`]: https://crates.io/crates/no-panic

## Liaison d'une bibliothèque externe à du code Rust

> ### Recommandation {{#check FFI-SAFEWRAPPING | Mise en place d'une encapsulation sûre pour les bibliothèques externes}}
>
> L'interfaçage entre une bibliothèque écrite dans un autre langage et
> du code Rust doit être réalisé en deux parties :
>
> - un module bas-niveau, potentiellement *caché*, qui traduit de façon très
>   proche l'API originale en des blocs `extern` ;
> - un module qui assure la sûreté mémoire et les invariants de sécurité au
>   niveau de Rust.
>
> Si l'API bas-niveau est exposée, cela doit être fait dans un *crate* dédiée
> ayant un nom de la forme `*-sys`.

La *crate* [rust-bindgen] peut être utilisée pour générer automatiquement la
partie bas-niveau du *binding* depuis les fichiers *header* C.

<!--
<mark>TODO</mark> example
-->

## Liaison entre une bibliothèque Rust et du code d'un autre langage

> ### Recommandation {{#check FFI-CAPI | Exposition exclusive d'API dédiée et compatible avec le C}}
>
> Dans un développement sécurisé en Rust, exposer un bibliothèque Rust à un
> langage externe doit être uniquement fait par le biais d'une **API dédiée et
> compatible avec le C**.

La *crate* [cbindgen] peut être utilisée pour générer automatiquement les
*bindings* C ou C++ pour l'API Rust compatible avec le C d'une bibliothèque
Rust.

### Exemple minimal d'une bibliothèque Rust exportée vers du C

`src/lib.rs`:

```rust,noplaypen
/// Compteur opaque
pub struct Counter(u32);

impl Counter {
    /// Crée un compteur (initialisé à 0)
    fn new() -> Self {
        Self(0)
    }
    /// Récupère la valeur courante du compteur
    fn get(&self) -> u32 {
        self.0
    }
    /// Incrémente la valeur du compteur s'il n'y a pas de dépassement
    fn incr(&mut self) -> bool {
        if let Some(n) = self.0.checked_add(1) {
            self.0 = n;
            true
        } else {
            false
        }
    }
}

// API compatible avec le C

#[no_mangle]
pub unsafe extern "C" fn counter_create() -> *mut Counter {
    Box::into_raw(Box::new(Counter::new()))
}

#[no_mangle]
pub unsafe extern "C" fn counter_incr(counter: *mut Counter) -> std::os::raw::c_int {
    if let Some(counter) = counter.as_mut() {
        if counter.incr() {
            0
        } else {
            -1
        }
    } else {
        -2
    }
}

#[no_mangle]
pub unsafe extern "C" fn counter_get(counter: *const Counter) -> u32 {
    if let Some(counter) = counter.as_ref() {
        return counter.get();
    }
    return 0;
}

#[no_mangle]
pub unsafe extern fn counter_destroy(counter: *mut Counter) -> std::os::raw::c_int {
    if !counter.is_null() {
        let _ = Box::from_raw(counter); // get box and drop
        return 0;
    }
    return -1;
}
```

En utilisant [cbindgen] (`[cbindgen] -l c > counter.h`), il est possible de
générer un *header* C cohérent, `counter.h` :

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Counter Counter;

Counter *counter_create(void);

int counter_destroy(Counter *counter);

uint32_t counter_get(const Counter *counter);

int counter_incr(Counter *counter);
```

`counter_main.c`:

```c
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

#include "counter.h"

int main(int argc, const char** argv) {
    if (argc < 2) {
        return -1;
    }
    size_t n = (size_t)strtoull(argv[1], NULL, 10);

    Counter* c = counter_create();
    for (size_t i=0; i < n; i++) {
        if (counter_incr(c) != 0) {
            printf("overflow\n");
            counter_destroy(c);
            return -1;
        }
    }

    printf("%" PRIu32 "\n", counter_get(c));
    counter_destroy(c);

    return 0;
}
```
