import React, { useState, useRef, useImperativeHandle, forwardRef } from "react";
import "./../../../css/Popup.css"; // We'll move the styles here

const Popup = forwardRef(({ message: defaultMessage = "Saved!", type: defaultType = "", duration = 3000 }, ref) => {
    const [visible, setVisible] = useState(false);
    const [message, setMessage] = useState(defaultMessage);
    const [type, setType] = useState(defaultType);
    const timeoutRef = useRef(null);

    // Expose the show function to parent components
    useImperativeHandle(ref, () => ({
        show(newMessage = defaultMessage, newType = defaultType) {
            setMessage(newMessage);
            setType(newType);
            setVisible(true);

            // Clear any existing timeout
            if (timeoutRef.current) clearTimeout(timeoutRef.current);

            timeoutRef.current = setTimeout(() => {
                setVisible(false);
            }, duration);
        }
    }));

    return (
        <div className={`popup ${type} ${visible ? "visible" : ""}`}>
            {message}
        </div>
    );
});

export default Popup;
