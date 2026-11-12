
export default function LobbyID({ roomID }) {
    const copyToClipboard = async () => {
        // Works on https and localhost
        if (navigator.clipboard && window.isSecureContext) {
            try {
                await navigator.clipboard.writeText(roomID);
                return;
            } catch (err) {
                console.error("Failed to copy text: ", err);
            }
        }

        // Fallback for plain http
        const textArea = document.createElement("textarea");
        textArea.value = roomID;
        textArea.style.position = "fixed";
        textArea.style.opacity = "0";
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();
        try {
            document.execCommand("copy");
        } catch (err) {
            console.error("Fallback copy failed: ", err);
        }
        document.body.removeChild(textArea);
    };

    return (
        <>
            <div className="d-grid mt-3 p-3">
                <button type="button" className="btn btn-primary btn-lg" onClick={copyToClipboard}>
                    Lobby ID: {roomID} <span className="badge text-bg-secondary">Click to copy</span>
                </button>
            </div>
        </>
    );
}
