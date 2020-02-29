//
//  File: sniff.m
//  Project: sniffM&K
//
//  Created by: Patrick Wardle
//  Copyright:  2017 Objective-See
//  License:    Creative Commons Attribution-NonCommercial 4.0 International License
//
//  Compile:
//   a) Xcode, Product->Build
//
//   or ...
//
//   b) $ clang -o sniffMK sniffMK.m -framework Cocoa -framework Carbon
//
//   Run (as root):
//   # ./sniffMK
//
//  Notes:
//   a) code, largely based on altermouse.c/alterkeys.c (amit singh/http://osxbook.com)
//   b) run with '-mouse' for just mouse events or '-keyboard' for just key events
//

#include <stdio.h>
#import <Carbon/Carbon.h>
#import <Foundation/Foundation.h>
#import <ApplicationServices/ApplicationServices.h>

//event tap
static CFMachPortRef eventTap = NULL;

//map a printable keycode to a string
// ->code based on: https://stackoverflow.com/a/33584460
NSString* keyCodeToString(CGEventRef event, CGEventType type)
{
    //keycode as string
    NSString* keyCodeAsString = nil;
    
    //status
    OSStatus status = !noErr;
    
    //(key) code
    CGKeyCode keyCode = 0;
    
    //keyboard layout data
    CFDataRef keylayoutData = NULL;
    
    //keyboard layout
    const UCKeyboardLayout* keyboardLayout = NULL;
    
    //key action
    UInt16 keyAction = 0;
    
    //modifer state
    UInt32 modifierState = 0;
    
    //dead key
    UInt32 deadKeyState = 0;
    
    //max length
    UniCharCount maxStringLength = 255;
    
    //actual lenth
    UniCharCount actualStringLength = 0;
    
    //string
    UniChar unicodeString[maxStringLength];
    
    //zero out
    memset(unicodeString, 0x0, sizeof(unicodeString));
    
    //get code
    keyCode = (CGKeyCode)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);
    
    //get key layout data
    keylayoutData = (CFDataRef)TISGetInputSourceProperty(TISCopyCurrentKeyboardInputSource(), kTISPropertyUnicodeKeyLayoutData);
    if(NULL == keylayoutData)
    {
        //bail
        goto bail;
    }
    
    //get keyboard layout
    keyboardLayout = (const UCKeyboardLayout*)CFDataGetBytePtr(keylayoutData);
    if(NULL == keyboardLayout)
    {
        //bail
        goto bail;
    }
    
    //set key action down
    if(kCGEventKeyDown == type)
    {
        //down
        keyAction = kUCKeyActionDown;
    }
    //set key action up
    else
    {
        //up
        keyAction = kUCKeyActionUp;
    }
    
    //TODO:
    // set modifierState based on event flags?
    
    //translate
    status = UCKeyTranslate(keyboardLayout, keyCode, keyAction, modifierState, LMGetKbdType(), 0, &deadKeyState, maxStringLength, &actualStringLength, unicodeString);
    if( (noErr != status) ||
        (0 == actualStringLength) )
    {
        //bail
        goto bail;
    }

    //init string
    keyCodeAsString = [[NSString stringWithCharacters:unicodeString length:(NSUInteger)actualStringLength] lowercaseString];
    
bail:
    
    return keyCodeAsString;
}

//build string of key modifiers (shift, command, etc)
// ->code based on: https://stackoverflow.com/a/4425180/3854841
NSMutableString* extractKeyModifiers(CGEventRef event)
{
    //key modify(ers)
    NSMutableString* keyModifiers = nil;
    
    //flags
    CGEventFlags flags = 0;
    
    //alloc
    keyModifiers = [NSMutableString string];
    
    //get flags
    flags = CGEventGetFlags(event);
    
    //control
    if(YES == !!(flags & kCGEventFlagMaskControl))
    {
        //add
        [keyModifiers appendString:@"control "];
    }
    
    //alt
    if(YES == !!(flags & kCGEventFlagMaskAlternate))
    {
        //add
        [keyModifiers appendString:@"alt "];
    }
    
    //command
    if(YES == !!(flags & kCGEventFlagMaskCommand))
    {
        //add
        [keyModifiers appendString:@"command "];
    }
    
    //shift
    if(YES == !!(flags & kCGEventFlagMaskShift))
    {
        //add
        [keyModifiers appendString:@"shift "];
    }
    
    //caps lock
    if(YES == !!(flags & kCGEventFlagMaskAlphaShift))
    {
        //add
        [keyModifiers appendString:@"caps lock "];
    }
    
    return keyModifiers;
}

