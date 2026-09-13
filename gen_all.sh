D=`date "+%Y%m%d_%H%M%S"`

mkdir -p _out_backup/$D
mv _out/* _out_backup/$D
(cd tarnish && cargo run --bin=air_wooftrim_main > ../_out/air_wooftrim_$D.dxf)
(cd tarnish && cargo run --bin=air_frgasket_main > ../_out/air_frgasket_$D.dxf)
(cd tarnish && cargo run --bin=air_frtrim_main > ../_out/air_frtrim_$D.dxf)
(cd tarnish && cargo run --bin=air_woofgasket_main > ../_out/air_woofgasket_$D.dxf)
(cd tarnish && cargo run --bin=air_stand_main > ../_out/air_stand_$D.dxf)
(cd tarnish && cargo run --bin=air_baffle_main > ../_out/air_baffle_$D.dxf)
(cd tarnish && cargo run --bin=air_xovercover_main > ../_out/air_xovercover_$D.dxf)
(cd tarnish && cargo run --bin=air_standgasket_main > ../_out/air_standgasket_$D.dxf)
