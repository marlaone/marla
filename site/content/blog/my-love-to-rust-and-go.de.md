# My love to rust and go

## Einleitung

Viele Jahre lang war ich hauptsächlich PHP-Backend und JS-Frontend Developer, bis ich vor ein paar Jahren für mich Go entdeckt hatte.
Go hat für mich eine ganz neue Welt eröffnet - ich war erst immer gegen diese ganzen Typen basierten Sprachen. "Das ist doch alles unnötig kompliziert, wenn man gut programmiert, dann ist PHP auch stabil und bugfrei", dachte ich mir immer.
Doch Go ist auch so leicht zu programmieren UND bietet einfach eine wunderschöne Typensicherheit und vor allem - Go wird nicht interpretiert. Asynchrone Programmierung war auf einmal viel einfacher.

Viele Side Projects entstanden daraus erstmal, um die Programmiersprache kennenzulernen.

Ein Turnier-Tool, über das wir unsere Firmen internen FIFA Turniere verwalten konnten.

[Bild vom System]

Frames, die von einem bestimmtem Windows-Fenster (Blobby Volley) eingefangen wurden und mit OpenCV wurden dort drin die Blobs, sowie der Ball erkannt. Darauf habe ich anhand der Informationen einen Bot geschrieben, der über einen virtuellen Xbox Controller gesteuert wurde.

[Bild vom Blobby Volley mit Debug Ansicht]

Und ab da fing Go einfach an für mich richtig cool zu werden. Man kann damit einfach alle möglichen Probleme lösen. Egal, ob ein Standard HTTP Server oder Windows Dienste zu steuern, Machine Learning, usw. Gefühlt ist alles möglich. In PHP wären solche Sideprojects für mich undenkbar gewesen.

Also direkt ans nächste Side Project: Warum nicht ein 2D Spiel in Go programmieren?

[Bild vom Spiel]

Und so enstand für mich eine große Liebe für Go.

--Side Note: Ja, das sind viele Side Projects und keins davon hat es bis zu einem finalen Produkt geschafft. Aber das ist noch ein Thema für eine spätere Stunde.--

## Die Große Liebe zu Go

Von da an, waren alle meine Projekte in Go und bis heute möchte ich auch gerne noch das JavaScript im Frontend durch Go ersetzt bekommen, damit ich nur noch mit den coolen Kids spielen darf. Es gibt da auch Möglichkeiten, aber das ist alles noch nicht ausgereift genug für große, produktive Projekte.

Die Liebe war mittlerweile so groß, dass meine Mutter mir sogar den Gopher gehäkelt hat, der mich nun immer auf meinem Schreibtisch begleitet.

[Bild von Gophy]

Nachdem ich meine Arbeitskollegen auch lang genug genervt habe und auch mit einem Proof-of-Concept mal gezeigt hatte, wie viel einfacher, stabiler und performanter wir in Go entwickeln könnten, fand sich die Programmiersprache auch schnell in meinem Arbeitsalltag wieder. Wir waren eh an einem Punkt, andem wir unser komplettes Produkt neu entwickeln wollten und somit war der Zeitpunkt für Go perfekt.

Unser Tracking wurde dann auch von PHP auf Go umgebaut und statt 80.000 Requests pro Minute, schaffen wir nun 72.000.000 Requests pro Minute. Natürlich macht nicht nur Go alleine diesen Unterschied, aber alleine, dass wir nun viel asynchroner arbeiten können, statt in einer interpretierten Programmiersprache festzuhängen, lassen sich ganz andere Architekturen implementieren.

Und auch dem Klima tun wir damit was Gutes. Nachdem wir das neue Tracking bei einem Kunden Live geschaltet hatten, kühlte sich die CPU vom im Durchschnitt von 80°C auf 40°C runter - also ein viel geringerer Energieverbrauch. Wir bräuchten nun theoretisch auch gar nicht mehr Server mit super starker Hardware und können dadurch auch nochmal Geld einsparen, aber das Thema ist da auch noch mal komplexer und darum geht es hier auch gar nicht.

[Bild von der CPU]

Gefesselt von der neuen Performance, ging es noch weiter an den Gedanken: "Wie lässt sich das Ganze noch mehr optimieren?" und nachdem das "Go"-Spiel durchgespielt war, entdeckte ich Rust.

