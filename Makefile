sakura: set-sakura-env

vagrant: set-vagrant-env

set-sakura-env:
	ln -snf ./submodules/local-git/.env.sakura ./.env

set-vagrant-env:
	ln -snf ./submodules/local-git/.env.vagrant ./.env
