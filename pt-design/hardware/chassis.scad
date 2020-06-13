$fn=200;

// ALL SIZES IN mm

COLOR_WHITE = "#FFE8E8";
KEY_CAP_HEIGHT = 1;
KEY_BASE_HEIGHT = 1;
KEY_TEXT_SIZE = 2;
KEY_GAP = 0.5;
KEY_RADIUS = 0.5;
BEZEL = 1;
GOLDEN_RATIO = 1.61;
PHONE_HEIGHT = 30;
PHONE_WIDTH = 70;
PHONE_HEADER = 10;
PHONE_DEPTH = 5;
CHASSIS_DEPTH = 10;
COVER_DEPTH = 1;

module key(x, y, width, txt, key_height) {
    translate([x, y]) {
        color(COLOR_WHITE) {

            //cube([width, key_height, 1]);
            minkowski() {
                translate([KEY_RADIUS, KEY_RADIUS, 0])
                    cube([width - 2 * KEY_RADIUS, key_height - 2 * KEY_RADIUS, KEY_BASE_HEIGHT]);
                cylinder(r=KEY_RADIUS, h=0.01);
            }

            keycap_radius = (key_height / 2) * 0.8;
            keycap_offset = (key_height / 2);
            
            hull() {
            translate([keycap_offset, keycap_offset, KEY_BASE_HEIGHT]) 
                cylinder(r=keycap_radius, h=KEY_CAP_HEIGHT);
            translate([width - keycap_offset, keycap_offset, KEY_BASE_HEIGHT]) 
                cylinder(r=keycap_radius, h=KEY_CAP_HEIGHT);
            }
        }
        color("#000000") {
            translate([width / 2, key_height / 2, KEY_CAP_HEIGHT + KEY_BASE_HEIGHT - 0.3])
            text(txt, valign="center", halign="center", 
                 size=KEY_TEXT_SIZE, font="SFProText");
        }
    }
}

module gap_key(key_col, key_row, key_width, txt, key_height) {
    offset = key_height + KEY_GAP;
    key(key_row * offset, key_col * offset, key_width * offset - KEY_GAP, txt, key_height);
}

function key_height(keyboard_height) = (keyboard_height - (4 * KEY_GAP)) / 5;
function keyboard_width(keyboard_height) = 14.4 * (key_height(keyboard_height) + KEY_GAP) - KEY_GAP;

module keyboard(x, y, z, height) {
    unit = key_height(height);
    translate([x, y, z]) {
        gap_key(0, 0, 1, "fn", unit);
        gap_key(0, 1, 1, "ct", unit);
        gap_key(0, 2, 1, "op", unit);
        gap_key(0, 3, 1.2, "cm", unit);
        gap_key(0, 4.2, 5, "", unit);
        gap_key(0, 9.2, 1.2, "cm", unit);
        gap_key(0, 10.4, 1, "op", unit);
        gap_key(0, 11.4, 1, "l", unit);
        gap_key(0, 12.4, 1, "ud", unit);
        gap_key(0, 13.4, 1, "r", unit);
        gap_key(1, 0, 2.2, "shift", unit);
        gap_key(1, 2.2, 1, "Z", unit);
        gap_key(1, 3.2, 1, "X", unit);
        gap_key(1, 4.2, 1, "C", unit);
        gap_key(1, 5.2, 1, "V", unit);
        gap_key(1, 6.2, 1, "B", unit);
        gap_key(1, 7.2, 1, "N", unit);
        gap_key(1, 8.2, 1, "M", unit);
        gap_key(1, 9.2, 1, ",", unit);
        gap_key(1, 10.2, 1, ".", unit);
        gap_key(1, 11.2, 1, "/", unit);
        gap_key(1, 12.2, 2.2, "shift", unit);
        gap_key(2, 0, 1.7, "caps", unit);
        gap_key(2, 1.7, 1, "A", unit);
        gap_key(2, 2.7, 1, "S", unit);
        gap_key(2, 3.7, 1, "D", unit);
        gap_key(2, 4.7, 1, "F", unit);
        gap_key(2, 5.7, 1, "G", unit);
        gap_key(2, 6.7, 1, "H", unit);
        gap_key(2, 7.7, 1, "J", unit);
        gap_key(2, 8.7, 1, "K", unit);
        gap_key(2, 9.7, 1, "L", unit);
        gap_key(2, 10.7, 1, ";", unit);
        gap_key(2, 11.7, 1, "'", unit);
        gap_key(2, 12.7, 1.7, "enter", unit);
        gap_key(3, 0, 1.4, "tab", unit);
        gap_key(3, 1.4, 1, "Q", unit);
        gap_key(3, 2.4, 1, "W", unit);
        gap_key(3, 3.4, 1, "E", unit);
        gap_key(3, 4.4, 1, "R", unit);
        gap_key(3, 5.4, 1, "T", unit);
        gap_key(3, 6.4, 1, "Y", unit);
        gap_key(3, 7.4, 1, "U", unit);
        gap_key(3, 8.4, 1, "I", unit);
        gap_key(3, 9.4, 1, "O", unit);
        gap_key(3, 10.4, 1, "P", unit);
        gap_key(3, 11.4, 1, "[", unit);
        gap_key(3, 12.4, 1, "]", unit);
        gap_key(3, 13.4, 1, "\\", unit);
        gap_key(4, 0, 1, "~", unit);
        gap_key(4, 1, 1, "1", unit);
        gap_key(4, 2, 1, "2", unit);
        gap_key(4, 3, 1, "3", unit);
        gap_key(4, 4, 1, "4", unit);
        gap_key(4, 5, 1, "5", unit);
        gap_key(4, 6, 1, "6", unit);
        gap_key(4, 7, 1, "7", unit);
        gap_key(4, 8, 1, "8", unit);
        gap_key(4, 9, 1, "9", unit);
        gap_key(4, 10, 1, "0", unit);
        gap_key(4, 11, 1, "-", unit);
        gap_key(4, 12, 1, "+", unit);
        gap_key(4, 13, 1.4, "del", unit);
    }
}

module chassis(x, y, width, height) {
    union() {
        speaker_size = PHONE_HEIGHT - PHONE_HEADER;
        translate([0, height - speaker_size, 0])
            cube([speaker_size, speaker_size, CHASSIS_DEPTH - 3]);

        difference() {
            cube([width, height, CHASSIS_DEPTH]);
            translate([BEZEL, BEZEL, BEZEL])
                minkowski() {
                    cube([width - 2 * BEZEL - 2 * (KEY_GAP + KEY_RADIUS), 
                          height - 2 * BEZEL - 2 * (KEY_GAP + KEY_RADIUS), 
                          CHASSIS_DEPTH - BEZEL]);
                    translate([KEY_RADIUS + KEY_GAP, KEY_RADIUS + KEY_GAP, KEY_RADIUS + KEY_GAP])
                        sphere(r=KEY_RADIUS + KEY_GAP);
                }
        }
    }
}

keyboard_height = PHONE_HEIGHT * GOLDEN_RATIO;
keyboard_width = keyboard_width(keyboard_height);
keyboard_offset = BEZEL + KEY_GAP;
keyboard_depth = CHASSIS_DEPTH - KEY_BASE_HEIGHT;

keyboard(keyboard_offset, keyboard_offset, keyboard_depth, keyboard_height);

chassis_height = PHONE_HEIGHT + keyboard_height + BEZEL + 2 * KEY_GAP;
chassis_width = keyboard_width + 2 * BEZEL + 2 * KEY_GAP;

chassis(0, 0, chassis_width, chassis_height);


