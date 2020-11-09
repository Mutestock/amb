FROM nginx
RUN rm /etc/nginx/conf.d/default.conf
COPY ./default.conf /etc/nginx/conf.d/default.conf
COPY ./proxy_not_found.html /var/www/html/proxy_not_found.html
COPY ./includes/ /etc/nginx/includes/
COPY ./ssl/ /etc/ssl/certs/nginx/