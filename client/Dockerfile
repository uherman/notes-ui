FROM node:20-alpine AS build-env

WORKDIR /app

COPY . ./

RUN npm install
RUN npm run build

FROM node:20-alpine

WORKDIR /app

COPY --from=build-env /app/build .
COPY --from=build-env /app/package.json .
COPY --from=build-env /app/node_modules ./node_modules

EXPOSE 3000

CMD ["node", "index.js"]