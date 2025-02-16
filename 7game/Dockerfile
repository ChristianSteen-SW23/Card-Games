FROM node:21-alpine

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

COPY . .

ENV VITE_BACKEND_IP=${VITE_BACKEND_IP}

RUN npm run build

EXPOSE 8081

CMD ["npm", "run", "preview"]
