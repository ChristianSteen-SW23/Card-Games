import ScreenController from "./ScreenController";
import Popup from "./components/subComponents/helperComponents/Popup";
import { useRef, useEffect } from "react";
import { getSetting } from "./js/settings";
import { setPopupRef, showPopup } from "./js/popupController";

function App() {
    const popupRef = useRef();

    useEffect(() => {
        setPopupRef(popupRef.current);
        //showPopup("TESTTTTTTTTT", "success");
    }, []
    );
    return (
        <>
            <div className="position-relative min-vh-100">
                <ScreenController />
            </div>
            <Popup ref={popupRef} duration={getSetting("popupTime")} />
        </>
    );
}

export default App;
