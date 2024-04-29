
genplot:
    cargo run --release --bin plot_times | grep -v INFO | grep -v "^$" > tubaitu_plot.data
    gnuplot "plotting_ins.txt"