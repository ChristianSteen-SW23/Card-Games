FROM node:21-alpine

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install


EXPOSE 3069

CMD ["node", "index.js"]
