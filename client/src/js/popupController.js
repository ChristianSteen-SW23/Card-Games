let popupRef = null;
let messageQueue = [];

export function setPopupRef(ref) {
    popupRef = ref;

    if (popupRef && popupRef.show) {
        messageQueue.forEach(({ message, type }) => popupRef.show(message, type));
        messageQueue = [];
    }
}

export function showPopup(message, type = "success") {
    if (popupRef && popupRef.show) {
        popupRef.show(message, type);
    } else {
        messageQueue.push({ message, type });
    }
}