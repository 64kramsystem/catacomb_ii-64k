cmake_minimum_required(VERSION 2.8)
project(TheCatacomb)

include(CheckFunctionExists)
include(FindPkgConfig)

check_function_exists(stricmp STRICMP_EXISTS)
check_function_exists(strnicmp STRNICMP_EXISTS)
if(NOT STRICMP_EXISTS)
	add_definitions(-Dstricmp=strcasecmp)
endif(NOT STRICMP_EXISTS)
if(NOT STRNICMP_EXISTS)
	add_definitions(-Dstrnicmp=strncasecmp)
endif(NOT STRNICMP_EXISTS)

pkg_search_module(SDL2 REQUIRED sdl2)
include_directories(${SDL2_INCLUDE_DIRS})

add_executable(catacomb
	cat_play.c
	catacomb.c
	catasm.c
	cpanel.c
	objects.c
	pcrlib_a.c
	pcrlib_c.c
	rleasm.c
)

target_link_libraries(catacomb ${SDL2_LIBRARIES})
