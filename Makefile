sakura: set-sakura-env

vagrant: set-vagrant-env

travis: set-travis-env

set-sakura-env:
	ln -snf ./submodules/local-git/.env.sakura ./.env

set-vagrant-env:
	ln -snf ./submodules/local-git/.env.vagrant ./.env

set-travis-env:
	ln -snf ./.env.travis ./.env
