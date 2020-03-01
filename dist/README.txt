           **********************************
                   SONG GARDEN ALPHA
           **********************************

                    ~| dependency |~

Song Garden requires you to install Portaudio on your
computer. You can do this by opening Terminal and pasting
the following two commands:

        (skip if you already have homebrew installed)
        /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

        brew install portaudio

To begin, double click on launcher.sh -- you will be asked
to give your computer password. This is required to access
your keyboard, please type it in and press ENTER.

                    ~| controls |~

   [][][][][][][][][][][][][][][][][][][][][][_PROJECT]
   [ROUTES][BACK][][][YEL][][][][GRN][][][PLAY][STOP][]
   [_____]      [][][]   [][][][]   [][][][]  [MODULES]
   [_____][OCT+][OCT-][][BLU][][][PNK][][][HELP][_____]
   [][][][][__________RED_____________][][][ARROW KEYS]

Each region of the UI will present you with up to five 
actions highlighted in YELLOW, BLUE, PINK, GREEN, and RED. 

To move your selection around the screen, use the arrow keys. 

To make a selection, use R, V, M, I, or SPACE:

              R:YELLOW            I:GREEN
              V:BLUE              M:PINK
                       SPACE:RED

These keys were chosen to fit within the piano keyboard,
which is mapped to the following keys:

        W:C# E:D#      T:F# Y:G# U:A#      O:C# P:D#
      A:C  S:D  D:E  F:F  G:G  H:A  J:B  K:C  L:D ;:E ,:F

To go to a lower or higher octave, press Z or X

To view the key mappings at any time, press ?

To go back to the previous screen, press Q

To change parameters, many regions require you to press and 
hold down a selection key while using the arrow keys to 
affect it. 

                                      : (hold +
              PARAM                   :  UP DOWN)
              ................        :
              (hold + LEFT RIGHT)     PARAM

If you would like to try the demo, press YELLOW


                    ~| modules |~

You will be dropped on the keyboard module. To navigate
between modules, simply use the arrow keys to move past the
top or bottom edge of the screen. 

To view the loaded modules, press DELETE. The project view 
allows you to save and close your project, and delete 
individual modules.

To delete a module, move to it and press PINK.

To save your project, move to the bottom and press RED.

To close your project without saving, press YELLOW

To load a new module, press Q to exit the project view and 
press ENTER. 

Any user-written modules placed into "modules/" will be 
available here.

Use YELLOW, BLUE, PINK, GREEN, or RED to load a module


                    ~| patching|~

The sound engine is a modular network of instruments, 
effects, and routes, even the timeline and computer keyboard 
are modules. Patching allows an audio and/or MIDI signal to 
be sent between modules via routes.

To show the patch view, press TAB.

A route is like a mixer and a cable splitter which will sum 
its inputs and feed any number of outputs. 

To grab an input or output, press and hold YELLOW BLUE GREEN 
PINK OR RED and press LEFT or RIGHT to move it between route.
The previous connection will be deleted, and the new patch will 
be made only when you release your selection.

To add a new route, move to the bottom of the screen and
press RED. 

To remove the last route, press YELLOW.


                    ~| timeline |~

The timeline records and plays back audio and MIDI data. The
various functions are labeled below:

        RECORD                ZOOM: 1X 127   @
                                       BPM  / 
        TOGGLE LOOP
                        << LOOP REGION >>
        ADD TRACK   {{...!...!...!...!...!...!   TEMPO GRID
                      |
        r m s i       |   ::::::. AUDIO REGION
        | | | |INPUT   |  
        | | | MONITOR |   - _ - - MIDI REGION
        | | SOLO      |   
        | MUTE        |
        RECORD        |

To arm a track for MIDI recording, move to its track header 
and press RED once. For audio recording, press RED again. To
disarm, press RED one more time.

                   Off:  MIDI:  Audio:
                         …      ∿
                   r     R      R

To record, move to the top left and press RED. When finished
press RIGHT BRACKET to stop. Press LEFT BRACKET to play.

The patch view for the timeline enumerates one input and one
output for each track. The output of one track can be
patched into another track.

To navigate the timeline, move to a recorded region and press 
LEFT or RIGHT to scrub forward or backward. On first press, 
the playhead will move at quarter speed, press LEFT or RIGHT 
repeatedly to speed up or change direction.

When an audio or MIDI region is selected, press YELLOW to
split it into two regions about the playhead. Press and
hold GREEN and use LEFT and RIGHT to move the region. Press 
PINK to delete a region.

If you wish to edit at more precise scale, move to the top
right then press and hold GREEN and use UP and DOWN to
change the zoom level. 

Changing the BPM and meter of the project will not affect
the timing of the regions, only the grid and metronome.
Change the BPM by holding RED and pressing UP and DOWN.
Change the meter by holding YELLOW or BLUE et cetera.


                    ~| writing modules |~

It is possible to write new instruments and effects in 
the Faust DSP language. To write a new module, create a 
new file named "mymodule.dsp" and place it in the "modules/" 
folder.

Check out asynth.dsp for a minimal example. 

Faust documentation:
https://faust.grame.fr


                    ~| feedback |~

Please note that this is an alpha release and you could 
experience errors and crashes at any time. Please save your 
songs frequently!

If you experience any issues please email me at jrkolsby@mac.com
with a description of the error, any project file that caused 
it, and contents of the files in the "logs/" folder.