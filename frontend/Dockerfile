FROM node:22-alpine AS build
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
COPY --from=build /app/build ./build
COPY --from=build /app/package.json ./
ENV NODE_ENV=production PORT=3000 BODY_SIZE_LIMIT=Infinity
EXPOSE 3000
CMD ["node", "build"]
