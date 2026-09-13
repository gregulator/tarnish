D=`date "+%Y%m%d_%H%M%S"`

mkdir -p _out_backup/$D
mv _out/* _out_backup/$D
(cd tarnish && cargo run --bin=supercube_a_frontpanel_main > ../_out/supercube_a_frontpanel_$D.dxf)
(cd tarnish && cargo run --bin=supercube_b_cavity_main > ../_out/supercube_b_cavity_$D.dxf)
(cd tarnish && cargo run --bin=supercube_c_tweetermount_main > ../_out/supercube_c_tweetermount_$D.dxf)
(cd tarnish && cargo run --bin=supercube_f_backpanel_main > ../_out/supercube_f_backpanel_$D.dxf)
