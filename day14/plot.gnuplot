set datafile separator comma

set terminal pngcairo
set output "result.png"

plot "result.txt" u 1:2
