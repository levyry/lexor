# Notes

## paradigmaváltozás

- Már az OO paradigma térnyerése előtt sok funkcionális nyelv volt
  - LISP: 1960as években jelent meg, és nagyon sok mindenben az első volt (GC, első osztályú függvények)
  - ML: a funkcionális programozás nagyapja; a korhoz képest nagyon modern típusrendszer, mintaillesztés
- A 80as-90es évekre viszont megjelent az OOP
  - C++-ban már osztályok, öröklés, polimorfizmus, viszont a designban ott vannak a procedurális C nyomai
  - Java a leghíresebb OO nyelv, ami már szín tisztán csak OO-ra alapozott
- A 2010-es évek után viszont kiábrándultak az emberek a pure OO-ból
  - 2011-es C++ szabványban már lambdák és constexpr kulcsszó
  - 2020-as évekre pedig már a tradícionálisan "OO" nyelvek is elkezdtek egyre több régi funkcionális elemet beemelni, pl rekordok, mintailleszté
  - 2025-re odáig jutottunk, hogy a több mint 30 éves Linux kernelbe egy olyan nyelvet vettek be, ami már kimondottan multiparadigmális szemlélettel lett designolva a 2010-es évek második felében

- A legtöbb új nyelv múr multiparadigmális, mint például a Rust, Kotlin
- És azok a nyelvek, amik eredetileg OO alapokra lettek designolva szintén kezdenek multiparadigmálissá válni
- Elvárjuk a mérnökeinktől, hogy jó minőségű kódot írjanak ezekben a nyelvekben, amelyekhez manapság már elengedhetetlenek a funkcionális elemek, mint például streams, mintaillesztés
- Ezeknek a fogalmaknak a matematikai alapjait kevesen ismerik, kevés gyakorlatuk van vele

- Az én projektem pedig pont ezt a problémát szeretné orvosolni

## feladat

- A feladatom az volt, hogy egy webes eszközt készítsek (így megnövelve az elérhetőséget)
- amely úgynevezett SKI-kifejezés redukciós lépéseit mutatja meg, illetve kiértékelési fát rajzol fel
- ez lényegében nem más, mint egy (egyszerűsített) funkcionális programozási nyelv interpreterének az implementálása, úgy, hogy közben az interpreter belső működését vizualizálnom kell
- emellett különböző kiértékelési stratégiákat is támogatnia kell a programnak, hogy különböző viselkedési mintákat össze tudjunk hasonlítani

## kezdeti állapot

- előző félévben a Funkcionális programozás C++-ban szabadon választható tárgy keretein belül hallottam először mélyebben a lambda kalkulusról
- az önálló laboratóriumi feladat megkezdése előtt pedig Stephen Wolfram írásaiból ismerkedtem meg az SKI-kalkulussal
    - igen, ez ugyan az a Wolfram, aki a WolframAlpha-t csinálta.

## két kalkulus

- beszéljünk egy kicsit a két kalkulusról. mik a szabályaik, hogyan működnek.

- a lambda kalkulus egy általános célú turing teljes programozási nyelv.
- ez nem azt jelenti, hogy sokan programoznak is benne, hiszen ez a nyelv inkább matematikai jelentőséggel bír, de meglepően sok olyan funkcionális nyelv van, ami csak minimálisan változtat a lambda kalkuluson
- három fő konstrukció van a nyelvben:
  - változók, ezek ugyan azok mint minden más nyelvben
  - absztrakciók, amik a függvény definícióknak felelnek meg
  - applikációk, ami a függvényhívásnak felel meg
  - és ennyi! az olyan hasznos konstrukciók mint a számok, alapműveletek, if-else, logikai kapuk, stb azok mind absztrakciókként definiálhatók, ezért nem kell külön axiomatikusan kimondani a létezésüket

- alább láthattok egy példát egy lambda függvényre ez egy kétváltozós függvény, ami összeadja a két paraméterét.
- mellesleg megjegyzem, hogy pythonban nagyon hasonlóan lehet lambda függvényeket definiálni. ott a görög lambda betű helyett angolul kell kiírni, hogy "lambda", és a pont helyett pedig kettőspontot kell tenni, viszont ezen kívül ugyan úgy működik mint a lambda kalkulus

