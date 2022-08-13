/* The Catacomb Source Code
 * Copyright (C) 1993-2014 Flat Rock Software
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

/*
** catacomb II -- the c translation...
*/

#ifdef _MSC_VER
#include <io.h>
#else
#include <unistd.h>
#endif
#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <sys/stat.h>

#include "catdefs.h"
#include "catacomb.h"
#include "pcrlib.h"
#include "ngrabca2.h"
#include "sounds.h"

/*=================*/
/*		   */
/* typed constants */
/*     		   */
/*=================*/
  char altmeters[14][14] = {
 {127,127,127,127,127,127,127,127,127,127,127,127,127},
 {23,127,127,127,127,127,127,127,127,127,127,127,127},
 {23,25,127,127,127,127,127,127,127,127,127,127,127},
 {23,24,25,127,127,127,127,127,127,127,127,127,127},
 {23,24,24,25,127,127,127,127,127,127,127,127,127},
 {23,24,24,24,25,127,127,127,127,127,127,127,127},
 {23,24,24,24,24,25,127,127,127,127,127,127,127},
 {23,24,24,24,24,24,25,127,127,127,127,127,127},
 {23,24,24,24,24,24,24,25,127,127,127,127,127},
 {23,24,24,24,24,24,24,24,25,127,127,127,127},
 {23,24,24,24,24,24,24,24,24,25,127,127,127},
 {23,24,24,24,24,24,24,24,24,24,25,127,127},
 {23,24,24,24,24,24,24,24,24,24,24,25,127},
 {23,24,24,24,24,24,24,24,24,24,24,24,25} };

  char meters[14][14] = {
 {127,127,127,127,127,127,127,127,127,127,127,127,127},
 {26,127,127,127,127,127,127,127,127,127,127,127,127},
 {26,28,127,127,127,127,127,127,127,127,127,127,127},
 {26,27,28,127,127,127,127,127,127,127,127,127,127},
 {26,27,27,28,127,127,127,127,127,127,127,127,127},
 {26,27,27,27,28,127,127,127,127,127,127,127,127},
 {26,27,27,27,27,28,127,127,127,127,127,127,127},
 {26,27,27,27,27,27,28,127,127,127,127,127,127},
 {26,27,27,27,27,27,27,28,127,127,127,127,127},
 {26,27,27,27,27,27,27,27,28,127,127,127,127},
 {26,27,27,27,27,27,27,27,27,28,127,127,127},
 {26,27,27,27,27,27,27,27,27,27,28,127,127},
 {26,27,27,27,27,27,27,27,27,27,27,28,127},
 {26,27,27,27,27,27,27,27,27,27,27,27,28} };

 dirtype opposite[9] =
    {south,west,north,east,southwest,northwest,northeast,southeast,nodir};


/*==================*/
/*		    */
/* global variables */
/*		    */
/*==================*/
  exittype gamexit; /*determines what to do after playloop*/

  int oldtiles [NUMTILES];		/*tile displayed last refresh*/
  int background[87][86];		/*base map*/
  int view[87][86];			/*base map with objects drawn in*/
  int originx, originy;			/*current world location of ul corn*/
  byte priority [maxpics+1];		/*tile draw overlap priorities*/

  sword items[6],saveitems[6];
  int shotpower;			/*0-13 characters in power meter*/
  int side;	                        /*which side shots come from*/
  int boltsleft;			/*number of shots left in a bolt*/

  activeobj o[maxobj+1],saveo[1];	/*everything that moves is here*/
  objtype obj , altobj;			/*total info about objecton and alt*/
  int altnum;				/*o[#] of altobj*/
  int numobj,objecton;			/*number of objects in o now*/

  objdeftype objdef [lastclass];

  boolean playdone, leveldone;

  boolean tempb;
  char *tempp;

  int chkx,chky,chkspot;		/*spot being checked by walk*/

  word frameon;
  char *grmem;
  classtype clvar;

  int VGAPAL;				// just to make pcrlib happy

  boolean exitdemo,resetgame;
  statetype gamestate;

  ControlStruct ctrl;

  char *pics, *picsexact;

  unsigned EGADATASTART;

  sdword savescore;

