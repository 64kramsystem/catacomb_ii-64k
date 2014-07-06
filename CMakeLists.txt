cmake_minimum_required(VERSION 2.8)
project(TheCatacomb)

include(CheckFunctionExists)

check_function_exists(stricmp STRICMP_EXISTS)
check_function_exists(strnicmp STRNICMP_EXISTS)
if(NOT STRICMP_EXISTS)
	add_definitions(-Dstricmp=strcasecmp)
endif(NOT STRICMP_EXISTS)
if(NOT STRNICMP_EXISTS)
	add_definitions(-Dstrnicmp=strncasecmp)
endif(NOT STRNICMP_EXISTS)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/CMake/;${CMAKE_MODULE_PATH}")
find_package(SDL2)
include_directories(${SDL2_INCLUDE_DIR})

if(MSVC)
	add_definitions("/wd4996")
endif(MSVC)

# Typically reasonable to assume that an h file has a c/cpp file with it, so
# we can take the easy way and glob them.
file(GLOB HEADER_FILES *.h)

add_executable(catacomb
	${HEADER_FILES}
	cat_play.c
	catacomb.c
	catasm.c
	cpanel.c
	objects.c
	pcrlib_a.c
	pcrlib_c.c
	rleasm.c
)

target_link_libraries(catacomb ${SDL2_LIBRARY})
