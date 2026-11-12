export const valueColor = (value, min, max) => {
    const percent = max === min
        ? 0
        : Math.min(Math.max((value - min) / (max - min), 0), 1); // 0 to 1
    const red = Math.round(128 + percent * 127);   // 128 (grey) to 255 (red)
    const other = Math.round(128 - percent * 64);  // 128 (grey) to 64 (red)
    return `rgb(${red}, ${other}, ${other})`;
};
