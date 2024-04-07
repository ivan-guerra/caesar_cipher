cmake_minimum_required(VERSION 3.16)

project(
  cracker
  DESCRIPTION "Caesar Cipher Cracking Functions"
  LANGUAGES CXX)

add_library(${PROJECT_NAME} STATIC)

target_sources(${PROJECT_NAME} PRIVATE cracker.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC ${CC_INCLUDE_DIR})

target_link_libraries(${PROJECT_NAME} PRIVATE cipher)
