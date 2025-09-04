/*
Note: The following document here uses guidelines from:
- https://socket.io/how-to/use-with-react
*/
import { io } from "socket.io-client";
import { showPopup } from "./js/popupController";

const SOCKET_IP = import.meta.env.VITE_FRONT_SOCKET_IP || "0.0.0.0:443";
console.log("IP", SOCKET_IP)
const URL = import.meta.env.MODE === "production" ? `http://${SOCKET_IP}` : "http://localhost:443";

export const socket = io(URL);

socket.io.on("reconnect_error", (error) => {
    showPopup(`Socket Error: ${error.message}`, "error");
})

socket.io.on("open", (connected) => {
    showPopup(`Connected to the game server`, "success");
})

/* The following might be needed when deploying to production,
as socket.io makes use of different protocols. http
*/
// export const socket = io(URL, {
//   transports: ['websocket'],
//   upgrade: false
// });
