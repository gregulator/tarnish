D=`date "+%Y%m%d_%H%M%S"`

mkdir -p _out_backup/$D
mv _out/* _out_backup/$D
(cd tarnish && cargo run --bin=wooftrim_main > ../_out/wooftrim_$D.dxf)
(cd tarnish && cargo run --bin=frtrim_main > ../_out/frtrim_$D.dxf)
(cd tarnish && cargo run --bin=woofgasket_main > ../_out/woofgasket_$D.dxf)
(cd tarnish && cargo run --bin=stand_main > ../_out/stand_$D.dxf)
(cd tarnish && cargo run --bin=baffle_main > ../_out/baffle_$D.dxf)
