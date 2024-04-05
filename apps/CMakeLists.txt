cmake_minimum_required(VERSION 3.16)

project(
  ccipher
  DESCRIPTION "Caesar Cipher Encryption/Decryption Driver"
  LANGUAGES CXX)

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME} PRIVATE ccipher.cpp)

target_link_libraries(${PROJECT_NAME} PUBLIC cipher)

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION ${CC_BIN_DIR})
