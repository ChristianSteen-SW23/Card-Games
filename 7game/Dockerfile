FROM node:21-alpine

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

# Copy the entire frontend project
COPY . .

# Copy the .env file from the build context
COPY ../.env .env

RUN npm run build

EXPOSE 8081

CMD ["npm", "run", "preview"]
