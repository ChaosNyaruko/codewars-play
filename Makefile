TITLE:=""
CONST:="pub mod "
MOD:="$(CONST)$(TITLE);"
new:
	touch ./src/$(TITLE).rs && echo $(MOD) >> src/lib.rs
