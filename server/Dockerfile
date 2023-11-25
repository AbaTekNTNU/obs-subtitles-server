FROM node:18

WORKDIR /app

COPY package.json /app
COPY yarn.lock /app
RUN yarn

COPY . /app

EXPOSE 3000

CMD ["node", "app.js"]
