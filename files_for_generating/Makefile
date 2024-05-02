fes_imatges:
	rm -f pngs/*
	./to_png.sh
video:
	make fes_imatges
	ffmpeg -framerate 60 -i pngs/first_test__%04d.png -pix_fmt yuv420p /tmp/output.mp4
clean:
	rm images/*
	rm pngs/*