//NOLAN ADDED
	boolean GODMODE = false;


/****************************************************************************/

//////////////////////////////////
//
// function prototypes
//
//////////////////////////////////

extern void drawobj (void);
extern void eraseobj (void);
extern void doall (void);
extern void cgarefresh (void);
extern void egarefresh (void);
void dofkeys (void);



/*==============================*/
/*			        */
/* xxxrefresh                   */
/* refresh the changed areas of */
/* the tiles map in the various */
/* graphics modes.              */
/*			        */
/*==============================*/

char demowin [5][16] = {
  {14,15,15,15,15,15,15,15,15,15,15,15,15,15,15,16},
  {17,' ','-','-','-',' ','D','E','M','O',' ','-','-','-',' ',18},
  {17,'S','P','A','C','E',' ','T','O',' ','S','T','A','R','T',18},
  {17,'F','1',' ','T','O',' ','G','E','T',' ','H','E','L','P',18},
  {19,20,20,20,20,20,20,20,20,20,20,20,20,20,20,21} };

void refresh (void)
{
  int x,y,basex,basey;
  word underwin [5][16];

  basex=originx+4;
  basey=originy+17;
  if (indemo)
  {
    for (y=0; y<=4; y++)
      for (x=0; x<=15; x++)
	{
	  underwin[y][x]=view[y+basey][x+basex];
	  view[y+basey][x+basex]=demowin[y][x];
	};
  }

  WaitVBL ();
  if (grmode==CGAgr)
    cgarefresh();
  else
    egarefresh();

  if (indemo)
  {
    for (y=0; y<=4; y++)
      for (x=0; x<=15; x++)
	view[y+basey][x+basex]=underwin[y][x];
  }

  WaitVBL ();
}


void simplerefresh(void)
{
  WaitVBL ();
  if (grmode==CGAgr)
    cgarefresh();
  else
    egarefresh();

}

/*
===================
=
= loadgrfiles
=
= Loads the tiles and sprites, and sets up the pointers and tables
=
===================
*/

void loadgrfiles ()
{
  if (picsexact != NULL)
    free (picsexact);
  if (grmode==CGAgr)
  {
    picsexact = pics = (char *)bloadin((char*)"CGACHARS.CA2");
    installgrfile ((char*)"CGAPICS.CA2",0,0);
  }
  else
  {
    picsexact = pics = (char *)bloadin((char*)"EGACHARS.CA2");
    installgrfile ((char*)"EGAPICS.CA2",0,0);
  }
}


/*======================================*/
/*				        */
/* restore                              */
/* redraws every tile on the tiled area */
/* by setting oldtiles to -1.  used to  */
/* erase any temporary windows.         */
/*				        */
/*======================================*/

void clearold (void)
{
  memset (&oldtiles,0xff,sizeof(oldtiles)); /*clear all oldtiles*/
};


void restore (void)
{
  clearold ();
  simplerefresh ();
};



/*      */
/* help */
/*      */
boolean wantmore (void)
{
  sx=2;
  sy=20;
  print ("(space for more/esc)");
  sx=12;
  sy=21;
  ch = get ();
  if (ch==27)
    return false;

  return true;
};


/*	   */
/* charpic */
/*	   */
static void charpic(int x,int y, classtype c, dirtype dir, int stage)
{
  int xx,yy,size,tilenum;

  size=objdef[c].size;
  tilenum=objdef[c].firstchar+size*size
    * ((dir & objdef[c].dirmask)*objdef[c].stages+stage);

  for (yy=y;yy<=y+size-1;yy++)
    for (xx=x;xx<=x+size-1;xx++)
	drawchartile (xx,yy,tilenum++);
};

