

FROM node:10

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

COPY . .

EXPOSE 3000

CMD ["npm", "start"]



#FROM node:latest as build-stage
#WORKDIR /app
#COPY package*.json ./
#RUN npm install
#COPY ./ .
#RUN npm run build
#
#FROM nginx:latest as production-stage
#RUN mkdir /app
#COPY --from=build-stage /app/dist /app
#COPY nginx.conf /etc/nginx/nginx.conf
