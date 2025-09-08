import ScreenController from "./ScreenController";
import Popup from "./components/subComponents/helperComponents/Popup";
import { useRef, useEffect } from "react";
import { setPopupRef } from "./js/popupController";

function App() {
    const popupRef = useRef();

    useEffect(() => {
        setPopupRef(popupRef.current);
    }, []
    );
    return (
        <>
            <div className="position-relative min-vh-100">
                <ScreenController />
            </div>
            <Popup ref={popupRef} />
        </>
    );
}

export default App;
