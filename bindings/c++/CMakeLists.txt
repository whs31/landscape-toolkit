cmake_minimum_required(VERSION 3.15)

project(LandscapeToolkit
  VERSION 1.0.0
  DESCRIPTION "Landscape Elevation Data Toolkit"
  HOMEPAGE_URL "com.radar-mms.uav"
  LANGUAGES C CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON CACHE BOOL "Export all symbols")

add_subdirectory(libs/libra)
add_library(landscape_toolkit SHARED IMPORTED)
if(WIN32)
  message(STATUS "[LandscapeToolkit] Searching for .dll in ${PROJECT_SOURCE_DIR}/plugins...")
  set_target_properties(landscape_toolkit PROPERTIES IMPORTED_LOCATION
    ${PROJECT_SOURCE_DIR}/plugins/Linux/x64/landscape_toolkit.dll
  )
elseif(LINUX)
  message(STATUS "[LandscapeToolkit] Searching for .so in ${PROJECT_SOURCE_DIR}/plugins...")
  set_target_properties(landscape_toolkit PROPERTIES IMPORTED_LOCATION
    ${PROJECT_SOURCE_DIR}/plugins/Linux/x64/liblandscape_toolkit.so
  )
else()
  message(FATAL_ERROR "[LandscapeToolkit] Unknown system arch.")
endif()

add_library(${PROJECT_NAME} STATIC)
#add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME}
  PUBLIC
    include/LandscapeToolkit/LandscapeToolkit
    include/LandscapeToolkit/landscape_toolkit.h
    include/LandscapeToolkit/rust_landscape_toolkit_ffi.h
  PRIVATE
    #main.cpp
)

target_link_libraries(${PROJECT_NAME}
  PUBLIC
    Libra
  PRIVATE
    landscape_toolkit
)

target_include_directories(${PROJECT_NAME}
  PUBLIC
    include
)