- az SKI kalkulus sok szempontból hasonlít a lambda kalkulushoz, de még inkább letisztultabb szeretne lenni
- szintén turing teljes, viszont itt már sokkal kevesebb eszközzel dolgozunk. ugyanis itt nem adhatunk meg tetszőlegesen bármilyen absztrakciót (azaz függvényt) oly könnyen, hanem 3 explicit definiált függvényből, más szóval kombinátorból kell felépítenünk őket applikációk alkalmazásával.
- ezáltal teljesen változómentes: csak függvényeket hívunk meg más függvényekkel, és az eredményül kapott függvényeket más függvényekkel, stb...
- alább látható a három kombinátornak a lambda kalkulus beli definíciója. ha ezeket a függvényeket sokszor egymással meghívjuk, akkor bármilyen más függvény, vagy program viselkedése leírható velük

## architektúra

- a projekthez a Rust nyelv mellett döntöttem ezeknek a kombinátor programoknak a kiértékelése nagyon számításigényes folyamat tud lenni, és fontos volt, hogy a webes alkalmazás minél gyorsabban meg tudja jeleníteni a compiler belső állapotát a képernyőn

- a program frontendje (ami egy webes felület lesz) az az úgynevezett elm arhcitektúrát követi
  - ebben az architektúrában három fő elem van:
  - a UI, amivel a felhasználó interakcióba lép. ezek az interakciók, mint például gombok lenyomása, vagy a kurzor mozgatása "üzeneteket" eredményeznek
  - ezeket az üzeneteket az `update` függvény dolgozza fel. ezek általában valami belső adat reprezentáción változtatnak. a változtatások hatására egy új belső model jön létre, amit a `view` függvény meg tud jeleníteni a felhasználónak.

  - a UI az egy message-queueba teszi az üzeneteket, vagy parancsokat, amit az update minden frame után feldolgoz

  - továbbá fontos szerep volt még az, hogy a teljesen kliensoldalon működjön az alkalmazás. ezt úgy értem el, hogy a teljes programot WASM-re fordítottam, így a felhasználó böngészőjében tud futni

  - viszont azért, hogy a UI ne fagyjon be, a redukciós számításokat egy Web Workerbe tettem, ami így egy külön szálra kerül

- a backend az egy compiler felépítését követte
  - a compilernek is van egy frontendje, ez az, amikor beolvassuk a lambda vagy ski kifejezéseket, parse-oljuk őket
  - a compiler backend pedig egy G-machine alapú redukciós gép. emellett az architektúra kialakításánál külön figyelmet kellett szánnom annak, hogy a kiértékelési folyamat egyes lépései nyomonkövethetőek legyenek

- a webes frontend és a backend között egy API réteg található, amely a Facade azaz Homlokzat mintát valósítja meg. 
- ennek az a lényege, hogy egy alrendszerről egy egyszerűsített, minimális API réteget biztosítunk.
- a webes frontendnek nem kell tudnia, hogy milyen sorrendben kell meghívni milyen compileres függvényeket, neki elég csak egy nagy `kiertekeles` függvény, aminek odaadja az inputot, és vissza kapja mindent ami kell ahhoz, hogy meg tudja jeleníteni a belső állapotokat

- itt van egy egyszerűsített diagram, amely bemutatja az architektúrát. látható rajta a frontendben a UI update view körforgás, illetve a backenden belül a parser és a redukciós gépek is. külön kellett egy a lambda kalkulusnak, külön egy az SKI kalkulusnak.

## redukciós gépek

- ez a lényegi része a projektnek. a G-machine a graph reduction machine rövidítése
- itt a "gráf" az maga az absztrakt szintaxis fa: a kombinátoros programunk kifejezései ugyan úgy egy fa struktúrába szervezhetőek.
- ekkor a levelek, azok maguk a kombinátorok, a belső csúcsok pedig az "applikációk", azaz hogy melyik kombinátornak melyik másik kombinátorokat adjuk meg bemenetként
- ekkor a gráfon magán módosítunk menet közben, csúcsokat törlünk ki, éleket rajzolunk át, stb
- ez maga a tankönyvi algoritmus

