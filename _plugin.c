/************************** BEGIN CInterface.h **************************/
/************************************************************************
 FAUST Architecture File
 Copyright (C) 2018 GRAME, Centre National de Creation Musicale
 ---------------------------------------------------------------------
 This Architecture section is free software; you can redistribute it
 and/or modify it under the terms of the GNU General Public License
 as published by the Free Software Foundation; either version 3 of
 the License, or (at your option) any later version.
 
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with this program; If not, see <http://www.gnu.org/licenses/>.
 
 EXCEPTION : As a special exception, you may create a larger work
 that contains this FAUST architecture section and distribute
 that work under terms of your choice, so long as this FAUST
 architecture section is not modified.
 ************************************************************************/

#ifndef CINTERFACE_H
#define CINTERFACE_H

#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif

#include <stdlib.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif
    
struct Soundfile;

/*******************************************************************************
 * Minimal architecture, main will never be called
 ******************************************************************************/

int main() {}

#define max(a,b) ((a < b) ? b : a)
#define min(a,b) ((a < b) ? a : b)

/*******************************************************************************
 * UI and Meta classes for C or LLVM generated code.
 ******************************************************************************/

// -- widget's layouts

typedef void (* openTabBoxFun) (void* ui_interface, const char* label);
typedef void (* openHorizontalBoxFun) (void* ui_interface, const char* label);
typedef void (* openVerticalBoxFun) (void* ui_interface, const char* label);
typedef void (*closeBoxFun) (void* ui_interface);

// -- active widgets

typedef void (* addButtonFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone);
typedef void (* addCheckButtonFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone);
typedef void (* addVerticalSliderFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step);
typedef void (* addHorizontalSliderFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step);
typedef void (* addNumEntryFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step);

// -- passive widgets

typedef void (* addHorizontalBargraphFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone, FAUSTFLOAT min, FAUSTFLOAT max);
typedef void (* addVerticalBargraphFun) (void* ui_interface, const char* label, FAUSTFLOAT* zone, FAUSTFLOAT min, FAUSTFLOAT max);

// -- soundfiles
    
typedef void (* addSoundfileFun) (void* ui_interface, const char* label, const char* url, struct Soundfile** sf_zone);

typedef void (* declareFun) (void* ui_interface, FAUSTFLOAT* zone, const char* key, const char* value);

typedef struct {

    void* uiInterface;

    openTabBoxFun openTabBox;
    openHorizontalBoxFun openHorizontalBox;
    openVerticalBoxFun openVerticalBox;
    closeBoxFun closeBox;
    addButtonFun addButton;
    addCheckButtonFun addCheckButton;
    addVerticalSliderFun addVerticalSlider;
    addHorizontalSliderFun addHorizontalSlider;
    addNumEntryFun addNumEntry;
    addHorizontalBargraphFun addHorizontalBargraph;
    addVerticalBargraphFun addVerticalBargraph;
    addSoundfileFun addSoundfile;
    declareFun declare;

} UIGlue;

typedef void (* metaDeclareFun) (void* ui_interface, const char* key, const char* value);

typedef struct {

    void* metaInterface;
    
    metaDeclareFun declare;

} MetaGlue;

/***************************************
 *  Interface for the DSP object
 ***************************************/

typedef char dsp_imp;
    
typedef dsp_imp* (* newDspFun) ();
typedef void (* deleteDspFun) (dsp_imp* dsp);
typedef void (* allocateDspFun) (dsp_imp* dsp);
typedef void (* destroyDspFun) (dsp_imp* dsp);
typedef int (* getNumInputsFun) (dsp_imp* dsp);
typedef int (* getNumOutputsFun) (dsp_imp* dsp);
typedef void (* buildUserInterfaceFun) (dsp_imp* dsp, UIGlue* ui);
typedef void (* initFun) (dsp_imp* dsp, int sample_rate);
typedef void (* clearFun) (dsp_imp* dsp);
typedef int (* getSampleRateFun) (dsp_imp* dsp);
typedef void (* computeFun) (dsp_imp* dsp, int len, FAUSTFLOAT** inputs, FAUSTFLOAT** outputs);
typedef void (* metadataFun) (MetaGlue* meta);
typedef void (* classInitFun) (int sample_rate);
typedef const char* (* getJSONFun) ();
    
/***************************************
 * DSP memory manager functions
 ***************************************/

typedef void* (* allocateFun) (void* manager_interface, size_t size);
typedef void (* destroyFun) (void* manager_interface, void* ptr);

typedef struct {
    
    void* managerInterface;
    
    allocateFun allocate;
    destroyFun destroy;
    
} ManagerGlue;

#ifdef __cplusplus
}
#endif

#endif
/**************************  END  CInterface.h **************************/
/* ------------------------------------------------------------
copyright: "(c)Romain Michon, CCRMA (Stanford University), GRAME"
license: "MIT"
name: "StandardChurchBell"
Code generated with Faust 2.18.3 (https://faust.grame.fr)
Compilation options: -lang c -scal -ftz 0
------------------------------------------------------------ */

#ifndef  __mydsp_H__
#define  __mydsp_H__

#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif 


