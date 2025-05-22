import QtQuick
import QtQuick.Controls.Fusion

Window {
    title: qsTr("Artchive")
    visible: true
    minimumWidth: 640
    minimumHeight: 480
    maximumWidth: 640
    maximumHeight: 480

    MouseArea {
        anchors.fill: parent
        onClicked: forceActiveFocus()
    }

    Text {
        id: working_directory_heading
        text: qsTr("Working Directory: ")
        x: 25
        y: 15
        font.bold: true
    }
    TextEdit {
        id: current_directory
        text: designFiles.dir_path
        readOnly: true
        anchors {
            top: working_directory_heading.top
            left: working_directory_heading.right
        }
    }

    property string init_dir_path: ""

    TextField {
        id: design_path_id
        placeholderText: qsTr("/design/path")
        // renderType: Text.QtRendering
        onTextChanged: init_dir_path = text
        Keys.onReturnPressed: designFiles.set_dir_path(init_dir_path)
        x: 25
        y: 50
        width: 300
        color: "#000"
        placeholderTextColor: design_path_id.activeFocus ? "transparent" : "#535353"
        font.pointSize: 14
        background: Rectangle {
            width: parent.width
            height: parent.height
            color: "#EAEAEA"
            border.width: 1
            border.color: design_path_id.activeFocus ? "#000" : "#FFF"
            radius: 3
        }
    }
    Button {
        id: submit_button
        onClicked: designFiles.set_dir_path(init_dir_path)
        x: 25
        y: 85
        topPadding: 5
        rightPadding: 15
        bottomPadding: 5
        leftPadding: 15
        contentItem: Text {
            text: "Submit"
            color: "#000"
            font.bold: true
        }
        background: Rectangle {
            width: parent.width
            height: parent.height
            color: '#26A4F4'
            opacity: submit_button.down ? 0.75 : (submit_button.hovered ? 0.5 : 0.25)
            radius: 3
        }
    }
    Explorer {
        anchors.centerIn: parent
    }
    Image {
        id: icon_badge
        source: "qrc:/assets/icons/icon-chive.png"
        anchors {
            right: parent.right
            bottom: parent.bottom
        }
        width: 50
        height: 50
    }
}