- viszont ezen sok szempontból tudunk javítani
- egy könnyen észrevehető optimalizáció lehet egy új fajta csúcs bevezetése a gráfban: az indirekciós csúcs
  - ez lényegében úgy viselkedik mint egy pointer: ha a gráfunkban van egy olyan részfa, amely megismétlődne, akkor a teljes deep clone helyett elég, ha csak egy pointerrel hivatkozunk rá
  
  - ez a kiértékelés végeredményén nem változtat, sőt nagyon is felgyorsítja a folyamatot, hiszen ha egy részfa mondjuk 5 helyen szerepel a kifejezésünkben, akkor mind az öt helyen egyszerre redukálódik

- maga az adatszerkezet megválasztása is egy fontos döntés. ugye gráfokkal kell dolgoznunk, de nagyon sok féle módon tudunk egy gráfot ábrázolni
- hagyományosan egy rekurzív definíciót adunk egy csúcsra, ahol pointerekkel tud hivatkozni a szüleire, gyerekeire.
  - ezekből a csúcsokból építjük fel az absztrakt szintaxisfát
  - ezzel a megközelítéssel viszont több probléma is van
  - az egyik a pointer chasing:
    - képzeljük el, hogy egy csúcsnak az 5. nagyszülejéhez kéne eljutnunk.
    - ekkor fogjuk a jelenlegi csúcsban a pointerünket, dereferáljuk, utána a szülő csúcsban is dereferáljuk, utána a nagyszülő csúcsban is, stb stb
    - minden egyes dereferáláskor a memóriához kell fordulnunk, viszont a csúcsaink a memóriában teljesen véletlenszerűen vannak szétszórva
    - lényegében lehetetlen kihasználni a cacheünket, folyamatosan teljesen új memória területeket olvasunk ki, kiszámíthatatlanul
  - a másik olyan probléma a sok allokáció
    - ahogy kiértékeljük a gráfot, úgy folyton új csúcsokat kell felvennünk, és régi csúcsokat kell kitörölnünk, másodpercenként akár több milliószor
    - ez a sok pici allokáció nagyon lassú tud lenni. az allokátor azt szereti, ha ritkán de nagyokat allokálunk.

- ezért el is tértem ettől a megközelítéstől, és egy aréna allokátor alapú Flat-ASTs adatszerkezetet választottam
  - az aréna allokátornak az a lényege, hogy egyszerre  lefoglalunk egy nagy memóriaterületet, és azon belül mi tudunk sok új objektumot létrehozni, törölni, stb, és nem kell mindig külön allokálni és deallokálni. amikor végeztünk, akkor az egész foglalást egyszerre tudjuk deallokálni
  - a Flat-AST lényege pedig az, hogy nem pointerekkel hivatkoznak egymásra a csúcsok, hanem az aréna allokátor indexeivel. így az összes csúcs egymás mellett van a memóriában, kicsi gráfoknál akár az L1 cachebe bele férhet az egész

- a redukciós gépnek valahogy el kell döntenie, hogy milyen szabályok szerint redukáljon.
- két fő kategória létezik:
  - normal sorrend, amit szokás hívni leftmost-outmostnak is, és
  - applikatív sorrend, amelyet szokás hívni leftmost-innermostnak is
  - ezek azt határozzák meg, hogy egy adott kifejezésnél melyik az első kombinátor, ami kiértékelünk; milyen sorrendben kell haladni

- ezen a kettőn túl még meg szokás különböztetni a call-by-name és call-by-value stratégiákat. az előző kettő stratégia az úgy viselkedik, hogy ha egy kombinátor nem értékelhető ki (mondjuk mert nem hívtuk meg elég paraméterrel), akkor rekurzívan kiértékeli azokat a maradék paramétereket, amik vannak. a call-by-* család az nem csinál ilyet: ha a jelenleg kiértékelendő kombinátornak nincsen elég argumentuma, akkor megáll.

