MIDI synth based on Mutable Instruments Elements module by Emilie Gillet (https://github.com/pichenettes/eurorack)

TODO:
* Encoder driver
  * Przetestować ostatnią wersję
  * git squash
* HW: podłączyć wyświetlacz i przetestować
TODO PoC:
* Elements: API v1
  v Note on/off
  v Jeden parametr
  v RunElements wywoływane cyklicznie
  * Test
  * Integracja z enkoderem i LCD
* Przycisk do wyzwalania dźwięku

TODO Prototyp:
v Kilka stron przełączanych przyciskami
v Refactor mods in ui
v Definicja wszystkich stron i parametrów
v Bug: akcja przyciski wywoływana w kółko cały czas jak naciśniętu
v Zablokowanie generowania dźwięku podczas odświeżania całego ekranu
* Wyzwalanie dźwięku 
  * pod przyciskiem jednego enkodera
* Testy

Etapy/milestones:
v PoC: sterujemy jednym parametrem, wyzwalamy dźwięk przyciskiem
* Prototyp: sterujemy wszystkimi parametrami dostępnymi na kilku stronach UI
* Alpha: obsługa wejścia MIDI
* Complete: zapisywanie patchy, konfiguracja
* Release: dedykowane PCB, obudowa

Notes:
* parametry syntezy i ich zakresy: elements/cv_scaler.cc#177
* Potencjometry: elements/drivers/pots_adc.h#L36

## License

This software and hardware is released under [The GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html) or later.

UI Design:
* Standardowy wygląd ekranu:
  * 5 przycisków na górze do przełączania między ekranami
  * 4 pokrętła/przełączniki/przyciski na dole do sterowania parametrami/etc
  * Naciśnięcie pokrętła wraca do wartości sprzed ostatniej edycji
* Ekrany
  * Bow: Mix, Timbre, Contour, empty
  * Blow: Mix, Timbre, Contour (the same as in Bow), Flow
  * Strike: Mix, Timbre, empty, Mallet
    * Przyciski Bow, Blow, Strike, Resonator, System
  * Resonator: Geometry, Brightness, Damping, Position
  * Output: Space
    * Przyciski Resonator, Output, empty, Exciter, System
  * Config: Midi channel
  * Patch: Browse, Load, Save
    * Przyciski: Patch, Config, empty, Exciter, Resonator


Framework UI:
* Widget
  * Func: Render, Poll input device, Notify on value change
  * API: trait UIWidget
  * Impl: Button, Encoder
* Page/Mode/Screen
  * Func: Setup, own, render widgets, handle value changes
  * API: trait UIScreen
  * Impl: separate for each screen
* Controller
  * Func: Setup, own, switch screens
  * Impl: struct UIController
