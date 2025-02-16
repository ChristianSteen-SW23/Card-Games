FROM node:21-alpine

WORKDIR /app

ARG VITE_FRONT_SOCKET_IP

ENV VITE_FRONT_SOCKET_IP=${VITE_FRONT_SOCKET_IP}

COPY package.json package-lock.json ./

RUN npm install

COPY . .

RUN npm run build

EXPOSE 8081

CMD ["npm", "run", "preview", "--", "--port", "8081", "--host"]