- a különbséget a normál sorrend és applikatív sorrend között legjobban grafikusan lehet ábrázolni:

- vegyünk egy SKI kalkulus kifejezést. itt a piros levelek az S kombinátornak, a kék levelek a K kombinátornak felelnek meg
- a szürke csúcsok azok applikációk. a sötétebb piros és kék csúcsok az egyes kombinátorok utolsó argumentumait jelölik
    - például: *elmutogatja*

- nézzük, hogy a normal sorrend melyik kombinátorral kezdené a kiértékelést. mivel leftmost-outmost, ezért a "legbaloldaibb, legkülső" kombinátorral kezd. ezt úgy kaphatjuk meg, hogy elindulunk a fánk gyökeréből, és addig megyünk balra ameddig csak tudunk.
- ezzel a kombinátorral kezdjük a kiértékelést. fontos, hogy ennek a kombinátornak az első argumentuma maga is egy komplikált részfa. ehhez egyáltalán nem nyúlúnk. ezt a részfát csak mozgatjuk, másolgatjuk ha kell, stb, viszont bele nem nézünk addig, ameddig nem muszáj
- ebből a szempontból ez egy lusta kiértékelés, sok más modern programozási nyelv is ilyet használ: csak akkor végzünk el számításokat, ha már nagyon muszáj

- az applikatív sorrend a leftmost-innermost. ehhez először megkeressük a legmélyebbre ágyazott kifejezést (tehát mintha alulról indulnánk el), és kiválasztjuk itt is a legbaloldalibb ilyen csúcsot. 
- fontos megjegyezni, hogy itt lényegében a külső, piros karikás S-nek az argumentum részfáját értékeljük ki mielőtt a S-ig eljutnánk. ebből a szempontból ez egy szigorú kiértékelés: mielőtt meghívunk egy függvényt, először kiértékeljük a paramétereit. a legtöbb imperatív nyelv ilyen, példál C++, Java, Rust

## a lambda kihívások

- a lambda kalkulusról eddig keveset beszéltem, viszont volt pár specifikus kihívás ezen a téren is
- általánosságban a lambda kalkulus kifejezések kiértékelése sokkal bonyolultabb mint az SKI kalkulus esetén
- ez főleg ahhoz vezethető vissza, hogy a lambda kalkulusban vannak változók. ezekre a változókra valahogy kell hivatkozni. viszont ha a változóinknak vannak neveik, akkor figyelnünk kell arra, hogy soha se tévesszük össze őket. erről szól az alpha ekvivalencia

- ehelyett én egy másik irányt válaszottam, és a változóneveket de Bruijn-indexekre cseréltem.
- a de Bruijn-indexek (mint ahogy a neve is sugallja) számok. számokkal pedig már sokkal könnyebb dolgozni
- itt az indexek azt fogják jelenteni, hogy a mély lambda-absztrakció és applikáció láncokban hány szinttel feljebb található az a lambda absztrakció, ami egy adott változót bevezetett
- erre egy példa lább látható: ez a K kombinátor definíciója. a jobboldali X-et kötő "lambda X" az két szinttel feljebb van ,tehát ez a két kifejezés az teljesen egyenértékű

- ezzel megoldottuk a változóneveket, viszont most meg kellett valósítsam az oda-vissza konverziót a sima lambda kalkulus és a de Bruijn-forma között
- ez viszont azért érte meg, mert két legyet ütöttem egy csapásra: ez segített a SKI -> lambda konverzióban is.

## tesztelés

- a program tesztelése közel sem volt triviális. az egységtesztek mellett a redukciós gépek helyességét úgynevezett property-based testinggel vizsgáltam.
- ez hasonló a fuzzinghoz. a fuzzing során általában azt tesztelik, hogy különböző input paraméterekre a program crashel-e
- a property-based testingnél általánosan azt vizsgáljuk, hogy különböző inputok esetén valamilyen tulajdonság, azaz property az fennáll-e
- a SKI kalkulus szabályait egészen könnyen lehet propertykként definiálni

