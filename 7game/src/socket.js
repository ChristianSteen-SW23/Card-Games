/*
Note: The following document here uses guidelines from:
- https://socket.io/how-to/use-with-react
*/
import { io } from "socket.io-client";

const SOCKET_IP = import.meta.env.VITE_FRONT_SOCKET_IP || "localhost:3069";
console.log("IP", SOCKET_IP)
const URL = import.meta.env.MODE === "production" ? `http://${SOCKET_IP}` : "http://localhost:3069";

export const socket = io(URL);

/* The following might be needed when deploying to production,
as socket.io makes use of different protocols. http
*/
// export const socket = io(URL, {
//   transports: ['websocket'],
//   upgrade: false
// });