void help (void)
{
  int x,y;

  centerwindow (20,20);
  print ("  C A T A C O M B   \n");
  print ("   - - - - - - -    \n");
  print (" by John Carmack    \n");
  print ("                    \n");
  print ("\n");
  print ("f1 = help           \n");
  print ("f2 = control panel  \n");
  print ("f3 = game reset     \n");
  print ("f4 = save game      \n");
  print ("f5 = load saved game\n");
  print ("f9 = pause          \n");
  print ("f10 / ESC = quit    \n");
  print ("\n");
  print ("hit fire at the demo\n");
  print ("to begin playing.   \n");
  if (!wantmore())
    return;

  centerwindow (20,20);
  print ("\nKeyboard controls:  \n\n");
  print ("move    : arrows    \n");
  print ("button1 : ctrl      \n");
  print ("button2 : alt       \n");
  print ("\nTo switch to mouse \n");
  print ("or joystick control,\n");
  print ("hit f2             \n");

  if (!wantmore())
    return;

  centerwindow (20,20);
  print ("Button 1 / ctrl key:\n");
  print ("Builds shot power.  \n");
  print ("If the shot power   \n");
  print ("meter is full when  \n");
  print ("the button is       \n");
  print ("released, a super   \n");
  print ("shot will be        \n");
  print ("launched.           \n");
  print ("\n");

  for (y=11; y<=18; y++)
    for (x=3; x<=20; x++)
      drawchartile (x,y,128);

  charpic (4,14,player,east,2);
  charpic (19,15,shot,east,1);
  charpic (17,14,shot,east,0);
  charpic (15,15,shot,east,1);
  charpic (8,14,bigshot,east,0);

  if (!wantmore())
    return;

  centerwindow (20,20);
  print ("Button 2 / alt key:\n");
  print ("Allows you to move  \n");
  print ("without changing the\n");
  print ("direction you are   \n");
  print ("facing.  Good for   \n");
  print ("searching walls and \n");
  print ("fighting retreats.  \n");

  for (y=11; y<=18; y++)
    for (x=3; x<=20; x++)
      if (y==15)
	drawchartile (x,y,129);
      else if (y==16)
	drawchartile (x,y,131);
      else
	drawchartile (x,y,128);
  charpic (6,13,player,south,2);
  sx=6;
  sy=15;
  print ("\35\35\36\36\37\37");

  if (!wantmore())
    return;

  centerwindow (20,20);
  print ("\"P\" or \"space\" will \n");
  print ("take a healing      \n");
  print ("potion if you have  \n");
  print ("one.  This restores \n");
  print ("the body meter to   \n");
  print ("full strength.  Keep\n");
  print ("a sharp eye on the  \n");
  print ("meter, because when \n");
  print ("it runs out, you are\n");
  print ("dead!               \n\n");
  print ("\"B\" will cast a bolt\n");
  print ("spell if you have   \n");
  print ("any.  You can mow   \n");
  print ("down a lot of       \n");
  print ("monsters with a bit \n");
  print ("of skill.           \n");

  if (!wantmore())
    return;

  centerwindow (20,20);
  print ("\"N\" or \"enter\" will \n");
  print ("cast a nuke spell.  \n");
  print ("This usually wipes  \n");
  print ("out all the monsters\n");
  print ("near you.  Consider \n");
  print ("it a panic button   \n");
  print ("when you are being  \n");
  print ("mobbed by monsters! \n\n");

  printchartile ("               \200\200\200\n");
  printchartile ("POTIONS:       \200\242\200\n");
  printchartile ("               \200\200\200\n");
  printchartile ("SCROLLS:       \200\243\200\n");
  printchartile (" (BOLTS/NUKES) \200\200\200\n");
  printchartile ("TREASURE:      \200\247\200\n");
  printchartile (" (POINTS)      \200\200\200\n");
  printchartile ("               \200\200\200\n");

  wantmore();

};

/*       */
/* reset */
/*       */
void reset(void)
{
  centerwindow (18,1);
  print ("reset game (y/n)?");
  ch= get ();
  if (ch=='y')
    {
      gamexit=killed;
      playdone=true;
    };
};



/*=========================================================================*/


/*==============================*/
/*			        */
/* loadlevel / savelevel        */
/* loads map level into memory, */
/* and sets everything up.      */
/*			        */
/*==============================*/

