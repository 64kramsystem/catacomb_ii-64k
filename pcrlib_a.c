/* The Catacomb Source Code
 * Copyright (C) 1993-2014 Flat Rock Software
 * Copyright (C) 2014 Flat Braden "Blzut3" Obrzut
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#include <time.h>
#include <SDL_timer.h>
#include <SDL_thread.h>

#include "pcrlib.h"

static word SPKactive = 0; //set non zero when started

char *SoundData; //two word pointer to SPKR file, PARA aligned
soundtype soundmode = 1; //0=nosound, 1=SPKR, 2= adlib...
static int OldInt8; //StartupSPK saves here, Shutdown restores
static byte ExtraInts; //number of PlaySPKR's to a regular int 8
static byte Intcount; //counter for extraints, call OldInt8 at 0
static word sndspeed; //timer count speed

static word SndPtr; //Pointer to frequency of current sound
static byte SndPriority; //current sound's priority

static word pausesndptr;
static byte pausepriority;
static byte pauseextraints;
static byte pauseintcount;
static word pausespeed;

int _dontplay = 0; //set to 1 to avoid all interrupt and timer stuff

int xormask = 0; //to invert characters
int _yshift = 0; //to shift char lines

//========
//
// YLOOKUP has the offsets from screen mem of each screen line (multiples of 8)
//
//========

word CGAylookup[200] = {
	   0,8192,  80,8272, 160,8352, 240,8432, 320,8512, 400,8592, 480,8672,
	 560,8752, 640,8832, 720,8912, 800,8992, 880,9072, 960,9152,1040,9232,
	1120,9312,1200,9392,1280,9472,1360,9552,1440,9632,1520,9712,1600,9792,
	1680,9872,1760,9952,1840,10032,1920,10112,2000,10192,2080,10272,2160,10352,
	2240,10432,2320,10512,2400,10592,2480,10672,2560,10752,2640,10832,2720,10912,
	2800,10992,2880,11072,2960,11152,3040,11232,3120,11312,3200,11392,3280,11472,
	3360,11552,3440,11632,3520,11712,3600,11792,3680,11872,3760,11952,3840,12032,
	3920,12112,4000,12192,4080,12272,4160,12352,4240,12432,4320,12512,4400,12592,
	4480,12672,4560,12752,4640,12832,4720,12912,4800,12992,4880,13072,4960,13152,
	5040,13232,5120,13312,5200,13392,5280,13472,5360,13552,5440,13632,5520,13712,
	5600,13792,5680,13872,5760,13952,5840,14032,5920,14112,6000,14192,6080,14272,
	6160,14352,6240,14432,6320,14512,6400,14592,6480,14672,6560,14752,6640,14832,
	6720,14912,6800,14992,6880,15072,6960,15152,7040,15232,7120,15312,7200,15392,
	7280,15472,7360,15552,7440,15632,7520,15712,7600,15792,7680,15872,7760,15952,
	7840,16032,7920,16112
};

word EGAylookup[256] = {
	   0,  40,  80, 120, 160, 200, 240, 280, 320, 360, 400, 440, 480, 520,
	 560, 600, 640, 680, 720, 760, 800, 840, 880, 920, 960,1000,1040,1080,
	1120,1160,1200,1240,1280,1320,1360,1400,1440,1480,1520,1560,1600,1640,
	1680,1720,1760,1800,1840,1880,1920,1960,2000,2040,2080,2120,2160,2200,
	2240,2280,2320,2360,2400,2440,2480,2520,2560,2600,2640,2680,2720,2760,
	2800,2840,2880,2920,2960,3000,3040,3080,3120,3160,3200,3240,3280,3320,
	3360,3400,3440,3480,3520,3560,3600,3640,3680,3720,3760,3800,3840,3880,
	3920,3960,4000,4040,4080,4120,4160,4200,4240,4280,4320,4360,4400,4440,
	4480,4520,4560,4600,4640,4680,4720,4760,4800,4840,4880,4920,4960,5000,
	5040,5080,5120,5160,5200,5240,5280,5320,5360,5400,5440,5480,5520,5560,
	5600,5640,5680,5720,5760,5800,5840,5880,5920,5960,6000,6040,6080,6120,
	6160,6200,6240,6280,6320,6360,6400,6440,6480,6520,6560,6600,6640,6680,
	6720,6760,6800,6840,6880,6920,6960,7000,7040,7080,7120,7160,7200,7240,
	7280,7320,7360,7400,7440,7480,7520,7560,7600,7640,7680,7720,7760,7800,
	7840,7880,7920,7960,8000,8040,8080,8120,8160,8200,8240,8280,8320,8360,
	8400,8440,8480,8520,8560,8600,8640,8680,8720,8760,8800,8840,8880,8920,
	8960,9000,9040,9080,9120,9160,9200,9240,9280,9320,9360,9400,9440,9480,
	9520,9560,9600,9640,9680,9720,9760,9800,9840,9880,9920,9960,10000,10040,
	10080,10120,10160,10200
};

word VGAylookup[200] = {
	   0, 320, 640, 960,1280,1600,1920,2240,2560,2880,3200,3520,3840,4160,
	4480,4800,5120,5440,5760,6080,6400,6720,7040,7360,7680,8000,8320,8640,
	8960,9280,9600,9920,10240,10560,10880,11200,11520,11840,12160,12480,12800,13120,
	13440,13760,14080,14400,14720,15040,15360,15680,16000,16320,16640,16960,17280,17600,
	17920,18240,18560,18880,19200,19520,19840,20160,20480,20800,21120,21440,21760,22080,
	22400,22720,23040,23360,23680,24000,24320,24640,24960,25280,25600,25920,26240,26560,
	26880,27200,27520,27840,28160,28480,28800,29120,29440,29760,30080,30400,30720,31040,
	31360,31680,32000,32320,32640,-32576,-32256,-31936,-31616,-31296,-30976,-30656,-30336,-30016,
	-29696,-29376,-29056,-28736,-28416,-28096,-27776,-27456,-27136,-26816,-26496,-26176,-25856,-25536,
	-25216,-24896,-24576,-24256,-23936,-23616,-23296,-22976,-22656,-22336,-22016,-21696,-21376,-21056,
	-20736,-20416,-20096,-19776,-19456,-19136,-18816,-18496,-18176,-17856,-17536,-17216,-16896,-16576,
	-16256,-15936,-15616,-15296,-14976,-14656,-14336,-14016,-13696,-13376,-13056,-12736,-12416,-12096,
	-11776,-11456,-11136,-10816,-10496,-10176,-9856,-9536,-9216,-8896,-8576,-8256,-7936,-7616,
	-7296,-6976,-6656,-6336,-6016,-5696,-5376,-5056,-4736,-4416,-4096,-3776,-3456,-3136,
	-2816,-2496,-2176,-1856
};

void StartupSound()
{
	printf("STUB: %s\n", __FUNCTION__);

	soundmode = 0;
	_dontplay = 1;
}

void ShutdownSound()
{
	if(_dontplay)
		return;

	FIXME
}
void PlaySound(int sound)
{
	if(_dontplay)
		return;

	FIXME
}
void StopSound()
{
	if(_dontplay)
		return;

	FIXME
}
void PauseSound()
{
	if(_dontplay)
		return;

	FIXME
}
void ContinueSound()
{
	if(_dontplay)
		return;

	FIXME
}
void WaitEndSound()
{
	if(_dontplay)
		return;

	FIXME
}

static void UpdateSPKR() FIXME

static word rndindex;
static byte rndtable[256] = {
	  0,   8, 109, 220, 222, 241, 149, 107,  75, 248, 254, 140,  16,  66,
	 74,  21, 211,  47,  80, 242, 154,  27, 205, 128, 161,  89,  77,  36,
	 95, 110,  85,  48, 212, 140, 211, 249,  22,  79, 200,  50,  28, 188,
	 52, 140, 202, 120,  68, 145,  62,  70, 184, 190,  91, 197, 152, 224,
	149, 104,  25, 178, 252, 182, 202, 182, 141, 197,   4,  81, 181, 242,
	145,  42,  39, 227, 156, 198, 225, 193, 219,  93, 122, 175, 249,   0,
	175, 143,  70, 239,  46, 246, 163,  53, 163, 109, 168, 135,   2, 235,
	 25,  92,  20, 145, 138,  77,  69, 166,  78, 176, 173, 212, 166, 113,
	 94, 161,  41,  50, 239,  49, 111, 164,  70,  60,   2,  37, 171,  75,
	136, 156,  11,  56,  42, 146, 138, 229,  73, 146,  77,  61,  98, 196,
	135, 106,  63, 197, 195,  86,  96, 203, 113, 101, 170, 247, 181, 113,
	 80, 250, 108,   7, 255, 237, 129, 226,  79, 107, 112, 166, 103, 241,
	 24, 223, 239, 120, 198,  58,  60,  82, 128,   3, 184,  66, 143, 224,
	145, 224,  81, 206, 163,  45,  63,  90, 168, 114,  59,  33, 159,  95,
	 28, 139, 123,  98, 125, 196,  15,  70, 194, 253,  54,  14, 109, 226,
	 71,  17, 161,  93, 186,  87, 244, 138,  20,  52, 123, 251,  26,  36,
	 17,  46,  52, 231, 232,  76,  31, 221,  84,  37, 216, 165, 212, 106,
	197, 242,  98,  43,  39, 175, 254, 145, 190,  84, 118, 222, 187, 136,
	120, 163, 236, 249
};
//
// Random # Generator vars
//
static word indexi; // Rnd#Generator
static word indexj;
static word LastRnd;
static word RndArray[17];

static word baseRndArray[] = { 1,1,2,3,5,8,13,21,54,75,129,204,323,527,850,1377,2227 };

//=================================================
//
// Init RND generator
// if randomize is false, the counter is set to 0
//
// 11-Sep-90	LR	FIX initialization to use TIME!
//=================================================

void initrnd(boolean randomize) FIXME

//=================================================
//
// Return a random # between 0-?
// Exit : AX = 0-max value
//
// 11-Sep-90 LR -modify to save registers!
//=================================================

void rnd(word maxval) FIXME

//=================================================
//
// Init table based RND generator
// if randomize is false, the counter is set to 0
//
//=================================================

void initrndt(boolean randomize)
{
	rndindex = randomize ? time(NULL)&0xFF : 0;
}

//=================================================
//
// Return a random # between 0-255
// Exit : AX = value
//
//=================================================

int rndt()
{
	rndindex = (rndindex+1)&0xFF;
	return rndtable[rndindex];
}

enum { VBL_TIME = 1000/70 };
SDL_sem *vblsem;
SDL_TimerID vbltimer;
static Uint32 VBLCallback(Uint32 interval, void *usr)
{
	SDL_SemPost(vblsem);
	return VBL_TIME;
}

static void ShutdownEmulatedVBL()
{
	SDL_RemoveTimer(vbltimer);
	SDL_DestroySemaphore(vblsem);
}

void SetupEmulatedVBL()
{
	vblsem = SDL_CreateSemaphore(0);

	vbltimer = SDL_AddTimer(VBL_TIME, VBLCallback, NULL);
	atexit(ShutdownEmulatedVBL);
}

void WaitVBL()
{
	do
	{
		SDL_SemWait(vblsem);
	}
	while(SDL_SemValue(vblsem));
}

void drawchar(int x, int y, int charnum)
{
	byte *vbuf = screenseg + (y<<3)*screenpitch + (x<<3);
	byte *src;
	unsigned i;

	switch(grmode)
	{
	case EGAgr:
		src = (byte*)charptr + charnum*8;
		for (i = 0;i < 8;++i, ++src)
		{
			const byte chan[4] = {
				src[egaplaneofs[0]],
				src[egaplaneofs[1]],
				src[egaplaneofs[2]],
				src[egaplaneofs[3]]
			};

			*vbuf++ = EGA(chan,7);
			*vbuf++ = EGA(chan,6);
			*vbuf++ = EGA(chan,5);
			*vbuf++ = EGA(chan,4);
			*vbuf++ = EGA(chan,3);
			*vbuf++ = EGA(chan,2);
			*vbuf++ = EGA(chan,1);
			*vbuf = EGA(chan,0);

			vbuf += screenpitch-7;
		}
		break;
	case CGAgr:
		src = (byte*)charptr + charnum*16;
		for (i = 0;i < 8;++i, src += 2)
		{
			*vbuf++ = (src[0]>>6)&3;
			*vbuf++ = (src[0]>>4)&3;
			*vbuf++ = (src[0]>>2)&3;
			*vbuf++ = (src[0]>>0)&3;
			*vbuf++ = (src[1]>>6)&3;
			*vbuf++ = (src[1]>>4)&3;
			*vbuf++ = (src[1]>>2)&3;
			*vbuf = (src[1]>>0)&3;

			vbuf += screenpitch-7;
		}
		break;
	case VGAgr:
		src = (byte*)charptr + charnum*64;
		// [BL] More or less guessing here since we don't have VGA files to
		// test against.
		for (i = 0;i < 8;++i, src += 8, vbuf += screenpitch-7)
			*(qword*)vbuf = *(qword*)src;
		break;
	}
}

void drawpic(int x, int y, int picnum)
{
	byte *vbuf = screenseg + y*screenpitch + x;
	byte *src;
	unsigned i;

	unsigned picwidth = pictable[picnum].width;
	unsigned picheight = pictable[picnum].height;
	src = (byte*)picptr + pictable[picnum].shapeptr;

	switch(grmode)
	{
		case EGAgr:
			do
			{
				i = picwidth;
				do
				{
					const byte chan[4] = {
						src[egaplaneofs[0]],
						src[egaplaneofs[1]],
						src[egaplaneofs[2]],
						src[egaplaneofs[3]]
					};
					++src;

					*vbuf++ = EGA(chan,7);
					*vbuf++ = EGA(chan,6);
					*vbuf++ = EGA(chan,5);
					*vbuf++ = EGA(chan,4);
					*vbuf++ = EGA(chan,3);
					*vbuf++ = EGA(chan,2);
					*vbuf++ = EGA(chan,1);
					*vbuf++ = EGA(chan,0);
				}
				while(--i);
				vbuf += screenpitch-(picwidth<<3);
			}
			while(--picheight);
			break;
		case CGAgr:
			do
			{
				i = picwidth;
				do
				{
					*vbuf++ = (src[0]>>6)&3;
					*vbuf++ = (src[0]>>4)&3;
					*vbuf++ = (src[0]>>2)&3;
					*vbuf++ = (src[0]>>0)&3;
					++src;
				}
				while(--i);
				vbuf += screenpitch-(picwidth<<2);
			}
			while(--picheight);
			break;
		case VGAgr:
			// [BL] My best guess.
			do
			{
				i = picwidth;
				do
				{
					*vbuf++ = *src++;
				}
				while(--i);
				vbuf += screenpitch-picwidth;
			}
			while(--picheight);
			break;
	}
}
