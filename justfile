
genplot:
    cargo run --release --bin plot_times  > tribai_plot.data
    gnuplot "plotting_ins.txt"