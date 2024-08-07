FROM node:18.17.1

COPY . /app
WORKDIR /app 

RUN npm install -g serve
RUN npm install
RUN npm run react-build

EXPOSE 4000
CMD [ "serve", "-s", "build", "-l", "4000" ]