target remote :3333
load
monitor tpiu config internal /tmp/itm.fifo uart off 168000000
monitor itm port 0 on
break main
continue