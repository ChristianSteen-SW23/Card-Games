import { useState } from "react";
import { socket } from "./../../socket";

export default function LobbyID( { roomID } ) {
    const copyToClipboard = () => {      
        navigator.clipboard.writeText(roomID)
          .then(() => {
          })
          .catch(err => {
            console.error('Failed to copy text: ', err);
          });
      };

    return (
        <>
            <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={copyToClipboard}>
                Lobby ID: {roomID} <span className="badge text-bg-secondary">Click to copy</span>
            </button>
        </>
    );
}