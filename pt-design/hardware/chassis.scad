$fn=100;

// ALL SIZES IN mm

COLOR_WHITE = "#FFE8E8";
KEY_CAPHEIGHT = 1;
KEY_HEIGHT = 9.5;
KEY_TEXT_SIZE = 2;
KEY_GAP = 0.5;
KEY_RADIUS = 1;
KEYBOARD_WIDTH = 14.4;
KEYBOARD_HEIGHT = 5;

BEZEL_WIDTH = 1.5;

module key(x, y, width, txt) {
    translate([x, y]) {
		color(COLOR_WHITE) {

			//cube([width, KEY_HEIGHT, 1]);
			minkowski() {
				translate([KEY_RADIUS, KEY_RADIUS, 0])
					cube([width - 2 * KEY_RADIUS, KEY_HEIGHT - 2 * KEY_RADIUS, 1]);
				cylinder(r=KEY_RADIUS, h=0.01);
			}

			keycap_radius = (KEY_HEIGHT / 2) * 0.8;
			keycap_offset = (KEY_HEIGHT / 2);
			
			hull() {
			translate([keycap_offset, keycap_offset, 1]) 
				cylinder(r=keycap_radius, h=KEY_CAPHEIGHT);
			translate([width - keycap_offset, keycap_offset, 1]) 
				cylinder(r=keycap_radius, h=KEY_CAPHEIGHT);
			}
		}
		color("#000000") {
			translate([width / 2, KEY_HEIGHT / 2, KEY_CAPHEIGHT + 0.6])
			text(txt, valign="center", halign="center", 
				size=KEY_TEXT_SIZE, font="SFProText");
		}
    }
}

module gap_key(key_col, key_row, key_width, txt) {
    offset = KEY_HEIGHT + KEY_GAP;
    key(key_row * offset, key_col * offset, key_width * offset - KEY_GAP, txt);
}

module keyboard(x, y, z) {
    translate([x, y, z]) {
        gap_key(0, 0, 1, "fn");
        gap_key(0, 1, 1, "ct");
        gap_key(0, 2, 1, "op");
        gap_key(0, 3, 1.2, "cm");
        gap_key(0, 4.2, 5, "");
        gap_key(0, 9.2, 1.2, "cm");
        gap_key(0, 10.4, 1, "op");
        gap_key(0, 11.4, 1, "l");
        gap_key(0, 12.4, 1, "ud");
        gap_key(0, 13.4, 1, "r");
        gap_key(1, 0, 2.2, "shift");
        gap_key(1, 2.2, 1, "Z");
        gap_key(1, 3.2, 1, "X");
        gap_key(1, 4.2, 1, "C");
        gap_key(1, 5.2, 1, "V");
        gap_key(1, 6.2, 1, "B");
        gap_key(1, 7.2, 1, "N");
        gap_key(1, 8.2, 1, "M");
        gap_key(1, 9.2, 1, ",");
        gap_key(1, 10.2, 1, ".");
        gap_key(1, 11.2, 1, "/");
        gap_key(1, 12.2, 2.2, "shift");
        gap_key(2, 0, 1.7, "caps");
        gap_key(2, 1.7, 1, "A");
        gap_key(2, 2.7, 1, "S");
        gap_key(2, 3.7, 1, "D");
        gap_key(2, 4.7, 1, "F");
        gap_key(2, 5.7, 1, "G");
        gap_key(2, 6.7, 1, "J");
        gap_key(2, 7.7, 1, "K");
        gap_key(2, 8.7, 1, "L");
        gap_key(2, 9.7, 1, ";");
        gap_key(2, 10.7, 1, "'");
        gap_key(2, 11.7, 1, "\"");
        gap_key(2, 12.7, 1.7, "enter");
        gap_key(3, 0, 1.4, "tab");
        gap_key(3, 1.4, 1, "Q");
        gap_key(3, 2.4, 1, "W");
        gap_key(3, 3.4, 1, "E");
        gap_key(3, 4.4, 1, "R");
        gap_key(3, 5.4, 1, "T");
        gap_key(3, 6.4, 1, "Y");
        gap_key(3, 7.4, 1, "U");
        gap_key(3, 8.4, 1, "I");
        gap_key(3, 9.4, 1, "O");
        gap_key(3, 10.4, 1, "P");
        gap_key(3, 11.4, 1, "[");
        gap_key(3, 12.4, 1, "]");
        gap_key(3, 13.4, 1, "\\");
        gap_key(4, 0, 1, "~");
        gap_key(4, 1, 1, "1");
        gap_key(4, 2, 1, "2");
        gap_key(4, 3, 1, "3");
        gap_key(4, 4, 1, "4");
        gap_key(4, 5, 1, "5");
        gap_key(4, 6, 1, "6");
        gap_key(4, 7, 1, "7");
        gap_key(4, 8, 1, "8");
        gap_key(4, 9, 1, "9");
        gap_key(4, 10, 1, "0");
        gap_key(4, 11, 1, "-");
        gap_key(4, 12, 1, "+");
        gap_key(4, 13, 1.4, "del");
    }
}

keyboard(0, 0, 0);

cube([14.4 * (KEY_HEIGHT + KEY_GAP) - KEY_GAP, 5 * (KEY_HEIGHT + KEY_GAP) - KEY_GAP, 1]);

/*
color(COLOR_WHITE) {
	difference() {
		minkowski() {
			cube([147.5, 100, 4]);
			sphere(r=1);
		}
		translate([BEZEL_WIDTH, BEZEL_WIDTH, BEZEL_WIDTH])
			minkowski() {
				cube([147.5 - 2 * BEZEL_WIDTH, 100 - 2 * BEZEL_WIDTH, 3]);
				sphere(r=1);
			}
	}
}
*/