## Die exotische Neue

Nichtsahnend und eigentlich in einer glücklichen Beziehung mit Go kam Rust um die Ecke. Angeblich die schnellste Programmiersprache nach C (oder sogar genau so schnell), die man lernen kann. Und das sogar, ohne die Memory-Kopfschmerzen von C.

Das klang interessant. Also baute ich ein kleines Side Project in Rust. Unsere Auszubildende hatte an einem Bundeswettbewerb für Informatik teilgenommen und dort gab es eine interessante Aufgabe, bei der man einen Algorithmus entwickeln musste, der bestimmte Bedingungen erfüllt und die beste Route von Start bis Ziel findet.
Ich hatte das Ganze schon in Go gebaut und das Programm brauchte für den größten Beispieldatensatz 3 Sekunden.

Nachdem ich erstmal an Rusts Compiler und seinem Ownership System fast an einer Existenzkrise gescheitert bin, lief nach ein paar Tagen tatsächlich das Programm und es war BLAZINGLY... slow.. Warum? Rust soll doch schneller sein? Nach ewigen pprof analysieren bin ich immer wieder bei der HashMap von Rust gelandet. Aber wie kann denn bitte eine Map so langsam sein? In Go ist sie doch auch schnell? Nach ewigem googlen, hatte ich zwei Meinungen über HashMaps gefunden. "HashMaps sind schnell" und "HashMaps sind langsam". Perfekt und nun? Das Ganze noch nicht richtig verstanden, habe ich einfach alle HashMaps durch Vecs ersetzt und plötzlich war ich um ein vielfaches schneller. Warum? Soweit ich das verstanden habe, funktionieren Go Maps intern genau so. Bei kleinen "Maps", sind die Maps in Wirklichkeit Arrays und werden erst bei großen "Maps" zu "HashMaps". Es ist halt schneller ein komplettes, kleines Array nach einem Key zu suchen, als das über eine HashMap zu lösen.
Also war das wohl mein Ansatz, den ich in Rust verfolgen musste und blieb bei den Vecs.

--Achtung: Das mit den Maps ist nur eine Theorie. Wer es genauer wissen möchte, kann sich natürlich über Onkel Google nochmal richtig informieren.--

Aber trotzdem war das Rust Programm immer noch langsamer als mein Go Programm. Go bei 3 Sekunden, Rust bei 10 Sekunden. Das ist doch doof.
Habe dann einfach mal `cargo build --release` geschrieben und die Binary ausgeführt. Aufeinmal brauchte Rust nur noch 1.2 Sekunden. Also schneller als Go! Durch Go kannte ich nicht, dass es einen Unterschied in der Performance bei `go run` und `go build` gibt.
Also wieder was gelernt. Jetzt hatte mich Rust! Es ist schneller als Go! Und nachdem die steile Lernkurve von Rust überstanden war, machte die Programmiersprache sogar richtig Spaß. Die Syntax gefiel einem total. Praktische Funktionen wie `.map()`, `.from()`, `.unwrap_or()`, `.unwrap_or_default()` gingen total ins Blut über. `Result<T,E>` und `Option<T>` sind total schöne Enums. Ach und Mensch, ENUMS! Wie geil sind die bitte in Rust? Aber ich schweife wieder ab.

Also weiter in ein neues Projekt. Ein Produkt von uns, dass Millionen von Datensätze aus einer MongoDB Live lädt, analysiert und aggregiert, wäre doch ein super Beispielprojekt. Vor allem, weil es in PHP wirklich nicht schnell ist.

Gesagt, getan - COMPILE ERRORS. Wochen habe ich an async in Rust verzweifelt. Nie wollte es wie ich. Vor allem weil "async" in Rust auch nur so halb existiert. Dafür braucht man eine extra Runtime wie `Tokio`.
Irgendwann lief es irgendwie, aber der Code war grauenvoll und es war auch nicht schnell. Und dank einem Mutex, blockierten irgendwann meine HTTP Requests. Es war die Hölle.
Also Rust erstmal wieder aufgegeben und lieber zurück auf Go gewechselt.

