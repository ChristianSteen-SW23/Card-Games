FROM node:21-alpine

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

COPY . .  
COPY .env .env  # Copy environment variables

EXPOSE 3069

CMD ["node", "index.js"]
