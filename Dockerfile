# Usa uma imagem base do Nginx
FROM nginx:latest

COPY ./dist/* /usr/share/nginx/html/

EXPOSE 80
