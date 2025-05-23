cmake_minimum_required(VERSION 3.16)

project(artchive VERSION 0.1 LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
SET(AUTORCC_OPTIONS "/main.qrc")

find_package(Qt6 6.2 COMPONENTS Quick REQUIRED)
find_package(Qt6 6.2 COMPONENTS Sql REQUIRED)

qt_add_executable(appartchive
    main.cpp
)

qt_add_qml_module(appartchive
    URI artchive
    VERSION 1.0
    QML_FILES main.qml
    QML_FILES Explorer.qml
    RESOURCE_PREFIX /
    RESOURCES main.qrc
    SOURCES designfiles.h designfiles.cpp
)


# Qt for iOS sets MACOSX_BUNDLE_GUI_IDENTIFIER automatically since Qt 6.1.
# If you are developing for iOS or macOS you should consider setting an
# explicit, fixed bundle identifier manually though.
set_target_properties(appartchive PROPERTIES
#    MACOSX_BUNDLE_GUI_IDENTIFIER com.example.appartchive
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_link_libraries(appartchive
    PRIVATE Qt6::Quick
    PRIVATE Qt6::Sql
)

include(GNUInstallDirs)
install(TARGETS appartchive
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
