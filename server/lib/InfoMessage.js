export { sendErrorMessage };

function sendErrorMessage(playerID, io, message, type) {
    io.to(playerID).emit("errorMessage", { message: message, type: type });
}