void loadlevel(void)
{
  int i;

  classtype tokens[256-230]  =
    {player,teleporter,goblin,skeleton,ogre,gargoyle,dragon,turbogre,
     guns,gune,secretgate,nothing,nothing,nothing,nothing,nothing,
     nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,
     nothing,nothing};

  char filename[64],st[64];
  int x,y,xx,yy;
  byte btile;
  char sm[4096],rle[4096];

  strcpy (filename,"LEVEL");
  itoa (level,st,10);
  strcat (filename,st);
  strcat (filename,".CA2");

  LoadFile (filename,rle);
  RLEExpand(&rle[4],sm,4096);

  numobj=0;
  o[0].x=13;          /*just defaults if no player token is found*/
  o[0].y=13;
  o[0].stage=0;
  o[0].delay=0;
  o[0].dir=east;
  o[0].oldx=0;
  o[0].oldy=0;
  o[0].oldtile=-1;


  for (yy=0; yy<64; yy++)
    for (xx=0; xx<64; xx++)
      {
	btile=sm[yy*64+xx];
	if (btile<230)
	  background[yy+topoff][xx+leftoff]=btile;
	else
	  {

/*hit a monster token*/
	    background[yy+topoff][xx+leftoff]=blankfloor;
	    if (tokens[btile-230]==player)

/*the player token determines where you start in level*/

	      {
		o[0].x=xx+topoff;
		o[0].y=yy+leftoff;
	      }
            else

/*monster tokens add to the object list*/

	      {
		numobj++;
		o[numobj].active=false;
		o[numobj].class=tokens[btile-230];
		o[numobj].x=xx+leftoff;
		o[numobj].y=yy+topoff;
		o[numobj].stage=0;
		o[numobj].delay=0;
		o[numobj].dir=(dirtype)(rndt()/64);  /*random 0-3*/
		o[numobj].hp=objdef[o[numobj].class].hitpoints;
		// [BL] These were just x/y which are uninitialized
		o[numobj].oldx=o[numobj].x;
		o[numobj].oldy=o[numobj].y;
		o[numobj].oldtile=-1;
	      };

	    };

	  };



  originx = o[0].x-11;
  originy = o[0].y-11;

  shotpower=0;
  for (y=topoff-1; y<65+topoff; y++)
    for (x=leftoff-1; x<64+leftoff; x++)
      view[y][x]=background[y][x];

  sx=33;                  /*print the new level number on the right window*/
  sy=1;
  printint (level);
  print (" ");          /*in case it went from double to single digit*/
  restore();



  for (i=0;i<6;i++)
    saveitems[i] = items[i];
  savescore = score;
  saveo[0] = o[0];
};


/*==========================================================================*/


/*
==============
=
= drawside
=
==============
*/
void drawside ()
{
  int i;

  for (sx=0;sx<40;sx++)
    drawchar (sx,24,0);

  for (sy=0;sy<24;sy++)
    drawchar (39,sy,0);

  drawwindow (24,0,38,23);  /*draw the right side window*/
  print ("  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n");
  print (" shot power\n\n\n    body\n\n\n");
  printhighscore();
  printbody();
  printshotpower();
  printscore();
  sx=33;                  /*print the new level number on the right window*/
  sy=1;
  printint (level);

  drawpic (25*8,17*8,SIDEPIC);

  for (i=1; i<=items[1] && i<11; i++)
    drawchar (26+i,7,31);  /*key icon*/

  for (i=1; i<=items[2] && i<11; i++)
    drawchar (26+i,8,29);  /*potion icon*/

  for (i=1; i<=items[3] && i<11; i++)
    drawchar (26+i,9,30);  /*scroll icon*/

  for (i=1; i<=items[5] && i<11; i++)
    drawchar (26+i,10,30);  /*scroll icon*/
}


/*================================*/
/*				  */
/* playsetup                      */
/* set up all data for a new game */
/* does not start it playing      */
/*				  */
/*================================*/

void playsetup()
{
  int i;

  shotpower=0;
  bar (0,0,23,23,0);

  if (level==0)			// not restarting a saved game
  {
    for (i=1; i<6; i++)
      items[i]=0;
    score=0;
    level=1;
    o[0].active = true;
    o[0].class = player;
    o[0].hp = 13;
    o[0].dir=west;
    o[0].stage=0;
    o[0].delay=0;

    drawside ();
  /*give them a few items to start with*/

    givenuke();
    givenuke();
    givebolt();
    givebolt();
    givebolt();
    givepotion();
    givepotion();
    givepotion();
  }
  else
    drawside ();

};



