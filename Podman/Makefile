.PHONY: ssh_config build_image stop_server start_server tests clean connect

ssh/podman_ssh_id:
	mkdir -p ssh
	ssh-keygen -f ssh/podman_ssh_id -t ed25519 -P ''
	cp ssh/podman_ssh_id ~/.ssh/
	cp ssh/podman_ssh_id.pub ~/.ssh/
	ssh-keygen -R "[localhost]:2222"

ssh_config:
	@grep -qxF 'Host localhost' ~/.ssh/config \
	   || /usr/bin/echo -ne "\n# RYST START\nHost localhost\n   IdentityFile ~/.ssh/podman_ssh_id\n   User myuser\n   port 2222\n# RYST STOP\n" \
	   >> ~/.ssh/config

build_image: ssh/podman_ssh_id
	@if [ -z "$$(podman images -q localhost/testing)" ]; then \
	   echo "Image localhost/testing does not exist: building..."; \
	   podman build -t testing . ; \
	else \
	   echo "Podman image already created"; \
	fi

stop_server:
	@podman stop ryst_testing 2> /dev/null || echo "Server ryst_testing not running"

start_server: build_image stop_server
	podman run --name ryst_testing -id --rm --publish 2222:2222 localhost/testing
	@grep -qxF '[localhost]:2222' ~/.ssh/known_hosts \
	   || ssh-keyscan -t ed25519 -p 2222 localhost >> ~/.ssh/known_hosts

tests: ssh_config start_server
	cd ../tests/data/  || echo "Unable to cd to ~/tests/data folder"
	# ssh myuser@localhost -p 2222 ls -al || exit 1  # Cheap smoke test
	cargo test --test integration_tests -- --nocapture || echo "Test Failed"
	$(MAKE) stop_server
	ssh-keygen -R "[localhost]:2222"

clean: stop_server
	# Remove the extra ssh keys
	@ssh-keygen -R "[localhost]:2222" || echo "known_hosts already clean"
	rm ~/.ssh/podman_ssh_id || echo "cleaned ~/.ssh/podman_ssh_id"
	rm ~/.ssh/podman_ssh_id.pub || echo "cleaned ~/.ssh/podman_ssh_id.pub"
	@rm -rf ssh || echo "ssh folder already gone"
	# Remove the Podman container and image
	@podman rmi -i localhost/testing || echo "Testing image already gone"
	# Remove the extra entries in ~/.ssh/config
	@sed -i '/# RYST START/,/# RYST END/d' ~/.ssh/config || echo "cleanup ~/.ssh/config"
	@sed -i '$${/^$$/d;}' ~/.ssh/config || echo "cleanup ~/.ssh/config"

connect:
	podman exec -it ryst_testing /bin/sh
