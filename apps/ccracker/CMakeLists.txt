cmake_minimum_required(VERSION 3.16)

project(
  ccracker
  DESCRIPTION "Caesar Cipher Cracker Driver"
  LANGUAGES CXX)

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME} PRIVATE ccracker.cpp)

target_link_libraries(${PROJECT_NAME} PUBLIC cracker)

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION ${CC_BIN_DIR})