/*
=============
=
= repaintscreen
=
=============
*/

void repaintscreen ()
{
  switch (gamestate)
  {
    case intitle:
      drawpic (0,0,TITLEPIC);
      break;
    case ingame:
      restore ();
      drawside ();
      printscore ();
      sx=33;                  /*print the new level number on the right window*/
      sy=1;
      printint (level);
      break;
    case inscores:
      restore ();
      drawside ();
      printscore ();
      sx=33;                  /*print the new level number on the right window*/
      sy=1;
      printint (level);
      indemo = demoplay;
      break;
    default:
      sx=sy=10;
      print ("Bad gamestate!");
      clearkeys ();
      get();
  }

}


/*
=============
=
= dofkeys
=
= Checks to see if an F-key is being pressed and handles it
=
=============
*/

void dofkeys (void)
{
  int handle;
  int key=bioskey(1);
  if (key==SDL_SCANCODE_ESCAPE)			// make ESC into F10
    key=SDL_SCANCODE_F10;
  if (key<SDL_SCANCODE_F1 || key>SDL_SCANCODE_F10)
    return;

  switch (key)
  {
    case SDL_SCANCODE_F1:			// F1
      clearkeys ();
      help ();
      break;
    case SDL_SCANCODE_F2:          	// F2
      clearkeys ();
      controlpanel ();
      break;
    case SDL_SCANCODE_F3:			// F3
      clearkeys ();
      expwin (18,1);
      print ("RESET GAME (Y/N)?");
      ch=toupper(get());
      if (ch=='Y')
	resetgame = true;
      break;

    case SDL_SCANCODE_F4:			// F4
      clearkeys ();
      expwin (22,4);
      if (indemo != notdemo)
      {
	print ("Can't save game here!");
	get ();
	break;
      }
      print ("Save as game #(1-9):");
      ch=toupper(get());
      drawchar (sx,sy,ch);
      if (ch<'1' || ch>'9')
	break;
      //
      // save game
      //
      strcpy (str,"GAME0.CA2");
      str[4]=ch;
      if (_Verify(str))
      {
	print ("\nGame exists,\noverwrite (Y/N)?");
	ch=get();
	if (ch!='Y' && ch!='y')
	  break;
	sx=leftedge;
	print ("                    ");
	sy--;
	sx=leftedge;
	print ("                    ");
	sx=leftedge;
	sy--;
      }
      if ((handle = open(str, O_WRONLY | O_BINARY | O_CREAT | O_TRUNC, S_IREAD | S_IWRITE)) == -1)
	return;
      write(handle, &saveitems, sizeof(items));
      write(handle, &savescore, sizeof(score));
      write(handle, &level, sizeof(level));
      write(handle, &saveo[0],sizeof(o[0]));

      close(handle);
      print ("\nGame saved.  Hit F5\n");
      print ("when you wish to\n");
      print ("restart the game.");
      get();
      break;

    case SDL_SCANCODE_F5:			// F5
      clearkeys ();
      expwin (22,4);
      print ("Load game #(1-9):");
      ch=toupper(get());
      drawchar (sx,sy,ch);
      if (ch<'1' || ch>'9')
	break;
      //
      // load game
      //
      strcpy (str,"GAME0.CA2");
      str[4]=ch;
      if ((handle = open(str, O_RDONLY | O_BINARY, S_IWRITE | S_IREAD)) == -1)
      {
	print ("\nGame not found.");
	get ();
	break;
      }
      read(handle, &items, sizeof(items));
      read(handle, &score, sizeof(score));
      read(handle, &level, sizeof(level));
      read(handle, &o[0],sizeof(o[0]));
      close(handle);
      exitdemo = true;
      if (indemo != notdemo)
	playdone = true;
      drawside ();		// draw score, icons, etc
      leveldone = true;
      break;
    case SDL_SCANCODE_F9:			// F9
      clearkeys ();
      expwin (7,1);
      print ("PAUSED");
      get ();
      break;
    case SDL_SCANCODE_F10:			// F10
      clearkeys ();
      expwin (12,1);
      print ("QUIT (Y/N)?");
      ch=toupper(get());
      if (ch=='Y')

	_quit ((char*)"");
      break;

    default:
      return;
  }

  clearold ();
  clearkeys ();
  repaintscreen ();
}



