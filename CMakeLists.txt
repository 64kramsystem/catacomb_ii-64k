cmake_minimum_required(VERSION 2.8)
project(TheCatacomb)

include(CheckFunctionExists)
if(NOT WIN32)
	include(FindPkgConfig)
endif(NOT WIN32)

check_function_exists(stricmp STRICMP_EXISTS)
check_function_exists(strnicmp STRNICMP_EXISTS)
if(NOT STRICMP_EXISTS)
	add_definitions(-Dstricmp=strcasecmp)
endif(NOT STRICMP_EXISTS)
if(NOT STRNICMP_EXISTS)
	add_definitions(-Dstrnicmp=strncasecmp)
endif(NOT STRNICMP_EXISTS)

if(WIN32)
	set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/CMake/;${CMAKE_MODULE_PATH}")
	find_package(SDL2)
	include_directories(${SDL2_INCLUDE_DIR})
else(WIN32)
	if(UNIX)
		pkg_search_module(SDL2 REQUIRED sdl2)
		include_directories(${SDL2_INCLUDE_DIRS})
	endif(UNIX)
endif(WIN32)


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

if(WIN32)
	target_link_libraries(catacomb ${SDL2_LIBRARY})
else(WIN32)
	if(UNIX)
		target_link_libraries(catacomb ${SDL2_LIBRARIES})
	endif(UNIX)
endif(WIN32)
