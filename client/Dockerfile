FROM node:21-alpine

WORKDIR /app

ARG VITE_FRONT_SOCKET_IP
ARG VITE_APP_VERSION

ENV VITE_FRONT_SOCKET_IP=${VITE_FRONT_SOCKET_IP}
ENV VITE_APP_VERSION=${VITE_APP_VERSION}

COPY package.json package-lock.json ./

RUN npm install

COPY . .

RUN npm run build

EXPOSE 8081

CMD ["npm", "run", "preview", "--", "--port", "8081", "--host"]