- kellett viszont egy orákulum
  - az orákulum egy 3rd party, bizonyítottan helyes program, amivel össze tudjuk hasonlítani a mi programunk eredményét. ha a proprerty testert futtatom a saját programomon is, illetve az orákulumon is, és az eredményük megegyezik minden esetre, akkor egészen biztos lehetek abban, hogy helyesen implementáltam a redukciót

## visuals

- a mélyebb megértéshez elengedhetetlen pár intuitív vizuális elem
- ezért a backend az struktúrált adatként továbbítja a redukciós fát a frontendnek
- aki pedig szemantikailag színezve jeleníti meg a kifejezéseket

- látható, hogy amikor egy S kombinátor értékelődik ki, akkor az pirossal van jelölve (a paraméterei pedig piros keretet kapnak), a K kombinátor esetében pedig kékkel.
- itt minden egyes sor, az egy kiértékelés: kiértékeljük a soron következő kombinátort, aztán pedig az új állapotot egy sorral lejjebb mutatjuk
- körülbelül a kép felső harmadánál láthatjuk a részfa megosztás hatását: a sima S K K argumentumok után egy extra K is ki van színezve. ez azért van, mert arra a K-ra igazából egy indirekciós csúcs hivatkozik. ezért ami azzal a K-val történik, az két különböző helyen is egyszerre végbe megy

- emellett az absztrakt szintaxisfát is meg kell jeleníteni minden egyes lépéshez
- ezt nagyon sok munka lenne folyton újraszámolni, tehát agresszív cachelést kellett implementálnom, hogy egy gráf csak egyszer legyen kiszámítva

## ui

- az egui keretrendszert válaszottam, ami hasonló a Dear ImGuihoz, ha azt ismeri valaki
- ez egy immediate mode gui, ami azt jelenti hogy minden egyes frameben újraszámolja magát, és nulláról újrarajzolja az egész képernyőt
- ennek számos előnye van, például nagyon leegyszerűsíti a frontend logikáját, viszont például layoutozni benne nehezebb
- továbbá dockingot is támogat az app: amikor az ablakokat tetszőlegesen tudod mozgatni, átméretezni, elfelezni, stb

- itt egy kép az applikáció felületéről
- *magyarázd el az egészet*

## tervek

- még sok ötletem van a továbbfejlesztésére

- az indirekció ki/be kapcsolása az egyik, hogy a hatása jobban látszódjon

- egy "guided learning" mód, ahol konkrét feladatokat kell megoldani SKI kombinátor logika segítségével

- a nyelvtant bővíthetném új elemekkel
  - említettem, hogy például az if-else, logikai kapuk, és számok az szintén csak függvényenek a lambda kalkulusban. ettől viszont még a parser nem tudja rögtön függvénnyé alakítani őket. 
  - ennek a támogatása sok lehetőséget nyitna meg, akkor már szinte tényleg egy egész használható programozási nyelvet kapnánk 

- beszéltem SKI -> lambda konverzióról, pontosabban arról, hogy mennyire könnyű. ez fordítva nagyon nem igaz: tetszőleges lambda kifejezésnek megtalálni az SKI párját kimondottan nehéz.
- léteznek algoritmusok amikkel ez a konverzió is lehetséges, de az egyszerűbbek azok exponenciálisak (mind futásidőben, mind a produkált SKI kifejezés méretében mérve).
- 8 évvel ezelőtt valaki talált egy O(n)-es algoritmust ami memoizációt használ, viszont az eléggé bonyolult. minden esetre érdekes kihív lenne implementálni az algoritmust, csak pár száz sor

- végezetül pedig a frontend személyre szabhatóságát növelém, több beállítási lehetőséggel (például a színezések ki/be kapcsolása, etc.)

## összegezve

- a program egy interaktív, webes felületet nyújt az SKI és lambda kalkulussal való ismerkedéshez
- sokkal gyorsabb, mint bármelyik másik online elérhető implementáció
- emellett a redukciós gépek belső állapotáról egy intuitív, vizuális képet tud nyújtani a felhasználónak

## fin

köszönöm a figyelmet!