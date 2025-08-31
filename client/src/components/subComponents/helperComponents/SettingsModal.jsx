import { useEffect, useState } from "react";
import { getSettings, updateSetting, DEFAULT_SETTINGS } from "./../../../js/settings";

export default function SettingsModal({ show, onClose }) {
    const [settings, setSettings] = useState(DEFAULT_SETTINGS);

    // Load settings into local state when modal opens
    useEffect(() => {
        if (show) {
            setSettings(getSettings());
        }
    }, [show]);

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        const newValue = type === "checkbox" ? checked : value;
        setSettings((prev) => ({ ...prev, [name]: newValue }));
    };

    const handleSave = () => {
        for (const [key, value] of Object.entries(settings)) {
            updateSetting(key, value);
        }
        onClose(); // close modal after saving
    };

    if (!show) return null;

    return (
        <div className="modal d-block" tabIndex="-1" style={{ background: "rgba(0,0,0,0.5)" }}>
            <div className="modal-dialog modal-dialog-centered">
                <div className="modal-content">
                    <div className="modal-header">
                        <h5 className="modal-title">Settings</h5>
                        <button type="button" className="btn-close" onClick={onClose}></button>
                    </div>
                    <div className="modal-body">
                        {/* Theme select */}
                        <div className="mb-3">
                            <label htmlFor="theme" className="form-label">Theme</label>
                            <select
                                className="form-select"
                                id="theme"
                                name="theme"
                                value={settings.theme}
                                onChange={handleChange}
                            >
                                <option value="light">Light</option>
                                {/* <option value="dark">Dark</option> */}
                            </select>
                        </div>

                        {/* Popup time */}
                        <div className="mb-3">
                            <label htmlFor="popupTime" className="form-label">Popup Time (ms)</label>
                            <input
                                type="number"
                                className="form-control"
                                id="popupTime"
                                name="popupTime"
                                value={settings.popupTime}
                                onChange={handleChange}
                                min={1000}
                                step={1000}
                            />
                        </div>

                        {/* Four colours toggle */}
                        <div className="form-check">
                            <input
                                className="form-check-input"
                                type="checkbox"
                                id="fourColours"
                                name="fourColours"
                                checked={settings.fourColours}
                                onChange={handleChange}
                            />
                            <label className="form-check-label" htmlFor="fourColours">
                                Use Four-Coloured Cards
                            </label>
                        </div>
                    </div>

                    <div className="modal-footer">
                        <button className="btn btn-secondary" onClick={onClose}>Cancel</button>
                        <button className="btn btn-primary" onClick={handleSave}>Save</button>
                    </div>
                </div>
            </div>
        </div>
    );
}
