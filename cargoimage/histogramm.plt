#! /usr/bin/gnuplot -persist
# set terminal postscript eps enhanced color solid
# set output "Rostelecom.ps"
#set terminal png size 1024, 768
#set output "Rostelecom.png"
set style histogram clustered gap 5 title textcolor lt -1
set style data histograms
set datafile separator ';'
set grid xtics ytics
# set xdata time
# set timefmt '%d.%m.%Y;%H:%M:%S'

set ylabel "Britness"
set xlabel 'Number'
set title "Histogramma"

plot "histogramm.csv" using 2:xtic(1) ti col 
# plot "histogramm.csv" using 1:2 with lines title "Brithness"
 
