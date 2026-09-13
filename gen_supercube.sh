D=`date "+%Y%m%d_%H%M%S"`

mkdir -p _out_backup/$D
mv _out/* _out_backup/$D
(cd tarnish && cargo run --bin=supercube_b_cavity_main > ../_out/supercube_b_cavity_main_$D.dxf)