#ifdef __cplusplus
extern "C" {
#endif

#include <math.h>
#include <stdlib.h>

static float fmydspSIG0Wave0[350] = {0.691910982f,0.62233299f,0.54865098f,0.46330601f,0.82694602f,0.74951297f,0.224199995f,0.642678022f,0.760442019f,0.326054007f,0.276463002f,0.359344006f,0.182579994f,0.686765015f,0.457159013f,0.839015007f,0.845337987f,0.372377008f,0.306416988f,0.147380993f,0.359706998f,0.653536975f,0.27553001f,0.401232988f,0.435416996f,0.251480997f,0.190062001f,0.773371994f,0.315014005f,0.228811994f,0.521511972f,0.411541998f,0.720762014f,1.0f,0.286502004f,0.338937998f,0.119994998f,0.432289004f,0.409676999f,0.156271994f,0.298871011f,0.250786006f,0.640775979f,0.209430993f,0.17001f,0.390013993f,0.301697999f,0.799413025f,0.980580986f,0.38499999f,0.82543999f,0.818894029f,0.349615991f,0.235395998f,0.783164024f,0.821914017f,0.28411001f,0.43028599f,0.507670999f,0.32625401f,0.260488003f,0.273364007f,0.205180004f,0.714851975f,0.479950011f,0.803637028f,0.683942974f,0.355370998f,0.406924009f,0.656256974f,0.423025012f,0.413515002f,0.38635999f,0.384786993f,0.389447987f,0.813367009f,0.234988004f,1.0f,0.311268002f,0.350244999f,0.403856009f,0.646143019f,0.500485003f,0.833553016f,0.431768f,0.467063993f,0.298979014f,0.487412989f,0.514907002f,0.369383007f,0.106197f,0.494224012f,0.816079021f,0.535807014f,0.379873008f,0.380201012f,0.606306016f,0.516116977f,0.748449028f,0.556948006f,0.587065995f,0.584423006f,0.39486599f,0.341120988f,0.433458f,0.455987006f,0.361236989f,0.429390013f,0.122969002f,0.133175001f,0.505176008f,0.513984978f,0.0554618984f,0.604942024f,0.372074008f,0.381125987f,0.314354002f,0.499635994f,0.518710971f,0.923792005f,0.259543985f,0.576516986f,0.553915024f,0.585443974f,0.245369002f,1.0f,0.117757f,0.977317989f,0.652862012f,0.509314001f,0.148550004f,0.506402016f,0.180059001f,0.356005013f,0.386810005f,0.279354006f,0.205791995f,0.551055014f,0.689107001f,0.44572401f,0.30685699f,0.324746996f,0.603621006f,0.394466013f,0.288612992f,0.264696985f,0.60611999f,0.202739999f,0.267271012f,0.925656021f,0.439227998f,0.425884008f,0.626632988f,0.547204018f,0.230021998f,0.225654006f,0.392697006f,0.493474007f,0.149857f,0.0604047999f,0.693889022f,0.740270972f,0.175485f,0.704998016f,0.329732001f,0.153026f,0.125744f,0.286994994f,0.278878003f,0.812372029f,0.0562173985f,0.241478994f,0.294524997f,0.358833998f,0.171047002f,0.847603977f,0.172279999f,0.975210011f,0.892072976f,0.613987029f,0.0659212992f,0.301582992f,0.0610846989f,0.125438005f,0.145151004f,0.180086002f,0.124231003f,0.260161012f,0.337572992f,0.203742996f,0.655798018f,0.425893009f,0.902347028f,0.50068599f,0.311172992f,0.215561002f,0.349590987f,0.0854218006f,0.0805061981f,1.0f,0.338652015f,0.295396f,0.698314011f,0.664972007f,0.118983001f,0.0881905034f,0.311580002f,0.391135991f,0.151914999f,0.239503995f,0.685742021f,0.884332001f,0.288516015f,0.768688023f,0.274850994f,0.0490311012f,0.0357864983f,0.293303013f,0.249460995f,0.493770987f,0.340983987f,0.467622995f,0.216630995f,0.255234987f,0.0988695025f,0.461979985f,0.147247002f,0.640196025f,1.0f,0.551937997f,0.0453732014f,0.189906999f,0.0197541993f,0.0309216995f,0.769837022f,0.360417992f,0.384041011f,0.867434025f,0.398948014f,0.171847999f,0.748651981f,0.301957011f,0.860611022f,0.958674014f,0.549030006f,0.272753f,0.372752994f,0.0180728007f,0.0292352997f,0.850199997f,0.224583f,0.214805007f,0.670319021f,0.586432993f,0.0435141996f,0.0388574004f,0.144811004f,0.157060996f,0.155569002f,0.418334007f,0.673655987f,0.749572992f,0.337354004f,0.747254014f,0.255997002f,0.0239656009f,0.0310718995f,0.721086979f,0.700616002f,0.199050993f,0.511843979f,0.84948498f,0.700681984f,0.778657973f,0.171288997f,0.261972994f,0.129227996f,0.328597009f,0.781821012f,0.583813012f,0.080671303f,0.416875988f,0.0118201999f,0.00868562981f,1.0f,0.461883992f,0.186882004f,0.641363978f,0.994705021f,0.501901984f,0.566448987f,0.0678844973f,0.139736995f,0.462581992f,0.318655998f,0.233946994f,0.495941013f,0.0314028002f,0.0146477995f,0.704320014f,0.124953002f,0.132549003f,0.457125992f,0.378636003f,0.0169361997f,0.0195493996f,0.204154998f,0.29440099f,0.271367013f,0.730857015f,0.459322006f,0.433077991f,0.325170994f,0.734535992f,0.416204989f,0.0128730005f,0.0388488993f,0.821566999f,0.863682985f,0.0920531005f,0.393972009f,0.539543986f,0.832051992f,0.842732012f,0.241144001f,0.479557991f,0.283091992f,0.477845013f,0.385473013f,0.436587006f,0.144308001f,0.64239502f,0.0215790998f,0.00779028982f,0.563714027f,0.838279009f,0.41000399f,0.829086006f,1.0f,0.630598009f,0.0233728997f,0.496217012f,0.711041987f,0.91426599f,0.695042014f,0.33189401f,0.89844197f,0.0285679996f,0.0174966007f,0.482845992f};

typedef struct {
	
	int fmydspSIG0Wave0_idx;
	
} mydspSIG0;

static mydspSIG0* newmydspSIG0() { return (mydspSIG0*)calloc(1, sizeof(mydspSIG0)); }
static void deletemydspSIG0(mydspSIG0* dsp) { free(dsp); }

int getNumInputsmydspSIG0(mydspSIG0* dsp) {
	return 0;
	
}
int getNumOutputsmydspSIG0(mydspSIG0* dsp) {
	return 1;
	
}
int getInputRatemydspSIG0(mydspSIG0* dsp, int channel) {
	int rate;
	switch (channel) {
		default: {
			rate = -1;
			break;
		}
		
	}
	return rate;
	
}
int getOutputRatemydspSIG0(mydspSIG0* dsp, int channel) {
	int rate;
	switch (channel) {
		case 0: {
			rate = 0;
			break;
		}
		default: {
			rate = -1;
			break;
		}
		
	}
	return rate;
	
}

static void instanceInitmydspSIG0(mydspSIG0* dsp, int sample_rate) {
	dsp->fmydspSIG0Wave0_idx = 0;
	
}

static void fillmydspSIG0(mydspSIG0* dsp, int count, float* table) {
	/* C99 loop */
	{
		int i;
		for (i = 0; (i < count); i = (i + 1)) {
			table[i] = fmydspSIG0Wave0[dsp->fmydspSIG0Wave0_idx];
			dsp->fmydspSIG0Wave0_idx = ((1 + dsp->fmydspSIG0Wave0_idx) % 350);
			
		}
		
	}
	
};

static float mydsp_faustpower2_f(float value) {
	return (value * value);
	
}
static float ftbl0mydspSIG0[350];

#ifndef FAUSTCLASS 
#define FAUSTCLASS mydsp
#endif
#ifdef __APPLE__ 
#define exp10f __exp10f
#define exp10 __exp10
#endif

typedef struct {
	
	FAUSTFLOAT fHslider0;
	int fSampleRate;
	float fConst0;
	float fConst1;
	FAUSTFLOAT fHslider1;
	float fConst2;
	float fConst3;
	float fConst4;
	float fConst5;
	float fConst6;
	int iRec3[2];
	float fConst7;
	float fConst8;
	float fRec2[3];
	float fConst9;
	float fRec1[3];
	FAUSTFLOAT fButton0;
	float fVec0[2];
	float fConst10;
	FAUSTFLOAT fHslider2;
	float fVec1[2];
	float fRec4[2];
	float fConst11;
	float fConst12;
	float fConst13;
	float fConst14;
	float fConst15;
	float fRec0[3];
	FAUSTFLOAT fEntry0;
	float fConst16;
	float fConst17;
	float fConst18;
	float fRec5[3];
	float fConst19;
	float fConst20;
	float fConst21;
	float fRec6[3];
	float fConst22;
	float fConst23;
	float fConst24;
	float fRec7[3];
	float fConst25;
	float fConst26;
	float fConst27;
	float fRec8[3];
	float fConst28;
	float fConst29;
	float fConst30;
	float fRec9[3];
	float fConst31;
	float fConst32;
	float fConst33;
	float fRec10[3];
	float fConst34;
	float fConst35;
	float fConst36;
	float fRec11[3];
	float fConst37;
	float fConst38;
	float fConst39;
	float fRec12[3];
	float fConst40;
	float fConst41;
	float fConst42;
	float fRec13[3];
	float fConst43;
	float fConst44;
	float fConst45;
	float fRec14[3];
	float fConst46;
	float fConst47;
	float fConst48;
	float fRec15[3];
	float fConst49;
	float fConst50;
	float fConst51;
	float fRec16[3];
	float fConst52;
	float fConst53;
	float fConst54;
	float fRec17[3];
	float fConst55;
	float fConst56;
	float fConst57;
	float fRec18[3];
	float fConst58;
	float fConst59;
	float fConst60;
	float fRec19[3];
	float fConst61;
	float fConst62;
	float fConst63;
	float fRec20[3];
	float fConst64;
	float fConst65;
	float fConst66;
	float fRec21[3];
	float fConst67;
	float fConst68;
	float fConst69;
	float fRec22[3];
	float fConst70;
	float fConst71;
	float fConst72;
	float fRec23[3];
	float fConst73;
	float fConst74;
	float fConst75;
	float fRec24[3];
	float fConst76;
	float fConst77;
	float fConst78;
	float fRec25[3];
	float fConst79;
	float fConst80;
	float fConst81;
	float fRec26[3];
	float fConst82;
	float fConst83;
	float fConst84;
	float fRec27[3];
	float fConst85;
	float fConst86;
	float fConst87;
	float fRec28[3];
	float fConst88;
	float fConst89;
	float fConst90;
	float fRec29[3];
	float fConst91;
	float fConst92;
	float fConst93;
	float fRec30[3];
	float fConst94;
	float fConst95;
	float fConst96;
	float fRec31[3];
	float fConst97;
	float fConst98;
	float fConst99;
	float fRec32[3];
	float fConst100;
	float fConst101;
	float fConst102;
	float fRec33[3];
	float fConst103;
	float fConst104;
	float fConst105;
	float fRec34[3];
	float fConst106;
	float fConst107;
	float fConst108;
	float fRec35[3];
	float fConst109;
	float fConst110;
	float fConst111;
	float fRec36[3];
	float fConst112;
	float fConst113;
	float fConst114;
	float fRec37[3];
	float fConst115;
	float fConst116;
	float fConst117;
	float fRec38[3];
	float fConst118;
	float fConst119;
	float fConst120;
	float fRec39[3];
	float fConst121;
	float fConst122;
	float fConst123;
	float fRec40[3];
	float fConst124;
	float fConst125;
	float fConst126;
	float fRec41[3];
	float fConst127;
	float fConst128;
	float fConst129;
	float fRec42[3];
	float fConst130;
	float fConst131;
	float fConst132;
	float fRec43[3];
	float fConst133;
	float fConst134;
	float fConst135;
	float fRec44[3];
	float fConst136;
	float fConst137;
	float fConst138;
	float fRec45[3];
	float fConst139;
	float fConst140;
	float fConst141;
	float fRec46[3];
	float fConst142;
	float fConst143;
	float fConst144;
	float fRec47[3];
	float fConst145;
	float fConst146;
	float fConst147;
	float fRec48[3];
	float fConst148;
	float fConst149;
	float fConst150;
	float fRec49[3];
	float fConst151;
	float fConst152;
	float fConst153;
	float fRec50[3];
	float fConst154;
	float fConst155;
	float fConst156;
	float fRec51[3];
	float fConst157;
	float fConst158;
	float fConst159;
	float fRec52[3];
	float fConst160;
	float fConst161;
	float fConst162;
	float fRec53[3];
	
} mydsp;

mydsp* newmydsp() { 
	mydsp* dsp = (mydsp*)calloc(1, sizeof(mydsp));
	return dsp;
}

void deletemydsp(mydsp* dsp) { 
	free(dsp);
}

void metadatamydsp(MetaGlue* m) { 
	m->declare(m->metaInterface, "basics.lib/name", "Faust Basic Element Library");
	m->declare(m->metaInterface, "basics.lib/version", "0.0");
	m->declare(m->metaInterface, "copyright", "(c)Romain Michon, CCRMA (Stanford University), GRAME");
	m->declare(m->metaInterface, "description", "Standard church bell physical model.");
	m->declare(m->metaInterface, "envelopes.lib/author", "GRAME");
	m->declare(m->metaInterface, "envelopes.lib/copyright", "GRAME");
	m->declare(m->metaInterface, "envelopes.lib/license", "LGPL with exception");
	m->declare(m->metaInterface, "envelopes.lib/name", "Faust Envelope Library");
	m->declare(m->metaInterface, "envelopes.lib/version", "0.0");
	m->declare(m->metaInterface, "filename", "standardBell.dsp");
	m->declare(m->metaInterface, "filters.lib/name", "Faust Filters Library");
	m->declare(m->metaInterface, "filters.lib/version", "0.0");
	m->declare(m->metaInterface, "license", "MIT");
	m->declare(m->metaInterface, "maths.lib/author", "GRAME");
	m->declare(m->metaInterface, "maths.lib/copyright", "GRAME");
	m->declare(m->metaInterface, "maths.lib/license", "LGPL with exception");
	m->declare(m->metaInterface, "maths.lib/name", "Faust Math Library");
	m->declare(m->metaInterface, "maths.lib/version", "2.1");
	m->declare(m->metaInterface, "name", "StandardChurchBell");
	m->declare(m->metaInterface, "noises.lib/name", "Faust Noise Generator Library");
	m->declare(m->metaInterface, "noises.lib/version", "0.0");
}

int getSampleRatemydsp(mydsp* dsp) { return dsp->fSampleRate; }

int getNumInputsmydsp(mydsp* dsp) {
	return 0;
	
}
int getNumOutputsmydsp(mydsp* dsp) {
	return 2;
	
}
int getInputRatemydsp(mydsp* dsp, int channel) {
	int rate;
	switch (channel) {
		default: {
			rate = -1;
			break;
		}
		
	}
	return rate;
	
}
int getOutputRatemydsp(mydsp* dsp, int channel) {
	int rate;
	switch (channel) {
		case 0: {
			rate = 1;
			break;
		}
		case 1: {
			rate = 1;
			break;
		}
		default: {
			rate = -1;
			break;
		}
		
	}
	return rate;
	
}

void classInitmydsp(int sample_rate) {
	mydspSIG0* sig0 = newmydspSIG0();
	instanceInitmydspSIG0(sig0, sample_rate);
	fillmydspSIG0(sig0, 350, ftbl0mydspSIG0);
	deletemydspSIG0(sig0);
	
}

void instanceResetUserInterfacemydsp(mydsp* dsp) {
	dsp->fHslider0 = (FAUSTFLOAT)1.0f;
	dsp->fHslider1 = (FAUSTFLOAT)6500.0f;
	dsp->fButton0 = (FAUSTFLOAT)0.0f;
	dsp->fHslider2 = (FAUSTFLOAT)0.5f;
	dsp->fEntry0 = (FAUSTFLOAT)0.0f;
	
}

void instanceClearmydsp(mydsp* dsp) {
	/* C99 loop */
	{
		int l0;
		for (l0 = 0; (l0 < 2); l0 = (l0 + 1)) {
			dsp->iRec3[l0] = 0;
			
		}
		
	}
	/* C99 loop */
	{
		int l1;
		for (l1 = 0; (l1 < 3); l1 = (l1 + 1)) {
			dsp->fRec2[l1] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l2;
		for (l2 = 0; (l2 < 3); l2 = (l2 + 1)) {
			dsp->fRec1[l2] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l3;
		for (l3 = 0; (l3 < 2); l3 = (l3 + 1)) {
			dsp->fVec0[l3] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l4;
		for (l4 = 0; (l4 < 2); l4 = (l4 + 1)) {
			dsp->fVec1[l4] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l5;
		for (l5 = 0; (l5 < 2); l5 = (l5 + 1)) {
			dsp->fRec4[l5] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l6;
		for (l6 = 0; (l6 < 3); l6 = (l6 + 1)) {
			dsp->fRec0[l6] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l7;
		for (l7 = 0; (l7 < 3); l7 = (l7 + 1)) {
			dsp->fRec5[l7] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l8;
		for (l8 = 0; (l8 < 3); l8 = (l8 + 1)) {
			dsp->fRec6[l8] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l9;
		for (l9 = 0; (l9 < 3); l9 = (l9 + 1)) {
			dsp->fRec7[l9] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l10;
		for (l10 = 0; (l10 < 3); l10 = (l10 + 1)) {
			dsp->fRec8[l10] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l11;
		for (l11 = 0; (l11 < 3); l11 = (l11 + 1)) {
			dsp->fRec9[l11] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l12;
		for (l12 = 0; (l12 < 3); l12 = (l12 + 1)) {
			dsp->fRec10[l12] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l13;
		for (l13 = 0; (l13 < 3); l13 = (l13 + 1)) {
			dsp->fRec11[l13] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l14;
		for (l14 = 0; (l14 < 3); l14 = (l14 + 1)) {
			dsp->fRec12[l14] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l15;
		for (l15 = 0; (l15 < 3); l15 = (l15 + 1)) {
			dsp->fRec13[l15] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l16;
		for (l16 = 0; (l16 < 3); l16 = (l16 + 1)) {
			dsp->fRec14[l16] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l17;
		for (l17 = 0; (l17 < 3); l17 = (l17 + 1)) {
			dsp->fRec15[l17] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l18;
		for (l18 = 0; (l18 < 3); l18 = (l18 + 1)) {
			dsp->fRec16[l18] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l19;
		for (l19 = 0; (l19 < 3); l19 = (l19 + 1)) {
			dsp->fRec17[l19] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l20;
		for (l20 = 0; (l20 < 3); l20 = (l20 + 1)) {
			dsp->fRec18[l20] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l21;
		for (l21 = 0; (l21 < 3); l21 = (l21 + 1)) {
			dsp->fRec19[l21] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l22;
		for (l22 = 0; (l22 < 3); l22 = (l22 + 1)) {
			dsp->fRec20[l22] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l23;
		for (l23 = 0; (l23 < 3); l23 = (l23 + 1)) {
			dsp->fRec21[l23] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l24;
		for (l24 = 0; (l24 < 3); l24 = (l24 + 1)) {
			dsp->fRec22[l24] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l25;
		for (l25 = 0; (l25 < 3); l25 = (l25 + 1)) {
			dsp->fRec23[l25] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l26;
		for (l26 = 0; (l26 < 3); l26 = (l26 + 1)) {
			dsp->fRec24[l26] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l27;
		for (l27 = 0; (l27 < 3); l27 = (l27 + 1)) {
			dsp->fRec25[l27] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l28;
		for (l28 = 0; (l28 < 3); l28 = (l28 + 1)) {
			dsp->fRec26[l28] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l29;
		for (l29 = 0; (l29 < 3); l29 = (l29 + 1)) {
			dsp->fRec27[l29] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l30;
		for (l30 = 0; (l30 < 3); l30 = (l30 + 1)) {
			dsp->fRec28[l30] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l31;
		for (l31 = 0; (l31 < 3); l31 = (l31 + 1)) {
			dsp->fRec29[l31] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l32;
		for (l32 = 0; (l32 < 3); l32 = (l32 + 1)) {
			dsp->fRec30[l32] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l33;
		for (l33 = 0; (l33 < 3); l33 = (l33 + 1)) {
			dsp->fRec31[l33] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l34;
		for (l34 = 0; (l34 < 3); l34 = (l34 + 1)) {
			dsp->fRec32[l34] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l35;
		for (l35 = 0; (l35 < 3); l35 = (l35 + 1)) {
			dsp->fRec33[l35] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l36;
		for (l36 = 0; (l36 < 3); l36 = (l36 + 1)) {
			dsp->fRec34[l36] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l37;
		for (l37 = 0; (l37 < 3); l37 = (l37 + 1)) {
			dsp->fRec35[l37] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l38;
		for (l38 = 0; (l38 < 3); l38 = (l38 + 1)) {
			dsp->fRec36[l38] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l39;
		for (l39 = 0; (l39 < 3); l39 = (l39 + 1)) {
			dsp->fRec37[l39] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l40;
		for (l40 = 0; (l40 < 3); l40 = (l40 + 1)) {
			dsp->fRec38[l40] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l41;
		for (l41 = 0; (l41 < 3); l41 = (l41 + 1)) {
			dsp->fRec39[l41] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l42;
		for (l42 = 0; (l42 < 3); l42 = (l42 + 1)) {
			dsp->fRec40[l42] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l43;
		for (l43 = 0; (l43 < 3); l43 = (l43 + 1)) {
			dsp->fRec41[l43] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l44;
		for (l44 = 0; (l44 < 3); l44 = (l44 + 1)) {
			dsp->fRec42[l44] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l45;
		for (l45 = 0; (l45 < 3); l45 = (l45 + 1)) {
			dsp->fRec43[l45] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l46;
		for (l46 = 0; (l46 < 3); l46 = (l46 + 1)) {
			dsp->fRec44[l46] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l47;
		for (l47 = 0; (l47 < 3); l47 = (l47 + 1)) {
			dsp->fRec45[l47] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l48;
		for (l48 = 0; (l48 < 3); l48 = (l48 + 1)) {
			dsp->fRec46[l48] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l49;
		for (l49 = 0; (l49 < 3); l49 = (l49 + 1)) {
			dsp->fRec47[l49] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l50;
		for (l50 = 0; (l50 < 3); l50 = (l50 + 1)) {
			dsp->fRec48[l50] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l51;
		for (l51 = 0; (l51 < 3); l51 = (l51 + 1)) {
			dsp->fRec49[l51] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l52;
		for (l52 = 0; (l52 < 3); l52 = (l52 + 1)) {
			dsp->fRec50[l52] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l53;
		for (l53 = 0; (l53 < 3); l53 = (l53 + 1)) {
			dsp->fRec51[l53] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l54;
		for (l54 = 0; (l54 < 3); l54 = (l54 + 1)) {
			dsp->fRec52[l54] = 0.0f;
			
		}
		
	}
	/* C99 loop */
	{
		int l55;
		for (l55 = 0; (l55 < 3); l55 = (l55 + 1)) {
			dsp->fRec53[l55] = 0.0f;
			
		}
		
	}
	
}

void instanceConstantsmydsp(mydsp* dsp, int sample_rate) {
	dsp->fSampleRate = sample_rate;
	dsp->fConst0 = min(192000.0f, max(1.0f, (float)dsp->fSampleRate));
	dsp->fConst1 = (3.14159274f / dsp->fConst0);
	dsp->fConst2 = tanf((31.415926f / dsp->fConst0));
	dsp->fConst3 = (1.0f / dsp->fConst2);
	dsp->fConst4 = (1.0f / (((dsp->fConst3 + 1.41421354f) / dsp->fConst2) + 1.0f));
	dsp->fConst5 = mydsp_faustpower2_f(dsp->fConst2);
	dsp->fConst6 = (1.0f / dsp->fConst5);
	dsp->fConst7 = (((dsp->fConst3 + -1.41421354f) / dsp->fConst2) + 1.0f);
	dsp->fConst8 = (2.0f * (1.0f - dsp->fConst6));
	dsp->fConst9 = (0.0f - (2.0f / dsp->fConst5));
	dsp->fConst10 = (0.00400000019f * dsp->fConst0);
	dsp->fConst11 = (0.00200000009f * dsp->fConst0);
	dsp->fConst12 = (500.0f / dsp->fConst0);
	dsp->fConst13 = powf(0.00100000005f, (975.121521f / dsp->fConst0));
	dsp->fConst14 = (cosf((25908.1484f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst13)));
	dsp->fConst15 = mydsp_faustpower2_f(dsp->fConst13);
	dsp->fConst16 = powf(0.00100000005f, (72.7954712f / dsp->fConst0));
	dsp->fConst17 = (cosf((25122.877f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst16)));
	dsp->fConst18 = mydsp_faustpower2_f(dsp->fConst16);
	dsp->fConst19 = powf(0.00100000005f, (68.4358597f / dsp->fConst0));
	dsp->fConst20 = (cosf((25092.4668f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst19)));
	dsp->fConst21 = mydsp_faustpower2_f(dsp->fConst19);
	dsp->fConst22 = powf(0.00100000005f, (11.6631393f / dsp->fConst0));
	dsp->fConst23 = (cosf((23809.377f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst22)));
	dsp->fConst24 = mydsp_faustpower2_f(dsp->fConst22);
	dsp->fConst25 = powf(0.00100000005f, (11.4415998f / dsp->fConst0));
	dsp->fConst26 = (cosf((23789.8984f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst25)));
	dsp->fConst27 = mydsp_faustpower2_f(dsp->fConst25);
	dsp->fConst28 = powf(0.00100000005f, (7.94609594f / dsp->fConst0));
	dsp->fConst29 = (cosf((23389.7227f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst28)));
	dsp->fConst30 = mydsp_faustpower2_f(dsp->fConst28);
	dsp->fConst31 = powf(0.00100000005f, (7.79781151f / dsp->fConst0));
	dsp->fConst32 = (cosf((23367.418f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst31)));
	dsp->fConst33 = mydsp_faustpower2_f(dsp->fConst31);
	dsp->fConst34 = powf(0.00100000005f, (6.35503197f / dsp->fConst0));
	dsp->fConst35 = (cosf((23114.0176f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst34)));
	dsp->fConst36 = mydsp_faustpower2_f(dsp->fConst34);
	dsp->fConst37 = powf(0.00100000005f, (2.86326194f / dsp->fConst0));
	dsp->fConst38 = (cosf((21902.6816f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst37)));
	dsp->fConst39 = mydsp_faustpower2_f(dsp->fConst37);
	dsp->fConst40 = powf(0.00100000005f, (2.09878469f / dsp->fConst0));
	dsp->fConst41 = (cosf((21315.832f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst40)));
	dsp->fConst42 = mydsp_faustpower2_f(dsp->fConst40);
	dsp->fConst43 = powf(0.00100000005f, (0.664678693f / dsp->fConst0));
	dsp->fConst44 = (cosf((18382.6523f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst43)));
	dsp->fConst45 = mydsp_faustpower2_f(dsp->fConst43);
	dsp->fConst46 = powf(0.00100000005f, (0.661161721f / dsp->fConst0));
	dsp->fConst47 = (cosf((18365.75f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst46)));
	dsp->fConst48 = mydsp_faustpower2_f(dsp->fConst46);
	dsp->fConst49 = powf(0.00100000005f, (0.601628184f / dsp->fConst0));
	dsp->fConst50 = (cosf((18059.0684f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst49)));
	dsp->fConst51 = mydsp_faustpower2_f(dsp->fConst49);
	dsp->fConst52 = powf(0.00100000005f, (0.589509189f / dsp->fConst0));
	dsp->fConst53 = (cosf((17991.3984f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst52)));
	dsp->fConst54 = mydsp_faustpower2_f(dsp->fConst52);
	dsp->fConst55 = powf(0.00100000005f, (0.551191032f / dsp->fConst0));
	dsp->fConst56 = (cosf((17763.9473f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst55)));
	dsp->fConst57 = mydsp_faustpower2_f(dsp->fConst55);
	dsp->fConst58 = powf(0.00100000005f, (0.0766132995f / dsp->fConst0));
	dsp->fConst59 = (cosf((7457.76416f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst58)));
	dsp->fConst60 = mydsp_faustpower2_f(dsp->fConst58);
	dsp->fConst61 = powf(0.00100000005f, (0.0762493014f / dsp->fConst0));
	dsp->fConst62 = (cosf((7421.76123f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst61)));
	dsp->fConst63 = mydsp_faustpower2_f(dsp->fConst61);
	dsp->fConst64 = powf(0.00100000005f, (0.0622998103f / dsp->fConst0));
	dsp->fConst65 = (cosf((5829.40723f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst64)));
	dsp->fConst66 = mydsp_faustpower2_f(dsp->fConst64);
	dsp->fConst67 = powf(0.00100000005f, (0.0621597022f / dsp->fConst0));
	dsp->fConst68 = (cosf((5810.92871f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst67)));
	dsp->fConst69 = mydsp_faustpower2_f(dsp->fConst67);
	dsp->fConst70 = powf(0.00100000005f, (0.0455945022f / dsp->fConst0));
	dsp->fConst71 = (cosf((3101.66919f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst70)));
	dsp->fConst72 = mydsp_faustpower2_f(dsp->fConst70);
	dsp->fConst73 = powf(0.00100000005f, (0.0454900004f / dsp->fConst0));
	dsp->fConst74 = (cosf((3080.33154f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst73)));
	dsp->fConst75 = mydsp_faustpower2_f(dsp->fConst73);
	dsp->fConst76 = powf(0.00100000005f, (0.392319173f / dsp->fConst0));
	dsp->fConst77 = (cosf((16514.7246f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst76)));
	dsp->fConst78 = mydsp_faustpower2_f(dsp->fConst76);
	dsp->fConst79 = powf(0.00100000005f, (0.389138192f / dsp->fConst0));
	dsp->fConst80 = (cosf((16482.6797f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst79)));
	dsp->fConst81 = mydsp_faustpower2_f(dsp->fConst79);
	dsp->fConst82 = powf(0.00100000005f, (0.340076268f / dsp->fConst0));
	dsp->fConst83 = (cosf((15936.7969f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst82)));
	dsp->fConst84 = mydsp_faustpower2_f(dsp->fConst82);
	dsp->fConst85 = powf(0.00100000005f, (0.338892877f / dsp->fConst0));
	dsp->fConst86 = (cosf((15922.2832f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst85)));
	dsp->fConst87 = mydsp_faustpower2_f(dsp->fConst85);
	dsp->fConst88 = powf(0.00100000005f, (0.282201737f / dsp->fConst0));
	dsp->fConst89 = (cosf((15130.916f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst88)));
	dsp->fConst90 = mydsp_faustpower2_f(dsp->fConst88);
	dsp->fConst91 = powf(0.00100000005f, (0.201628014f / dsp->fConst0));
	dsp->fConst92 = (cosf((13517.6445f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst91)));
	dsp->fConst93 = mydsp_faustpower2_f(dsp->fConst91);
	dsp->fConst94 = powf(0.00100000005f, (0.199414521f / dsp->fConst0));
	dsp->fConst95 = (cosf((13460.9072f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst94)));
	dsp->fConst96 = mydsp_faustpower2_f(dsp->fConst94);
	dsp->fConst97 = powf(0.00100000005f, (0.179417238f / dsp->fConst0));
	dsp->fConst98 = (cosf((12904.9082f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst97)));
	dsp->fConst99 = mydsp_faustpower2_f(dsp->fConst97);
	dsp->fConst100 = powf(0.00100000005f, (0.173593029f / dsp->fConst0));
	dsp->fConst101 = (cosf((12726.4033f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst100)));
	dsp->fConst102 = mydsp_faustpower2_f(dsp->fConst100);
	dsp->fConst103 = powf(0.00100000005f, (0.169658288f / dsp->fConst0));
	dsp->fConst104 = (cosf((12600.9912f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst103)));
	dsp->fConst105 = mydsp_faustpower2_f(dsp->fConst103);
	dsp->fConst106 = powf(0.00100000005f, (0.128075823f / dsp->fConst0));
	dsp->fConst107 = (cosf((10965.7295f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst106)));
	dsp->fConst108 = mydsp_faustpower2_f(dsp->fConst106);
	dsp->fConst109 = powf(0.00100000005f, (0.128049657f / dsp->fConst0));
	dsp->fConst110 = (cosf((10964.4727f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst109)));
	dsp->fConst111 = mydsp_faustpower2_f(dsp->fConst109);
	dsp->fConst112 = powf(0.00100000005f, (0.123937115f / dsp->fConst0));
	dsp->fConst113 = (cosf((10762.4053f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst112)));
	dsp->fConst114 = mydsp_faustpower2_f(dsp->fConst112);
	dsp->fConst115 = powf(0.00100000005f, (0.123170547f / dsp->fConst0));
	dsp->fConst116 = (cosf((10723.7012f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst115)));
	dsp->fConst117 = mydsp_faustpower2_f(dsp->fConst115);
	dsp->fConst118 = powf(0.00100000005f, (0.114822067f / dsp->fConst0));
	dsp->fConst119 = (cosf((10279.1025f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst118)));
	dsp->fConst120 = mydsp_faustpower2_f(dsp->fConst118);
	dsp->fConst121 = powf(0.00100000005f, (0.106748313f / dsp->fConst0));
	dsp->fConst122 = (cosf((9803.84277f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst121)));
	dsp->fConst123 = mydsp_faustpower2_f(dsp->fConst121);
	dsp->fConst124 = powf(0.00100000005f, (0.0880413875f / dsp->fConst0));
	dsp->fConst125 = (cosf((8479.1582f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst124)));
	dsp->fConst126 = mydsp_faustpower2_f(dsp->fConst124);
	dsp->fConst127 = powf(0.00100000005f, (0.0879903063f / dsp->fConst0));
	dsp->fConst128 = (cosf((8475.01172f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst127)));
	dsp->fConst129 = mydsp_faustpower2_f(dsp->fConst127);
	dsp->fConst130 = powf(0.00100000005f, (0.449765325f / dsp->fConst0));
	dsp->fConst131 = (cosf((17037.2969f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst130)));
	dsp->fConst132 = mydsp_faustpower2_f(dsp->fConst130);
	dsp->fConst133 = powf(0.00100000005f, (0.450442016f / dsp->fConst0));
	dsp->fConst134 = (cosf((17042.8887f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst133)));
	dsp->fConst135 = mydsp_faustpower2_f(dsp->fConst133);
	dsp->fConst136 = powf(0.00100000005f, (0.828528881f / dsp->fConst0));
	dsp->fConst137 = (cosf((19053.8848f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst136)));
	dsp->fConst138 = mydsp_faustpower2_f(dsp->fConst136);
	dsp->fConst139 = powf(0.00100000005f, (0.547182798f / dsp->fConst0));
	dsp->fConst140 = (cosf((17738.877f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst139)));
	dsp->fConst141 = mydsp_faustpower2_f(dsp->fConst139);
	dsp->fConst142 = powf(0.00100000005f, (0.845986068f / dsp->fConst0));
	dsp->fConst143 = (cosf((19114.3926f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst142)));
	dsp->fConst144 = mydsp_faustpower2_f(dsp->fConst142);
	dsp->fConst145 = powf(0.00100000005f, (1.25059175f / dsp->fConst0));
	dsp->fConst146 = (cosf((20160.0391f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst145)));
	dsp->fConst147 = mydsp_faustpower2_f(dsp->fConst145);
	dsp->fConst148 = powf(0.00100000005f, (2.94316006f / dsp->fConst0));
	dsp->fConst149 = (cosf((21951.25f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst148)));
	dsp->fConst150 = mydsp_faustpower2_f(dsp->fConst148);
	dsp->fConst151 = powf(0.00100000005f, (3.49180698f / dsp->fConst0));
	dsp->fConst152 = (cosf((22241.2188f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst151)));
	dsp->fConst153 = mydsp_faustpower2_f(dsp->fConst151);
	dsp->fConst154 = powf(0.00100000005f, (3.64010167f / dsp->fConst0));
	dsp->fConst155 = (cosf((22308.8262f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst154)));
	dsp->fConst156 = mydsp_faustpower2_f(dsp->fConst154);
	dsp->fConst157 = powf(0.00100000005f, (9901.3584f / dsp->fConst0));
	dsp->fConst158 = (cosf((26168.3984f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst157)));
	dsp->fConst159 = mydsp_faustpower2_f(dsp->fConst157);
	dsp->fConst160 = powf(0.00100000005f, (1207654.5f / dsp->fConst0));
	dsp->fConst161 = (cosf((26313.8535f / dsp->fConst0)) * (0.0f - (2.0f * dsp->fConst160)));
	dsp->fConst162 = mydsp_faustpower2_f(dsp->fConst160);
	
}

void instanceInitmydsp(mydsp* dsp, int sample_rate) {
	instanceConstantsmydsp(dsp, sample_rate);
	instanceResetUserInterfacemydsp(dsp);
	instanceClearmydsp(dsp);
}

void initmydsp(mydsp* dsp, int sample_rate) {
        fprintf(stderr, "Size of float: %ld bytes\n", sizeof(float));
	classInitmydsp(sample_rate);
        fprintf(stderr, "Hello after classInit\n");
	instanceInitmydsp(dsp, sample_rate);
        fprintf(stderr, "Hello after instanceInit\n");
}

void buildUserInterfacemydsp(mydsp* dsp, UIGlue* ui_interface) {
        fprintf(stderr, "BUILDING INTERFACE\n");
	ui_interface->openVerticalBox(ui_interface->uiInterface, "standardBell");
        fprintf(stderr, "CALLED VERTICAL BOX\n");
	ui_interface->declare(ui_interface->uiInterface, &dsp->fEntry0, "0", "");
	ui_interface->addNumEntry(ui_interface->uiInterface, "strikePosition", &dsp->fEntry0, 0.0f, 0.0f, 4.0f, 1.0f);
	ui_interface->declare(ui_interface->uiInterface, &dsp->fHslider1, "1", "");
	ui_interface->addHorizontalSlider(ui_interface->uiInterface, "strikeCutOff", &dsp->fHslider1, 6500.0f, 20.0f, 20000.0f, 1.0f);
	ui_interface->declare(ui_interface->uiInterface, &dsp->fHslider2, "2", "");
	ui_interface->addHorizontalSlider(ui_interface->uiInterface, "strikeSharpness", &dsp->fHslider2, 0.5f, 0.00999999978f, 5.0f, 0.00999999978f);
	ui_interface->declare(ui_interface->uiInterface, &dsp->fHslider0, "3", "");
	ui_interface->addHorizontalSlider(ui_interface->uiInterface, "gain", &dsp->fHslider0, 1.0f, 0.0f, 1.0f, 0.00999999978f);
	ui_interface->declare(ui_interface->uiInterface, &dsp->fButton0, "4", "");
	ui_interface->addButton(ui_interface->uiInterface, "gate", &dsp->fButton0);
	ui_interface->closeBox(ui_interface->uiInterface);
	
}

void computemydsp(mydsp* dsp, int count, FAUSTFLOAT** inputs, FAUSTFLOAT** outputs) {
        fprintf(stderr, "hello 1\n");
        /*
	FAUSTFLOAT* output0 = outputs[0];
	FAUSTFLOAT* output1 = outputs[1];
	float fSlow0 = tanf((dsp->fConst1 * (float)dsp->fHslider1));
	float fSlow1 = (1.0f / fSlow0);
	float fSlow2 = (((fSlow1 + 1.41421354f) / fSlow0) + 1.0f);
	float fSlow3 = ((float)dsp->fHslider0 / fSlow2);
	float fSlow4 = (1.0f / fSlow2);
	float fSlow5 = (((fSlow1 + -1.41421354f) / fSlow0) + 1.0f);
	float fSlow6 = (2.0f * (1.0f - (1.0f / mydsp_faustpower2_f(fSlow0))));
	float fSlow7 = (float)dsp->fButton0;
	float fSlow8 = (float)dsp->fHslider2;
	float fSlow9 = (dsp->fConst10 * fSlow8);
	float fSlow10 = (dsp->fConst11 * fSlow8);
	float fSlow11 = (dsp->fConst12 / fSlow8);
	int iSlow12 = (50 * (int)(float)dsp->fEntry0);
	float fSlow13 = ftbl0mydspSIG0[(iSlow12 + 47)];
	float fSlow14 = ftbl0mydspSIG0[(iSlow12 + 46)];
	float fSlow15 = ftbl0mydspSIG0[(iSlow12 + 45)];
	float fSlow16 = ftbl0mydspSIG0[(iSlow12 + 44)];
	float fSlow17 = ftbl0mydspSIG0[(iSlow12 + 43)];
	float fSlow18 = ftbl0mydspSIG0[(iSlow12 + 42)];
	float fSlow19 = ftbl0mydspSIG0[(iSlow12 + 41)];
	float fSlow20 = ftbl0mydspSIG0[(iSlow12 + 40)];
	float fSlow21 = ftbl0mydspSIG0[(iSlow12 + 36)];
	float fSlow22 = ftbl0mydspSIG0[(iSlow12 + 35)];
	float fSlow23 = ftbl0mydspSIG0[(iSlow12 + 31)];
	float fSlow24 = ftbl0mydspSIG0[(iSlow12 + 30)];
	float fSlow25 = ftbl0mydspSIG0[(iSlow12 + 29)];
	float fSlow26 = ftbl0mydspSIG0[(iSlow12 + 28)];
	float fSlow27 = ftbl0mydspSIG0[(iSlow12 + 27)];
	float fSlow28 = ftbl0mydspSIG0[(iSlow12 + 5)];
	float fSlow29 = ftbl0mydspSIG0[(iSlow12 + 4)];
	float fSlow30 = ftbl0mydspSIG0[(iSlow12 + 3)];
	float fSlow31 = ftbl0mydspSIG0[(iSlow12 + 2)];
	float fSlow32 = ftbl0mydspSIG0[(iSlow12 + 1)];
	float fSlow33 = ftbl0mydspSIG0[iSlow12];
	float fSlow34 = ftbl0mydspSIG0[(iSlow12 + 23)];
	float fSlow35 = ftbl0mydspSIG0[(iSlow12 + 22)];
	float fSlow36 = ftbl0mydspSIG0[(iSlow12 + 21)];
	float fSlow37 = ftbl0mydspSIG0[(iSlow12 + 20)];
	float fSlow38 = ftbl0mydspSIG0[(iSlow12 + 19)];
	float fSlow39 = ftbl0mydspSIG0[(iSlow12 + 18)];
	float fSlow40 = ftbl0mydspSIG0[(iSlow12 + 17)];
	float fSlow41 = ftbl0mydspSIG0[(iSlow12 + 16)];
	float fSlow42 = ftbl0mydspSIG0[(iSlow12 + 15)];
	float fSlow43 = ftbl0mydspSIG0[(iSlow12 + 14)];
	float fSlow44 = ftbl0mydspSIG0[(iSlow12 + 13)];
	float fSlow45 = ftbl0mydspSIG0[(iSlow12 + 12)];
	float fSlow46 = ftbl0mydspSIG0[(iSlow12 + 11)];
	float fSlow47 = ftbl0mydspSIG0[(iSlow12 + 10)];
	float fSlow48 = ftbl0mydspSIG0[(iSlow12 + 9)];
	float fSlow49 = ftbl0mydspSIG0[(iSlow12 + 8)];
	float fSlow50 = ftbl0mydspSIG0[(iSlow12 + 7)];
	float fSlow51 = ftbl0mydspSIG0[(iSlow12 + 6)];
	float fSlow52 = ftbl0mydspSIG0[(iSlow12 + 24)];
	float fSlow53 = ftbl0mydspSIG0[(iSlow12 + 25)];
	float fSlow54 = ftbl0mydspSIG0[(iSlow12 + 32)];
	float fSlow55 = ftbl0mydspSIG0[(iSlow12 + 26)];
	float fSlow56 = ftbl0mydspSIG0[(iSlow12 + 33)];
	float fSlow57 = ftbl0mydspSIG0[(iSlow12 + 34)];
	float fSlow58 = ftbl0mydspSIG0[(iSlow12 + 37)];
	float fSlow59 = ftbl0mydspSIG0[(iSlow12 + 38)];
	float fSlow60 = ftbl0mydspSIG0[(iSlow12 + 39)];
	float fSlow61 = ftbl0mydspSIG0[(iSlow12 + 48)];
	float fSlow62 = ftbl0mydspSIG0[(iSlow12 + 49)];
	{
		int i;
		for (i = 0; (i < count); i = (i + 1)) {
			dsp->iRec3[0] = ((1103515245 * dsp->iRec3[1]) + 12345);
			dsp->fRec2[0] = ((4.65661287e-10f * (float)dsp->iRec3[0]) - (dsp->fConst4 * ((dsp->fConst7 * dsp->fRec2[2]) + (dsp->fConst8 * dsp->fRec2[1]))));
			dsp->fRec1[0] = ((dsp->fConst4 * (((dsp->fConst6 * dsp->fRec2[0]) + (dsp->fConst9 * dsp->fRec2[1])) + (dsp->fConst6 * dsp->fRec2[2]))) - (fSlow4 * ((fSlow5 * dsp->fRec1[2]) + (fSlow6 * dsp->fRec1[1]))));
			dsp->fVec0[0] = fSlow7;
			dsp->fVec1[0] = fSlow8;
			dsp->fRec4[0] = ((((fSlow7 - dsp->fVec0[1]) > 0.0f) > 0) ? 0.0f : min(fSlow9, ((dsp->fRec4[1] + (dsp->fConst10 * (fSlow8 - dsp->fVec1[1]))) + 1.0f)));
			int iTemp0 = (dsp->fRec4[0] < fSlow10);
			float fTemp1 = (fSlow3 * ((dsp->fRec1[2] + (dsp->fRec1[0] + (2.0f * dsp->fRec1[1]))) * (iTemp0 ? ((dsp->fRec4[0] < 0.0f) ? 0.0f : (iTemp0 ? (fSlow11 * dsp->fRec4[0]) : 1.0f)) : ((dsp->fRec4[0] < fSlow9) ? ((fSlow11 * (0.0f - (dsp->fRec4[0] - fSlow10))) + 1.0f) : 0.0f))));
			dsp->fRec0[0] = (fTemp1 - ((dsp->fConst14 * dsp->fRec0[1]) + (dsp->fConst15 * dsp->fRec0[2])));
			dsp->fRec5[0] = (fTemp1 - ((dsp->fConst17 * dsp->fRec5[1]) + (dsp->fConst18 * dsp->fRec5[2])));
			dsp->fRec6[0] = (fTemp1 - ((dsp->fConst20 * dsp->fRec6[1]) + (dsp->fConst21 * dsp->fRec6[2])));
			dsp->fRec7[0] = (fTemp1 - ((dsp->fConst23 * dsp->fRec7[1]) + (dsp->fConst24 * dsp->fRec7[2])));
			dsp->fRec8[0] = (fTemp1 - ((dsp->fConst26 * dsp->fRec8[1]) + (dsp->fConst27 * dsp->fRec8[2])));
			dsp->fRec9[0] = (fTemp1 - ((dsp->fConst29 * dsp->fRec9[1]) + (dsp->fConst30 * dsp->fRec9[2])));
			dsp->fRec10[0] = (fTemp1 - ((dsp->fConst32 * dsp->fRec10[1]) + (dsp->fConst33 * dsp->fRec10[2])));
			dsp->fRec11[0] = (fTemp1 - ((dsp->fConst35 * dsp->fRec11[1]) + (dsp->fConst36 * dsp->fRec11[2])));
			dsp->fRec12[0] = (fTemp1 - ((dsp->fConst38 * dsp->fRec12[1]) + (dsp->fConst39 * dsp->fRec12[2])));
			dsp->fRec13[0] = (fTemp1 - ((dsp->fConst41 * dsp->fRec13[1]) + (dsp->fConst42 * dsp->fRec13[2])));
			dsp->fRec14[0] = (fTemp1 - ((dsp->fConst44 * dsp->fRec14[1]) + (dsp->fConst45 * dsp->fRec14[2])));
			dsp->fRec15[0] = (fTemp1 - ((dsp->fConst47 * dsp->fRec15[1]) + (dsp->fConst48 * dsp->fRec15[2])));
			dsp->fRec16[0] = (fTemp1 - ((dsp->fConst50 * dsp->fRec16[1]) + (dsp->fConst51 * dsp->fRec16[2])));
			dsp->fRec17[0] = (fTemp1 - ((dsp->fConst53 * dsp->fRec17[1]) + (dsp->fConst54 * dsp->fRec17[2])));
			dsp->fRec18[0] = (fTemp1 - ((dsp->fConst56 * dsp->fRec18[1]) + (dsp->fConst57 * dsp->fRec18[2])));
			dsp->fRec19[0] = (fTemp1 - ((dsp->fConst59 * dsp->fRec19[1]) + (dsp->fConst60 * dsp->fRec19[2])));
			dsp->fRec20[0] = (fTemp1 - ((dsp->fConst62 * dsp->fRec20[1]) + (dsp->fConst63 * dsp->fRec20[2])));
			dsp->fRec21[0] = (fTemp1 - ((dsp->fConst65 * dsp->fRec21[1]) + (dsp->fConst66 * dsp->fRec21[2])));
			dsp->fRec22[0] = (fTemp1 - ((dsp->fConst68 * dsp->fRec22[1]) + (dsp->fConst69 * dsp->fRec22[2])));
			dsp->fRec23[0] = (fTemp1 - ((dsp->fConst71 * dsp->fRec23[1]) + (dsp->fConst72 * dsp->fRec23[2])));
			dsp->fRec24[0] = (fTemp1 - ((dsp->fConst74 * dsp->fRec24[1]) + (dsp->fConst75 * dsp->fRec24[2])));
			dsp->fRec25[0] = (fTemp1 - ((dsp->fConst77 * dsp->fRec25[1]) + (dsp->fConst78 * dsp->fRec25[2])));
			dsp->fRec26[0] = (fTemp1 - ((dsp->fConst80 * dsp->fRec26[1]) + (dsp->fConst81 * dsp->fRec26[2])));
			dsp->fRec27[0] = (fTemp1 - ((dsp->fConst83 * dsp->fRec27[1]) + (dsp->fConst84 * dsp->fRec27[2])));
			dsp->fRec28[0] = (fTemp1 - ((dsp->fConst86 * dsp->fRec28[1]) + (dsp->fConst87 * dsp->fRec28[2])));
			dsp->fRec29[0] = (fTemp1 - ((dsp->fConst89 * dsp->fRec29[1]) + (dsp->fConst90 * dsp->fRec29[2])));
			dsp->fRec30[0] = (fTemp1 - ((dsp->fConst92 * dsp->fRec30[1]) + (dsp->fConst93 * dsp->fRec30[2])));
			dsp->fRec31[0] = (fTemp1 - ((dsp->fConst95 * dsp->fRec31[1]) + (dsp->fConst96 * dsp->fRec31[2])));
			dsp->fRec32[0] = (fTemp1 - ((dsp->fConst98 * dsp->fRec32[1]) + (dsp->fConst99 * dsp->fRec32[2])));
			dsp->fRec33[0] = (fTemp1 - ((dsp->fConst101 * dsp->fRec33[1]) + (dsp->fConst102 * dsp->fRec33[2])));
			dsp->fRec34[0] = (fTemp1 - ((dsp->fConst104 * dsp->fRec34[1]) + (dsp->fConst105 * dsp->fRec34[2])));
			dsp->fRec35[0] = (fTemp1 - ((dsp->fConst107 * dsp->fRec35[1]) + (dsp->fConst108 * dsp->fRec35[2])));
			dsp->fRec36[0] = (fTemp1 - ((dsp->fConst110 * dsp->fRec36[1]) + (dsp->fConst111 * dsp->fRec36[2])));
			dsp->fRec37[0] = (fTemp1 - ((dsp->fConst113 * dsp->fRec37[1]) + (dsp->fConst114 * dsp->fRec37[2])));
			dsp->fRec38[0] = (fTemp1 - ((dsp->fConst116 * dsp->fRec38[1]) + (dsp->fConst117 * dsp->fRec38[2])));
			dsp->fRec39[0] = (fTemp1 - ((dsp->fConst119 * dsp->fRec39[1]) + (dsp->fConst120 * dsp->fRec39[2])));
			dsp->fRec40[0] = (fTemp1 - ((dsp->fConst122 * dsp->fRec40[1]) + (dsp->fConst123 * dsp->fRec40[2])));
			dsp->fRec41[0] = (fTemp1 - ((dsp->fConst125 * dsp->fRec41[1]) + (dsp->fConst126 * dsp->fRec41[2])));
			dsp->fRec42[0] = (fTemp1 - ((dsp->fConst128 * dsp->fRec42[1]) + (dsp->fConst129 * dsp->fRec42[2])));
			dsp->fRec43[0] = (fTemp1 - ((dsp->fConst131 * dsp->fRec43[1]) + (dsp->fConst132 * dsp->fRec43[2])));
			dsp->fRec44[0] = (fTemp1 - ((dsp->fConst134 * dsp->fRec44[1]) + (dsp->fConst135 * dsp->fRec44[2])));
			dsp->fRec45[0] = (fTemp1 - ((dsp->fConst137 * dsp->fRec45[1]) + (dsp->fConst138 * dsp->fRec45[2])));
			dsp->fRec46[0] = (fTemp1 - ((dsp->fConst140 * dsp->fRec46[1]) + (dsp->fConst141 * dsp->fRec46[2])));
			dsp->fRec47[0] = (fTemp1 - ((dsp->fConst143 * dsp->fRec47[1]) + (dsp->fConst144 * dsp->fRec47[2])));
			dsp->fRec48[0] = (fTemp1 - ((dsp->fConst146 * dsp->fRec48[1]) + (dsp->fConst147 * dsp->fRec48[2])));
			dsp->fRec49[0] = (fTemp1 - ((dsp->fConst149 * dsp->fRec49[1]) + (dsp->fConst150 * dsp->fRec49[2])));
			dsp->fRec50[0] = (fTemp1 - ((dsp->fConst152 * dsp->fRec50[1]) + (dsp->fConst153 * dsp->fRec50[2])));
			dsp->fRec51[0] = (fTemp1 - ((dsp->fConst155 * dsp->fRec51[1]) + (dsp->fConst156 * dsp->fRec51[2])));
			dsp->fRec52[0] = (fTemp1 - ((dsp->fConst158 * dsp->fRec52[1]) + (dsp->fConst159 * dsp->fRec52[2])));
			dsp->fRec53[0] = (fTemp1 - ((dsp->fConst161 * dsp->fRec53[1]) + (dsp->fConst162 * dsp->fRec53[2])));
			float fTemp2 = (0.0199999996f * (((dsp->fRec0[0] - dsp->fRec0[2]) * fSlow13) + (((dsp->fRec5[0] - dsp->fRec5[2]) * fSlow14) + (((dsp->fRec6[0] - dsp->fRec6[2]) * fSlow15) + (((dsp->fRec7[0] - dsp->fRec7[2]) * fSlow16) + (((dsp->fRec8[0] - dsp->fRec8[2]) * fSlow17) + (((dsp->fRec9[0] - dsp->fRec9[2]) * fSlow18) + (((dsp->fRec10[0] - dsp->fRec10[2]) * fSlow19) + (((dsp->fRec11[0] - dsp->fRec11[2]) * fSlow20) + (((dsp->fRec12[0] - dsp->fRec12[2]) * fSlow21) + (((dsp->fRec13[0] - dsp->fRec13[2]) * fSlow22) + (((dsp->fRec14[0] - dsp->fRec14[2]) * fSlow23) + (((dsp->fRec15[0] - dsp->fRec15[2]) * fSlow24) + (((dsp->fRec16[0] - dsp->fRec16[2]) * fSlow25) + (((dsp->fRec17[0] - dsp->fRec17[2]) * fSlow26) + (((dsp->fRec18[0] - dsp->fRec18[2]) * fSlow27) + (((dsp->fRec19[0] - dsp->fRec19[2]) * fSlow28) + (((dsp->fRec20[0] - dsp->fRec20[2]) * fSlow29) + (((dsp->fRec21[0] - dsp->fRec21[2]) * fSlow30) + (((dsp->fRec22[0] - dsp->fRec22[2]) * fSlow31) + (((dsp->fRec23[0] - dsp->fRec23[2]) * fSlow32) + (((dsp->fRec24[0] - dsp->fRec24[2]) * fSlow33) + (((dsp->fRec25[0] - dsp->fRec25[2]) * fSlow34) + (((dsp->fRec26[0] - dsp->fRec26[2]) * fSlow35) + (((dsp->fRec27[0] - dsp->fRec27[2]) * fSlow36) + (((dsp->fRec28[0] - dsp->fRec28[2]) * fSlow37) + (((dsp->fRec29[0] - dsp->fRec29[2]) * fSlow38) + (((dsp->fRec30[0] - dsp->fRec30[2]) * fSlow39) + (((dsp->fRec31[0] - dsp->fRec31[2]) * fSlow40) + (((dsp->fRec32[0] - dsp->fRec32[2]) * fSlow41) + (((dsp->fRec33[0] - dsp->fRec33[2]) * fSlow42) + (((dsp->fRec34[0] - dsp->fRec34[2]) * fSlow43) + (((dsp->fRec35[0] - dsp->fRec35[2]) * fSlow44) + (((dsp->fRec36[0] - dsp->fRec36[2]) * fSlow45) + (((dsp->fRec37[0] - dsp->fRec37[2]) * fSlow46) + (((dsp->fRec38[0] - dsp->fRec38[2]) * fSlow47) + (((dsp->fRec39[0] - dsp->fRec39[2]) * fSlow48) + (((dsp->fRec40[0] - dsp->fRec40[2]) * fSlow49) + (((dsp->fRec41[0] - dsp->fRec41[2]) * fSlow50) + (((dsp->fRec42[0] - dsp->fRec42[2]) * fSlow51) + ((((((dsp->fRec43[0] - dsp->fRec43[2]) * fSlow52) + (((dsp->fRec44[0] - dsp->fRec44[2]) * fSlow53) + (((dsp->fRec45[0] - dsp->fRec45[2]) * fSlow54) + (((dsp->fRec46[0] - dsp->fRec46[2]) * fSlow55) + (((((dsp->fRec47[0] - dsp->fRec47[2]) * fSlow56) + ((dsp->fRec48[0] - dsp->fRec48[2]) * fSlow57)) + ((dsp->fRec49[0] - dsp->fRec49[2]) * fSlow58)) + ((dsp->fRec50[0] - dsp->fRec50[2]) * fSlow59)))))) + ((dsp->fRec51[0] - dsp->fRec51[2]) * fSlow60)) + ((dsp->fRec52[0] - dsp->fRec52[2]) * fSlow61)) + ((dsp->fRec53[0] - dsp->fRec53[2]) * fSlow62))))))))))))))))))))))))))))))))))))))))));
                        fprintf(stderr, "hello index %i\n", i);
			output0[i] = (FAUSTFLOAT)fTemp2;
			output1[i] = (FAUSTFLOAT)fTemp2;
			dsp->iRec3[1] = dsp->iRec3[0];
			dsp->fRec2[2] = dsp->fRec2[1];
			dsp->fRec2[1] = dsp->fRec2[0];
			dsp->fRec1[2] = dsp->fRec1[1];
			dsp->fRec1[1] = dsp->fRec1[0];
			dsp->fVec0[1] = dsp->fVec0[0];
			dsp->fVec1[1] = dsp->fVec1[0];
			dsp->fRec4[1] = dsp->fRec4[0];
			dsp->fRec0[2] = dsp->fRec0[1];
			dsp->fRec0[1] = dsp->fRec0[0];
			dsp->fRec5[2] = dsp->fRec5[1];
			dsp->fRec5[1] = dsp->fRec5[0];
			dsp->fRec6[2] = dsp->fRec6[1];
			dsp->fRec6[1] = dsp->fRec6[0];
			dsp->fRec7[2] = dsp->fRec7[1];
			dsp->fRec7[1] = dsp->fRec7[0];
			dsp->fRec8[2] = dsp->fRec8[1];
			dsp->fRec8[1] = dsp->fRec8[0];
			dsp->fRec9[2] = dsp->fRec9[1];
			dsp->fRec9[1] = dsp->fRec9[0];
			dsp->fRec10[2] = dsp->fRec10[1];
			dsp->fRec10[1] = dsp->fRec10[0];
			dsp->fRec11[2] = dsp->fRec11[1];
			dsp->fRec11[1] = dsp->fRec11[0];
			dsp->fRec12[2] = dsp->fRec12[1];
			dsp->fRec12[1] = dsp->fRec12[0];
			dsp->fRec13[2] = dsp->fRec13[1];
			dsp->fRec13[1] = dsp->fRec13[0];
			dsp->fRec14[2] = dsp->fRec14[1];
			dsp->fRec14[1] = dsp->fRec14[0];
			dsp->fRec15[2] = dsp->fRec15[1];
			dsp->fRec15[1] = dsp->fRec15[0];
			dsp->fRec16[2] = dsp->fRec16[1];
			dsp->fRec16[1] = dsp->fRec16[0];
			dsp->fRec17[2] = dsp->fRec17[1];
			dsp->fRec17[1] = dsp->fRec17[0];
			dsp->fRec18[2] = dsp->fRec18[1];
			dsp->fRec18[1] = dsp->fRec18[0];
			dsp->fRec19[2] = dsp->fRec19[1];
			dsp->fRec19[1] = dsp->fRec19[0];
			dsp->fRec20[2] = dsp->fRec20[1];
			dsp->fRec20[1] = dsp->fRec20[0];
			dsp->fRec21[2] = dsp->fRec21[1];
			dsp->fRec21[1] = dsp->fRec21[0];
			dsp->fRec22[2] = dsp->fRec22[1];
			dsp->fRec22[1] = dsp->fRec22[0];
			dsp->fRec23[2] = dsp->fRec23[1];
			dsp->fRec23[1] = dsp->fRec23[0];
			dsp->fRec24[2] = dsp->fRec24[1];
			dsp->fRec24[1] = dsp->fRec24[0];
			dsp->fRec25[2] = dsp->fRec25[1];
			dsp->fRec25[1] = dsp->fRec25[0];
			dsp->fRec26[2] = dsp->fRec26[1];
			dsp->fRec26[1] = dsp->fRec26[0];
			dsp->fRec27[2] = dsp->fRec27[1];
			dsp->fRec27[1] = dsp->fRec27[0];
			dsp->fRec28[2] = dsp->fRec28[1];
			dsp->fRec28[1] = dsp->fRec28[0];
			dsp->fRec29[2] = dsp->fRec29[1];
			dsp->fRec29[1] = dsp->fRec29[0];
			dsp->fRec30[2] = dsp->fRec30[1];
			dsp->fRec30[1] = dsp->fRec30[0];
			dsp->fRec31[2] = dsp->fRec31[1];
			dsp->fRec31[1] = dsp->fRec31[0];
			dsp->fRec32[2] = dsp->fRec32[1];
			dsp->fRec32[1] = dsp->fRec32[0];
			dsp->fRec33[2] = dsp->fRec33[1];
			dsp->fRec33[1] = dsp->fRec33[0];
			dsp->fRec34[2] = dsp->fRec34[1];
			dsp->fRec34[1] = dsp->fRec34[0];
			dsp->fRec35[2] = dsp->fRec35[1];
			dsp->fRec35[1] = dsp->fRec35[0];
			dsp->fRec36[2] = dsp->fRec36[1];
			dsp->fRec36[1] = dsp->fRec36[0];
			dsp->fRec37[2] = dsp->fRec37[1];
			dsp->fRec37[1] = dsp->fRec37[0];
			dsp->fRec38[2] = dsp->fRec38[1];
			dsp->fRec38[1] = dsp->fRec38[0];
			dsp->fRec39[2] = dsp->fRec39[1];
			dsp->fRec39[1] = dsp->fRec39[0];
			dsp->fRec40[2] = dsp->fRec40[1];
			dsp->fRec40[1] = dsp->fRec40[0];
			dsp->fRec41[2] = dsp->fRec41[1];
			dsp->fRec41[1] = dsp->fRec41[0];
			dsp->fRec42[2] = dsp->fRec42[1];
			dsp->fRec42[1] = dsp->fRec42[0];
			dsp->fRec43[2] = dsp->fRec43[1];
			dsp->fRec43[1] = dsp->fRec43[0];
			dsp->fRec44[2] = dsp->fRec44[1];
			dsp->fRec44[1] = dsp->fRec44[0];
			dsp->fRec45[2] = dsp->fRec45[1];
			dsp->fRec45[1] = dsp->fRec45[0];
			dsp->fRec46[2] = dsp->fRec46[1];
			dsp->fRec46[1] = dsp->fRec46[0];
			dsp->fRec47[2] = dsp->fRec47[1];
			dsp->fRec47[1] = dsp->fRec47[0];
			dsp->fRec48[2] = dsp->fRec48[1];
			dsp->fRec48[1] = dsp->fRec48[0];
			dsp->fRec49[2] = dsp->fRec49[1];
			dsp->fRec49[1] = dsp->fRec49[0];
			dsp->fRec50[2] = dsp->fRec50[1];
			dsp->fRec50[1] = dsp->fRec50[0];
			dsp->fRec51[2] = dsp->fRec51[1];
			dsp->fRec51[1] = dsp->fRec51[0];
			dsp->fRec52[2] = dsp->fRec52[1];
			dsp->fRec52[1] = dsp->fRec52[0];
			dsp->fRec53[2] = dsp->fRec53[1];
			dsp->fRec53[1] = dsp->fRec53[0];
			
		}
		
	}
    */
	
}

#ifdef __cplusplus
}
#endif


#endif
