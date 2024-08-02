FROM node:18.17.1

WORKDIR .
COPY . ./

RUN npm install -g serve
RUN npm isntall
RUN npm run react-build

EXPOSE 4000
CMD [ "serve", "-s", "build", "-l", "4000" ]