import QtQuick

Item {
    id: explorer

    Rectangle {
        id: explorerContainer
        width: 300
        height: 200
        border.width: 1
        border.color: "#E5E5E5"
        radius: 5
        anchors.centerIn: parent

        ListView {
            model: designFiles.items
            width: parent.width
            height: parent.height
            anchors.centerIn: parent
            spacing: 25
            topMargin: 10
            rightMargin: 10
            bottomMargin: 10
            leftMargin: 10

            delegate: Item {
                width: parent.width

                TextInput {
                    text: designFiles.m_items
                    readOnly: true
                    selectByMouse: true
                    // color: designFiles.colors[index]
                }

                // To print out the file names in the console
                Component.onCompleted: {
                    console.log("File name: " + modelData);
                }
            }
        }
    }
}