//callback for mouse/keyboard events
// ->for now, just format, then print the event to stdout
CGEventRef eventCallback(CGEventTapProxy proxy, CGEventType type, CGEventRef event, void *refcon)
{
    //(mouse) location
    CGPoint location = {0};
    
    //(key) code
    CGKeyCode keyCode = 0;
    
    //key modify(ers)
    NSMutableString* keyModifiers = nil;
    
    //what type?
    // ->pretty print

    bool isUpEvent = false;

    switch(type)
    {
        //key down
        case kCGEventKeyDown:
            
            //get key modifiers
            keyModifiers = extractKeyModifiers(event);
            isUpEvent = false;
            break;
            
        //key up
        case kCGEventKeyUp:
            isUpEvent = true;
            break;
        
        // event tap timeout
        case kCGEventTapDisabledByTimeout:
            CGEventTapEnable(eventTap, true);
            //printf("Event tap timed out: restarting tap");
            return event;
        
        default:
            break;
    }
    
    //for key presses
    // ->dump extra info
    if( (kCGEventKeyDown == type) || (kCGEventKeyUp == type) )
    {
        //get code
        keyCode = (CGKeyCode)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);
        
        //any key modifiers?
        if(0 != keyModifiers.length)
        {
            //dbg msg
            //printf("key modifiers: %s\n", keyModifiers.UTF8String);
        }
        
        //dbg msg
        //printf("keycode: %#x/%d/%s\n\n", keyCode, keyCode, keyCodeToString(event, type).UTF8String);
        if (isUpEvent) {
            switch(keyCode) {
                case 0: fprintf(stderr, "NOTE_OFF:60 "); break; // a
                case 13: fprintf(stderr, "NOTE_OFF:61 "); break; // w
                case 1: fprintf(stderr, "NOTE_OFF:62 "); break; // s
                case 14: fprintf(stderr, "NOTE_OFF:63 "); break; // e
                case 2: fprintf(stderr, "NOTE_OFF:64 "); break; // d
                case 3: fprintf(stderr, "NOTE_OFF:65 "); break; // f
                case 17: fprintf(stderr, "NOTE_OFF:66 "); break; // t
                case 5: fprintf(stderr, "NOTE_OFF:67 "); break; // g
                case 16: fprintf(stderr, "NOTE_OFF:68 "); break; // y
                case 4: fprintf(stderr, "NOTE_OFF:69 "); break; // h
                case 32: fprintf(stderr, "NOTE_OFF:70 "); break; // u
                case 38: fprintf(stderr, "NOTE_OFF:71 "); break; // j
                case 40: fprintf(stderr, "NOTE_OFF:72 "); break; // k
                case 31: fprintf(stderr, "NOTE_OFF:73 "); break; // o
                case 37: fprintf(stderr, "NOTE_OFF:74 "); break; // l
                case 35: fprintf(stderr, "NOTE_OFF:75 "); break; // p
                case 41: fprintf(stderr, "NOTE_OFF:76 "); break; // ;
                case 39: fprintf(stderr, "NOTE_OFF:77 "); break; // '
                case 15: // r
                case 9: // v
                case 46: // m
                case 34: // i
                case 49: // space
                    fprintf(stdout, "DESELECT ");
                    break; 
                default:
                    break;
            }
        } else {
            if (keyCode != *(CGKeyCode *)refcon) {
                switch(keyCode) {
                    case 0: fprintf(stderr, "NOTE_ON:60:1 "); break; // a
                    case 13: fprintf(stderr, "NOTE_ON:61:1 "); break; // w
                    case 1: fprintf(stderr, "NOTE_ON:62:1 "); break; // s
                    case 14: fprintf(stderr, "NOTE_ON:63:1 "); break; // e
                    case 2: fprintf(stderr, "NOTE_ON:64:1 "); break; // d
                    case 3: fprintf(stderr, "NOTE_ON:65:1 "); break; // f
                    case 17: fprintf(stderr, "NOTE_ON:66:1 "); break; // t
                    case 5: fprintf(stderr, "NOTE_ON:67:1 "); break; // g
                    case 16: fprintf(stderr, "NOTE_ON:68:1 "); break; // y
                    case 4: fprintf(stderr, "NOTE_ON:69:1 "); break; // h
                    case 32: fprintf(stderr, "NOTE_ON:70:1 "); break; // u
                    case 38: fprintf(stderr, "NOTE_ON:71:1 "); break; // j
                    case 40: fprintf(stderr, "NOTE_ON:72:1 "); break; // k
                    case 31: fprintf(stderr, "NOTE_ON:73:1 "); break; // o
                    case 37: fprintf(stderr, "NOTE_ON:74:1 "); break; // l
                    case 35: fprintf(stderr, "NOTE_ON:75:1 "); break; // p
                    case 41: fprintf(stderr, "NOTE_ON:76:1 "); break; // ;
                    case 39: fprintf(stderr, "NOTE_ON:77:1 "); break; // '
                    case 33: fprintf(stdout, "PLAY "); break; // [
                    case 30: fprintf(stdout, "STOP "); break; // ]
                    case 44: fprintf(stdout, "HELP "); break; // ?
                    case 48: fprintf(stdout, "ROUTE "); break; // tab
                    case 6: fprintf(stderr, "OCTAVE:0 "); // z
                            fprintf(stdout, "OCTAVE:0 "); break; 
                    case 7: fprintf(stderr, "OCTAVE:1 "); // x
                            fprintf(stdout, "OCTAVE:1 "); break; 
                    case 15: fprintf(stdout, "R "); break; // r
                    case 9: fprintf(stdout, "V "); break; // v
                    case 46: fprintf(stdout, "M "); break; // m
                    case 34: fprintf(stdout, "I "); break; // i
                    case 49: fprintf(stdout, "SPC "); break; // space
                    case 12: fprintf(stdout, "BACK "); break; // q
                    case 126: fprintf(stdout, "UP "); break; // up
                    case 125: fprintf(stdout, "DN "); break; // down
                    case 124: fprintf(stdout, "RT "); break; // right
                    case 123: fprintf(stdout, "LT "); break; // left
                    case 36: fprintf(stdout, "MODULES "); break; // enter
                    case 51: fprintf(stdout, "PROJECT "); break; // enter
                    default:
                        fprintf(stderr, "UNKNOWN:%d ", keyCode);
                        break;
                }
            }
        }
        if (isUpEvent && keyCode == *(CGKeyCode *)refcon) {
            *(CGKeyCode *)refcon = -1;
        } else if (keyCode != 126 && keyCode != 125 && keyCode != 124 && keyCode != 123) {
            *(CGKeyCode *)refcon = keyCode;
        }

        fflush(stdout);
        fflush(stderr);

    }

    //for mouse
    // ->print location
    else
    {
        //get location
        location = CGEventGetLocation(event);
        
        //dbg msg
        //printf("(x: %f, y: %f)\n\n", location.x, location.y);
    }
    
    return event;
}

