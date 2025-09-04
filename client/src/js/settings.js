import { GAME_MODES } from "./gameModes";

const SETTINGS_KEY = "userSettings";


const DEFAULT_SETTINGS = {
    popupTime: 5000,
    theme: "light",
    fourColours: true,
    defaultGame: GAME_MODES[0],
};

let cachedSettings = null;
function getSettings() {
    if (cachedSettings) return cachedSettings;
    const stored = localStorage.getItem(SETTINGS_KEY);
    if (!stored) {
        localStorage.setItem(SETTINGS_KEY, JSON.stringify(DEFAULT_SETTINGS));
        return { ...DEFAULT_SETTINGS };
    }
    try {
        cachedSettings = JSON.parse(stored);
        return cachedSettings;
    } catch {
        console.error("Could not read settings")
        return { ...DEFAULT_SETTINGS };
    }
}

function getSetting(key) {
    const settings = getSettings();
    return settings[key];
}

function updateSetting(key, value) {
    const settings = getSettings();
    settings[key] = value;
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
    cachedSettings = settings;
}

function resetSettings() {
    cachedSettings = { ...DEFAULT_SETTINGS };
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(DEFAULT_SETTINGS));
}

export {
    getSettings,
    getSetting,
    updateSetting,
    resetSettings,
    DEFAULT_SETTINGS
};
