.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

MODULES := service pretorius tasks webapp

build:
	for module in $(MODULES); do\
		$(MAKE) build-$$module;\
	done

build-service:
	$(MAKE) -C service

build-pretorius:
	$(MAKE) -C pretorius

build-tasks:
	$(MAKE) -C tasks

build-webapp:
	$(MAKE) -C webapp

install:
	for module in $(MODULES); do\
		$(MAKE) install-$$module;\
	done


install-pretorius:
	mkdir -p $(INSTALL_DIR)/pretorius
	make -C pretorius install
	chmod -R 755 $(INSTALL_DIR)/pretorius
	chown root:prodo $(INSTALL_DIR)/pretorius/.env
	chmod 740 $(INSTALL_DIR)/pretorius/.env

install-tasks:
	mkdir -p $(INSTALL_DIR)/tasks
	make -C tasks install
	chmod -R 755 $(INSTALL_DIR)/tasks
	chown root:prodo $(INSTALL_DIR)/tasks/.env
	chmod 740 $(INSTALL_DIR)/tasks/.env

install-webapp:
	mkdir -p $(INSTALL_DIR)/webapp
	$(MAKE) -C webapp install

install-service:
	$(MAKE) -C service install
	
	# NGINX configuration
	cp -u config/nginx.conf /etc/nginx/sites-available/prodo.phire.cz
	if [ ! -f /etc/nginx/sites-enabled/prodo.phire.cz ]; then\
		ln -s /etc/nginx/sites-available/prodo.phire.cz /etc/nginx/sites-enabled/prodo.phire.cz;\
	fi
	service nginx reload

	# Custom configuration
	# TODO -> copy uWSGI to /etc/prodo

	# Final messages
	@echo "=== After install ==="
	@echo "Apply migrations, please."
	# @echo "Make sure that cron job '30 2 * * * python manage.py createdailytasks' for user is set for user 'prodo'"


