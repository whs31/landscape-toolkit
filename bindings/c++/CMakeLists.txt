cmake_minimum_required(VERSION 3.15)

project(LandscapeToolkit
  VERSION 1.0.0
  DESCRIPTION "Landscape Elevation Data Toolkit"
  HOMEPAGE_URL "com.radar-mms.uav"
  LANGUAGES C CXX
)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
endif()

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON CACHE BOOL "Export all symbols")

if(${CMAKE_PROJECT_NAME} STREQUAL ${PROJECT_NAME})
  if(WIN32)
    include(FetchContent)
    FetchContent_Declare(
      googletest
      URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
    )
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)
  endif()

  find_package(GTest REQUIRED)
  enable_testing()
endif()


add_subdirectory(libs/libra)
add_library(landscape_toolkit SHARED IMPORTED)
if(WIN32)
  message(STATUS "[LandscapeToolkit] Searching for .dll in ${PROJECT_SOURCE_DIR}/plugins...")
  set_target_properties(landscape_toolkit PROPERTIES IMPORTED_LOCATION
    ${PROJECT_SOURCE_DIR}/plugins/Windows/x64/landscape_toolkit.dll
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

target_sources(${PROJECT_NAME}
  PUBLIC
    include/LandscapeToolkit/LandscapeToolkit
    include/LandscapeToolkit/landscape_toolkit.h
    include/LandscapeToolkit/rust_landscape_toolkit_ffi.h
)

target_link_libraries(${PROJECT_NAME}
  PUBLIC
    Libra
    landscape_toolkit
)

target_include_directories(${PROJECT_NAME}
  PUBLIC
    include
)

if(${CMAKE_PROJECT_NAME} STREQUAL ${PROJECT_NAME})
  file(GLOB_RECURSE LED_TOOLKIT_TESTS tests/*)
  message(NOTICE "-- [LandscapeToolkit] Tests enabled")

  target_compile_definitions(${PROJECT_NAME}
    PRIVATE
      LED_TOOLKIT_TESTS
  )

  add_executable(${PROJECT_NAME}Test)

  target_sources(${PROJECT_NAME}Test
    PRIVATE
      ${LED_TOOLKIT_TESTS}
  )

  target_link_libraries(${PROJECT_NAME}Test
    PRIVATE
      GTest::GTest
      Libra
      ${PROJECT_NAME}
  )
else()
  message(STATUS "[LandscapeToolkit] Tests disabled")
endif()