/*=========================================================================*/

/*
=============
=
= dotitlepage
=
=============
*/

void dotitlepage (void)
{
  int i;
  drawpic (0,0,TITLEPIC);
  UpdateScreen();

  gamestate=intitle;
  for (i=0;i<300;i++)
  {
    WaitVBL ();
    indemo = notdemo;
    ctrl = ControlPlayer (1);
    if (ctrl.button1 || ctrl.button2 || keydown[SDL_SCANCODE_SPACE])
    {
      level = 0;
      exitdemo = true;
      break;
    }
    indemo = demoplay;
    if (bioskey (1))
    {
      dofkeys ();
      UpdateScreen();
    }
    if (exitdemo)
      break;
  }
  gamestate=ingame;
}


/*=========================================================================*/

/*
=============
=
= doendpage
=
=============
*/

void doendpage (void)
{
  WaitEndSound();
  drawpic (0,0,ENDPIC);
  PlaySound (TREASURESND);
  WaitEndSound();
  PlaySound (TREASURESND);
  WaitEndSound();
  PlaySound (TREASURESND);
  WaitEndSound();
  PlaySound (TREASURESND);
  WaitEndSound();

  drawwindow (0,0,17,9);
  print ("Congratulation! \n");
  print ("One as skilled  \n");
  print ("as yourself     \n");
  print ("deserves the    \n");
  print ("10,000,000 gold \n");
  print ("you pulled out  \n");
  print ("of the palace! ");
  clearkeys();
  get ();
  drawwindow (0,0,17,9);
  print ("Let us know what\n");
  print ("you enjoyed     \n");
  print ("about this game,\n");
  print ("so we can give  \n");
  print ("you more of it. \n");
  print ("Thank you for   \n");
  print ("playing!");
  get ();

}


/*=========================================================================*/


/*
=============
=
= dodemo
=
= Shows a random demo
=
=============
*/

void dodemo (void)
{
  int i;

  while (!exitdemo)
  {
    dotitlepage ();

    if (exitdemo)
      break;

    i=rnd(NUMDEMOS-1)+1;
    LoadDemo (i);
    level=0;
    playsetup ();
    playloop ();
    if (exitdemo)
      break;

    level = 0;
    gamestate=inscores;
    indemo = demoplay;
    _showhighscores ();
	UpdateScreen();
    for (i=0;i<500;i++)
    {
      WaitVBL ();
      indemo = notdemo;
      ctrl = ControlPlayer (1);
      if (ctrl.button1 || ctrl.button2 || keydown[SDL_SCANCODE_SPACE])
      {
        exitdemo = true;
        break;
      }
      if (bioskey (1))
        dofkeys ();
      if (exitdemo)
        break;
    }

  }
}

/*=========================================================================*/

/*
============
=
= gameover
=
= End game, check for high score
=
============
*/

void gameover (void)
{
  int i;

  expwin (11,4);
  print ("\n GAME OVER\n     ");
  UpdateScreen ();
  WaitEndSound ();
  for (i=0;i<120;i++)
    WaitVBL ();
  gamestate=inscores;
  _checkhighscore ();
  level = 0;
  for (i=0;i<500;i++)
  {
	 WaitVBL ();
	 ctrl = ControlPlayer (1);
	 if (ctrl.button1 || ctrl.button2 || keydown[SDL_SCANCODE_SPACE])
		break;
	 if (bioskey (1))
		dofkeys ();
	 if (exitdemo || indemo == demoplay)
		break;
  }
}


/***************************************************************************/
/***************************************************************************/

///////////////////////////////////////////////////////////////////////////
//
//	US_CheckParm() - checks to see if a string matches one of a set of
//		strings. The check is case insensitive. The routine returns the
//		index of the string that matched, or -1 if no matches were found
//
///////////////////////////////////////////////////////////////////////////
int
US_CheckParm(char *parm,char **strings)
{
	char	cp,cs,
			*p,*s;
	int		i;

	while (!isalpha(*parm))	// Skip non-alphas
		parm++;

	for (i = 0;*strings && **strings;i++)
	{
		for (s = *strings++,p = parm,cs = cp = 0;cs == cp;)
		{
			cs = *s++;
			if (!cs)
				return(i);
			cp = *p++;

			if (isupper(cs))
				cs = tolower(cs);
			if (isupper(cp))
				cp = tolower(cp);
		}
	}
	return(-1);
}

