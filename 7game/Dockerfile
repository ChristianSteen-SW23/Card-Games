FROM node:21-alpine

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

COPY . .  # Copy everything inside 7game/

RUN npm run build

EXPOSE 8081

CMD ["npm", "run", "preview"]
