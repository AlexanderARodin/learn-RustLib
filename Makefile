libname="liblearn_RustLib.rlib"


help:
	@echo 'there is no help.. yet'

run: release size
	@./target/release/$(binname)

edit:
	@vi ./src/lib.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

release:
	#@cargo rustc --release -- -C prefer-dynamic
	@cargo rustc --release

test:
	@cargo test

size:
	@ls -lAh ./target/release/$(libname)
	

clean:
	@cargo clean
