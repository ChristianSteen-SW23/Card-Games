let popupRef = null;

export function setPopupRef(ref) {
    popupRef = ref;
}

export function showPopup(message, type) {
    if (popupRef && popupRef.show) {
        popupRef.show(message, type);
    } else {
        console.warn("Popup ref is not ready yet.");
    }
}