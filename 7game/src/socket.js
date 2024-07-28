/*
Note: The following document here uses guidelines from:
- https://socket.io/how-to/use-with-react
*/
import { io } from "socket.io-client";

const URL = import.meta.env.MODE === "production" ? "http://49.13.58.73:3000" : "http://localhost:3000";

export const socket = io(URL);

/* The following might be needed when deploying to production,
as socket.io makes use of different protocols. http
*/
//export const socket = io(URL, {
//  transports: ['websocket'],
//  upgrade: false
//});
