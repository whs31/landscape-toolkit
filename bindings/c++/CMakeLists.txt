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

add_executable(${PROJECT_NAME} main.cpp)
