
genplot:
    cargo run --release --bin plot_times | grep -v INFO | grep -v "^$" > tribai_plot.data
    gnuplot "plotting_ins.txt"