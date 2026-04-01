FROM node:24-slim

WORKDIR /app

RUN npm init -y && npm install express

COPY server.js .

EXPOSE 8080
CMD ["node", "server.js"]
