###PALIT

Palit is a music hardware project which seeks to upcycle outdated laptop models into audio workstations for collaborative music production over IP. 

# Projects
User Interface (UI) - All things relating to the ncurses interface.
Hardware (HW) - All things relating to peripherals and hardware modifications to the laptop.
Design (DG) - All things related to mockups, layouts, and the customization of the hardware appearance beyond its functionality.
Sound Engine (SOUND) - All things related to the sampling, recording, and synthesis engine including integration with Faust and custom patches
Infrastructure (INF) - All things relating to the structure of this codebase including our testing suite, issue tracking, and the configuration of our Ubuntu OS and all required software. This includes the development environment in Ubuntu and the flashing of installable media for use on our laptops.  

# Incomplete
SOUND-2: pt-client/src/audioengine.rs
SOUND-3: implement control of faust synth from audioengine
UI-2: Splash screen with project listings
UI-3: Keyboard display with feedback for keypress events
UI-4: Display empty timeline for new project
HARD-2: Rewire Green function key (Fn) to a keyboard-event producing key
HARD-3: Purchase functional android phone
HARD-4: Replace laptop sreen with android phone
HARD-5: Document undetectable keybed note combinations and map to chords
HARD-6: Determine a set of keys which require redundant connections to unused keys in order to maximize chord combinations for the top (polyphonic) keybed.
DG-3: Paint four function keys
DG-4: Paint bottom keybed
DG-5: Paint laptop casing and general keys CLASSIC BEIGE
INF-1: Implement testing suite which takes keyboard input and project files and outputs UI sequences and stored audio files
INF-2: Design project filetype around .xml or .json (Hopefully compatible with android)

# Complete
SOUND-1: build a dsp file
UI-1: Display the splash graphic
DG-1: First iteration mockup for user flow 
HARD-1: Acquire Lenovo Thinkpad T400
DG-1: Acquire vinyl paints in four-function colors and SELECT RED
DG-2: Paint top keybed using glossy white and black nail polish
