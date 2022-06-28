FROM node:14-alpine AS frontend

WORKDIR /app
COPY package.json .
COPY package-lock.json .
RUN yarn install --production
COPY . .
RUN npm run build

FROM nginx:1.23-alpine
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=frontend /app/build /usr/share/nginx/html
