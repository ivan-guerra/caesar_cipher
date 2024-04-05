cmake_minimum_required(VERSION 3.16)

project(
  cipher
  DESCRIPTION "Caesar Cipher Encrypt/Decrypt Function"
  LANGUAGES CXX)

add_library(${PROJECT_NAME} STATIC)

target_sources(${PROJECT_NAME} PRIVATE cipher.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC ${CC_INCLUDE_DIR})