/***************************************************************************/
/***************************************************************************/

/*=========================*/
/*			   */
/* m a i n   p r o g r a m */
/*			   */
/*=========================*/

int _argc;
char** _argv;
int main (int argc, char* argv[])
{
	int i;

	_argc = argc;
	_argv = argv;

	if (_argc > 1 && stricmp(_argv[1], "/VER") == 0)
	{
		printf(
			"CatacombSDL\n"
			"Version 1.03\n"
			"\n"
			"Usage: catacomb [windowed <width> <height>] [screen <num>]\n"
			"\n"
			"Ported by Braden \"Blzut3\" Obrzut and Rene \"Havoc\" Nicolaus\n"
			"Includes PC Speaker emulator by K1n9_Duk3\n"
			"Based on The Catacomb source code:\n"
			"Copyright 1990-1993 Softdisk Publishing\n"
			"Copyright 1993-2014 Flat Rock Software\n");
		exit(0);
	}

  initobjects();

  memset (&priority,99,sizeof(priority));

  priority[blankfloor]=0;
  for (i=objdef[teleporter].firstchar; i<=objdef[teleporter].firstchar+20;i++)
	 priority[i]=0;
  for (clvar=dead2; clvar<=dead5; clvar++)
    for (i=objdef[clvar].firstchar; i<=objdef[clvar].firstchar+
    objdef[clvar].size*objdef[clvar].size; i++)
      priority[i]=0;		/*deadthing*/
  for (i=152; i<=161; i++)
    priority[i]=2;		/*shots*/
  for (i=objdef[bigshot].firstchar; i<= objdef[bigshot].firstchar + 31; i++)
    priority[i]=2;		/*bigshot*/
  for (i=0; i<=tile2s-1; i++)
    if (priority [i]==99)
      priority[i]=3;		/*most 1*1 tiles are walls, etc*/
  priority[167]=1;		// chest
  for (i=tile2s; i<=maxpics; i++)
    if (priority[i]==99)
      priority[i]=4;		/*most bigger tiles are monsters*/
  for (i=objdef[player].firstchar; i<= objdef[player].firstchar + 63; i++)
    priority[i]=5;		/*player*/


  side=0;

  int x, y;
  for (x=0; x<=85; x++)
    {
      for (y=0; y<=topoff-1; y++)
	{
	  view[x][y]=solidwall;
	  view[x][85-y]=solidwall;
	  background[x][y]=solidwall;
	  background[x][85-y]=solidwall;
	};
		view[86][x]=solidwall;
	 };
  for (y=11; y<=74; y++)
	 for (x=0; x<=leftoff-1; x++)
		{
	view[x][y]=solidwall;
	view[85-x][y]=solidwall;
	background[x][y]=solidwall;
	background[85-x][y]=solidwall;
		};


//   puts ("CATACOMB II is executing");

//  _dontplay = 1;	// no sounds for debugging and profiling

  _numlevels = 30;
  _maxplayers = 1;

  _extension = "CA2";

  _setupgame ();

  expwin (33,13);
  print ("  Softdisk Publishing presents\n\n");
  print ("          The Catacomb\n\n");
  print ("        By John Carmack\n\n");
  print ("       Copyright 1990-93\n");
  print ("      Softdisk Publishing");
  print ("\n\n");
  print ("\n\n");
  print ("         Press a key:");
  get();

  clearkeys ();

  screencentery = 11;
  screencenterx = 11;

  exitdemo = false;
  level = 0;

  while (1)			// go until quit () is called
  {
	 dodemo ();
	 playsetup ();
	 indemo = notdemo;
	 gamestate = ingame;
	 playloop ();
	 if (!indemo)
	 {
		exitdemo = false;
		if (level > numlevels)
	doendpage ();		// finished all levels
		gameover ();
	 }
  }
  
  return 0;
}

