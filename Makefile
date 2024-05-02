all:
	cargo run --bin tubaitu -- rand && make video
video:
	make fes_imatges
	rm -f output.mp4
	ffmpeg -framerate 60 -i /tmp/tubaitu_svgs/images/tubaitu_snapshot__%04d.png ./output.mp4

fes_imatges:
	./to_png.sh
