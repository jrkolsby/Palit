# PalIt

Palit is a music hardware project which seeks to upcycle outdated laptop models into audio workstations for collaborative music production over IP. 

### Projects
1. User Interface (UI) - All things relating to the ncurses interface.
2. Hardware (HW) - All things relating to peripherals and hardware modifications to the laptop.
3. Design (DG) - All things related to mockups, layouts, and the customization of the hardware appearance beyond its functionality.
4. Sound Engine (SOUND) - All things related to the sampling, recording, and synthesis engine including integration with Faust and custom patches
5. Infrastructure (INF) - All things relating to the structure of this codebase including our testing suite, issue tracking, and the configuration of our Ubuntu OS and all required software. This includes the development environment in Ubuntu and the flashing of installable media for use on our laptops.  

## Sprints

### OCTOBER 1

- [X] SOUND-1: build a dsp file
- [X] SOUND-2: Implement play and stop to ALSA output
- [X] SOUND-5: Multitrack mixing
- [X] SOUND-6: Receive Midi events in pt-sound

- [X] UI-1: Display the splash graphic
- [X] UI-2: Splash screen with project listings
- [X] UI-4: Display empty timeline for new project
- [X] UI-5: Timeline Cursor
- [X] UI-6: Layered rendering
- [X] UI-8: Multi Cursor
- [X] UI-9: Receive ticks from pt-sound playback

- [X] HARD-1: Acquire Lenovo Thinkpad T400

- [X] DG-1: First iteration mockup for user flow 
- [X] DG-1: Acquire vinyl paints in four-function colors and SELECT RED
- [X] DG-2: Paint top keybed using glossy white and black nail polish
- [X] DG-4: Paint bottom keybed

- [X] INF-2: Design project filetype around .xml or .json (Hopefully compatible with android)
- [X] INF-3: Implement view state
- [X] INF-4: Remove cursive dep in favor of termion
- [X] INF-7: Debug console
- [X] INF-8: Read XML file and update timeline state
- [X] INF-9: Update pt-client to loop which polls sdl2 keyboard and /tmp/pt-sound

### NOVEMBER 1

- [X] SOUND-9: Root mixer for timelines and synth (DSP Graph)
- [X] SOUND-11: Arpeggio
- [X] SOUND-12: Chord Gen

### DECEMBER 1

- [X] UI-20: Refactor components to be color-independent
- [X] UI-10: Render partial waveforms
- [X] UI-18: Routes view
- [X] UI-19: Timeline editing

### JANUARY 1 2020

- [X] UI-21: Add route, patch io

- [X] SOUND-10: Midi Tape
- [X] SOUND-14: Timeline editing
- [X] SOUND-15: Timeline XML save/load (BIG BRANCH)

- [X] INF-11: Double buffer

### FEBRUARY 1 2020

- [X] UI-17: Add, remove module 

- [X] SOUND-13: Timeline looping
- [X] SOUND-19: Timeline audio recording
- [X] SOUND-21: Timeline scrubbing

### MARCH 1 ( v1.0 )

- [X] SOUND-3: Faust synth compilation (faust2vst)

- [X] INF-10: Faust UI generation (faust2vst)
- [X] INF-12: Sound actions are sent to correct modules
- [X] INF-13: Common Lib

- [ ] UI-23: Project serialization & save 
- [X] UI-24: Timeline zoom
- [ ] UI-25: Monitor control
- [X] UI-26: Proper metronome tick
- [ ] UI-27: Track dB faders (gain control)
- [ ] UI-28: Help view
- [ ] UI-29: Error view
- [X] UI-30: Midi Region
- [ ] UI-31: Move, split, trim, duplicate, delete audio and midi region

### APRIL 1

- [ ] UI-14: Network setup
- [ ] UI-22: Snap to grid
- [ ] UI-32: Undo Redo Client

- [ ] SOUND-23: Undo Redo Sound
- [ ] SOUND-4: ALSA input module
- [ ] SOUND-18: Radio module and directory API
- [ ] SOUND-20: Import audio file to timeline
- [ ] SOUND-22: Global sample rate conversion

- [ ] INF-5: Unicode Support in linux TTY with KMSCON

### MAY 1

- [ ] SOUND-16: VST audio host (Steinberg API)
- [ ] SOUND-17: VST client host (XML UI to text layout)

- [ ] SOUND-7: Send midi events from pt-client
- [ ] SOUND-8: Fix audio underruns

- [ ] UI-7: Loading screen
- [ ] UI-12: Error Screen
- [ ] UI-15: Keyboard setup

- [ ] HARD-3: Purchase functional android phone
- [ ] HARD-4: Replace laptop sreen with android phone replacement screen
- [ ] HARD-5: Document undetectable keybed note combinations and map to chords
- [ ] HARD-6: Determine a set of keys which require redundant connections to unused keys in order to maximize chord combinations for the top (polyphonic) keybed.
- [ ] HARD-7: Encoder usage with function keys

- [ ] DG-3: Paint four function keys
- [ ] DG-5: Paint laptop casing and general keys CLASSIC BEIGE

- [ ] INF-1: Implement testing suite which takes keyboard input and project files and outputs UI sequences and stored audio files