//main interface
// ->parse args, then sniff (forever)
int main(int argc, const char * argv[])
{
    //event mask
    // ->events to sniff
    CGEventMask eventMask = 0;
    
    //run loop source
    CFRunLoopSourceRef runLoopSource = NULL;

    //pool
    @autoreleasepool
    {
        //dbg msg
        //printf("mouse/keyboard sniffer\nbased on code from amit singh (http://osxbook.com)\n\n");
        
        //gotta be r00t
        // unless this program has been added to 'Security & Privacy' -> 'Accessibility'
        if(0 != geteuid())
        {
            //err msg/bail
            fprintf(stdout, "ERROR ");
            goto bail;
        }
        
        //'-mouse'
        // ->just sniff mouse
        if( (2 == argc) &&
            (0 == strcmp(argv[1], "-mouse")) )
        {
            //dbg msg
            //printf("initializing event mask for 'mouse' events\n");
            
            //init event mask with mouse events
            // ->add 'CGEventMaskBit(kCGEventMouseMoved)' if you want to also capture (noisy) mouse move events
            eventMask = CGEventMaskBit(kCGEventLeftMouseDown) | CGEventMaskBit(kCGEventLeftMouseUp) | CGEventMaskBit(kCGEventRightMouseDown) | CGEventMaskBit(kCGEventRightMouseUp) |
                        CGEventMaskBit(kCGEventLeftMouseDragged) | CGEventMaskBit(kCGEventRightMouseDragged);

        }
        
        //'-keyboard'
        // ->just sniff keyboard
        else if( (2 == argc) &&
                 (0 == strcmp(argv[1], "-keyboard")) )
        {
            //dbg msg
            //printf("initializing event mask for 'keyboard' events\n");
            
            //init event mask with mouse events
            // ->add 'CGEventMaskBit(kCGEventMouseMoved)' for mouse move events
            eventMask = CGEventMaskBit(kCGEventKeyDown) | CGEventMaskBit(kCGEventKeyUp);
            
        }
        
        //sniff both!
        else
        {
            //dbg msg
            //printf("initializing event mask for both 'mouse' and 'keyboard' events\n");
            
            //init event with mouse events & key presses
            eventMask = CGEventMaskBit(kCGEventLeftMouseDown) | CGEventMaskBit(kCGEventLeftMouseUp) | CGEventMaskBit(kCGEventRightMouseDown) | CGEventMaskBit(kCGEventRightMouseUp) |
                        CGEventMaskBit(kCGEventLeftMouseDragged) | CGEventMaskBit(kCGEventRightMouseDragged) | CGEventMaskBit(kCGEventKeyDown) | CGEventMaskBit(kCGEventKeyUp);
            
        }
        
        //create event tap
        CGKeyCode lastKeyCode = 0; 
        eventTap = CGEventTapCreate(kCGSessionEventTap, kCGHeadInsertEventTap, 0, eventMask, eventCallback, &lastKeyCode);
        if(NULL == eventTap)
        {
            //err msg/bail (failed to create event tap)
            fprintf(stdout, "ERROR ");
            goto bail;
        }
        
        //dbg msg
        //printf("created event tap\n");
        
        //run loop source
        runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0);
        
        //add to current run loop.
        CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, kCFRunLoopCommonModes);
        
        //enable tap
        CGEventTapEnable(eventTap, true);
        
        //dbg msg
        //printf("enabled event tap to commence sniffing\n\n");
        
        //go, go, go
        CFRunLoopRun();
    }
    
bail:
    
    //release event tap
    if(NULL != eventTap)
    {
        //release
        CFRelease(eventTap);
        
        //unset
        eventTap = NULL;
    }
    
    //release run loop src
    if(NULL != runLoopSource)
    {
        //release
        CFRelease(runLoopSource);
        
        //unset
        runLoopSource = NULL;
    }
    
    return 0;
}
