import ScreenController from "./ScreenController";
import Popup from "./components/subComponents/helperComponents/Popup";
import { useRef, useEffect } from "react";
import { setPopupRef } from "./js/popupController";
import { getSetting } from "./js/settings";

function App() {
    const popupRef = useRef();

    useEffect(() => {
        setPopupRef(popupRef.current);
        
        // Apply theme on mount and listen for storage changes
        const applyTheme = () => {
            const theme = getSetting("theme") || "light";
            document.documentElement.setAttribute("data-theme", theme);
        };
        
        applyTheme();
        
        // Listen for storage changes (when settings are updated)
        const handleStorageChange = (e) => {
            if (e.key === "userSettings") {
                applyTheme();
            }
        };
        
        window.addEventListener("storage", handleStorageChange);
        
        // Also listen for custom event for same-window updates
        const handleSettingsChange = () => {
            applyTheme();
        };
        window.addEventListener("settingsChanged", handleSettingsChange);
        
        return () => {
            window.removeEventListener("storage", handleStorageChange);
            window.removeEventListener("settingsChanged", handleSettingsChange);
        };
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