Das Produkt habe ich dann erstmal wieder in meiner alten, großen Liebe Go gebaut. Und es ging so schnell, so leicht. Alles war so schön wie früher.

Jedoch immer wieder musste ich an meine Abenteuer mit Rust denken. Das war so wild und man hat sich so herausgefordert gefühlt. Es war nicht alles so einfach, aber wenn es dann funktioniert hat, war die Befriedigung umso Größer. Moment. Irgendwie schweife ich gerade in einen erwachsenen Roman ab.

Ich habe mich dann noch mal ein paar Wochen an die Rust Version den Produkts gewagt. Und irgendwann hat es *Klick* gemacht mit dem async und das Projekt lief stabil. Und vor allem performant. Das war wieder ein sehr schöner Moment.

Und so ging es dann in meinem Leben weiter. Tagsüber verbrachte ich die Zeit mit Go und Abends habe ich mich immer wieder mit neuen Herausforderungen in Rust ausgetobt.

## Führe ich eine Polygame Beziehung?

Was ich mir im privaten Leben nie vorstellen könnte, ist in der Programmierung schon um einiges schwieriger. Ich liebe Go, Go hat mich zu einem so viel besseren Programmierer gemacht. Ich habe in kurzer Zeit so viel Neues gelernt. Jedoch hat Go mich auch zu Rust geführt und Rust ist einfach ein spannendes Abenteuer, dass mein Programmiererherz immer noch ein Stück höher schlagen lässt als Go.

### Was wäre, wenn ich nur noch Monogam programmieren dürfte?

Die Entscheidung würde mir leicht fallen. So sehr Rust auch immer wieder in Side Projects den Platz vor Go findet, würde meine Entscheidung sofort "Go" lauten. Es kommt natürlich auf den Anwendungsfall an, in dem man programmiert. Da ich jedoch hauptsächlich Web Backends baue, ist Go da einfach das beste Match.
Mir fällt es so unfassbar leicht in Go ein Projekt aufzusetzen. Egal, ob es ein Web Backend ist oder sonst etwas. Außerdem ist der Einstieg ein vielfaches leichter in Go als in Rust. Also auch aus Arbeitgeber-Sicht ist es viel einfacher einen Programmierer Go beizubringen, als Rust. Ich würde zwar behaupten, dass ich mittlerweile Standard Web Projects in Rust und Go wahrscheinlich gleich schnell bauen kann, weil es für beide Programmiersprachen einfach geniale Open Source Projekte gibt, dennoch liefert Go einfach insgesamt die beste Developer Experience.

## Fazit *TRIGGER WARNUNG*

Beide Programmiersprachen werden weiterhin einen Platz in meinem Herzen haben, aber Go bleibt meine große Liebe. Go hat mich zu einem richtigen Mann... ich meine Programmierer gemacht. Mir ganz neue Denkweisen aufgezeigt und neuen Spaß an der Programmierung in meiner Freizeit gebracht. Diese Website ist zwar trotzdem in Rust gebaut, weil das exotische Abenteuer mich zu sehr gereizt hat, aber Go würde ich immer vorziehen, wenn man mir die Pistole auf die Brust hält.

Ich finde es dennoch auch gut, wenn man in der Programmierung mal über den Tellerrand hinaus schaut und neue Programmiersprachen lernt. Dadurch lernt man viel über Strukturen, Datentypen und Performance. Warum ist das Eine besser als das Andere? Die Reise mit Go und Rust hat mich zu einem viel besseren Programmierer gemacht, weil ich die Programmierung und eine Programmiersprache nun besser verstehe und mich schneller in Problemen zurecht finde.

Abschließende triggernde Worte: Im Backend sehe ich keine Existenzberechtigung für andere Programmiersprachen als Rust und Go. Rust erfüllt die besten low level Aufgaben und Go die besten High Level Aufgaben. Kein Mensch braucht mehr C oder Java. Über diese "Programmiersprachen" Python und JavaScript brauchen wir erst gar nicht anfangen zu sprechen. Und gab es sonst überhaupt noch Programmiersprachen?

Und nachdem ich damit erfolgreich über 90% der IT Leute getriggert, wünsche ich dir trotzdem einen schönen Tag!


